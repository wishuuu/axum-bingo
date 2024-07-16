use npm_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let npm_path = std::env::current_dir().unwrap().join("npm");

    let npm_status = NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile()?)
        .set_path(npm_path)
        .init_env()
        .install(None)
        .run("css")
        .exec()?;

    if !npm_status.success() {
        println!("cargo:warning=npm failed with: {}", npm_status);
    }

    Ok(())
}