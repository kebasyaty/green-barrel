![Logo](https://github.com/kebasyaty/mango-orm/raw/master/metamorphose/images/logo.svg)

# metamorphose

### Macros collection for converting Structure to Model, for a [mango-orm](https://github.com/kebasyaty/mango-orm "mango-orm") project.

![crates.io](https://img.shields.io/crates/v/metamorphose)
![crates.io](https://img.shields.io/static/v1?label=rustc&message=v1.52%2B&color=red)
![crates.io](https://img.shields.io/crates/d/metamorphose)
![crates.io](https://img.shields.io/crates/l/metamorphose)

## Macros

#### Model

> Macro for converting Structure to mango-orm Model.
> The model can access the database.
> The model can create, update, and delete documents in collections.

## Requirements

- [quote](https://crates.io/crates/quote "quote")
- [regex](https://crates.io/crates/regex "regex")
- [serde_json](https://crates.io/crates/serde_json "serde_json")
- [syn](https://crates.io/crates/syn "syn")
- [serde](https://crates.io/crates/serde "serde")

## Changelog

- **v0.5.4** _Fix ModelName::key() method. See documentation: **mango_orm > models > hooks > Hooks**._
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
