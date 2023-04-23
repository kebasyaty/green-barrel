![Logo](https://github.com/kebasyaty/mango-orm/raw/master/metamorphose/images/logo.svg)

# Metamorphose

### Macros collection for converting Structure to Model, for a [Green Barrel](https://github.com/kebasyaty/green-barrel "green-barrel") project.

[![crates.io](https://img.shields.io/crates/v/metamorphose "crates.io")](https://crates.io/crates/metamorphose "crates.io")
[![crates.io](https://img.shields.io/static/v1?label=rustc&message=v1.57%2B&color=red "crates.io")](https://www.rust-lang.org/ "crates.io")
![crates.io](https://img.shields.io/crates/d/metamorphose)
![crates.io](https://img.shields.io/crates/l/metamorphose)
[![Metamorphose on docs.rs][docsrs-image]][docsrs]

[docsrs-image]: https://docs.rs/metamorphose/badge.svg
[docsrs]: https://docs.rs/metamorphose

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

## Changelog

- **v1.1.0** _Removed 12 types of fields, the remaining fields have been renamed. See the documentation - https://docs.rs/green-barrel/latest/green_barrel/fields/index.html_
- **v1.3.2-beta** _Fixed basic example and updated readme file._
- **v1.3.0-beta** _Transition to asynchronous code._
- **v1.2.0-beta** _Updated dependency mongodb to version 2._
- **v1.1.9-beta** _Updated **README.md** file._
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
