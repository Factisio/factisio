use super::field_cache::FieldCache;
use factisio_model::entity::Entity;
use factisio_model::field::Field;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum EntityCache {
  DatabaseTable {
    entity: Rc<Entity>,
    fields_by_sql_column_name: FxHashMap<String, Rc<FieldCache>>,
    fields_by_graphql_field_name: FxHashMap<String, Rc<FieldCache>>,
  },
}

impl EntityCache {
  // Another associated function, taking two arguments:
  pub fn new(entity: Rc<Entity>) -> EntityCache {
    match &*entity {
      Entity::DatabaseTable { fields, .. } => {
        let mut fields_by_sql_column_name = FxHashMap::default();
        let mut fields_by_graphql_field_name = FxHashMap::default();
        for field in fields.iter() {
          match &**field {
            Field::ScalarDatabaseColumn {
              sql_column_name,
              graphql_field_name,
              ..
            } => {
              let field_cache = Rc::new(FieldCache::new(Rc::clone(&field)));
              fields_by_sql_column_name.insert(sql_column_name.clone(), Rc::clone(&field_cache));
              fields_by_graphql_field_name
                .insert(graphql_field_name.clone(), Rc::clone(&field_cache));
            }
          }
        }
        return EntityCache::DatabaseTable {
          entity: Rc::clone(&entity),
          fields_by_sql_column_name,
          fields_by_graphql_field_name,
        };
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
  fn constructor() {
    let value = Rc::new(Entity::DatabaseTable {
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
    });

    insta::assert_debug_snapshot!(EntityCache::new(Rc::clone(&value)));
    insta::assert_debug_snapshot!(serde_json::to_string_pretty(&EntityCache::new(Rc::clone(
      &value
    )))
    .unwrap());
  }
}
