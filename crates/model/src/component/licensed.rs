use license::License;

use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_BASE;

properties!(LicensedProperties, (LICENSE, "license", ""), (ATTRIBUTION, "attribution", ""));

component_ty!(COMPONENT_LICENSED, NAMESPACE_BASE, COMPONENT_NAME_LICENSED, "licensed");

component_model!(
    Licensed,
    data license string,
    data attribution string,
);

pub trait SPDXLicensed: Licensed {
    fn set_license_checked(&self, license: &str) {
        if let Ok(license) = license.parse::<&dyn License>() {
            self.set_license(license.id());
        }
    }

    fn to_license(&self) -> Option<&dyn License> {
        self.get_license().and_then(|license| license.parse::<&dyn License>().ok())
    }

    fn is_osi_approved(&self) -> bool {
        self.get_license()
            .and_then(|license| license.parse::<&dyn License>().ok())
            .map(|license| license.is_osi_approved())
            .unwrap_or(false)
    }

    fn is_deprecated(&self) -> bool {
        self.get_license()
            .and_then(|license| license.parse::<&dyn License>().ok())
            .map(|license| license.is_deprecated())
            .unwrap_or(false)
    }
}
