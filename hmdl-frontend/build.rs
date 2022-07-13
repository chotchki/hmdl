use npm_rs::*;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:build_dir={}", out_dir); //Path to the build for inclusion in binary

    //BUILD_PATH
    NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
        .with_env("BUILD_PATH", out_dir)
        .init_env()
        .install(None)
        .run("build")
        .exec()
        .unwrap();
}
