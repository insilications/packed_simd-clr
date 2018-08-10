//! aobench: Ambient Occlusion Renderer benchmark.
//!
//! Based on [aobench](https://code.google.com/archive/p/aobench/) by Syoyo
//! Fujita.
#![deny(warnings)]

#[macro_use]
extern crate structopt;
extern crate aobench_lib;
extern crate time;

use aobench_lib::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// Command-line arguments.
#[derive(StructOpt, Debug)]
struct Opt {
    /// Image width.
    width: usize,
    /// Image height.
    height: usize,

    /// Algorithm
    #[structopt(short = "a", long = "algo")]
    algo: String,

    /// Output file.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

const ALGORITHMS: &[&'static str] = &[
    "scalar", "scalar_par", "vector", "vector_par", "tiled", "tiled_par"
];

fn main() {
    let opt = Opt::from_args();
    let mut scene = aobench_lib::scene::Random::default();
    let mut img = Image::new(opt.width, opt.height);

    let algorithm_name = opt.algo.as_str();

    if let Some(algorithm) = ALGORITHMS.iter().find(|&&a| a == algorithm_name) {
        let d = time::Duration::span(|| match *algorithm {
            "scalar" => scalar::ao(&mut scene, 2, &mut img),
            "scalar_par" => scalar_parallel::ao(&mut scene, 2, &mut img),
            "vector" => vector::ao(&mut scene, 2, &mut img),
            "vector_par" => vector_parallel::ao(&mut scene, 2, &mut img),
            "tiled" => tiled::ao(&mut scene, 2, &mut img),
            "tiled_par" => tiled_parallel::ao(&mut scene, 2, &mut img),
            _ => unreachable!(),
        });
        let image_path =
            opt.output.unwrap_or_else(|| PathBuf::from(format!("image_{}.png", algorithm)));
        img.write_png(&image_path, false)
            .expect("failed to write image");

        println!("time: {} ms", d.num_milliseconds());
    } else {
        let mut error = format!(
            "unknown algorithm: \"{}\"\nAvailable algorithms:",
            algorithm_name
        );
        for a in ALGORITHMS {
            error.push_str(&format!("\n- {}", a));
        }
        panic!(error);
    }
}