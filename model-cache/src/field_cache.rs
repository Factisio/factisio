use factisio_model::field::Field;

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FieldCache<'a> {
  ScalarDatabaseColumn { field: &'a Field },
}

impl<'a> FieldCache<'a> {
  // Another associated function, taking two arguments:
  pub fn new(field: &'a Field) -> FieldCache<'a> {
    match field {
      Field::ScalarDatabaseColumn { .. } => {
        return FieldCache::ScalarDatabaseColumn { field };
      }
    };
  }
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use factisio_model::sql_type;

  #[test]
  fn serialize() {
    let value = Field::ScalarDatabaseColumn {
      name: "id".to_string(),
      sql_type: sql_type::Type::Text,
      sql_column_name: "id_col".to_string(),
      graphql_field_name: "id".to_string(),
      graphql_type_name: "String".to_string(),
      graphql_order_by_asc: "id_ASC".to_string(),
      graphql_order_by_desc: "id_DESC".to_string(),
    };

    insta::assert_debug_snapshot!(serde_json::to_string_pretty(&FieldCache::new(&value)).unwrap());
  }
}
