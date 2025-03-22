use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Material;
use crate::material::Metal;
use crate::object::Sphere;
use colored::Colorize;
use nalgebra::Vector3;
use serde::Deserialize;
use std::ops::Range;
use std::sync::Arc;

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

    pub defocus_angle: f32,
    pub focus_dist: f32,
}

#[derive(Deserialize)]
#[serde(tag = "material", deny_unknown_fields)]
enum MaterialDef {
    #[serde(rename = "lambertian")]
    Lambertian { albedo: [f32; 3] },

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
    fn into_material(self) -> Arc<dyn Material> {
        match self {
            MaterialDef::Lambertian { albedo } => Arc::new(Lambertian::new(Vector3::new(
                albedo[0], albedo[1], albedo[2],
            ))),
            MaterialDef::Metal { albedo, roughness } => Arc::new(Metal::new(
                Vector3::new(albedo[0], albedo[1], albedo[2]),
                roughness,
            )),
            MaterialDef::Dielectric { refraction_index } => {
                Arc::new(Dielectric::new(refraction_index))
            }
            MaterialDef::Glass {} => Arc::new(Dielectric::new(1.5)),
            MaterialDef::Water {} => Arc::new(Dielectric::new(1.33)),
        }
    }
}

#[derive(Deserialize)]
struct RawSphere {
    #[serde(rename = "position")]
    center: [f32; 3],
    radius: f32,
    #[serde(flatten)]
    material_def: MaterialDef,
}

impl RawSphere {
    fn into_sphere(self) -> Sphere {
        Sphere {
            center: Vector3::new(self.center[0], self.center[1], self.center[2]),
            radius: self.radius,
            material: self.material_def.into_material(),
        }
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
            ObjectDef::Sphere(raw) => Ok(Object::Sphere(raw.into_sphere())),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub camera: CameraOptions,
    #[serde(default)]
    pub objects: Vec<Object>,
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
