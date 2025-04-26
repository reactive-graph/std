use static_files::NpmBuild;
use std::env;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Missing env var CARGO_MANIFEST_DIR");
    println!("CARGO_MANIFEST_DIR={}", cargo_manifest_dir);
    let web_resource_root_path = match env::var("WORKSPACE_ROOT_MANIFEST_DIR") {
        Ok(workspace_root_manifest_dir) => {
            let cargo_pkg_name = env::var("CARGO_PKG_NAME")
                .expect("Missing env var CARGO_PKG_NAME")
                .strip_prefix("reactive-graph-tooling-")
                .expect("Failed to strip prefix")
                .to_string();
            let yarn_location = Path::new(&workspace_root_manifest_dir)
                .join("plugins")
                .join(cargo_pkg_name)
                .join("web/node_modules/yarn/bin/yarn");
            let yarn_location_dir = yarn_location.to_str().expect("Failed to get yarn location dir");
            yarn_location
                .canonicalize()
                .unwrap_or_else(|_| panic!("Failed to canonicalize {}", &yarn_location_dir))
        }
        Err(_) => Path::new(&cargo_manifest_dir).join("web/node_modules/yarn/bin/yarn"),
    };
    println!("PATH={}", web_resource_root_path.to_str().unwrap());
    let result = NpmBuild::new("web")
        .executable(web_resource_root_path.to_str().unwrap())
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
