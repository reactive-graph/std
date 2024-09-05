use crate::NAMESPACE_METADATA;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(Id3Properties, (ID3_TALB, "id3_talb", ""));

component_ty!(COMPONENT_ID3, NAMESPACE_METADATA, COMPONENT_NAME_ID3, "id3");

component_model!(
    Id3,
    trigger,
    get id3_talb string,
);
