use crate::plugins::EntityTypeProvider;
use crate::providers::LogicalEntityTypeProviderImpl;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_model_logical::NAMESPACE_LOGICAL;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec![
        EntityTypeId::new_from_type(NAMESPACE_LOGICAL, "and"),
        EntityTypeId::new_from_type(NAMESPACE_LOGICAL, "and3"),
        EntityTypeId::new_from_type(NAMESPACE_LOGICAL, "if_then_else"),
        EntityTypeId::new_from_type(NAMESPACE_LOGICAL, "nand"),
        EntityTypeId::new_from_type(NAMESPACE_LOGICAL, "nor"),
        EntityTypeId::new_from_type(NAMESPACE_LOGICAL, "not"),
        EntityTypeId::new_from_type(NAMESPACE_LOGICAL, "or"),
        EntityTypeId::new_from_type(NAMESPACE_LOGICAL, "toggle"),
        EntityTypeId::new_from_type(NAMESPACE_LOGICAL, "trigger"),
        EntityTypeId::new_from_type(NAMESPACE_LOGICAL, "xnor"),
        EntityTypeId::new_from_type(NAMESPACE_LOGICAL, "xor"),
    ];
    let entity_type_provider = LogicalEntityTypeProviderImpl {};
    let entity_types = entity_type_provider.get_entity_types();
    assert_eq!(expected_entity_types.len(), entity_types.len());
    assert_eq!(
        expected_entity_types.len(),
        entity_types
            .into_iter()
            .map(|entity_type| entity_type.ty)
            .filter(|ty| expected_entity_types.contains(&ty))
            .count()
    );
}
