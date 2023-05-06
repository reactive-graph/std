use static_files::NpmBuild;
use std::env;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("CARGO_MANIFEST_DIR={}", dir);
    let web_resource_root_dir = env::var("WEB_RESOURCE_DIR").unwrap_or(dir);
    let web_resource_root_path = Path::new(&web_resource_root_dir).join("web/node_modules/yarn/bin/yarn");
    println!("PATH={}", web_resource_root_path.to_str().unwrap());
    let result = NpmBuild::new("web")
        .executable(web_resource_root_path.to_str().unwrap())
        .install()
        .unwrap()
        .run("build")
        .unwrap()
        .target("web/dist")
        .to_resource_dir()
        .build();
    println!("cargo:rerun-if-changed=web");
    result
}
