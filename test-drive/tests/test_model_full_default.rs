use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};
use std::error::Error;

mod settings {
    // Project name.
    // Valid characters: _ a-z A-Z 0-9
    // Hint: PROJECT_NAM it is recommended not to change.
    // Max size: 20
    // First character: a-z A-Z
    pub const APP_NAME: &str = "test_app_name";
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 20
    // First character: a-z A-Z
    pub const DATABASE_NAME: &str = "test_app_name";
    // The unique key for this test.
    // To generate a key (This is not an advertisement): https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 16
    pub const UNIQUE_APP_KEY: &str = "dNTFT8173D6i3255";
    //
    pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 30
    // First character: a-z A-Z
    pub const SERVICE_NAME: &str = "test_service_name";
}

mod models {
    use super::*;
    use settings::{APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME, UNIQUE_APP_KEY};

    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        pub checkbox: BoolField,
        //
        pub date: DateField,
        pub datetime: DateTimeField,
        //
        pub file: FileField,
        pub image: ImageField,
        //
        pub number_i32: I32Field,
        //
        pub number_u32: U32Field,
        //
        pub number_i64: I64Field,
        //
        pub number_f64: F64Field,
        //
        pub select_text: ChoiceTextField,
        pub select_text_dyn: ChoiceTextDynField,
        pub select_text_mult: ChoiceTextMultField,
        pub select_text_mult_dyn: ChoiceTextMultDynField,
        //
        pub select_i32: ChoiceI32Field,
        pub select_i32_dyn: ChoiceI32DynField,
        pub select_i32_mult: ChoiceI32MultField,
        pub select_i32_mult_dyn: ChoiceI32MultDynField,
        //
        pub select_u32: ChoiceU32Field,
        pub select_u32_dyn: ChoiceU32DynField,
        pub select_u32_mult: ChoiceI32MultField,
        pub select_u32_mult_dyn: ChoiceU32MultDynField,
        //
        pub select_i64: ChoiceI64Field,
        pub select_i64_dyn: ChoiceI64DynField,
        pub select_i64_mult: ChoiceI64MultField,
        pub select_i64_mult_dyn: ChoiceI64MultDynField,
        //
        pub select_f64: ChoiceF64Field,
        pub select_f64_dyn: ChoiceF64DynField,
        pub select_f64_mult: ChoiceF64MultField,
        pub select_f64_mult_dyn: ChoiceF64MultDynField,
        //
        pub text: TextField,
        pub slug: SlugField,
        pub color: ColorField,
        pub email: EmailField,
        pub password: PasswordField,
        pub phone: PhoneField,
        pub url: URLField,
        pub ip: IPField,
    }

    impl Control for TestModel {
        fn custom() -> Self {
            Self {
                ..Default::default()
            }
        }
    }
}

mod migration {
    use super::*;

    // Get metadata list
    pub fn get_model_key_list() -> Result<Vec<String>, Box<dyn Error>> {
        let model_key_list = vec![models::TestModel::key()?];
        Ok(model_key_list)
    }

    // Migration
    pub async fn run_migration(client: &Client) -> Result<(), Box<dyn Error>> {
        // Caching metadata.
        models::TestModel::caching(client).await?;

        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_db(
            client,
            settings::APP_NAME,
            settings::UNIQUE_APP_KEY,
            get_model_key_list()?,
        )
        .await?;

        // Monitor initialization.
        let monitor = Monitor {
            app_name: settings::APP_NAME,
            unique_app_key: settings::UNIQUE_APP_KEY,
            // Register models
            model_key_list: get_model_key_list()?,
        };
        monitor.migrat(client).await?;

        Ok(())
    }
}

// TEST
// #################################################################################################
#[tokio::test]
async fn test_model_full_default() -> Result<(), Box<dyn Error>> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    //
    migration::run_migration(&client).await?;

    // YOUR CODE ...
    // =============================================================================================
    type TestModel = models::TestModel;
    //
    // Module: green-barrel/src/models/caching.rs
    // ---------------------------------------------------------------------------------------------
    // new
    assert!(TestModel::new().await.is_ok(), "new() != is_ok()");
    // to_json
    assert!(!TestModel::json().await?.is_empty(), "json() != is_empty()");
    //
    // Module: green-barrel/src/models/db_query_api/commons.rs
    // ---------------------------------------------------------------------------------------------
    // aggregate
    let pipeline = vec![doc! {}];
    let result = TestModel::aggregate(&client, pipeline, None).await;
    assert!(result.is_err(), "aggregate() != is_err()");
    // count_documents
    let result = TestModel::count_documents(&client, None, None).await?;
    assert_eq!(result, 0, "count_documents() != 0");
    // delete_many
    let query = doc! {};
    let result = TestModel::delete_many(&client, query, None).await?;
    assert!(result.is_valid(), "is_valid(): {}", result.err_msg());
    assert!(
        result.deleted_count()? == 0,
        "delete_many(): deleted_count() != 0"
    );
    // delete_one
    let query = doc! {};
    let result = TestModel::delete_one(&client, query, None).await?;
    assert!(result.is_valid(), "is_valid(): {}", result.err_msg());
    assert!(result.deleted_count()? == 0, "delete_one() != 0");
    // distinct
    let field_name = "checkbox";
    let filter = doc! {};
    let result = TestModel::distinct(&client, field_name, Some(filter), None).await?;
    assert!(result.is_empty(), "distinct() != is_empty()");
    // estimated_document_count
    let result = TestModel::estimated_document_count(&client, None).await?;
    assert_eq!(result, 0, "estimated_document_count != 0_i64");
    // find_many_to_doc_list
    let result = TestModel::find_many_to_doc_list(&client, None, None).await?;
    assert!(result.is_none(), "find_many_to_doc_list() != is_none()");
    // find_many_to_json
    let result = TestModel::find_many_to_json(&client, None, None).await?;
    assert!(result.is_none(), "find_many_to_json() != is_none");
    // find_one_to_doc
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_doc(&client, filter, None).await?;
    assert!(result.is_none(), "find_one_to_doc() != is_none()");
    // find_one_to_json
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_json(&client, filter, None).await?;
    assert!(result.is_empty(), "find_one_to_json() != is_empty()");
    // find_one_to_instance
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_instance(&client, filter, None).await?;
    assert!(result.is_none(), "find_one_to_instance() != is_none()");
    // find_one_and_delete
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_and_delete(&client, filter, None).await?;
    assert!(result.is_none(), "find_one_and_delete() != is_none()");
    // collection_name
    let result = TestModel::collection_name(&client).await?;
    assert!(!result.is_empty(), "name() != is_empty()");
    // namespace
    let result = TestModel::namespace(&client).await?;
    assert!(!result.db.is_empty(), "namespace() != is_empty()");
    assert!(!result.coll.is_empty(), "namespace() != is_empty()");

    // Delete test database
    // =============================================================================================
    del_test_db(
        &client,
        settings::APP_NAME,
        settings::UNIQUE_APP_KEY,
        migration::get_model_key_list()?,
    )
    .await?;

    Ok(())
}
