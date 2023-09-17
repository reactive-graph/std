use crate::NAMESPACE_METADATA;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(Id3Properties, (ID3_TALB, "id3_talb", ""));

component_ty!(COMPONENT_ID3, NAMESPACE_METADATA, COMPONENT_NAME_ID3, "id3");

component_model!(
    Id3,
    trigger,
    get id3_talb string,
);
