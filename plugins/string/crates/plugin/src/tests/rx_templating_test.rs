use crate::behaviour::entity::templating::TemplatingFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_result::ResultString;
use crate::model_result::ResultStringProperties::RESULT;
use crate::model_string::Templating;
use crate::model_string::TemplatingProperties::CONTEXT;
use crate::model_string::TemplatingProperties::TEMPLATE;
use crate::model_string::BEHAVIOUR_TEMPLATING;
use crate::model_string::COMPONENT_STRING_GATE;
use crate::model_string::ENTITY_TYPE_TEMPLATING;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use serde_json::json;

#[test]
fn rx_templating_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_TEMPLATING.clone())
        .property_with_default(&TEMPLATE)
        .property_with_default(&CONTEXT)
        .property_with_default(&RESULT)
        .component(COMPONENT_STRING_GATE.clone())
        .build();
    let templating = Templating::from(reactive_instance.clone());

    // No behaviour -> no change
    templating.template("{{greet}}, {{target}}");
    templating.context(json!({ "greet": "Hello", "target": "World"}));
    assert_eq!("", templating.result().unwrap());

    // With behaviour
    {
        let factory = TemplatingFactory::new(BEHAVIOUR_TEMPLATING.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        templating.context(json!({ "greet": "Hello", "target": "World"}));
        assert_eq!("Hello, World", templating.result().unwrap());

        // Tera Templating Documentation: https://tera.netlify.app/docs/#templates
        templating.template("{% for a in [1,2,3,] %}{{greet}}, {{target}}! {% endfor %}");
        assert_eq!("Hello, World! Hello, World! Hello, World! ", templating.result().unwrap());

        templating.template("{{greet}}, {{target}}");
        templating.context(json!({ "greet": "Servus", "target": "Inexor"}));
        assert_eq!("Servus, Inexor", templating.result().unwrap());
    }

    // No behaviour -> no change
    templating.context(json!({ "greet": "Hello", "target": "World"}));
    assert_eq!("Servus, Inexor", templating.result().unwrap());
}
