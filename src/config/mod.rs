use crate::material::Material;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::texture::Checkered;
use crate::material::texture::Image;
use crate::material::texture::Noise;
use crate::material::texture::SolidColor;
use crate::object::sphere::Sphere;
use clap::Parser;
use colored::Colorize;
use image::ImageReader;
use nalgebra::Vector3;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use std::error::Error;
use std::fmt;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of toml configuration file
    #[arg(short, long, value_parser=file_exists)]
    pub config: PathBuf,

    /// Path of file to save the render to
    #[arg(short, long, default_value = "render.png")]
    pub output: PathBuf,
}

fn file_exists(path: &str) -> Result<PathBuf, String> {
    let path_buf = PathBuf::from(path);
    if path_buf.is_file() {
        Ok(path_buf)
    } else {
        Err(format!("File does not exist: {}", path))
    }
}

#[derive(Deserialize, Debug)]
pub enum AspectRatios {
    #[serde(alias = "widescreen")]
    Widescreen,

    #[serde(alias = "square")]
    Square,

    #[serde(alias = "smartphone")]
    Smartphone,

    #[serde(alias = "standard")]
    Standard,

    #[serde(alias = "cinema")]
    Cinema,
}

impl AspectRatios {
    pub fn get_ratio(&self) -> (f32, f32) {
        match self {
            AspectRatios::Widescreen => (16.0, 9.0),
            AspectRatios::Square => (1.0, 1.0),
            AspectRatios::Smartphone => (9.0, 16.0),
            AspectRatios::Standard => (4.0, 3.0),
            AspectRatios::Cinema => (1.85, 1.0),
        }
    }

    pub fn get_height(&self, width: u32) -> u32 {
        let (ratio_x, ratio_y) = self.get_ratio();
        let ratio = ratio_x / ratio_y;
        std::cmp::max(1, (width as f32 / ratio) as u32)
    }
}

impl fmt::Display for AspectRatios {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (ratio_x, ratio_y) = self.get_ratio();
        write!(f, "{}:{}", ratio_x, ratio_y)
    }
}

#[serde_inline_default]
#[derive(Deserialize, Debug)]
pub struct CameraOptions {
    pub aspect_ratio: AspectRatios,
    pub image_width: u32,
    pub samples: u32,
    pub max_bounces: u32,
    pub threads: usize,
    pub fov: f32,

    pub look_from: [f32; 3],
    pub look_at: [f32; 3],
    pub vup: [f32; 3],

    #[serde_inline_default(0.0)]
    pub defocus_angle: f32,
    #[serde_inline_default(1.0)]
    pub focus_dist: f32,
}

impl CameraOptions {
    pub fn get_dimensions(&self) -> (u32, u32) {
        (
            self.image_width,
            self.aspect_ratio.get_height(self.image_width),
        )
    }
}

#[derive(Deserialize)]
#[serde(tag = "material", deny_unknown_fields)]
enum MaterialDef {
    #[serde(rename = "lambertian")]
    Lambertian { albedo: [f32; 3] },

    #[serde(rename = "checkered")]
    Checkered {
        even: Option<[f32; 3]>,
        odd: Option<[f32; 3]>,
        scale: Option<f32>,
    },

    #[serde(rename = "texture")]
    Texture { file: String },

    #[serde(rename = "noise")]
    Noise {
        scale: Option<f32>,
        turbulance: Option<u32>,
    },

    #[serde(rename = "metal")]
    Metal { albedo: [f32; 3], roughness: f32 },

    #[serde(rename = "dielectric")]
    Dielectric { refraction_index: f32 },

    #[serde(rename = "glass")]
    Glass {},

    #[serde(rename = "water")]
    Water {},
}

impl MaterialDef {
    fn into_material(self) -> Result<Arc<dyn Material>, Box<dyn Error>> {
        let args = Args::parse();
        let config_dir = args
            .config
            .parent()
            .ok_or("Failed to get parent directory")?;

        match self {
            MaterialDef::Lambertian { albedo } => Ok(Arc::new(Lambertian::new(Arc::new(
                SolidColor::new(Vector3::new(albedo[0], albedo[1], albedo[2])),
            )))),
            MaterialDef::Checkered { even, odd, scale } => {
                let scale = scale.unwrap_or(1.0);
                let even = even.unwrap_or([0.05, 0.05, 0.05]);
                let odd = odd.unwrap_or([0.95, 0.95, 0.95]);

                let even_color = SolidColor::new(Vector3::new(even[0], even[1], even[2]));
                let odd_color = SolidColor::new(Vector3::new(odd[0], odd[1], odd[2]));

                let checkered = Checkered::new(scale, Arc::new(even_color), Arc::new(odd_color));
                Ok(Arc::new(Lambertian::new(Arc::new(checkered))))
            }
            MaterialDef::Texture { file } => {
                let texture_path = config_dir.join(file);
                let buffer = ImageReader::open(texture_path)?.decode()?.to_rgb8();
                Ok(Arc::new(Lambertian::new(Arc::new(Image::new(buffer)))))
            }
            MaterialDef::Noise { scale, turbulance } => {
                let scale = scale.unwrap_or(1.0);
                let turbulance = turbulance.unwrap_or(1);
                Ok(Arc::new(Lambertian::new(Arc::new(Noise::new(
                    scale, turbulance,
                )))))
            }
            MaterialDef::Metal { albedo, roughness } => Ok(Arc::new(Metal::new(
                Vector3::new(albedo[0], albedo[1], albedo[2]),
                roughness,
            ))),
            MaterialDef::Dielectric { refraction_index } => {
                Ok(Arc::new(Dielectric::new(refraction_index)))
            }
            MaterialDef::Glass {} => Ok(Arc::new(Dielectric::new(1.5))),
            MaterialDef::Water {} => Ok(Arc::new(Dielectric::new(1.33))),
        }
    }
}

#[derive(Deserialize)]
struct RawSphere {
    #[serde(rename = "position")]
    center: [f32; 3],
    direction: Option<[f32; 3]>,
    radius: f32,
    #[serde(flatten)]
    material_def: MaterialDef,
}

impl RawSphere {
    fn into_sphere(self) -> Result<Sphere, Box<dyn Error>> {
        let center = Vector3::new(self.center[0], self.center[1], self.center[2]);
        let direction = match self.direction {
            None => Vector3::new(0.0, 0.0, 0.0),
            Some(direction) => Vector3::new(direction[0], direction[1], direction[2]),
        };
        let material = self.material_def.into_material()?;
        Ok(Sphere::new(center, direction, self.radius, material))
    }
}

#[derive(Deserialize)]
#[serde(tag = "shape")]
enum ObjectDef {
    #[serde(rename = "sphere")]
    Sphere(RawSphere),
}

#[derive(Debug)]
pub enum Object {
    Sphere(Sphere),
}

impl<'de> Deserialize<'de> for Object {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match ObjectDef::deserialize(deserializer)? {
            ObjectDef::Sphere(raw) => Ok(Object::Sphere(
                raw.into_sphere().map_err(serde::de::Error::custom)?,
            )),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub camera: CameraOptions,
    #[serde(default)]
    pub objects: Vec<Object>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (width, height) = self.camera.get_dimensions();
        let msg = [
            ("Dimensions", format!("{}x{}", width, height)),
            ("Aspect Ratio", format!("{}", self.camera.aspect_ratio)),
            ("Samples", format!("{}", self.camera.samples)),
            ("Max Bounces", format!("{}", self.camera.max_bounces)),
            ("Threads", format!("{}", self.camera.threads)),
            ("Field of View", format!("{}", self.camera.fov)),
            (
                "Look From",
                format!(
                    "[{:3}, {:3}, {:3}]",
                    self.camera.look_from[0].to_string(),
                    self.camera.look_from[1].to_string(),
                    self.camera.look_from[2].to_string(),
                ),
            ),
            (
                "Look At",
                format!(
                    "[{:3}, {:3}, {:3}]",
                    self.camera.look_at[0].to_string(),
                    self.camera.look_at[1].to_string(),
                    self.camera.look_at[2].to_string(),
                ),
            ),
            (
                "Vup",
                format!(
                    "[{:3}, {:3}, {:3}]",
                    self.camera.vup[0].to_string(),
                    self.camera.vup[1].to_string(),
                    self.camera.vup[2].to_string(),
                ),
            ),
            ("Defocus Angle", format!("{}", self.camera.defocus_angle)),
            ("Focus Distance", format!("{}", self.camera.focus_dist)),
            ("Objects", format!("{}", self.objects.len())),
        ]
        .map(|(k, v)| format!("│{:>14}: {:64}│", k.cyan().bold(), v))
        .join("\n");

        writeln!(
            f,
            "┌───{}{}┐",
            " Render Settings ".blue().bold(),
            "─".repeat(60)
        )?;
        writeln!(f, "{}", msg)?;
        write!(f, "└{}┘", "─".repeat(80))
    }
}

pub fn span_dump(config_content: &str, span: Range<usize>) -> String {
    let start = config_content[..span.start].lines().count();
    let end = config_content[..span.end].lines().count();

    config_content
        .lines()
        .enumerate()
        .skip_while(|(i, _)| *i < start - 1)
        .take_while(|(i, _)| *i < end)
        .map(|(i, line)| format!("{:4} | {}", (i + 1).to_string().blue(), line))
        .collect::<Vec<String>>()
        .join("\n")
}
