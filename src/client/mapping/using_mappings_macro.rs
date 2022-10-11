// macros to make things less painful to write


// puts a jni JObject into a CM
// `apply_object!(CM, JObject);`
#[macro_export]
macro_rules! apply_object {
    ($to_apply_cm:ident, $object_to_apply:expr) => {
        $to_apply_cm.apply_object($object_to_apply)
    };
}

// does what it says on the tin pretty much. will panic if the field or method wasn't added to the CM
// `let method_output: Result<JValue<'_>> = call_method_or_get_field!(..., &[]);`
// `let field_output: Result<JValue<'_>> = call_method_or_get_field!(...);`
#[macro_export]
macro_rules! call_method_or_get_field {
    // for fields
    ($env:expr, $cm:expr, $field_name:literal, $is_static:literal) => {{
        let mappings = if $is_static {
            $cm.get_static_field($field_name)
        } else {
            $cm.get_field($field_name)
        }.unwrap();

        if $is_static {
            $env.get_static_field(
                $cm.get_class(),
                mappings.get_name(),
                mappings.get_sig(),
            )
        } else {
            $env.get_field(
                $cm.get_object().unwrap(),
                mappings.get_name(),
                mappings.get_sig(),
            )
        }
    }};

    // for methods
    ($env:expr, $cm:expr, $method_name:literal, $is_static:literal, $method_args:expr) => {{
        let method = if $is_static {
            $cm.get_static_method($method_name)
        } else {
            $cm.get_method($method_name)
        }.unwrap();

        if $is_static {
            $env.call_static_method(
                $cm.get_class(),
                method.get_name(),
                method.get_sig(),
                $method_args,
            )
        } else {
            $env.call_method(
                $cm.get_object().unwrap(),
                method.get_name(),
                method.get_sig(),
                $method_args,
            )
        }
    }};
}