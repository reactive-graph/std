use crate::NAMESPACE_BASE;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(DescribableProperties, (DESCRIPTION, "description", ""));

component_ty!(COMPONENT_DESCRIBABLE, NAMESPACE_BASE, COMPONENT_NAME_DESCRIBABLE, "describable");

component_model!(
    Describable,
    data description string,
);

// #[component_type]
// pub fn trait Describable {
//
// }

// pub trait $ident: $crate::PropertyInstanceGetter + $crate::PropertyInstanceSetter {
// $(
// $crate::rx_accessor!($accessor_type $($accessor_name $accessor_data_type)?);
// )*
// }
