use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct JsonData {
    foo: i32,
    bar: String,
}

impl Serialize for JsonData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("JsonData", 2)?;
        s.serialize_field("foo", &self.foo)?;
        s.serialize_field("bar", &self.bar)?;
        s.end()
    }
}

pub fn func() {
    let serialized =
        serde_json::to_string(&JsonData { foo: 200, bar: String::from("Rust") }).unwrap();
    println!("serialized: {}", serialized); // => {"foo":200,"bar":"Rust"}

    let s = r#"{ "foo": 42, "bar": "hello" }"#;
    let deserialized: JsonData = serde_json::from_str(s).unwrap();
    println!("deserialized: {:?}", deserialized); // => JsonData { foo: 42, bar: "hello" }
}
