use license::License;
use semver::Version;
use serde_json::json;

use crate::builder::ReactiveEntityInstanceBuilder;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_std_base_model::COMPONENT_DESCRIBABLE;
use reactive_graph_std_base_model::COMPONENT_LICENSED;
use reactive_graph_std_base_model::COMPONENT_NAMED;
use reactive_graph_std_base_model::COMPONENT_VERSIONED;
use reactive_graph_std_base_model::Describable;
use reactive_graph_std_base_model::DescribableProperties::DESCRIPTION;
use reactive_graph_std_base_model::Licensed;
use reactive_graph_std_base_model::LicensedProperties::ATTRIBUTION;
use reactive_graph_std_base_model::LicensedProperties::LICENSE;
use reactive_graph_std_base_model::NAMESPACE_BASE;
use reactive_graph_std_base_model::Named;
use reactive_graph_std_base_model::NamedProperties::NAME;
use reactive_graph_std_base_model::SPDXLicensed;
use reactive_graph_std_base_model::SemVer;
use reactive_graph_std_base_model::Versioned;
use reactive_graph_std_base_model::VersionedProperties::VERSION;

// An empty model
entity_model!(ExampleNamed);
// Extend the empty model with the trait "Named"
// The trait is provided by a component_model!
impl Named for ExampleNamed {}

#[test]
fn component_named_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new_from_type(NAMESPACE_BASE, "example")
        .property(NAME, json!("name"))
        .component(COMPONENT_NAMED.clone())
        .build();
    let entity = ExampleNamed::from(reactive_instance);
    assert_eq!("name", entity.get_name().unwrap());
    entity.set_name("modified_name");
    assert_eq!("modified_name", entity.get_name().unwrap());
}

entity_model!(ExampleDescribable);
impl Describable for ExampleDescribable {}

#[test]
fn component_describable_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new_from_type(NAMESPACE_BASE, "example")
        .property(DESCRIPTION, json!("description"))
        .component(COMPONENT_DESCRIBABLE.clone())
        .build();
    let entity = ExampleDescribable::from(reactive_instance);
    assert_eq!("description", entity.get_description().unwrap());
    entity.set_description("modified_description");
    assert_eq!("modified_description", entity.get_description().unwrap());
}

entity_model!(ExampleLicensed);
impl Licensed for ExampleLicensed {}
impl SPDXLicensed for ExampleLicensed {}

#[test]
fn component_licensed_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new_from_type(NAMESPACE_BASE, "example")
        .property(LICENSE, json!("MIT"))
        .property(ATTRIBUTION, json!("(c) The Inexor Collective"))
        .component(COMPONENT_LICENSED.clone())
        .build();
    let entity = ExampleLicensed::from(reactive_instance);
    assert_eq!("MIT", entity.get_license().unwrap());
    assert!(entity.is_osi_approved());
    assert!(!entity.is_deprecated());
    assert_eq!("(c) The Inexor Collective", entity.get_attribution().unwrap());
    entity.set_license("GPL-3.0-or-later");
    assert_eq!("GPL-3.0-or-later", entity.get_license().unwrap());
    let license = entity.to_license().unwrap();
    assert_eq!("GPL-3.0-or-later".parse::<&dyn License>().unwrap().id(), license.id());
    entity.set_attribution("(c) 2011-2022 by The Inexor Collective");
    assert_eq!("(c) 2011-2022 by The Inexor Collective", entity.get_attribution().unwrap());
    entity.set_license_checked("GPL-3.0-only");
    assert_eq!("GPL-3.0-only", entity.get_license().unwrap());
    // Invalid license -> no change
    entity.set_license_checked("GPL-3.0-blah");
    assert_eq!("GPL-3.0-only", entity.get_license().unwrap());
}

entity_model!(ExampleVersioned);
impl Versioned for ExampleVersioned {}
impl SemVer for ExampleVersioned {}

#[test]
fn component_versioned_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new_from_type(NAMESPACE_BASE, "example")
        .property(VERSION, json!("0.8.0"))
        .component(COMPONENT_VERSIONED.clone())
        .build();
    let entity = ExampleVersioned::from(reactive_instance);
    assert_eq!("0.8.0", entity.get_version().unwrap());
    entity.set_version("0.9.0");
    assert_eq!("0.9.0", entity.get_version().unwrap());
    assert_eq!(Version::parse("0.9.0").unwrap(), entity.to_version().unwrap());
    entity.set_version_checked("1.0.0");
    assert_eq!(Version::parse("1.0.0").ok(), entity.to_version());
    // Invalid version -> no change
    entity.set_version_checked("A.B.C");
    assert_eq!(Version::parse("1.0.0").ok(), entity.to_version());
}
