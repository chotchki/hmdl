use npm_rs::*;

fn main() {
    NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
        .set_path("hearthstone-frontend")
        .init_env()
        .install(None)
        .run("build")
        .exec()
        .unwrap();
}
