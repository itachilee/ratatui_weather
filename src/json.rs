use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    code: u16,
    #[serde(rename = "type")]
    response_type: String,
    message: String,
    result: ResultType,
    extras: Option<serde_json::Value>,
    time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ResultType {
    TypeOne(TypeOneResult),
    TypeTwo(TypeTwoResult),
}

#[derive(Serialize, Deserialize, Debug)]
struct TypeOneResult {
    foo: String,
    bar: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TypeTwoResult {
    another_foo: String,
    another_bar: String,
    different_field: String,
}

fn test_json() {
    let json_data_one = r#"
    {
        "code": 200,
        "type": "success",
        "message": "",
        "result": {
            "foo": "",
            "bar": ""
        },
        "extras": null,
        "time": "2024-06-12 11:02:19"
    }
    "#;

    let json_data_two = r#"
    {
        "code": 200,
        "type": "success",
        "message": "",
        "result": {
            "another_foo": "",
            "another_bar": "",
            "different_field": ""
        },
        "extras": null,
        "time": "2024-06-12 11:02:19"
    }
    "#;

    let deserialized_one: ApiResponse =
        serde_json::from_str(json_data_one).expect("Failed to deserialize");
    let deserialized_two: ApiResponse =
        serde_json::from_str(json_data_two).expect("Failed to deserialize");

    println!("{:?}", deserialized_one);
    println!("{:?}", deserialized_two);

    // Serializing back to JSON
    let serialized_one = serde_json::to_string(&deserialized_one).expect("Failed to serialize");
    let serialized_two = serde_json::to_string(&deserialized_two).expect("Failed to serialize");
    println!("{}", serialized_one);
    println!("{}", serialized_two);
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::fmt::{Formatter, Write};
    use serde::{de, Deserializer, Serializer};
    use serde::de::{MapAccess, Visitor};
    use serde::ser::SerializeStruct;
    use super::*;

    #[test]
    fn test_to_json() {
        test_json();
    }


    #[derive(Debug, )]
    struct TestStruct {
        foo: String,
        bar: String,
    }

    impl<'de> Deserialize<'de> for TestStruct {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(MyStructVisitor)
        }
    }

    struct MyStructVisitor;

    impl<'de> Visitor<'de> for MyStructVisitor{
        type Value =TestStruct;
        fn visit_map<A>(self,mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut field1 = None;
            let mut field2 = None;

            while let Some(key) = map.next_key()? {
                match key {
                    "field1" => {
                        if field1.is_some() {
                            return Err(de::Error::duplicate_field("field1"));
                        }
                        field1 = Some(map.next_value()?);
                    }
                    "field2" => {
                        if field2.is_some() {
                            return Err(de::Error::duplicate_field("field2"));
                        }
                        field2 = Some(map.next_value()?);
                    }
                    _ => {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
            }

            let field1 = field1.ok_or_else(|| de::Error::missing_field("field1"))?;
            let field2 = field2.ok_or_else(|| de::Error::missing_field("field2"))?;
            Ok(TestStruct { foo: field1, bar: field2 })
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with keys 'field1' and 'field2'")
        }
    }

    impl Serialize for TestStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("Color", 2)?;
            state.serialize_field("foo", &self.foo)?;
            state.serialize_field("bar", &self.bar)?;
            state.end()
        }
    }


    #[test]
    fn test_serialize(){
        let foo =TestStruct{
            foo:"foo".to_string(),
            bar:"bar".to_string()
        };

        let str = serde_json::to_string(&foo).unwrap();

        println!("{}",str);

        assert_eq!("{\"foo\":\"foo\",\"bar\":\"bar\"}",str);

    }

    #[test]

    fn test_deserialize(){

        let str ="{\"field1\":\"foo\",\"field2\":\"bar\"}";
        let test = serde_json::from_str::<TestStruct>(str).unwrap();

        assert_eq!("foo",test.foo);
    }
}
