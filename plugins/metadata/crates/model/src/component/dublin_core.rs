use crate::NAMESPACE_METADATA;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(
    DublinCoreProperties,
    (DC_LANGUAGE, "dc_language", ""),
    (DC_TITLE, "dc_title", ""),
    (DC_SUBJECT, "dc_subject", ""),
    (DC_COVERAGE, "dc_coverage", ""),
    (DC_DESCRIPTION, "dc_description", ""),
    (DC_IDENTIFIER, "dc_identifier", ""),
    (DC_FORMAT, "dc_format", ""),
    (DC_TYPE, "dc_type", ""),
    (DC_CREATOR, "dc_creator", ""),
    (DC_CONTRIBUTOR, "dc_contributor", ""),
    (DC_DATE, "dc_date", ""),
    (DC_PUBLISHER, "dc_publisher", ""),
    (DC_RELATION, "dc_relation", ""),
    (DC_RIGHTS, "dc_rights", ""),
    (DC_SOURCE, "dc_source", "")
);

component_ty!(COMPONENT_DUBLIN_CORE, NAMESPACE_METADATA, COMPONENT_NAME_DUBLIN_CORE, "dublin_core");

component_model!(
    DublinCore,
    data dc_language string,
    data dc_title string,
    data dc_subject string,
    data dc_coverage string,
    data dc_description string,
    data dc_identifier string,
    data dc_format string,
    data dc_type string,
    data dc_creator string,
    data dc_contributor string,
    data dc_date string,
    data dc_publisher string,
    data dc_relation string,
    data dc_rights string,
    data dc_source string,
);
