use log::error;
use serde_json::json;
use serde_json::Value;
use tera::Context;
use tera::Tera;

use crate::model::*;
use crate::model_string::TemplatingProperties::CONTEXT;
use crate::model_string::TemplatingProperties::RESULT;
use crate::model_string::TemplatingProperties::TEMPLATE;
use crate::reactive::*;

entity_behaviour!(Templating, TemplatingFactory, TemplatingFsm, TemplatingBehaviourTransitions, TemplatingValidator);

behaviour_validator!(TemplatingValidator, ReactiveEntityInstance, TEMPLATE.as_ref(), CONTEXT.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for TemplatingBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let template = self.reactive_instance.get(TEMPLATE).ok_or(BehaviourInitializationFailed {})?;
        let context = self.reactive_instance.get(CONTEXT).ok_or(BehaviourInitializationFailed {})?;
        self.reactive_instance.set(RESULT, render(&template, &context));
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for TemplatingBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(CONTEXT.as_ref(), move |context: &Value| {
            if let Some(template) = reactive_instance.get(TEMPLATE) {
                reactive_instance.set(RESULT, render(&template, context));
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TEMPLATE.as_ref(), move |template: &Value| {
            if let Some(context) = reactive_instance.get(CONTEXT) {
                reactive_instance.set(RESULT, render(template, &context));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for TemplatingBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for TemplatingBehaviourTransitions {}

fn render(template: &Value, context: &Value) -> Value {
    match template.as_str() {
        Some(template) => match Context::from_value(context.clone()) {
            Ok(tera_context) => {
                let mut tera = Tera::default();
                tera.autoescape_on(vec![]);
                match tera.render_str(template, &tera_context) {
                    Ok(rendered) => json!(rendered),
                    Err(e) => {
                        error!("Rendering Error: {}", e);
                        json!({
                            "error": "Rendering Error",
                            "message": format!("{}", e),
                            "template": template,
                            "context": context
                        })
                    }
                }
            }
            Err(e) => {
                error!("Rendering Error: {}", e);
                json!({
                    "error": "Context Error",
                    "message": format!("{}", e),
                    "template": template,
                    "context": context
                })
            }
        },
        None => {
            json!({
                "error": "Template Error",
                "message": "Template not a string",
                "template": template,
                "context": context
            })
        }
    }
}
