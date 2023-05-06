use static_files::NpmBuild;
use std::env;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("CARGO_MANIFEST_DIR={}", dir);
    let web_resource_root_path = match env::var("WORKSPACE_ROOT_MANIFEST_DIR") {
        Ok(workspace_root_manifest_dir) => Path::new(&workspace_root_manifest_dir)
            .join("plugins")
            .join(env::var("CARGO_PKG_NAME").unwrap())
            .join("web/node_modules/yarn/bin/yarn")
            .canonicalize()
            .unwrap(),
        Err(_) => Path::new(&dir).join("web/node_modules/yarn/bin/yarn"),
    };
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
