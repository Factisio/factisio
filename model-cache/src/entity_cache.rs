use factisio_model::entity::Entity;
use factisio_model::field::Field;

use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum EntityCache<'a> {
  DatabaseTable {
    entity: &'a Entity,
    fields_by_sql_column_name: HashMap<String, &'a Field>,
    fields_by_graphql_field_name: HashMap<String, &'a Field>,
  },
}

pub fn create_from_entity<'a>(entity: &'a Entity) -> EntityCache<'a> {
  match entity {
    Entity::DatabaseTable { fields, .. } => {
      let mut fields_by_sql_column_name = HashMap::new();
      let mut fields_by_graphql_field_name = HashMap::new();
      for field in fields.iter() {
        match field {
          Field::ScalarDatabaseColumn {
            sql_column_name,
            graphql_field_name,
            ..
          } => {
            fields_by_sql_column_name.insert(sql_column_name.clone(), field);
            fields_by_graphql_field_name.insert(graphql_field_name.clone(), field);
          }
        }
      }
      return EntityCache::DatabaseTable {
        entity,
        fields_by_sql_column_name,
        fields_by_graphql_field_name,
      };
    }
  };
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use factisio_model::sql_type;

  #[test]
  fn serialize() {
    let value = Entity::DatabaseTable {
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
        Field::ScalarDatabaseColumn {
          name: "id".to_string(),
          sql_type: sql_type::Type::Text,
          sql_column_name: "id_col".to_string(),
          graphql_field_name: "id".to_string(),
          graphql_type_name: "String".to_string(),
          graphql_order_by_asc: "id_ASC".to_string(),
          graphql_order_by_desc: "id_DESC".to_string(),
        },
        Field::ScalarDatabaseColumn {
          name: "drone".to_string(),
          sql_type: sql_type::Type::Text,
          sql_column_name: "drone_col".to_string(),
          graphql_field_name: "drone".to_string(),
          graphql_type_name: "String".to_string(),
          graphql_order_by_asc: "drone_ASC".to_string(),
          graphql_order_by_desc: "drone_DESC".to_string(),
        },
      ],
    };

    insta::assert_debug_snapshot!(
      serde_json::to_string_pretty(&create_from_entity(&value)).unwrap()
    );
  }
}
