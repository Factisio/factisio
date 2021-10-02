use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Field {
  name: String,
  age: u8,
  phones: Vec<String>,
}

// Tests
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Field = serde_json::from_str(data);

    assert_eq!(p.name, "John Doe");
  }
}
