use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_METADATA;

properties!(Id3Properties, (ID3_TALB, "id3_talb", ""));

component_ty!(COMPONENT_ID3, NAMESPACE_METADATA, COMPONENT_NAME_ID3, "id3");

component_model!(
    Id3,
    trigger,
    get id3_talb string,
);
