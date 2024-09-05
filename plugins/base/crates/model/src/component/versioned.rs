use semver::Version;

use crate::NAMESPACE_BASE;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(VersionedProperties, (VERSION, "version", ""));

component_ty!(COMPONENT_VERSIONED, NAMESPACE_BASE, COMPONENT_NAME_VERSIONED, "versioned");

component_model!(
    Versioned,
    data version string,
);

pub trait SemVer: Versioned {
    fn set_version_checked(&self, version: &str) {
        if let Ok(version) = Version::parse(version) {
            self.set_version(version.to_string());
        }
    }

    fn to_version(&self) -> Option<Version> {
        self.get_version().and_then(|version| Version::parse(&version).ok())
    }
}
