![Logo](https://github.com/kebasyaty/mango-orm/raw/master/metamorphose/images/logo.svg)

# Metamorphose

### Macros collection for converting Structure to Model, for a [Green Barrel](https://github.com/kebasyaty/green-barrel "green-barrel") project.

[![crates.io](https://img.shields.io/crates/v/metamorphose "crates.io")](https://crates.io/crates/metamorphose "crates.io")
[![crates.io](https://img.shields.io/static/v1?label=rustc&message=v1.57%2B&color=red "crates.io")](https://www.rust-lang.org/ "crates.io")
![crates.io](https://img.shields.io/crates/d/metamorphose)
![crates.io](https://img.shields.io/crates/l/metamorphose)

## Macros

#### Model

> Macro for converting Structure to green-barrel Model.
> The model can access the database.
> The model can create, update, and delete documents in collections.

## Requirements

- [quote](https://crates.io/crates/quote "quote")
- [regex](https://crates.io/crates/regex "regex")
- [serde_json](https://crates.io/crates/serde_json "serde_json")
- [syn](https://crates.io/crates/syn "syn")
- [serde](https://crates.io/crates/serde "serde")

## Install mongodb (if not installed)

```shell
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
```

## Example Usage:

#### Cargo.toml

```toml
[dependencies]
green-barrel = "1.1.4-beta"
metamorphose = "1.1.4-beta"
regex = "1.6.0"
serde_json = "1.0.85"

[dependencies.mongodb]
default-features = false
features = ["sync"]
version = "1.2.5"

[dependencies.serde]
features = ["derive"]
version = "1.0.145"
```

#### src/settings.rs

```rust
// General settings for the project.
// Project name.
// Hint: PROJECT_NAM it is recommended not to change.
// Valid characters: _ a-z A-Z 0-9
// Max size: 20
// First character: a-z A-Z
pub const PROJECT_NAME: &str = "store";

// Unique project key.
// Hint: UNIQUE_PROJECT_KEY it is recommended not to change.
// Valid characters: a-z A-Z 0-9
// Size: 16
// Example: "7rzgacfqQB3B7q7T"
// To generate a key: https://randompasswordgen.com/
pub const UNIQUE_PROJECT_KEY: &str = "A3iBcq9K19287PN3";

// Settings for user accounts.
pub mod users {
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 30
    // First character: a-z A-Z
    pub const SERVICE_NAME: &str = "accounts";
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 20
    // First character: a-z A-Z
    pub const DATABASE_NAME: &str = "accounts";
    //
    pub const DB_CLIENT_NAME: &str = "default";
    pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
}
```

#### src/migration.rs

```rust
use crate::{models, settings};
use green_barrel::{Caching, Monitor, MONGODB_CLIENT_STORE};
use std::error::Error;

// Migration
pub fn run_migration() -> Result<(), Box<dyn Error>> {
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
        metadata_list: vec![models::User::meta()?],
    };
    // Run migration
    monitor.migrat()?;

    Ok(())
}
```

#### src/models.rs

```rust
use green_barrel::*;
use metamorphose::Model;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

use crate::settings::{
    users::{DATABASE_NAME, DB_CLIENT_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME},
    PROJECT_NAME, UNIQUE_PROJECT_KEY,
};

#[Model(
    is_use_add_valid = true,
    is_use_hooks = true,
    ignore_fields = "confirm_password" // Example: "field_name, field_name_2"
)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: InputText,
    pub slug: AutoSlug,
    pub first_name: InputText,
    pub last_name: InputText,
    pub email: InputEmail,
    pub phone: InputPhone,
    pub password: InputPassword,
    pub confirm_password: InputPassword,
    pub is_staff: CheckBox,
    pub is_active: CheckBox,
}

impl Control for User {
    fn custom_default() -> Self {
        Self {
            username: InputText {
                label: "Username".into(),
                placeholder: "Enter your username".into(),
                maxlength: 150,
                required: true,
                unique: true,
                hint: "Valid characters: a-z A-Z 0-9 _ @ + .<br>Max size: 150".into(),
                ..Default::default()
            },
            slug: AutoSlug {
                label: "Slug".into(),
                unique: true,
                readonly: true,
                hint: "To create a human readable url".into(),
                slug_sources: vec!["hash".into(), "username".into()],
                ..Default::default()
            },
            first_name: InputText {
                label: "First name".into(),
                placeholder: "Enter your First name".into(),
                maxlength: 150,
                ..Default::default()
            },
            last_name: InputText {
                label: "Last name".into(),
                placeholder: "Enter your Last name".into(),
                maxlength: 150,
                ..Default::default()
            },
            email: InputEmail {
                label: "E-mail".into(),
                placeholder: "Please enter your email".into(),
                required: true,
                unique: true,
                maxlength: 320,
                hint: "Your actual E-mail".into(),
                ..Default::default()
            },
            phone: InputPhone {
                label: "Phone number".into(),
                placeholder: "Please enter your phone number".into(),
                unique: true,
                maxlength: 30,
                hint: "Your actual phone number".into(),
                ..Default::default()
            },
            password: InputPassword {
                label: "Password".into(),
                placeholder: "Enter your password".into(),
                required: true,
                minlength: 8,
                hint: "Valid characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (<br>Min size: 8"
                    .into(),
                ..Default::default()
            },
            confirm_password: InputPassword {
                label: "Confirm password".into(),
                placeholder: "Repeat your password".into(),
                required: true,
                minlength: 8,
                ..Default::default()
            },
            is_staff: CheckBox {
                label: "is staff?".into(),
                checked: Some(true),
                hint: "User can access the admin site?".into(),
                ..Default::default()
            },
            is_active: CheckBox {
                label: "is active?".into(),
                checked: Some(true),
                hint: "Is this an active account?".into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl AdditionalValidation for User {
    fn add_validation<'a>(&self) -> Result<HashMap<&'a str, &'a str>, Box<dyn Error>> {
        // Hint: error_map.insert("field_name", "Error message.")
        let mut error_map = HashMap::<&'a str, &'a str>::new();

        // Get clean data
        let hash = self.hash.get().unwrap_or_default();
        let password = self.password.get().unwrap_or_default();
        let confirm_password = self.confirm_password.get().unwrap_or_default();
        let username = self.username.get().unwrap_or_default();

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

impl Hooks for User {
    fn pre_create(&self) {
        println!("!!!Pre Create!!!");
    }
    //
    fn post_create(&self) {
        println!("!!!Post Create!!!");
    }
    //
    fn pre_update(&self) {
        println!("!!!Pre Update!!!");
    }
    //
    fn post_update(&self) {
        println!("!!!Post Update!!!");
    }
    //
    fn pre_delete(&self) {
        println!("!!!Pre Delet!!!");
    }
    //
    fn post_delete(&self) {
        println!("!!!Post Delet!!!");
    }
}
```

#### src/main.rs

```rust
mod migration;
mod models;
mod settings;

use green_barrel::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Run migration.
    migration::run_migration()?;

    // Create model instance.
    // ---------------------------------------------------------------------------------------------
    let mut user = models::User::new()?;
    user.username.set("user_1");
    user.email.set("user_1_@noreply.net");
    user.password.set("12345678");
    user.confirm_password.value = Some("12345678".to_string()); // Example without the set() method
    user.is_staff.set(true);
    user.is_active.set(true);

    // Check Model.
    // ---------------------------------------------------------------------------------------------
    println!("\n\nCheck Modell:\n");
    let output_data = user.check(None)?;
    if output_data.is_valid() {
        println!("Hash: {:?}", user.hash.get());
        println!("Hash: {}", output_data.hash());

        println!("Created at: {:?}", user.created_at.get());
        println!("Updated at: {:?}", user.updated_at.get());
        println!("Created at: {:?}", output_data.created_at());
        println!("Updated at: {:?}", output_data.updated_at());

        println!("Object Id: {:?}", user.hash.obj_id()?);
        println!("Object Id: {:?}", output_data.obj_id()?);
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    //println!("Json:\n{}", output_data.json()?);
    //println!("Json for admin:\n{}", output_data.json_for_admin()?);

    // Create document in database.
    // ---------------------------------------------------------------------------------------------
    println!("\n\nCreate document in database:\n");
    let output_data = user.save(None, None)?;
    if output_data.is_valid() {
        // Update instance.
        user = output_data.update()?;

        println!("Hash: {}", user.hash.get().unwrap());
        println!("Hash: {}", output_data.hash());

        println!("Created at: {}", user.created_at.get().unwrap());
        println!("Updated at: {}", user.updated_at.get().unwrap());
        println!("Created at: {}", output_data.created_at().unwrap());
        println!("Updated at: {}", output_data.updated_at().unwrap());

        println!("Object Id: {:?}", user.hash.obj_id()?.unwrap());
        println!("Object Id: {:?}", output_data.obj_id()?.unwrap());

        //println!("Json:\n{}", output_data.json()?);

        println!("Slug: {}", user.slug.get().unwrap())
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    // Update document in database.
    // ---------------------------------------------------------------------------------------------
    println!("\n\nUpdate document in database:\n");
    if output_data.is_valid() {
        // Update instance.
        user = output_data.update()?;

        user.username.set("new_user_1");
        let output_data = user.save(None, None)?;

        if output_data.is_valid() {
            // Update instance.
            user = output_data.update()?;

            println!("Hash: {}", user.hash.get().unwrap());
            println!("Hash: {}", output_data.hash());

            println!("Created at: {}", user.created_at.get().unwrap());
            println!("Updated at: {}", user.updated_at.get().unwrap());
            println!("Created at: {}", output_data.created_at().unwrap());
            println!("Updated at: {}", output_data.updated_at().unwrap());

            println!("Object Id: {:?}", user.hash.obj_id()?.unwrap());
            println!("Object Id: {:?}", output_data.obj_id()?.unwrap());

            //println!("Json:\n{}", output_data.json()?);

            println!("Slug: {}", user.slug.get().unwrap())
        } else {
            // Printing errors to the console ( for development ).
            output_data.print_err();
        }
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    // Delete document in database.
    // ---------------------------------------------------------------------------------------------
    println!("\n\nDelete document in database:\n");
    let output_data = user.delete(None)?;
    if !output_data.is_valid() {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    Ok(())
}
```

## Changelog

- **v1.1.0-beta** _Added support for **Fixtures** - To populate the database with pre-created data._
- **v1.0.0-beta** _Not compatible with **green-barrel v0.x.x** and **metamorphose v0.x.x**_
- **v0.7.12** _Fixed **README.md**._
- **v0.7.8** _Fixed validation for multi-select fields._
- **v0.7.0** _Added trait **Administrator** for easier registration of Models in the administration panel._
- **v0.6.10** _Added the ability to customize html code for web forms. See documentation: **mango_orm > widgets > generate_html_code > GenerateHtmlCode > generate_html()**._
- **v0.6.9** _Rename trait **ToModel** to **Main**._
- **v0.6.0** _The **created_at** and **updated_at** fields are automatically added to the Model. The widget type is **inputDateTime** and **disabled = true, is_hide = true**. Updated **README.md**. Updated documentation._
- **v0.5.4** _Fixed ModelName::**key()** method. See documentation: **mango_orm > models > ToModel**._
- **v0.5.2** _Import optimized._
- **v0.5.0** _Added model attribute is_use_hooks. See documentation: **mango_orm > models > hooks > Hooks**._
- **v0.4.8** _Fixed error message text._
- **v0.4.6** _Updating by version of dependencies._
- **v0.4.4** _Optimization of the validation mechanism._
- **v0.4.3** _Improved validation for Slug fields._
- **v0.4.2** _Removed **hiddenSlug** field._
- **v0.4.1** _Added **is_hide** parameter for Widgets._
- **v0.4** _Added **inputSlug** and **hiddenSlug** fields._
- **v0.3** _Removed the Form macro._

## License

#### This project is licensed under the [MIT](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-MIT "MIT") and [Apache Version 2.0](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-APACHE "Apache Version 2.0")
