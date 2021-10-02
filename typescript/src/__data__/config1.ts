import { connector } from "../connectors/postgres";
import { GraphSchema } from "../graphschema/types";

export const graphSchema: GraphSchema<typeof connector["typeReference"]> = {
  databaseConnectionUri: {
    protocol: "postgres",
    user: "yoyo",
    password: "dd",
    host: "localhost",
    port: 5432,
    database: "toto",
    schema: "public"
  },
  entities: [
    {
      name: "User",
      type: "databaseTable",
      databaseSchemaName: "public",
      databaseTableName: "user",
      graphqlEntityTypeName: "User",
      graphqlFilterTypeName: "UserFilter",
      graphqlGetSingleOperationName: "user",
      graphqlGetListOperationName: "users",
      graphqlGetConnectionOperationName: "userConnection",
      graphqlDefaultOrderBy: "idAsc",
      graphqlDefaultFirst: 11,
      graphqlDefaultOffset: 0,
      fields: [
        {
          name: "id",
          type: "scalarDatabaseColumn",
          databaseTypeName: "text",
          databaseColumnName: "id",
          graphqlFieldName: "id",
          graphqlTypeName: "String",
          graphqlOrderByAsc: "idAsc",
          graphqlOrderByDesc: "idDesc"
        },
        {
          name: "name",
          type: "scalarDatabaseColumn",
          databaseTypeName: "text",
          databaseColumnName: "name",
          graphqlTypeName: "String",
          graphqlFieldName: "name",
          graphqlOrderByAsc: "nameAsc",
          graphqlOrderByDesc: "nameDesc"
        },
        {
          name: "email",
          type: "scalarDatabaseColumn",
          databaseTypeName: "text",
          databaseColumnName: "email",
          graphqlTypeName: "String",
          graphqlFieldName: "email",
          graphqlOrderByAsc: "emailAsc",
          graphqlOrderByDesc: "emailDesc"
        },
        {
          name: "organizations",
          type: "relationMany",
          relationName: "UserOrganization",
          graphqlGetConnectionFieldName: "organizationsConnection",
          graphqlGetListFieldName: "organizations",
          graphqlGetSingleFieldName: "organization",
          graphqlOrderByPrefix: "organizations"
        }

        // {
        //   name: "address",
        //   kind: "relationOne",
        //   relationName: "UserAddress",
        //   graphqlFieldName: "address",
        //   graphqlTypeName: "Address",
        //   graphqlGetSingleField: "address",
        //   graphqlOrderByAsc: "addressAsc",
        //   graphqlOrderByDesc: "addressDesc"
        // }
      ]
    },
    {
      name: "Address",
      type: "databaseTable",
      databaseSchemaName: "public",
      databaseTableName: "address",
      graphqlEntityTypeName: "Address",
      graphqlFilterTypeName: "AddressFilter",
      graphqlGetSingleOperationName: "address",
      graphqlGetListOperationName: "addresses",
      graphqlGetConnectionOperationName: "addressConnection",
      graphqlDefaultOrderBy: "idAsc",
      graphqlDefaultFirst: 13,
      graphqlDefaultOffset: 0,
      fields: [
        {
          name: "id",
          type: "scalarDatabaseColumn",
          databaseTypeName: "text",
          databaseColumnName: "id",
          graphqlFieldName: "id",
          graphqlTypeName: "String",
          graphqlOrderByAsc: "idAsc",
          graphqlOrderByDesc: "idDesc"
        },
        {
          name: "name",
          type: "scalarDatabaseColumn",
          databaseTypeName: "text",
          databaseColumnName: "name",
          graphqlFieldName: "name",
          graphqlTypeName: "String",
          graphqlOrderByAsc: "nameAsc",
          graphqlOrderByDesc: "nameDesc"
        }
        // {
        //   name: "users",
        //   kind: "relationOne",
        //   graphqlTypeName: "[User!]!"
        //   relationName: "Tutu",
        //   list: "users",
        //   connection: "usersConnection",
        //   getSingle: "user",
        //   graphqlOrderByAsc: "user",
        //   graphqlOrderByDesc: "user"
        // }
      ]
    },
    {
      name: "Organization",
      type: "databaseTable",
      databaseSchemaName: "public",
      databaseTableName: "organization",
      graphqlEntityTypeName: "Organization",
      graphqlFilterTypeName: "OrganizationFilter",
      graphqlGetSingleOperationName: "organization",
      graphqlGetListOperationName: "organizations",
      graphqlGetConnectionOperationName: "organizationsConnection",
      graphqlDefaultOrderBy: "idAsc",
      graphqlDefaultFirst: 17,
      graphqlDefaultOffset: 0,
      fields: [
        {
          name: "id",
          type: "scalarDatabaseColumn",
          databaseTypeName: "text",
          databaseColumnName: "id",
          graphqlFieldName: "id",
          graphqlTypeName: "String",
          graphqlOrderByAsc: "idAsc",
          graphqlOrderByDesc: "idDesc"
        },
        {
          name: "name",
          type: "scalarDatabaseColumn",
          databaseTypeName: "text",
          databaseColumnName: "name",
          graphqlFieldName: "name",
          graphqlTypeName: "String",
          graphqlOrderByAsc: "nameAsc",
          graphqlOrderByDesc: "nameDesc"
        },
        {
          name: "users",
          type: "relationMany",
          relationName: "UserOrganization",
          graphqlGetConnectionFieldName: "usersConnection",
          graphqlGetListFieldName: "users",
          graphqlGetSingleFieldName: "user",
          graphqlOrderByPrefix: "users"
        }
        // {
        //   name: "users",
        //   kind: "relationOne",
        //   graphqlTypeName: "[User!]!"
        //   relationName: "Tutu",
        //   list: "users",
        //   connection: "usersConnection",
        //   getSingle: "user",
        //   graphqlOrderByAsc: "user",
        //   graphqlOrderByDesc: "user"
        // }
      ]
    }
  ],
  relations: [
    {
      name: "UserOrganization",
      type: "manyToManyRelationTable",

      leftEntityName: "User",
      leftEntityFieldName: "organizations",
      leftDatabaseColumnNames: ["id"],

      relationLeftDatabaseColumnNames: ["user_id"],
      relationToLeftDatabaseForeignKeyConstraintName: "user_fkey",
      relationDatabaseSchemaName: "public",
      relationDatabaseTableName: "user_organization",
      relationToRightDatabaseForeignKeyConstraintName: "organization_fkey",
      relationRightDatabaseColumnNames: ["organization_id"],

      rightDatabaseColumnNames: ["id"],
      rightEntityFieldName: "users",
      rightEntityName: "Organization"
    }
    // {
    //   name: "UserAddress",
    //   source: { type: "constraint", name: "tutu" }
    // }
  ]
};
