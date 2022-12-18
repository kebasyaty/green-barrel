[![Logo](https://github.com/kebasyaty/green-barrel/raw/master/images/logo.svg "Logo")](https://github.com/kebasyaty/green-barrel "Logo")

# Green Barrel

#### ORM-like API MongoDB for Rust

**To simulate fields of type Many-to-One and Many-to-Many, a simplified alternative (Types of selective fields with dynamic addition of elements) is used.**

- Support for [GreenPanel](https://github.com/kebasyaty/green-panel "Green Panel") is temporarily unavailable.

[![crates.io](https://img.shields.io/crates/v/green-barrel "crates.io")](https://crates.io/crates/green-barrel "crates.io")
[![crates.io](https://img.shields.io/static/v1?label=rustc&message=v1.57%2B&color=red "crates.io")](https://www.rust-lang.org/ "crates.io")
![crates.io](https://img.shields.io/crates/d/green-barrel)
![crates.io](https://img.shields.io/crates/l/green-barrel)

## Attention

#### [MongoDB](https://www.mongodb.com/ "MongoDB") tested on version 4.4

#### [MongoDB Rust Driver](https://crates.io/crates/mongodb/ "MongoDB Rust Driver") updated as a dependency to version 2.

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
- [metamorphose](https://crates.io/crates/metamorphose "metamorphose")

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

## Road map

#### Field types

See documentation -[fields](https://docs.rs/green-barrel/1.3.22-beta/green_barrel/fields/index.html "fields").

#### Methods for Developers

[Main](https://docs.rs/green-barrel/1.3.22-beta/green_barrel/models/trait.Main.html "Main")

- hash()
- set_hash()
- obj_id()
- set_obj_id()
- created_at()
- updated_at()

[Caching](https://docs.rs/green-barrel/1.3.22-beta/green_barrel/models/caching/trait.Caching.html "Caching")

- meta()
- new()
- json()
- update_dyn_field()

[Control](https://docs.rs/green-barrel/1.3.22-beta/green_barrel/models/control/trait.Control.html "Control")

- custom_default()

[AdditionalValidation](https://docs.rs/green-barrel/1.3.22-beta/green_barrel/models/validation/trait.AdditionalValidation.html "AdditionalValidation")

- add_validation()

[Hooks](https://docs.rs/green-barrel/1.3.22-beta/green_barrel/models/hooks/trait.Hooks.html "Hooks")

- pre_create()
- post_create()
- pre_update()
- post_update()
- pre_delete()
- post_delete()

[QCommons](https://docs.rs/green-barrel/1.3.22-beta/green_barrel/models/db_query_api/commons/trait.QCommons.html "QCommons")

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

[QPaladins](https://docs.rs/green-barrel/1.3.22-beta/green_barrel/models/db_query_api/paladins/trait.QPaladins.html "QPaladins")

- check()
- save()
- delete()
- create_password_hash()
- verify_password()
- update_password()

[Fixtures](https://docs.rs/green-barrel/1.3.22-beta/green_barrel/models/fixtures/trait.Fixtures.html "Fixtures")

- run_fixture()

## Changelog

- **v1.3.21-beta** _Trait AdditionalValidation - Transition to asynchronous code._
- **v1.3.20-beta** _In all methods, the client parameter has been moved to the first position._
- **v1.3.10-beta** _Main changes - Added methods for indexing._
- **v1.3.6-beta** _Fixed checking of dates._
- **v1.3.5-beta** _Improved dates checking._
- **v1.3.2-beta** _Fixed basic example and updated readme file._
- **v1.3.0-beta** _Transition to asynchronous code._
- **v1.2.0-beta** _Updated dependency mongodb to version 2._
- **v1.1.14-beta** _Rename Model parameters: **is_add_docs**, **is_up_docs**, **is_del_docs** to **is_add_doc**, **is_up_doc**, **is_del_doc**._
- **v1.1.9-beta** _Tests updated and **README.md** file updated._
- **v1.1.0-beta** _Added support for **Fixtures** - To populate the database with pre-created data._
- **v1.0.16-beta** _Added parameter **target_dir** for field types **InputFile** and **InputImage**._
- **v1.0.10-beta** _Updated comments for dynamic field types._
- **v1.0.8-beta** _The 150 character limit has been removed from the **update_dyn_field()** method._
- **v1.0.7-beta** _The **administrator** module has been removed and moved to a separate project._
- **v1.0.0-beta** _Not compatible with **green-barrel v0.x.x** and **metamorphose v0.x.x**_
- **v0.12.14** _Fixed **README.md**._
- **v0.12.8** _The **db_update_dyn_widgets** method has been renamed to **update_dyn_wig** and has been heavily modernized. See documentation: **green-barrel > models > caching > Caching > update_dyn_wig**._
- **v0.12.4** _Made two critical fixes to the **check** method and updated unit tests._
- **v0.12.0** _Deep modernization of the **input_data** module and related modules._
- **v0.11.4** _**output_data** module moved from **widgets** directory to **models**._
- **v0.11.3** _**administrator** module moved from **db_query api** directory to **models**._
- **v0.11.2** _Renamed methods in trait **Administrator** - **instance_for_admin** to **actix_instance_for_admin** and **result_for_admin** to **actix_result_for_admin**._
- **v0.11.1** _Added enum **OutputDataAdmin** for easier registration of Models in the administration panel._
- **v0.11.0** _Added trait **Administrator** for easier registration of Models in the administration panel._
- **v0.10.100** _Added new type **UpdatePassword** to enum **OutputData**. Updated documentation._
- **v0.10.97** _Added field attribute check - **pattern**._
- **v0.10.95** _For optimization, the **output_data_to_html** mediator function has been excluded._
- **v0.10.94** _Added the ability to customize html code for web forms. See documentation: **mango_orm > widgets > generate_html > GenerateHtml > generate_html() > source**._
- **v0.10.93** _Rename trait **ToModel** to **Main**._
- **v0.10.92** _Added arguments for **to_html** methods. Arguments: **url_action**, **http_method** and **enctype**. See documentation: **mango_orm > widgets > output_data > OutputData > to_html**._
- **v0.10.90** _For the **OutputData** enum, the **output_data_to_html** method is extended with the **to_html** alias._
- **v0.10.20** _Removed the ability to change the created_at field of a model instance._
- **v0.10.0** _The **created_at** and **updated_at** fields are automatically added to the Model. The widget type is **inputDateTime** and **disabled = true, is_hide = true**. Updated **README.md**. Updated documentation._
- **v0.9.4-v.0.9.15** _Import optimized._
- **v0.9.0** _Added hook methods. See documentation: **mango_orm > models > hooks > Hooks**._
- **v0.8.26** _Add find_one_to_wig method. See documentation: **mango_orm > models > db_query_api > commons > QCommons**._
- **v0.8.0** _Deep modernization of common.rs and output_data.rs modules. See documentation: **mango_orm > models > db_query_api > commons > QCommons** and **mango_orm > models > output_data > Converters**._
- **v0.7.4** _Updated **README.md**, added model attributes._
- **v0.7.0** \*Added the ability to use the hash field in inputSlug - **slug_sources: r#"["hash", "username"]"#\***
- **v0.6.30** _Renamed methods: **wig()**, **json()**, **html()** -> **to_wig()**, **to_json()**, **to_html()**. Updated **README.md**. Updated documentation. Updated versions of dependencies._
- **v0.6.16** _Renamed the Forms module to Widgets._
- **v0.6.15** _Updating by version of dependencies._
- **v0.6.10** _Updated test for dynamic widgets._
- **v0.6.7** _Removed **hiddenSlug** field._
- **v0.6.6** _Added **is_hide** parameter for Widgets._
- **v0.6.5** _In the check() method, errors are redirected to the console, for fields of hidden type._
- **v0.6.4** _Fixes for fields of slug type._
- **v0.6** _1) Added inputSlug and hiddenSlug fields. 2) Fix - Added fields of hidden type to migration._
- **v0.5.4** _Optimization for creating thumbnails, for default images._
- **v0.5.3** _Improved cleaning of orphaned files._
- **v0.5** _Support for the Form macro has been removed._

## License

#### This project is licensed under the [MIT](https://github.com/kebasyaty/green-barrel/blob/master/LICENSE-MIT "MIT") and [Apache Version 2.0](https://github.com/kebasyaty/green-barrel/blob/master/LICENSE-APACHE "Apache Version 2.0")
