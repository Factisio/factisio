use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Field {
  ScalarDatabaseColumn {
    name: String,
    sql_type_name: String,
    sql_column_name: String,
    graphql_field_name: String,
    graphql_type_name: String,
    graphql_order_by_asc: String,
    graphql_order_by_desc: String,
  },
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let data = r#"
      {
        "name": "id",
        "type": "ScalarDatabaseColumn",
        "sqlTypeName": "text",
        "sqlColumnName": "id",
        "graphqlFieldName": "id",
        "graphqlTypeName": "String",
        "graphqlOrderByAsc": "idAsc",
        "graphqlOrderByDesc": "idDesc"
      }
      "#;

    match serde_json::from_str(data) {
      Ok(field) => match field {
        Field::ScalarDatabaseColumn {
          name,
          sql_type_name,
          sql_column_name,
          graphql_field_name: _,
          graphql_type_name: _,
          graphql_order_by_asc: _,
          graphql_order_by_desc: _,
        } => {
          assert_eq!(name, "id");
          assert_eq!(sql_type_name, "text");
          assert_eq!(sql_column_name, "id");
        }
      },
      Err(e) => println!("Error: {}", e),
    }
  }
}
