use inexor_rgf_model_metadata::DublinCoreProperties;

use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::entity_model;
use crate::model_metadata::DublinCore;
use crate::model_metadata::COMPONENT_DUBLIN_CORE;
use crate::model_metadata::NAMESPACE_METADATA;

entity_model!(MetaData);
impl DublinCore for MetaData {}

#[test]
fn dublin_core_test() {
    let mut builder = ReactiveEntityInstanceBuilder::new_from_type(NAMESPACE_METADATA, "metadata");
    builder.component(COMPONENT_DUBLIN_CORE.clone());
    DublinCoreProperties::properties().iter().for_each(|property| {
        builder.property(property.name.as_str(), property.value.clone());
    });
    let dublin_core = MetaData::from(builder.build());
    dublin_core.set_dc_date("2023-01-01");
    assert_eq!("2023-01-01", dublin_core.get_dc_date().unwrap());
}
