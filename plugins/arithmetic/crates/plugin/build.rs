extern crate rustc_version;
use rustc_version::Channel;
use rustc_version::version_meta;

fn main() {
    println!("cargo:rerun-if-changed=types");
    // Set cfg flags depending on release channel
    match version_meta().unwrap().channel {
        Channel::Stable => {
            println!("cargo:rustc-cfg=rustc_stable");
        }
        Channel::Beta => {
            println!("cargo:rustc-cfg=rustc_beta");
        }
        Channel::Nightly => {
            println!("cargo:rustc-cfg=rustc_nightly");
        }
        Channel::Dev => {
            println!("cargo:rustc-cfg=rustc_dev");
        }
    }
}
