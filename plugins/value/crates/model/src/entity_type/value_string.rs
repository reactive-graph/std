use reactive_graph_reactive_service_api::reactive_entity;

#[reactive_entity(namespace = "value", type_name = "value_string")]
pub struct ValueString {
    pub value: String,
}

#[cfg(test)]
mod tests {
    use crate::entity_type::value_string::ValueString;

    #[test]
    fn test_default() {
        let value_string = ValueString::default();
        assert_eq!(value_string.value, "");
    }

    #[test]
    fn test_constructor() {
        let value_string = ValueString::new("test".to_string());
        assert_eq!(value_string.value, "test");
        assert_eq!("test", value_string.value);
        assert_eq!(value_string.value, "test".to_string());
        assert_eq!("test".to_string(), value_string.value);
    }

    #[test]
    fn test_op() {
        let value_string = ValueString::new("TEST".to_string());
        value_string.value.op(|v| v.to_lowercase());
        assert_eq!(value_string.value, "test");
        value_string.value.op(|v| v.to_uppercase());
        assert_eq!(value_string.value, "TEST");
    }

    #[test]
    fn test_op_2() {
        let value_string_1 = ValueString::new("Hello World".to_string());
        value_string_1.value.op(|v| v.replace("Hello", "Servus"));
        assert_eq!("Servus World", value_string_1.value);
    }

    #[test]
    fn test_debug() {
        let value_string = ValueString::new("TEST".to_string());
        assert_eq!("\"TEST\"", format!("{:?}", value_string.value));
    }

    #[test]
    fn test_display() {
        let value_string = ValueString::new("TEST".to_string());
        assert_eq!("TEST", format!("{}", value_string.value));
    }

    #[test]
    fn test_add_assign() {
        let mut value_string_1 = ValueString::new("Hello");
        value_string_1.value += " World".to_string();
        assert_eq!("Hello World", value_string_1.value);
        let value_string_2 = ValueString::new("!");
        value_string_1.value += &value_string_2.value;
        assert_eq!("Hello World!", value_string_1.value);
        assert_eq!("!", value_string_2.value);
        value_string_1.value += &value_string_2.value;
        assert_eq!("Hello World!!", value_string_1.value);
    }

    #[test]
    fn test_stream_repeat() {
        let mut value_string_1 = ValueString::new("!");
        let value_string_2 = ValueString::new("");
        assert_eq!("", value_string_2.value);
        value_string_1.value >>= &value_string_2.value;
        assert_eq!("!", value_string_2.value);
        value_string_1.value += &value_string_2.value;
        assert_eq!("!!", value_string_2.value);
        value_string_1.value += &value_string_2.value;
        assert_eq!("!!!!", value_string_2.value);
        value_string_1.value += &value_string_2.value;
        assert_eq!("!!!!!!!!", value_string_2.value);
    }

    #[test]
    fn test_propagate() {
        let mut value_string_1 = ValueString::new("value1");
        let value_string_2 = ValueString::new("value2");
        assert_eq!(value_string_1.value, "value1");
        assert_eq!(value_string_2.value, "value2");
        value_string_1.value <<= &value_string_2.value;
        assert_eq!(value_string_1.value, "value2");
        assert_eq!(value_string_2.value, "value2");
    }
}
