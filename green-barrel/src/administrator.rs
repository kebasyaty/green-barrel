//! Helper methods for the admin panel.

use mongodb::bson::{doc, document::Document, oid::ObjectId};
use serde_json::Value;
use std::error::Error;

use crate::{
    helpers::Meta,
    models::db_query_api::{commons::QCommons, paladins::QPaladins},
    widgets::Widget,
};

/// The output data for the admin panel.
pub enum OutputDataAdmin<T> {
    Instance(Option<T>),
    EarlyResult(String),
}

/// Helper methods for the admin panel.
pub trait Administrator: QCommons + QPaladins {
    /// Json-line for admin panel.
    /// ( converts a widget map to a list, in the order of the Model fields )
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName{...};
    /// println!("{}", model_name.instance_to_json_for_admin()?);
    /// ```
    ///
    fn instance_to_json_for_admin(&self) -> Result<String, Box<dyn Error>> {
        // Get cached Model data.
        let (model_cache, _client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = model_cache.meta;
        //
        let map_widgets = model_cache.widget_map.clone();
        let model_json = self.self_to_json()?;
        let mut widget_list: Vec<Widget> = Vec::new();
        let hash = self.get_hash();
        // Get a list of widgets in the order of the model fields.
        for field_name in meta.fields_name.iter() {
            let mut widget = map_widgets.get(field_name).unwrap().clone();
            if !field_name.contains("password") {
                let field_json = model_json[field_name].clone();
                if field_json.is_string() {
                    widget.value = field_json.as_str().unwrap().to_string();
                } else if field_json.is_i64() {
                    widget.value = field_json.as_i64().unwrap().to_string();
                } else if field_json.is_u64() {
                    widget.value = field_json.as_u64().unwrap().to_string();
                } else if field_json.is_f64() {
                    widget.value = field_json.as_f64().unwrap().to_string();
                } else if field_json.is_array() {
                    let array = field_json.as_array().unwrap();
                    widget.value = serde_json::to_string(array)?;
                } else if field_json.is_boolean() {
                    widget.checked = field_json.as_bool().unwrap();
                } else if field_json.is_null() {
                    widget.value = String::new();
                }
                if field_name == "created_at" || field_name == "updated_at" {
                    widget.is_hide = false;
                }
            } else if !hash.is_empty() {
                widget.widget = "hiddenText".to_string();
                widget.input_type = "hidden".to_string();
                widget.value = String::new();
            }
            widget_list.push(widget);
        }
        //
        Ok(serde_json::to_string(&widget_list)?)
    }

    /// Get the model instance for actix-mango-panel.
    // *********************************************************************************************
    fn actix_instance_for_admin(
        doc_hash: Option<&str>,
        bytes: Option<&actix_web::web::BytesMut>,
        filter: Option<&Document>,
        dyn_data: Option<Value>,
    ) -> Result<OutputDataAdmin<Self>, Box<dyn Error>>
    where
        Self: serde::de::DeserializeOwned + Sized,
    {
        //
        if doc_hash.is_some() {
            // For - Get document
            let doc_hash = doc_hash.unwrap();
            if doc_hash.is_empty() {
                return Ok(OutputDataAdmin::EarlyResult(
                    Self::model_to_json_for_admin()?
                ));
            }
            let object_id = ObjectId::with_string(doc_hash);
            if object_id.is_err() {
                Err(format!(
                    "Model: `{}` > \
                    Method: `instance_for_admin` => \
                    Invalid document hash.",
                    Self::key()?
                ))?
            }
            let object_id = object_id.unwrap();
            let filter = doc! {"_id": object_id};
            Ok(OutputDataAdmin::Instance(Self::find_one_to_model_instance(
                filter, None,
            )?))
        } else if bytes.is_some() {
            // For - Save document
            Ok(OutputDataAdmin::Instance(Some(serde_json::from_slice::<
                Self,
            >(
                &bytes.unwrap()
            )?)))
        } else if filter.is_some() {
            // For - Delete document
            Ok(OutputDataAdmin::Instance(Self::find_one_to_model_instance(
                filter.unwrap().clone(),
                None,
            )?))
        } else if dyn_data.is_some() {
            // Update dynamic widget data
            Self::update_dyn_wig(dyn_data.unwrap())?;
            return Ok(OutputDataAdmin::EarlyResult(String::new()));
        } else {
            Err(format!(
                "Model: `{}` > \
                Method: `instance_for_admin` => \
                No match on function arguments.",
                Self::key()?
            ))?
        }
    }

    /// Get result for actix-mango-panel.
    // *********************************************************************************************
    fn actix_result_for_admin(
        &mut self,
        doc_hash: Option<&str>,
        bytes: Option<&actix_web::web::BytesMut>,
        filter: Option<&Document>,
    ) -> Result<String, Box<dyn Error>> {
        //
        if doc_hash.is_some() {
            // Get document
            return self.instance_to_json_for_admin();
        } else if bytes.is_some() {
            // Save document
            let output_data = self.save(None, None)?;
            return output_data.to_json_for_admin();
        } else if filter.is_some() {
            // Delete document
            let output_data = self.delete(None)?;
            if !output_data.is_valid() {
                return Ok(output_data.err_msg());
            }
        } else {
            Err(format!(
                "Model: `{}` > \
                Method: `result_for_admin` => \
                No match on function arguments.",
                Self::key()?
            ))?
        }
        //
        Ok(String::new())
    }
}
