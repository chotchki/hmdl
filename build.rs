use clap::Parser;
use npm_rs::*;
use sqlx_cli::Opt;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    //let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let schema_key = "DATABASE_URL";
    let schema_url = "sqlite://".to_string() + &out_dir + "/schema.db";

    //Run a migration for sqlx so it can compile queries
    env::set_var(schema_key, schema_url.clone());
    let command = vec!["create", "database", "reset"];

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            sqlx_cli::run(Opt::parse_from(command)).await.unwrap();
        });

    println!("cargo:rustc-env={}={}", schema_key, schema_url);
    //println!("cargo:rerun-if-changed={}", src_dir + "/migrations");

    NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
        .set_path("hearthstone-frontend")
        .init_env()
        .install(None)
        .run("build")
        .exec()
        .unwrap();
}
