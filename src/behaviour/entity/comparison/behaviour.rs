use std::sync::{Arc, RwLock};

use serde_json::json;
use serde_json::Value;

use crate::behaviour::entity::comparison::properties::StringComparisonProperties;
use crate::behaviour::entity::comparison::StringComparisonFunction;
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

pub type StringComparisonExpressionValue = ExpressionValue<String>;

/// Generic implementation of comparison_gates operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct StringComparison<'a> {
    pub lhs: RwLock<Stream<'a, StringComparisonExpressionValue>>,

    pub rhs: RwLock<Stream<'a, StringComparisonExpressionValue>>,

    pub f: StringComparisonFunction,

    pub internal_result: RwLock<Stream<'a, bool>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl StringComparison<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: StringComparisonFunction) -> StringComparison<'static> {
        let lhs = e
            .properties
            .get(StringComparisonProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> StringComparisonExpressionValue {
                match v.as_str() {
                    Some(lhs_str) => (OperatorPosition::LHS, String::from(lhs_str)),
                    None => (OperatorPosition::LHS, StringComparisonProperties::LHS.default_value().as_str().unwrap().into()),
                }
            });
        let rhs = e
            .properties
            .get(StringComparisonProperties::RHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> StringComparisonExpressionValue {
                match v.as_str() {
                    Some(rhs_str) => (OperatorPosition::RHS, String::from(rhs_str)),
                    None => (OperatorPosition::RHS, StringComparisonProperties::RHS.default_value().as_str().unwrap().into()),
                }
            });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(
                StringComparisonProperties::LHS.default_value().as_str().unwrap().into(),
                StringComparisonProperties::RHS.default_value().as_str().unwrap().into(),
            ),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(value.clone()),
                OperatorPosition::RHS => old_state.rhs(value.clone()),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs.clone(), e.rhs.clone()));

        let handle_id = e.properties.get(StringComparisonProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let string_comparison = StringComparison {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        string_comparison.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                e.set(StringComparisonProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        string_comparison
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for StringComparison<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for StringComparison<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(StringComparisonProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(StringComparisonProperties::RESULT.as_ref()).unwrap()
    }
}

impl Gate for StringComparison<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(StringComparisonProperties::RHS.as_ref(), value);
    }
}

impl Drop for StringComparison<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}
