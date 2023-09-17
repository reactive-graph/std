use crate::NAMESPACE_GIT;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(
    TransferProgressProperties,
    (RECEIVED_OBJECTS, "received_objects", 0),
    (TOTAL_OBJECTS, "total_objects", 0),
    (RECEIVED_BYTES, "received_bytes", 0),
    (LOCAL_OBJECTS, "local_objects", 0),
    (TOTAL_DELTAS, "total_deltas", 0),
    (INDEXED_DELTAS, "indexed_deltas", 0),
    (INDEXED_OBJECTS, "indexed_objects", 0)
);

component_ty!(COMPONENT_TRANSFER_PROGRESS, NAMESPACE_GIT, COMPONENT_NAME_TRANSFER_PROGRESS, "transfer_progress");

component_model!(
    TransferProgress,
    set received_objects u64,
    set total_objects u64,
    set received_bytes u64,
    set local_objects u64,
    set total_deltas u64,
    set indexed_deltas u64,
    set indexed_objects u64,
);
