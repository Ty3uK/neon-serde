#[macro_use]
extern crate neon;
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use neon::vm::{Call, JsResult};
use neon::js::{JsValue, JsNull, JsString};
use neon::scope::Scope;
use std::rc::Rc;

#[derive(Serialize, Debug, Deserialize)]
struct AnObject<'a> {
    a: u32,
    b: Vec<f64>,
    c: &'a str,
}

#[derive(Serialize, Debug, Deserialize, Eq, PartialEq)]
struct AnObjectTwo<'a> {
    a: u32,
    b: Vec<i64>,
    c: &'a str,
}

macro_rules! make_test {
    ($name:ident, $val:expr) => {
        fn $name(call: Call) -> JsResult<JsValue> {
            let scope = call.scope;
            let value = $val;

            let handle = neon_serde::to_value(&value, scope)?;
            Ok(handle)
        }
    };
}

make_test!(make_num_77, 77i32);
make_test!(make_num_32, 32u8);
make_test!(make_str_hello, "Hello World");
make_test!(make_num_array, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
make_test!(
    make_obj,
    AnObject {
        a: 1,
        b: vec![0.1f64, 1.1, 2.2, 3.3],
        c: "Hi",
    }
);
make_test!(make_map, {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map
});

fn expect_hello_world(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let value = "hello world";

    let arg0 = call.arguments.require(scope, 0)?.check::<JsValue>()?;

    let de_serialized :String = neon_serde::from_handle(arg0, scope)?;
    assert_eq!(value, &de_serialized);

    Ok(JsNull::new().upcast())
}

fn expect_obj(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let value = AnObjectTwo {
        a:1,
        b: vec![1,2],
        c: "abc"
    };

    let arg0 = call.arguments.require(scope, 0)?.check::<JsValue>()?;

    let de_serialized :AnObjectTwo = neon_serde::from_handle(arg0, scope)?;
    assert_eq!(value, de_serialized);

    Ok(JsNull::new().upcast())
}

register_module!(m, {
    m.export("make_num_77", make_num_77)?;
    m.export("make_num_32", make_num_32)?;
    m.export("make_str_hello", make_str_hello)?;
    m.export("make_num_array", make_num_array)?;
    m.export("make_obj", make_obj)?;
    m.export("make_map", make_map)?;
    m.export("expect_hello_world", expect_hello_world)?;
    m.export("expect_obj", expect_obj)?;
    Ok(())
});
