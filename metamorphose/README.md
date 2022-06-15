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
- **v0.4.8** *Fixed error message text.*
- **v0.4.6** *Updating by version of dependencies.*
- **v0.4.4** *Optimization of the validation mechanism.*
- **v0.4.3** *Improved validation for Slug fields.*
- **v0.4.2** *Removed **hiddenSlug** field.*
- **v0.4.1** *Added **is_hide** parameter for Widgets.*
- **v0.4** *Added **inputSlug** and **hiddenSlug** fields.*
- **v0.3** *Removed the Form macro..*

## License
#### This project is licensed under the [MIT](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-MIT "MIT") and [Apache Version 2.0](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-APACHE "Apache Version 2.0")
