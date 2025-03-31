use crate::geometry::Geometry;
use crate::geometry::axis::Axis;
use crate::geometry::cube::Cube;
use crate::geometry::quad::Quad;
use crate::geometry::rotate::Rotate;
use crate::geometry::sphere::Sphere;
use crate::geometry::translate::Translate;
use crate::geometry::volume::Volume;
use crate::material::Material;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::light::Light;
use crate::material::metal::Metal;
use crate::material::texture::Checkered;
use crate::material::texture::Image;
use crate::material::texture::Noise;
use crate::material::texture::SolidColor;
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
    pub fn get_ratio(&self) -> (f64, f64) {
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
        std::cmp::max(1, (width as f64 / ratio) as u32)
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
    pub fov: f64,

    pub look_from: [f64; 3],
    pub look_at: [f64; 3],
    pub vup: [f64; 3],

    #[serde_inline_default(0.0)]
    pub defocus_angle: f64,
    #[serde_inline_default(1.0)]
    pub focus_dist: f64,

    #[serde(default)]
    pub background: [f64; 3],
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
    Lambertian { albedo: [f64; 3] },

    #[serde(rename = "checkered")]
    Checkered {
        even: Option<[f64; 3]>,
        odd: Option<[f64; 3]>,
        scale: Option<f64>,
    },

    #[serde(rename = "texture")]
    Texture { file: String },

    #[serde(rename = "noise")]
    Noise {
        scale: Option<f64>,
        turbulance: Option<u32>,
    },

    #[serde(rename = "metal")]
    Metal { albedo: [f64; 3], roughness: f64 },

    #[serde(rename = "dielectric")]
    Dielectric { refraction_index: f64 },

    #[serde(rename = "glass")]
    Glass {},

    #[serde(rename = "water")]
    Water {},

    #[serde(rename = "light")]
    Light { emit: [f64; 3] },
}

impl MaterialDef {
    fn into_material(self) -> Result<Material, Box<dyn Error>> {
        let args = Args::parse();
        let config_dir = args
            .config
            .parent()
            .ok_or("Failed to get parent directory")?;

        match self {
            MaterialDef::Lambertian { albedo } => Ok(Lambertian::material(SolidColor::texture(
                Vector3::new(albedo[0], albedo[1], albedo[2]),
            ))),
            MaterialDef::Checkered { even, odd, scale } => {
                let scale = scale.unwrap_or(1.0);
                let even = even.unwrap_or([0.05, 0.05, 0.05]);
                let odd = odd.unwrap_or([0.95, 0.95, 0.95]);

                let even_color = Vector3::new(even[0], even[1], even[2]);
                let odd_color = Vector3::new(odd[0], odd[1], odd[2]);

                let checkered = Checkered::texture(scale, even_color, odd_color);
                Ok(Lambertian::material(checkered))
            }
            MaterialDef::Texture { file } => {
                let texture_path = config_dir.join(file);
                let buffer = ImageReader::open(texture_path)?.decode()?.to_rgb8();
                let image = Image::texture(buffer);
                Ok(Lambertian::material(image))
            }
            MaterialDef::Noise { scale, turbulance } => {
                let scale = scale.unwrap_or(1.0);
                let turbulance = turbulance.unwrap_or(1);
                Ok(Lambertian::material(Noise::texture(scale, turbulance)))
            }
            MaterialDef::Metal { albedo, roughness } => {
                Ok(Metal::material(Vector3::from(albedo), roughness))
            }
            MaterialDef::Dielectric { refraction_index } => {
                Ok(Dielectric::material(refraction_index))
            }
            MaterialDef::Glass {} => Ok(Dielectric::material(1.5)),
            MaterialDef::Water {} => Ok(Dielectric::material(1.33)),
            MaterialDef::Light { emit } => {
                Ok(Light::material(SolidColor::texture(Vector3::from(emit))))
            }
        }
    }
}

#[derive(Deserialize)]
struct RawTranslate {
    offset: [f64; 3],
}

#[derive(Deserialize)]
struct RawRotate {
    degrees: f64,
    axis: Axis,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Transform {
    #[serde(rename = "translate")]
    Translate(RawTranslate),
    #[serde(rename = "rotate")]
    Rotate(RawRotate),
}

impl Transform {
    pub fn apply(&self, geomtry: Geometry) -> Geometry {
        match self {
            Transform::Translate(trans) => {
                Translate::geometry(geomtry, Vector3::from(trans.offset))
            }
            Transform::Rotate(trans) => {
                Rotate::geometry(geomtry, trans.axis.clone(), trans.degrees)
            }
        }
    }
}

#[derive(Deserialize)]
struct RawVolume {
    density: f64,
    albedo: [f64; 3],
}

#[derive(Deserialize)]
struct RawSphere {
    #[serde(rename = "position")]
    center: [f64; 3],
    direction: Option<[f64; 3]>,
    radius: f64,
    #[serde(flatten)]
    material_def: MaterialDef,
    #[serde(default)]
    transform: Vec<Transform>,
    volume: Option<RawVolume>,
}

impl RawSphere {
    fn into_sphere(self) -> Result<Geometry, Box<dyn Error>> {
        let center = Vector3::from(self.center);
        let direction = match self.direction {
            None => Vector3::default(),
            Some(direction) => Vector3::from(direction),
        };
        let material = self.material_def.into_material()?;
        let geometry = Sphere::geometry(center, direction, self.radius, material);

        let geometry = match self.volume {
            Some(volume) => Volume::geometry(
                geometry,
                volume.density,
                SolidColor::texture(Vector3::from(volume.albedo)),
            ),
            None => geometry,
        };

        let geometry = self
            .transform
            .into_iter()
            .fold(geometry, |geom, transform| transform.apply(geom));

        Ok(geometry)
    }
}

#[derive(Deserialize)]
struct RawQuad {
    position: [f64; 3],
    u: [f64; 3],
    v: [f64; 3],
    #[serde(flatten)]
    material_def: MaterialDef,
    #[serde(default)]
    transform: Vec<Transform>,
}

impl RawQuad {
    fn into_quad(self) -> Result<Geometry, Box<dyn Error>> {
        let material = self.material_def.into_material()?;
        let geometry = Quad::geomtry(
            Vector3::from(self.position),
            Vector3::from(self.u),
            Vector3::from(self.v),
            material,
        );
        Ok(self
            .transform
            .into_iter()
            .fold(geometry, |geom, transform| transform.apply(geom)))
    }
}

#[derive(Deserialize)]
struct RawCube {
    a: [f64; 3],
    b: [f64; 3],
    #[serde(flatten)]
    material_def: MaterialDef,
    #[serde(default)]
    transform: Vec<Transform>,
    volume: Option<RawVolume>,
}

impl RawCube {
    fn into_cube(self) -> Result<Geometry, Box<dyn Error>> {
        let material = self.material_def.into_material()?;
        let geometry = Cube::geometry(Vector3::from(self.a), Vector3::from(self.b), material);

        let geometry = match self.volume {
            Some(volume) => Volume::geometry(
                geometry,
                volume.density,
                SolidColor::texture(Vector3::from(volume.albedo)),
            ),
            None => geometry,
        };

        let geometry = self
            .transform
            .into_iter()
            .fold(geometry, |geom, transform| transform.apply(geom));

        Ok(geometry)
    }
}

#[derive(Deserialize)]
#[serde(tag = "shape")]
enum ObjectDef {
    #[serde(rename = "sphere")]
    Sphere(RawSphere),
    #[serde(rename = "quad")]
    Quad(RawQuad),
    #[serde(rename = "cube")]
    Cube(RawCube),
}

impl<'de> Deserialize<'de> for Geometry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match ObjectDef::deserialize(deserializer)? {
            ObjectDef::Sphere(raw) => Ok(raw.into_sphere().map_err(serde::de::Error::custom)?),
            ObjectDef::Quad(raw) => Ok(raw.into_quad().map_err(serde::de::Error::custom)?),
            ObjectDef::Cube(raw) => Ok(raw.into_cube().map_err(serde::de::Error::custom)?),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub camera: CameraOptions,
    #[serde(default)]
    pub objects: Vec<Geometry>,
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
