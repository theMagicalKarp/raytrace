mod camera;
mod config;
mod interval;
mod material;
mod math;
mod noise;
mod object;
mod ray;

use camera::Camera;
use clap::Parser;
use colored::Colorize;
use config::Args;
use config::Config;
use config::Object;
use config::span_dump;
use object::bvh::BvhNode;
use object::hittable::Hittable;
use std::fs;
use std::sync::Arc;

fn main() {
    let args = Args::parse();

    let config_content = match fs::read_to_string(&args.config) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading config file: {}", e);
            return;
        }
    };

    let config: Config = match toml::from_str(&config_content) {
        Ok(config) => config,
        Err(e) => {
            if let Some(span) = e.span() {
                println!(
                    "{}{} {}",
                    "error".bold().red(),
                    ":".bold(),
                    e.message().bold()
                );
                println!(
                    "  {} {}:{}:{}",
                    "-->".blue(),
                    args.config.display(),
                    span.start,
                    span.end
                );
                println!("{}", span_dump(&config_content, span));
                return;
            }

            println!("Error parsing config file: {}", e);
            return;
        }
    };

    println!("{}", config);

    let camera = Camera::new(config.camera);
    let mut objects = Vec::<Arc<dyn Hittable>>::new();

    for object in config.objects {
        match object {
            Object::Sphere(sphere) => objects.push(Arc::new(sphere)),
        }
    }

    let world = BvhNode::new(objects);

    match camera.render(Arc::new(world)).save(args.output) {
        Ok(_) => println!("Image saved successfully."),
        Err(e) => println!("Error saving image: {}", e),
    }
}
