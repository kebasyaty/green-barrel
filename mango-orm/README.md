[![Logo](https://github.com/kebasyaty/mango-orm/raw/master/images/logo.svg "Logo")](https://github.com/kebasyaty/mango-orm "Logo")

# mango-orm

#### ORM-like API MongoDB for Rust

**To simulate fields of type ForeignKey and ManyToMany, a simplified alternative (Dynamic Widgets) is used. For examples of how to add fields to the Model, see [tests](https://github.com/kebasyaty/mango-orm/tree/master/test-drive/tests "tests"). For maximum convenience use [mango-panel](https://github.com/kebasyaty/mango-panel "mango-panel").**

[![crates.io](https://img.shields.io/crates/v/mango-orm "crates.io")](https://crates.io/crates/mango-orm "crates.io")
[![crates.io](https://img.shields.io/static/v1?label=rustc&message=v1.52%2B&color=red "crates.io")](https://www.rust-lang.org/ "crates.io")
![crates.io](https://img.shields.io/crates/d/mango-orm)
![crates.io](https://img.shields.io/crates/l/mango-orm)

## Attention

#### [MongoDB Rust Driver](https://crates.io/crates/mongodb/1.2.5 "MongoDB Rust Driver") version 1.2.5 is used.

**[Mango-panel](https://github.com/kebasyaty/mango-panel "mango-panel") is the recommended part of the entire ecosystem ( [mango-orm](https://github.com/kebasyaty/mango-orm "mango-orm"), [metamorphose](https://github.com/kebasyaty/mango-orm/tree/master/metamorphose "metamorphose"), [mango-panel](https://github.com/kebasyaty/mango-panel "mango-panel") ). For those who use [mango-panel](https://github.com/kebasyaty/mango-panel "mango-panel") - Follow our updates.**

## Requirements

- [mongodb](https://crates.io/crates/mongodb/1.2.5 "mongodb")
- [serde](https://crates.io/crates/serde "serde")
- [chrono](https://crates.io/crates/chrono "chrono")
- [image](https://crates.io/crates/image "image")
- [lazy_static](https://crates.io/crates/lazy_static "lazy_static")
- [rand](https://crates.io/crates/rand "rand")
- [regex](https://crates.io/crates/regex "regex")
- [rust-argon2](https://crates.io/crates/rust-argon2 "rust-argon2")
- [serde_json](https://crates.io/crates/serde_json "serde_json")
- [slug](https://crates.io/crates/slug "slug")
- [validator](https://crates.io/crates/validator "validator")
- [uuid](https://crates.io/crates/uuid "uuid")
- [metamorphose](https://crates.io/crates/metamorphose "metamorphose")

## Model attributes

**_( all attributes are optional )_**

| Attribute:          | Default:     | Description:                                                                |
| :------------------ | :----------- | :-------------------------------------------------------------------------- |
| db_client_name      | empty string | Used to connect to a MongoDB cluster.                                       |
| db_query_docs_limit | 1000         | limiting query results.                                                     |
| is_add_docs         | true         | Create documents in the database.                                           |
| is_up_docs          | true         | Update documents in the database.                                           |
| is_del_docs         | true         | Delete documents from the database.                                         |
| ignore_fields       | empty string | Fields that are not included in the database (separated by commas).         |
| is_use_add_valid | false | Allows additional validation - **impl AdditionalValidation for ModelName**. |
| is_use_hooks | false | Allows hooks methods - **impl Hooks for ModelName**. |

## Match field types and widget types

| Field type:             | Widget type:        |
| :---------------------- | :------------------ |
| Option< bool >          | "checkBox"          |
| -                       | -                   |
| Option< String >        | "inputSlug"         |
| -                       | -                   |
| Option< String >        | "inputColor"        |
| Option< String >        | "inputDate"         |
| Option< String >        | "inputDateTime"     |
| Option< String >        | "inputEmail"        |
| Option< String >        | "inputPassword"     |
| Option< String >        | "inputPhone"        |
| Option< String >        | "inputText"         |
| Option< String >        | "inputUrl"          |
| Option< String >        | "inputIP"           |
| Option< String >        | "inputIPv4"         |
| Option< String >        | "inputIPv6"         |
| -                       | -                   |
| Option< String >        | "textArea"          |
| -                       | -                   |
| Option< String >        | "inputFile"         |
| Option< String >        | "inputImage"        |
| -                       | -                   |
| Option< i32 >           | "numberI32"         |
| Option< u32 >           | "numberU32"         |
| Option< i64 >           | "numberI64"         |
| Option< f64 >           | "numberF64"         |
| -                       | -                   |
| Option< String >        | "radioText"         |
| Option< i32 >           | "radioI32"          |
| Option< u32 >           | "radioU32"          |
| Option< i64 >           | "radioI64"          |
| Option< f64 >           | "radioF64"          |
| -                       | -                   |
| Option< i32 >           | "rangeI32"          |
| Option< u32 >           | "rangeU32"          |
| Option< i64 >           | "rangeI64"          |
| Option< f64 >           | "rangeF64"          |
| -                       | -                   |
| Option< String >        | "selectText"        |
| Option< String >        | "selectTextDyn"     |
| Option< Vec< String > > | "selectTextMult"    |
| Option< Vec< String > > | "selectTextMultDyn" |
| Option< i32 >           | "selectI32"         |
| Option< i32 >           | "selectI32Dyn"      |
| Option< Vec< i32 > >    | "selectI32Mult"     |
| Option< Vec< i32 > >    | "selectI32MultDyn"  |
| Option< u32 >           | "selectU32"         |
| Option< u32 >           | "selectU32Dyn"      |
| Option< Vec< u32 > >    | "selectU32Mult"     |
| Option< Vec< u32 > >    | "selectU32MultDyn"  |
| Option< i64 >           | "selectI64"         |
| Option< i64 >           | "selectI64Dyn"      |
| Option< Vec< i64 > >    | "selectI64Mult"     |
| Option< Vec< i64 > >    | "selectI64MultDyn"  |
| Option< f64 >           | "selectF64"         |
| Option< f64 >           | "selectF64Dyn"      |
| Option< Vec< f64 > >    | "selectF64Mult"     |
| Option< Vec< f64 > >    | "selectF64MultDyn"  |
| -                       | -                   |
| Option< String >        | "hiddenText"        |
| Option< i32 >           | "hiddenI32"         |
| Option< u32 >           | "hiddenU32"         |
| Option< i64 >           | "hiddenI64"         |
| Option< f64 >           | "hiddenF64"         |

## Field attributes

**_( all attributes are optional )_**

| Attribute:   | Default:     | Description:                                                                                      |
| :----------- | :----------- | :------------------------------------------------------------------------------------------------ |
| id           | empty string | The value is determined automatically. Format: "model-name--field-name".                          |
| label        | empty string | Web form field name.                                                                              |
| widget       | "inputText"  | Widget name.                                                                                      |
| input_type   | "text"       | The value is determined automatically.                                                            |
| name         | empty string | The value is determined automatically.                                                            |
| value        | empty string | Default value.                                                                                    |
| accept       | empty string | Example: "image/jpeg,image/png,image/gif"                                                         |
| placeholder  | empty string | Displays prompt text.                                                                             |
| pattern      | empty string | Validating a field using a client-side regex.                                                     |
| minlength    | 0            | The minimum number of characters allowed in the text.                                             |
| maxlength    | 256          | The maximum number of characters allowed in the text.                                             |
| required     | false        | Mandatory field.                                                                                  |
| checked      | false        | A pre-activated radio button or checkbox.                                                         |
| unique       | false        | The unique value of a field in a collection.                                                      |
| disabled     | false        | Blocks access and modification of the element.                                                    |
| readonly     | false        | Specifies that the field cannot be modified by the user.                                          |
| step         | "1"          | Increment step for numeric fields.                                                                |
| min          | empty string | The lower value for entering a number or date.                                                    |
| max          | empty string | The top value for entering a number or date.                                                      |
| options      | empty array  | Example: r#"[[1,"Volvo"], [2,"Saab"]]"#                                                           |
| thumbnails   | empty array  | From one to four inclusive. Example: r#"[["xs",150],["sm",300],["md",600],["lg",1200]]"#          |
| slug_sources | empty array  | Example: r#"["title"]"# or r#"["hash", "username"]"# or r#"["email", "first_name", "last_name"]"# |
| is_hide      | false        | Hide field from user.                                                                             |
| other_attrs  | empty string | Example: r# "autofocus tabindex="some number" size="some number""#                                |
| css_classes  | empty string | Example: "class-name-1 class-name-2"                                                              |
| hint         | empty string | Additional explanation for the user.                                                              |
| warning      | empty string | The value is determined automatically.                                                            |
| error        | empty string | The value is determined automatically.                                                            |
| common_msg   | empty string | Messages common to the entire Form. The value is determined automatically.                        |

## Install mongodb (if not installed)

    ### Ubuntu, Mint:
    $ sudo apt install mongodb
    ## OR
    ### Ubuntu 20.04, Mint 20.x:
    $ sudo apt update
    $ sudo apt install dirmngr gnupg apt-transport-https ca-certificates
    $ wget -qO - https://www.mongodb.org/static/pgp/server-4.4.asc | sudo apt-key add -
    $ sudo add-apt-repository 'deb [arch=amd64] https://repo.mongodb.org/apt/ubuntu focal/mongodb-org/4.4 multiverse'
    $ sudo apt update
    $ sudo apt install mongodb-org
    $ sudo systemctl enable --now mongod
    # For check
    $ mongod --version
    $ mongo --eval 'db.runCommand({ connectionStatus: 1 })'
    $ reboot
    #
    ### Configuration file:
    $ sudo nano /etc/mongod.conf
    #
    ### Systemd:
    $ sudo systemctl status mongod
    $ sudo systemctl start mongod
    $ sudo systemctl stop mongod
    $ sudo systemctl restart mongod
    $ sudo systemctl enable mongod
    $ sudo systemctl disable mongod
    #
    ### Uninstall:
    $ sudo systemctl stop mongod
    $ sudo systemctl disable mongod
    $ sudo apt --purge remove mongodb\*  # OR (for mongodb-org) - $ sudo apt --purge remove mongodb-org\**
    $ sudo rm -r /var/log/mongodb
    $ sudo rm -r /var/lib/mongodb
    $ sudo rm -f /etc/mongod.conf
    $ sudo apt-add-repository --remove 'deb [arch=amd64] https://repo.mongodb.org/apt/ubuntu focal/mongodb-org/4.4 multiverse' # for mongodb-org
    $ sudo apt update

## Example Usage:

#### Cargo.toml

    [dependencies]
    mango-orm = "0.9"
    metamorphose = "0.5"
    regex = "1.5.6"
    serde_json = "1.0.81"

    [dependencies.mongodb]
    default-features = false
    features = ["sync"]
    version = "1.2.5"

    [dependencies.serde]
    features = ["derive"]
    version = "1.0.137"

#### src/settings.rs

    // General settings for the project.
    // Project name.
    // Hint: PROJECT_NAM it is recommended not to change.
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 21
    // First character: a-z A-Z
    pub const PROJECT_NAME: &str = "store";

    // Unique project key.
    // Hint: UNIQUE_PROJECT_KEY it is recommended not to change.
    // Valid characters: a-z A-Z 0-9
    // Size: 8-16
    // Example: "7rzgacfqQB3B7q7T"
    pub const UNIQUE_PROJECT_KEY: &str = "bhjRV8ry9X5LQBw";

    // Settings for user accounts.
    pub mod users {
        // Valid characters: _ a-z A-Z 0-9
        // Max size: 31
        // First character: a-z A-Z
        pub const SERVICE_NAME: &str = "accounts";
        // Valid characters: _ a-z A-Z 0-9
        // Max size: 21
        // First character: a-z A-Z
        pub const DATABASE_NAME: &str = "accounts";
        pub const DB_CLIENT_NAME: &str = "default";
        pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
    }

#### src/migration.rs

    use crate::{models, settings};
    use mango_orm::{Monitor, ToModel, MONGODB_CLIENT_STORE};

    // Migration Service `Mango`.
    pub fn mango_migration() -> Result<(), Box<dyn std::error::Error>> {
        // Caching MongoDB clients.
        {
            let mut client_store = MONGODB_CLIENT_STORE.write()?;
            client_store.insert(
                "default".to_string(),
                mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
            );
        }
        // Monitor initialization.
        let monitor = Monitor {
            project_name: settings::PROJECT_NAME,
            unique_project_key: settings::UNIQUE_PROJECT_KEY,
            // Register models.
            models: vec![models::UserProfile::meta()?],
        };
        // Run migration
        monitor.migrat()?;
        //
        Ok(())
    }

#### src/models.rs

    use mango_orm::*;
    use metamorphose::Model;
    use regex::RegexBuilder;
    use serde::{Deserialize, Serialize};

    use crate::settings::{
        users::{DATABASE_NAME, DB_CLIENT_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME},
        PROJECT_NAME, UNIQUE_PROJECT_KEY,
    };

    // User profiles
    #[Model(
        is_del_docs = false,
        is_use_add_valid = true,
        ignore_fields = "confirm_password"
    )]
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct UserProfile {
        #[serde(default)]
        #[field_attrs(
            widget = "inputText",
            label = "Username",
            placeholder = "Enter your username",
            unique = true,
            required = true,
            maxlength = 150,
            hint = "Valid characters: a-z A-Z 0-9 _ @ + .<br>Max size: 150"
        )]
        pub username: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputSlug",
            label = "Slug",
            unique = true,
            readonly = true,
            is_hide = true,
            hint = "To create a human readable url",
            slug_sources = r#"["hash", "username"]"#
        )]
        pub slug: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputText",
            label = "First name",
            placeholder = "Enter your First name",
            maxlength = 150
        )]
        pub first_name: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputText",
            label = "Last name",
            placeholder = "Enter your Last name",
            maxlength = 150
        )]
        pub last_name: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputEmail",
            label = "E-mail",
            placeholder = "Please enter your email",
            required = true,
            unique = true,
            maxlength = 320,
            hint = "Your actual E-mail"
        )]
        pub email: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputPhone",
            label = "Phone number",
            placeholder = "Please enter your phone number",
            unique = true,
            maxlength = 30,
            hint = "Your actual phone number"
        )]
        pub phone: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputPassword",
            label = "Password",
            placeholder = "Enter your password",
            required = true,
            minlength = 8,
            hint = "Valid characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (<br>Min size: 8"
        )]
        pub password: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputPassword",
            label = "Confirm password",
            placeholder = "Repeat your password",
            required = true,
            minlength = 8,
            hint = "Repeat your password"
        )]
        pub confirm_password: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "checkBox",
            label = "is staff?",
            checked = true,
            hint = "User can access the admin site?"
        )]
        pub is_staff: Option<bool>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "checkBox",
            label = "is active?",
            checked = true,
            hint = "Is this an active account?"
        )]
        pub is_active: Option<bool>,
    }

    impl AdditionalValidation for UserProfile {
        fn add_validation<'a>(
            &self,
        ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
            // Hint: error_map.insert("field_name", "Error message.")
            let mut error_map: std::collections::HashMap<&'a str, &'a str> =
                std::collections::HashMap::new();

            // Get clean data
            let hash = self.hash.clone().unwrap_or_default();
            let password = self.password.clone().unwrap_or_default();
            let confirm_password = self.confirm_password.clone().unwrap_or_default();
            let username = self.username.clone().unwrap_or_default();

            // Fields validation
            if hash.is_empty() && password != confirm_password {
                error_map.insert("confirm_password", "Password confirmation does not match.");
            }
            if !RegexBuilder::new(r"^[a-z\d_@+.]+$")
                .case_insensitive(true)
                .build()
                .unwrap()
                .is_match(username.as_str())
            {
                error_map.insert(
                    "username",
                    "Invalid characters present.<br>\
                     Valid characters: a-z A-Z 0-9 _ @ + .",
                );
            }

            Ok(error_map)
        }
    }

#### src/main.rs

    mod migration;
    mod models;
    mod settings;

    use mango_orm::*;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Run migration.
        migration::mango_migration()?;

        //
        let mut user = models::UserProfile {
            username: Some("user_1".to_string()),
            email: Some("user_1@@noreply.net".to_string()),
            password: Some("12345678".to_string()),
            confirm_password: Some("12345678".to_string()),
            is_staff: Some(false),
            ..Default::default() // or initialize the `hash` field - { hash: Some(String::new()) }
        };

        // Get data for form fields.
        // println!("Widget map:\n{:?}", models::UserProfile::to_wig()?);
        // println!("Json-line:\n{}", models::UserProfile::to_json()?);
        // println!("Html code:\n{}", models::UserProfile::to_html()?);
        //
        //
        // Check.
        // let result = user.check()?;
        // println!("Is valid: {}", result.is_valid());
        // println!("Hash: {}", result.hash()?);
        //
        // Get MongoDB ID from hash-line
        // println!("\nObject Id:\n{:?}\n", result.object_id()?);
        //
        // println!("\n\nWidget map:\n{:?}", result.to_wig());
        // println!("\n\nJson:\n{}", result.to_json()?);
        // println!("\n\nHtml:\n{}", result.to_html());
        // println!("\n\nBSON::Document:\n{:?}", result.to_doc());
        //
        // Printing errors to the console ( for development ).
        // if !result.is_valid() {
        //     result.print_err();
        // }
        //
        // Get data for form fields.
        // println!("Widget map:\n{:?}", result.to_wig());
        // println!("Json-line:\n{}", result.to_json()?);
        // println!("Html code:\n{}", result.to_html());
        //
        //
        // Create or update a document in the database ( check() + save() ).
        let result = user.save(None, None)?;
        println!("Is valid: {}", result.is_valid());
        println!("Hash: {}", result.hash()?);
        //
        // Get MongoDB ID from hash-line
        // println!("\nObject Id:\n{:?}\n", result.object_id()?);
        //
        println!("\nSlug: {}\n", result.to_wig().get("slug").unwrap().value);
        //
        // Printing errors to the console ( for development ).
        // if !result.is_valid() {
        //     result.print_err();
        // }
        // Get data for form fields.
        // println!("Widget map:\n{:?}", result.to_wig());
        // println!("Json-line:\n{}", result.to_json()?);
        // println!("Html code:\n{}", result.to_html());
        //
        //
        // Remove document from collection in database.
        // let output_data  = user.delete(None)?;
        // if !output_data.is_valid() {
        //     println!("{}", output_data.err_msg());
        // }
        //
        Ok(())
    }

## Changelog
- **v0.9.0** *Added hook methods. See documentation: **mango_orm > models > db_query_api > paladins > QPaladins**.*
- **v0.8.26** *Add find_one_to_wig method. See documentation: **mango_orm > models > db_query_api > commons > QCommons**.*
- **v0.8.0** *Deep modernization of common.rs and output_data.rs modules. See documentation: **mango_orm > models > db_query_api > commons > QCommons** and **mango_orm > models > output_data > Converters**.*
- **v0.7.4** *Updated **README.md**, added model attributes.*
- **v0.7.0** *Added the ability to use the hash field in inputSlug - **slug_sources: r#"["hash", "username"]"#\***
- **v0.6.30** *Renamed methods: **wig()**, **json()**, **html()** -> **to_wig()**, **to_json()**, **to_html()**. Updated **README.md**. Updated documentation. Updated versions of dependencies.*
- **v0.6.16** *Renamed the Forms module to Widgets.*
- **v0.6.15** *Updating by version of dependencies.*
- **v0.6.10** *Updated test for dynamic widgets.*
- **v0.6.7** *Removed **hiddenSlug** field.*
- **v0.6.6** *Added **is_hide** parameter for Widgets.*
- **v0.6.5** *In the check() method, errors are redirected to the console, for fields of hidden type.*
- **v0.6.4** *Fixes for fields of slug type.*
- **v0.6** *1) Added inputSlug and hiddenSlug fields. 2) Fix - Added fields of hidden type to migration.*
- **v0.5.4** *Optimization for creating thumbnails, for default images.*
- **v0.5.3** *Improved cleaning of orphaned files.*
- **v0.5** *Support for the Form macro has been removed.*

## License

#### This project is licensed under the [MIT](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-MIT "MIT") and [Apache Version 2.0](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-APACHE "Apache Version 2.0")
