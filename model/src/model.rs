use super::entity::Entity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(tag = "version", rename_all = "camelCase")]
pub enum Model {
  V1 { entities: Vec<Rc<Entity>> },
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use crate::field::Field;
  use crate::sql_type;

  #[test]
  fn serialize() {
    let value = Model::V1 {
      entities: vec![Rc::new(Entity::DatabaseTable {
        name: "person".to_string(),
        sql_schema_name: "public".to_string(),
        sql_table_name: "person_table".to_string(),
        graphql_entity_type_name: "Person".to_string(),
        graphql_filter_type_name: "PersonWhereFilter".to_string(),
        graphql_get_single_operation_name: "person".to_string(),
        graphql_get_list_operation_name: "persons".to_string(),
        graphql_get_connection_operation_name: "personConnection".to_string(),
        graphql_default_order_by: "id_ASC".to_string(),
        graphql_default_first: 10,
        graphql_default_offset: 0,
        fields: vec![
          Rc::new(Field::ScalarDatabaseColumn {
            name: "id".to_string(),
            sql_type: sql_type::Type::Text,
            sql_column_name: "id_col".to_string(),
            graphql_field_name: "id".to_string(),
            graphql_type_name: "String".to_string(),
            graphql_order_by_asc: "id_ASC".to_string(),
            graphql_order_by_desc: "id_DESC".to_string(),
          }),
          Rc::new(Field::ScalarDatabaseColumn {
            name: "drone".to_string(),
            sql_type: sql_type::Type::Text,
            sql_column_name: "drone_col".to_string(),
            graphql_field_name: "drone".to_string(),
            graphql_type_name: "String".to_string(),
            graphql_order_by_asc: "drone_ASC".to_string(),
            graphql_order_by_desc: "drone_DESC".to_string(),
          }),
        ],
      })],
    };

    insta::assert_debug_snapshot!(serde_json::to_string_pretty(&value).unwrap());
  }
}
