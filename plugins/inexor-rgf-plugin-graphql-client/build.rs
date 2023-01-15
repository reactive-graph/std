use static_files::NpmBuild;
use std::env;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&dir).join("web/node_modules/yarn/bin/yarn");
    println!("CARGO_MANIFEST_DIR={}", dir.as_str());
    println!("PATH={}", path.to_str().unwrap());
    let result = NpmBuild::new("web")
        .executable(path.to_str().unwrap())
        .install()
        .unwrap()
        .run("build")
        .unwrap()
        .target("web/dist/bundle")
        .to_resource_dir()
        .build();
    println!("cargo:rerun-if-changed=web");
    result
}
