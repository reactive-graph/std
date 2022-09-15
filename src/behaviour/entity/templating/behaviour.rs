use std::convert::AsRef;
use std::sync::Arc;
use std::sync::RwLock;

use log::error;
use serde_json::json;
use serde_json::Value;
use tera::Context;
use tera::Tera;

use crate::behaviour::entity::templating::properties::TemplatingProperties;
use crate::frp::Stream;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::expression::Expression;
use crate::reactive::entity::expression::ExpressionValue;
use crate::reactive::entity::expression::OperatorPosition;
use crate::reactive::entity::gate::Gate;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;

pub const TEMPLATING: &str = "templating";

pub type ValueExpressionValue = ExpressionValue<Value>;

pub struct Templating<'a> {
    pub lhs: RwLock<Stream<'a, ValueExpressionValue>>,

    pub rhs: RwLock<Stream<'a, ValueExpressionValue>>,

    pub internal_result: RwLock<Stream<'a, Value>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl Templating<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>) -> Templating<'static> {
        let lhs = e
            .properties
            .get(TemplatingProperties::TEMPLATE.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> ValueExpressionValue { (OperatorPosition::LHS, v.clone()) });
        let rhs = e
            .properties
            .get(TemplatingProperties::CONTEXT.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> ValueExpressionValue { (OperatorPosition::RHS, v.clone()) });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(TemplatingProperties::TEMPLATE.default_value(), TemplatingProperties::CONTEXT.default_value()),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(value.clone()),
                OperatorPosition::RHS => old_state.rhs(value.clone()),
            },
        );

        let internal_result = expression.map(move |e| render(e.lhs.clone(), e.rhs.clone()));

        let handle_id = e.properties.get(TemplatingProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let templating = Templating {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        templating.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                e.set(TemplatingProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        templating
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for Templating<'_> {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(TemplatingProperties::RESULT.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Operation for Templating<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(TemplatingProperties::TEMPLATE.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(TemplatingProperties::RESULT.as_ref()).unwrap()
    }
}

impl Gate for Templating<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(TemplatingProperties::CONTEXT.as_ref(), value);
    }
}

impl Drop for Templating<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}

fn render(template: Value, context: Value) -> Value {
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
