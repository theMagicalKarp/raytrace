mod camera;
mod config;
mod geometry;
mod interval;
mod material;
mod math;
mod noise;
mod ray;

use camera::Camera;
use clap::Parser;
use colored::Colorize;
use config::Args;
use config::Config;
use config::span_dump;
use geometry::bvh::BvhNode;
use std::fs;

fn main() {
    let args = Args::parse();

    let config_content = match fs::read_to_string(&args.config) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading config file: {}", e);
            return;
        }
    };

    let mut config: Config = match toml::from_str(&config_content) {
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

    config.camera.samples = match args.samples {
        Some(samples) => samples,
        None => config.camera.samples,
    };

    println!("{}", config);

    let camera = Camera::new(config.camera);
    let world = BvhNode::geometry(config.objects);

    match camera.render(&world).save(args.output) {
        Ok(_) => println!("Image saved successfully."),
        Err(e) => println!("Error saving image: {}", e),
    }
}
