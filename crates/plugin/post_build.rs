use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Deployment {
    pub target_dirs: Vec<String>,
}

fn main() {
    match fs::read_to_string("./.deployment.toml") {
        Ok(toml_string) => {
            let deployment: Result<Deployment, _> = toml::from_str(&toml_string);
            match deployment {
                Ok(deployment) => {
                    let mut crate_out_dir = env::var("CRATE_OUT_DIR").unwrap();
                    crate_out_dir.push_str("/libinexor_rgf_plugin_*.*");
                    for target_dir in deployment.target_dirs {
                        for entry in glob::glob(crate_out_dir.as_str()).unwrap() {
                            if let Ok(source_path) = entry {
                                let file_name = source_path.file_name().unwrap().to_str().unwrap();
                                if file_name.ends_with(".so") || file_name.ends_with(".dll") {
                                    let mut target_path = PathBuf::from(&target_dir);
                                    target_path.push(file_name);
                                    println!("Copy plugin from {} to {}", source_path.display(), target_path.display());
                                    let _ = fs::copy(source_path, target_path);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse .deployment.toml: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Could not read .deployment.toml: {}", e);
        }
    }
}
