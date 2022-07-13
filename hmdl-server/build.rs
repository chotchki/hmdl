use std::env;

fn main() {
    let schema_key = "DATABASE_URL";
    let schema_url = env::var("DEP_HMDLDB_DATABASE_URL").unwrap();
    println!("cargo:rustc-env={}={}", schema_key, schema_url);

    let frontend_dir = env::var("DEP_HMDLFRONTEND_BUILD_DIR").unwrap();
    println!("cargo:rustc-env=FRONTEND_BUILD_DIR={}", frontend_dir);
}
