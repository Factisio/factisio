use super::field::Field;
use schemars::{schema::RootSchema, schema_for};

pub fn generate_json_schema() -> RootSchema {
  return schema_for!(Field);
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn json_schema_ok() {
    let schema = generate_json_schema();
    // The json schema is too long to check here, so we just check its length
    // assert_eq!(serde_json::to_string_pretty(&schema).unwrap().len(), 49509);
    insta::assert_debug_snapshot!(serde_json::to_string_pretty(&schema).unwrap())
  }
}
