use semver::Version;

use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_BASE;

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
