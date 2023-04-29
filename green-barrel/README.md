[![Logo](https://github.com/kebasyaty/green-barrel/raw/master/images/logo.svg "Logo")](https://github.com/kebasyaty/green-barrel "Logo")

# Green Barrel

#### ORM-like API MongoDB for Rust

**For simulate relationship Many-to-One and Many-to-Many, a simplified alternative (Types of selective fields with dynamic addition of elements) is used.**

[![crates.io](https://img.shields.io/crates/v/green-barrel "crates.io")](https://crates.io/crates/green-barrel "crates.io")
[![crates.io](https://img.shields.io/static/v1?label=rustc&message=v1.57%2B&color=red "crates.io")](https://www.rust-lang.org/ "crates.io")
[![Green Barrel on docs.rs][docsrs-image]][docsrs]
![crates.io](https://img.shields.io/crates/l/green-barrel)
![crates.io](https://img.shields.io/crates/d/green-barrel)

[docsrs-image]: https://docs.rs/green-barrel/badge.svg
[docsrs]: https://docs.rs/green-barrel

## Attention

#### [MongoDB](https://www.mongodb.com/ "MongoDB") tested on version 4.4

- Support for [GreenPanel](https://github.com/kebasyaty/green-panel "Green Panel") is temporarily unavailable.

## Requirements

- [mongodb](https://crates.io/crates/mongodb "mongodb")
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
- [async-lock](https://crates.io/crates/async-lock "async-lock")
- [async-trait](https://crates.io/crates/async-trait "async-trait")
- [futures](https://crates.io/crates/futures "futures")
- [tokio](https://crates.io/crates/tokio "tokio")
- [rust-i18n](https://crates.io/crates/rust-i18n "rust-i18n")
- [metamorphose](https://crates.io/crates/metamorphose "metamorphose")

## Install mongodb (if not installed)

Follow the link [Install MongoDB](https://github.com/kebasyaty/green-barrel/blob/master/Install_MongoDB.md "Install MongoDB").

## Usage:

[Basic Example](https://github.com/kebasyaty/green-barrel/tree/master/examples/basic "Basic Example")

## Model parameters

**_( all parameters are optional )_**

| Parameter:          | Default:     | Description:                                                                                         |
| :------------------ | :----------- | :--------------------------------------------------------------------------------------------------- |
| db_query_docs_limit | 1000         | limiting query results.                                                                              |
| is_add_doc          | true         | Create documents in the database. **false** - Alternatively, use it to validate data from web forms. |
| is_up_doc           | true         | Update documents in the database.                                                                    |
| is_del_doc          | true         | Delete documents from the database.                                                                  |
| ignore_fields       | empty string | Fields that are not included in the database (separated by commas).                                  |
| is_use_add_valid    | false        | Allows additional validation - **impl AdditionalValidation for ModelName**.                          |
| is_use_hooks        | false        | Allows hooks methods - **impl Hooks for ModelName**.                                                 |

## Links to Documentation

#### Field types

See documentation -[fields](https://docs.rs/green-barrel/latest/green_barrel/fields/index.html "fields").

#### Methods for Developers

[Main](https://docs.rs/green-barrel/latest/green_barrel/models/trait.Main.html "Main")

- hash()
- set_hash()
- obj_id()
- set_obj_id()
- created_at()
- updated_at()

[Caching](https://docs.rs/green-barrel/latest/green_barrel/models/caching/trait.Caching.html "Caching")

- meta()
- new()
- json()
- update_dyn_field()

[Control](https://docs.rs/green-barrel/latest/green_barrel/models/control/trait.Control.html "Control")

- custom_default()

[AdditionalValidation](https://docs.rs/green-barrel/latest/green_barrel/models/validation/trait.AdditionalValidation.html "AdditionalValidation")

- add_validation()

[Hooks](https://docs.rs/green-barrel/latest/green_barrel/models/hooks/trait.Hooks.html "Hooks")

- pre_create()
- post_create()
- pre_update()
- post_update()
- pre_delete()
- post_delete()

[QCommons](https://docs.rs/green-barrel/latest/green_barrel/models/db_query_api/commons/trait.QCommons.html "QCommons")

- create_index()
- drop_index()
- create_indexes()
- drop_indexes()
- aggregate()
- count_documents()
- delete_many()
- delete_one()
- distinct()
- drop()
- estimated_document_count()
- find_many_to_doc_list()
- find_many_to_json()
- find_one_to_doc()
- find_one_to_json()
- find_one_to_instance()
- find_one_and_delete()
- collection_name()
- namespace()

[QPaladins](https://docs.rs/green-barrel/latest/green_barrel/models/db_query_api/paladins/trait.QPaladins.html "QPaladins")

- check()
- save()
- delete()
- create_password_hash()
- verify_password()
- update_password()

[Fixtures](https://docs.rs/green-barrel/latest/green_barrel/models/fixtures/trait.Fixtures.html "Fixtures")

- run_fixture()

## Changelog

[View the change history.](https://github.com/kebasyaty/green-barrel/blob/master/Changelog.md "View the change history.")

## License

#### This project is licensed under the [MIT](https://github.com/kebasyaty/green-barrel/blob/master/LICENSE-MIT "MIT") and [Apache Version 2.0](https://github.com/kebasyaty/green-barrel/blob/master/LICENSE-APACHE "Apache Version 2.0")
