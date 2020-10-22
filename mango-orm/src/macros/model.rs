//! # Create Model
//!
//!  `model` - Macro for converting Structure to Model.

// MACRO
// #################################################################################################
/// Macro for converting Structure to Model
#[macro_export]
macro_rules! model {
        ( struct $sname:ident { $($fname:ident : $ftype:ty),+ }
            $(#[$attrs:meta])* $($impls:item)+ ) => {

        #[derive(Serialize, Deserialize, Default, Clone, Debug)]
        pub struct $sname {
            $(pub $fname : $ftype),+
        }

        $(#[$attrs])*
        $($impls)+

        impl $sname {
            // Info Model
            // *************************************************************************************
            // Metadata (database name, collection name, etc)
            pub fn metadata<'a>() -> Result<Meta<'a>, Box<dyn Error>> {
                let mut meta: Meta = Self::meta()?;
                meta.service = meta.service.to_lowercase();
                meta.database = meta.database.to_lowercase();
                meta.model_name = stringify!($sname);
                meta.field_names = &[$(stringify!($fname)),*];
                meta.collection = format!("{}__{}",
                    meta.service, stringify!($sname).to_lowercase()
                );
                meta.field_types =  &[$(stringify!($fname)),*];
                    Ok(FIELD_NAMES.iter().map(|item| item.to_owned())
                    .zip([$(stringify!($ftype)),*].iter().map(|item| item.to_owned())).collect());
                Ok(meta)
            }

            // Form - Widgets, attributes (HashMap, Json), Html
            // *************************************************************************************
            // Get full map of Widgets (with widget for id field)
            pub fn widgets_full_map<'a>() -> Result<HashMap<&'a str, Widget>, Box<dyn Error>> {
                let mut map: HashMap<&str, Widget> = Self::widgets()?;
                if map.get("hash").is_none() {
                    map.insert(
                        "hash",
                        Widget {
                            value: FieldType::Hash,
                            hidden: true,
                            ..Default::default()
                        }
                    );
                }
                Ok(map)
            }

            // Add (if required) default form data to cache
            pub async fn form_cache() -> Result<(
                async_mutex::MutexGuard<'static, HashMap<String,
                FormCache>>, String), Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let meta: Meta = Self::metadata()?;
                let key = meta.collection.clone();
                let mut store: async_mutex::MutexGuard<'_, HashMap<String,
                    FormCache>> = FORM_CACHE.lock().await;
                let mut cache: Option<&FormCache> = store.get(&key);
                if cache.is_none() {
                    // Add a map of pure attributes of Form for page templates
                    let widgets: HashMap<&str, Widget> = Self::widgets_full_map()?;
                    let mut clean_attrs: HashMap<String, Transport> = HashMap::new();
                    let mut widget_map: HashMap<String, String> = HashMap::new();
                    for (field, widget) in &widgets {
                        clean_attrs.insert(field.to_string(), widget.clean_attrs(field)?);
                        widget_map.insert(
                            field.to_string(), widget.value.get_enum_type().to_string());
                    }
                    // Add default data
                    let form_cache = FormCache{
                        attrs_map: clean_attrs,
                        widget_map: widget_map,
                        ..Default::default()
                    };
                    // Save default data to cache
                    store.insert(key.clone(), form_cache);
                }

                Ok((store, key))
            }

            // Get a map of pure attributes of Form for page templates
            pub async fn form_map() -> Result<HashMap<String, Transport>, Box<dyn Error>> {
                let (store, key) = Self::form_cache().await?;
                let cache: Option<&FormCache> = store.get(&key);
                if cache.is_some() {
                    let clean_attrs: HashMap<String, Transport> = cache.unwrap().attrs_map.clone();
                    Ok(clean_attrs)
                } else {
                    Err(format!("Model: `{}` -> Method: `form_map()` : \
                                Did not receive data from cache.",
                        stringify!($sname)))?
                }
            }

            // Get Form attributes in Json format for page templates
            pub async fn form_json() -> Result<String, Box<dyn Error>> {
                let (mut store, key) = Self::form_cache().await?;
                let cache: Option<&FormCache> = store.get(&key);
                if cache.is_some() {
                    let cache: &FormCache = cache.unwrap();
                    if cache.attrs_json.is_empty() {
                        // Create Json-string
                        let mut form_cache: FormCache = cache.clone();
                        let attrs: HashMap<String, Transport> = form_cache.attrs_map.clone();
                        let mut json_text = String::new();
                        for (field, trans) in attrs {
                            let tmp = serde_json::to_string(&trans).unwrap();
                            if !json_text.is_empty() {
                                json_text = format!("{},\"{}\":{}", json_text, field, tmp);
                            } else {
                                json_text = format!("\"{}\":{}", field, tmp);
                            }
                        }
                        // Update data
                        form_cache.attrs_json = format!("{{{}}}", json_text);
                        // Save data to cache
                        store.insert(key, form_cache.clone());
                        // Return result
                        return Ok(form_cache.attrs_json);
                    }
                    Ok(cache.attrs_json.clone())
                } else {
                    Err(format!("Model: `{}` -> Method: `form_json()` : \
                                Did not receive data from cache.",
                        stringify!($sname)))?
                }
            }

            // Get Html Form of Model for page templates
            pub async fn form_html() ->
                Result<String, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let (mut store, key) = Self::form_cache().await?;
                let model_name: &str = &stringify!($sname).to_lowercase();
                let mut build_controls = false;
                let mut attrs: HashMap<String, Transport> = HashMap::new();
                //
                let cache: Option<&FormCache> = store.get(&key);
                if cache.is_some() {
                    let cache: &FormCache = cache.unwrap();
                    let is_cached: bool = cache.form_html.is_empty();
                    if is_cached {
                        build_controls = true;
                        attrs = cache.attrs_map.clone();
                    }
                    let controls = Self::html(
                        attrs,
                        model_name,
                        build_controls
                    )?;
                    if is_cached {
                         // Clone cache
                         let mut form_cache: FormCache = cache.clone();
                        // Update cache
                        form_cache.form_html = controls.clone();
                        // Save to cache
                        store.insert(key, form_cache.clone());
                        // Return result
                        return Ok(controls);
                    }
                    Ok(cache.form_html.clone())
                } else {
                    Err(format!("Model: `{}` -> Method: `form_html()` : \
                                Did not receive data from cache.",
                        stringify!($sname)))?
                }
            }

            // Validation of database queries
            // *************************************************************************************
            // Validation of `maxlength`
            fn check_maxlength(maxlength: usize, value: &str ) -> Result<(), Box<dyn Error>>  {
                if maxlength > 0 && value.encode_utf16().count() > maxlength {
                    Err(format!("Exceeds limit, maxlength={}.", maxlength))?
                }
                Ok(())
            }

            // Validation of `unique`
            async fn check_unique(
                is_update: bool, is_unique: bool, field_name: String, value_bson_pre: &Bson,
                value_type: &str, coll: &Collection) -> Result<(), Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                if !is_update && is_unique {
                    let filter: Document = match value_type {
                        "i64" => {
                            // For u32 and i64
                            let field_value: i64 = value_bson_pre.as_i64().unwrap();
                            doc!{ field_name.to_string() : Bson::Int64(field_value) }
                        }
                        _ => {
                            doc!{ field_name.to_string() : value_bson_pre }
                        }
                    };
                    let count: i64 = coll.count_documents(filter, None).await?;
                    if count > 0 {
                        Err("Is not unique.")?
                    }
                }
                Ok(())
            }

            // Accumulation of errors
            fn accumula_err(attrs: &Transport, err: &String) ->
                Result<String, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let mut tmp = attrs.error.clone();
                tmp = if !tmp.is_empty() { format!("{}<br>", tmp) } else { String::new() };
                Ok(format!("{}{}", tmp, err))
            }

            // Validation in regular expression (email, password, etc...)
            fn regex_validation(field_type: &str, value: &str) ->
                Result<(), Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                match field_type {
                    "InputEmail" => {
                        if !validate_email(value) {
                            Err("Invalid email address.")?
                        }
                    }
                    "InputColor" => {
                        if !REGEX_IS_COLOR_CODE.is_match(value) {
                            Err("Invalid Color code.")?
                        }
                    }
                    "InputUrl" => {
                        if !validate_url(value) {
                            Err("Invalid Url.")?
                        }
                    }
                    "InputIP" => {
                        if !validate_ip(value) {
                            Err("Invalid IP address.")?
                        }
                    }
                    "InputIPv4" => {
                        if !validate_ip_v4(value) {
                            Err("Invalid IPv4 address.")?
                        }
                    }
                    "InputIPv6" => {
                        if !validate_ip_v6(value) {
                            Err("Invalid IPv6 address.")?
                        }
                    }
                    "InputPassword" => {
                        if !REGEX_IS_PASSWORD.is_match(value) {
                            Err("Allowed characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (<br> \
                                 Minimum size 8 characters")?
                        }
                    }
                    "InputDate" => {
                        if !REGEX_IS_DATE.is_match(value) {
                            Err("Incorrect date format.<br>Example: 1970-02-28")?
                        }
                    }
                    "InputDateTime" => {
                        if !REGEX_IS_DATETIME.is_match(value) {
                            Err("Incorrect date and time format.<br>Example: 1970-02-28T00:00")?
                        }
                    }
                    _ => return Ok(()),
                }
                Ok(())
            }

            // Generate password hash and add to result document
            pub fn create_password_hash(field_value: &str) -> Result<String, Box<dyn Error>> {
                    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                            abcdefghijklmnopqrstuvwxyz\
                                            0123456789@#$%^&+=*!~)(";
                    const SALT_LEN: usize = 12;
                    let mut rng = rand::thread_rng();
                    let password: &[u8] = field_value.as_bytes();
                    let salt: String = (0..SALT_LEN)
                        .map(|_| {
                            let idx = rng.gen_range(0, CHARSET.len());
                            CHARSET[idx] as char
                        })
                        .collect();
                    let salt: &[u8] = salt.as_bytes();
                    let config = Config::default();
                    let hash: String = argon2::hash_encoded(password, salt, &config)?;
                    Ok(hash)
            }

            // Validation of Form
            pub async fn check(&self, client: &Client, output_format: OutputType) ->
                Result<OutputData, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                static MODEL_NAME: &str = stringify!($sname);
                let (mut store, key) = Self::form_cache().await?;
                let meta: Meta = Self::metadata()?;
                let mut global_err = false;
                let is_update: bool = !self.hash.is_empty();
                let mut attrs_map: HashMap<String, Transport> = HashMap::new();
                let ignore_fields: Vec<&str> = meta.ignore_fields;
                let coll: Collection = client.database(&meta.database).collection(&meta.collection);
                // Get preliminary data from the model
                let mut doc_pre: Document = to_document(self).unwrap();
                // Get data for model from database (if available)
                let mut doc_from_db: Document = if is_update {
                    let object_id: ObjectId = ObjectId::with_string(&self.hash)
                        .unwrap_or_else(|err| { panic!("{:?}", err) });
                    let filter: Document = doc!{"_id": object_id};
                    coll.find_one(filter, None).await?.unwrap()
                } else {
                    doc! {}
                };
                // Document for the final result
                let mut doc_res: Document = doc! {};

                // Validation of field by attributes (maxlength, unique, min, max, etc...)
                // ---------------------------------------------------------------------------------
                let cache: Option<&FormCache> = store.get(&key);
                if cache.is_some() {
                    let cache: &FormCache = cache.unwrap();
                    static FIELD_NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                    attrs_map = cache.attrs_map.clone();
                    let widget_map: HashMap<String, String> = cache.widget_map.clone();
                    // Apply custom check
                    {
                        let error_map: HashMap<&str, &str> = self.custom_check()?;
                        if !error_map.is_empty() { global_err = true; }
                        for (field_name, err_msg) in error_map {
                            let attrs: &mut Transport = attrs_map.get_mut(field_name).unwrap();
                            attrs.error = Self::accumula_err(&attrs, &err_msg.to_string()).unwrap();
                        }
                    }
                    // Loop over fields for validation
                    for field_name in FIELD_NAMES {
                        // Don't check the `hash` field
                        if field_name == &"hash" { continue; }
                        // Get field value for validation
                        let value_bson_pre: Option<&Bson> = doc_pre.get(field_name);
                        // Check field value
                        if value_bson_pre.is_none() {
                            Err(format!("Model: `{}` -> Field: `{}` -> Method: `check()` : \
                                        This field is missing.",
                                MODEL_NAME, field_name))?
                        }
                        //
                        let value_bson_pre: &Bson = value_bson_pre.unwrap();
                        let field_type: &str =
                            widget_map.get(&field_name.to_string()).unwrap();
                        let attrs: &mut Transport =
                                attrs_map.get_mut(&field_name.to_string()).unwrap();
                        // For each iteration of the loop
                        let mut local_err = false;
                        // Field validation
                        match field_type {
                            // Validation of text type fields
                            // ---------------------------------------------------------------------
                            "InputCheckBoxText" | "InputRadioText" | "InputColor"
                            | "InputEmail" | "InputPassword" | "InputTel"
                            | "InputText" | "InputUrl" | "InputIP" | "InputIPv4"
                            | "InputIPv6" | "TextArea" | "SelectText" => {
                                // Get field value for validation
                                // -----------------------------------------------------------------
                                let field_value: &str = value_bson_pre.as_str().unwrap();
                                // Validation for a required field
                                // -----------------------------------------------------------------
                                if attrs.required && field_value.is_empty() {
                                    global_err = true;
                                    local_err = true;
                                    attrs.error =
                                        Self::accumula_err(&attrs,
                                            &"Required field.".to_owned()).unwrap();
                                    attrs.value = field_value.to_string();
                                    continue;
                                }
                                // If the field is not required and there is no data in it,
                                // take data from the database
                                // -----------------------------------------------------------------
                                if is_update && !ignore_fields.contains(field_name) &&
                                    ((!attrs.required && field_value.is_empty()) ||
                                    field_type == "InputPassword") {
                                    let value_from_db: Option<&Bson> =
                                        doc_from_db.get(&field_name);

                                    if value_from_db.is_some() {
                                        doc_res.insert(field_name.to_string(),
                                            value_from_db.unwrap());
                                        continue;
                                    } else {
                                        Err(format!("Model: `{}` -> Field: `{}` -> Method: \
                                                    `check()` : \
                                                    This field is missing from the database.",
                                            MODEL_NAME, &field_name))?
                                    }
                                }
                                // Checking `maxlength`, `min length`, `max length`
                                // -----------------------------------------------------------------
                                Self::check_maxlength(attrs.maxlength, field_value)
                                    .unwrap_or_else(|err| {
                                        global_err = true;
                                        local_err = true;
                                        attrs.error =
                                        Self::accumula_err(&attrs, &err.to_string()).unwrap();
                                });
                                // -----------------------------------------------------------------
                                if !field_value.is_empty() {
                                    // Validation of range (`min` <> `max`)
                                    // ( Hint: The `validate_length()` method did not
                                    // provide the desired result )
                                    // -------------------------------------------------------------
                                    let min: f64 = attrs.min.parse().unwrap();
                                    let max: f64 = attrs.max.parse().unwrap();
                                    let len: f64 = field_value.encode_utf16().count() as f64;
                                    if (min > 0_f64 || max > 0_f64) &&
                                        !validate_range(Validator::Range{min: Some(min),
                                                        max: Some(max)}, len) {
                                        global_err = true;
                                        local_err = true;
                                        let msg = format!(
                                            "Length {} is out of range (min={} <> max={}).",
                                            len, min, max);
                                        attrs.error = Self::accumula_err(&attrs, &msg).unwrap();
                                    }
                                    // Validation of `unique`
                                    // -------------------------------------------------------------
                                    Self::check_unique(is_update, attrs.unique,
                                        field_name.to_string(), value_bson_pre, "str", &coll)
                                        .await.unwrap_or_else(|err| {
                                        global_err = true;
                                        local_err = true;
                                        attrs.error =
                                            Self::accumula_err(&attrs, &err.to_string())
                                                .unwrap();
                                    });
                                    // Validation in regular expression (email, password, etc...)
                                    // -------------------------------------------------------------
                                    Self::regex_validation(field_type, field_value)
                                        .unwrap_or_else(|err| {
                                        global_err = true;
                                        local_err = true;
                                        attrs.error =
                                            Self::accumula_err(&attrs, &err.to_string())
                                                .unwrap();
                                    });
                                }
                                // Insert result
                                // -----------------------------------------------------------------
                                if !local_err && !ignore_fields.contains(field_name) {
                                    match field_type {
                                        "InputPassword" => {
                                            if !field_value.is_empty() {
                                                // Generate password hash and add to result document
                                                let hash: String =
                                                    Self::create_password_hash(field_value)?;
                                                doc_res.insert(field_name.to_string(),
                                                    Bson::String(hash));
                                            }
                                        }
                                        _ => {
                                            // Insert result from other fields
                                            attrs.value = field_value.to_string();
                                            doc_res.insert(field_name.to_string(),
                                                Bson::String(field_value.to_string()));
                                        }
                                    }
                                }
                            }
                            "InputDate" | "InputDateTime" => {
                                // Get field value for validation
                                // -----------------------------------------------------------------
                                let field_value: &str = value_bson_pre.as_str().unwrap();
                                // Validation for a required field
                                // -----------------------------------------------------------------
                                if attrs.required && field_value.is_empty() {
                                    global_err = true;
                                    local_err = true;
                                    attrs.error =
                                        Self::accumula_err(&attrs,
                                            &"Required field.".to_owned()).unwrap();
                                    attrs.value = field_value.to_string();
                                    continue;
                                }
                                // If the field is not required and there is no data in it,
                                // take data from the database
                                // -----------------------------------------------------------------
                                if is_update && !ignore_fields.contains(field_name)
                                    && !attrs.required && field_value.is_empty() {
                                    let value_from_db: Option<&Bson> =
                                        doc_from_db.get(&field_name);

                                    if value_from_db.is_some() {
                                        doc_res.insert(field_name.to_string(),
                                            value_from_db.unwrap());
                                        continue;
                                    } else {
                                        Err(format!("Model: `{}` -> Field: `{}` -> Method: \
                                                    `check()` : \
                                                    This field is missing from the database.",
                                            MODEL_NAME, &field_name))?
                                    }
                                }
                                if field_value.is_empty() { continue; }
                                // Validation in regular expression
                                // -----------------------------------------------------------------
                                Self::regex_validation(field_type, field_value)
                                    .unwrap_or_else(|err| {
                                    global_err = true;
                                    local_err = true;
                                    attrs.error =
                                        Self::accumula_err(&attrs, &err.to_string())
                                            .unwrap();
                                });
                                if local_err { continue; }
                                // Create Date and Time Object
                                // -----------------------------------------------------------------
                                let dt_value: DateTime<Utc> = {
                                    let field_value: String = if field_type == "InputDate" {
                                        format!("{}T00:00", field_value.to_string())
                                    } else {
                                        field_value.to_string()
                                    };
                                    DateTime::<Utc>::from_utc(
                                        NaiveDateTime::parse_from_str(
                                            &field_value, "%Y-%m-%dT%H:%M")?, Utc)
                                };
                                // Create dates for `min` and `max` attributes values to
                                // check, if the value of user falls within the range
                                // between these dates
                                if !attrs.min.is_empty() && !attrs.max.is_empty() {
                                    let dt_min: DateTime<Utc> = {
                                        let min_value: String = if field_type == "InputDate" {
                                            format!("{}T00:00", attrs.min.clone())
                                        } else {
                                            attrs.min.clone()
                                        };
                                        DateTime::<Utc>::from_utc(
                                            NaiveDateTime::parse_from_str(
                                                &min_value, "%Y-%m-%dT%H:%M")?, Utc)
                                    };
                                    let dt_max: DateTime<Utc> = {
                                        let max_value: String = if field_type == "InputDate" {
                                            format!("{}T00:00", attrs.max.clone())
                                        } else {
                                            attrs.max.clone()
                                        };
                                        DateTime::<Utc>::from_utc(
                                            NaiveDateTime::parse_from_str(
                                                &max_value, "%Y-%m-%dT%H:%M")?, Utc)
                                    };
                                    if dt_value < dt_min || dt_value > dt_max {
                                        global_err = true;
                                        attrs.error =
                                            Self::accumula_err(&attrs,
                                                &"Date out of range between `min` and` max`."
                                                .to_owned()
                                            ).unwrap();
                                        continue;
                                    }
                                }
                                // Create datetime in bson type
                                // -----------------------------------------------------------------
                                let dt_value_bson = Bson::DateTime(dt_value);
                                // Validation of `unique`
                                // -----------------------------------------------------------------
                                Self::check_unique(is_update, attrs.unique
                                    , field_name.to_string(), &dt_value_bson
                                    , "datetime", &coll)
                                    .await.unwrap_or_else(|err| {
                                    global_err = true;
                                    local_err = true;
                                    attrs.error =
                                        Self::accumula_err(&attrs, &err.to_string())
                                            .unwrap();
                                });
                                // Insert result
                                // -----------------------------------------------------------------
                                if !local_err {
                                    doc_res.insert(field_name.to_string(),
                                        dt_value_bson);
                                } else {
                                    doc_res.insert(field_name.to_string(), Bson::Null);
                                }
                            }
                            "InputCheckBoxI32" | "InputRadioI32" | "InputNumberI32"
                            | "InputRangeI32" | "SelectI32" => {
                                // Get field value for validation
                                // -----------------------------------------------------------------
                                let field_value: i32 = value_bson_pre.as_i32().unwrap();
                                // Validation of `unique`
                                // -----------------------------------------------------------------
                                Self::check_unique(is_update, attrs.unique,
                                    field_name.to_string(), value_bson_pre, "i32", &coll)
                                    .await.unwrap_or_else(|err| {
                                    global_err = true;
                                    local_err = true;
                                    attrs.error =
                                        Self::accumula_err(&attrs, &err.to_string())
                                            .unwrap();
                                });
                                // Validation of range (`min` <> `max`)
                                // -----------------------------------------------------------------
                                let min: f64 = attrs.min.parse().unwrap();
                                let max: f64 = attrs.max.parse().unwrap();
                                let num: f64 = field_value as f64;
                                if (min > 0_f64 || max > 0_f64) &&
                                    !validate_range(Validator::Range{min: Some(min),
                                                    max: Some(max)}, num) {
                                    global_err = true;
                                    local_err = true;
                                    let msg = format!(
                                        "Number {} is out of range (min={} <> max={}).",
                                        num, min, max);
                                    attrs.error = Self::accumula_err(&attrs, &msg).unwrap();
                                }
                                // Insert result
                                // -----------------------------------------------------------------
                                if !local_err && !ignore_fields.contains(field_name) {
                                    attrs.value = field_value.to_string();
                                    doc_res.insert(field_name.to_string(),
                                        Bson::Int32(field_value));
                                }
                            }
                            "InputCheckBoxU32" | "InputRadioU32" | "InputNumberU32"
                            | "InputRangeU32" | "SelectU32" | "InputCheckBoxI64"
                            | "InputRadioI64" | "InputNumberI64" | "InputRangeI64"
                            | "SelectI64" => {
                                // Get field value for validation
                                // -----------------------------------------------------------------
                                let field_value: i64 = value_bson_pre.as_i64().unwrap();
                                // Validation of `unique`
                                // -----------------------------------------------------------------
                                Self::check_unique(is_update, attrs.unique,
                                    field_name.to_string(), value_bson_pre, "i64", &coll)
                                    .await.unwrap_or_else(|err| {
                                    global_err = true;
                                    local_err = true;
                                    attrs.error =
                                        Self::accumula_err(&attrs, &err.to_string())
                                            .unwrap();
                                });
                                // Validation of range (`min` <> `max`)
                                // -----------------------------------------------------------------
                                let min: f64 = attrs.min.parse().unwrap();
                                let max: f64 = attrs.max.parse().unwrap();
                                let num: f64 = field_value as f64;
                                if (min > 0_f64 || max > 0_f64) &&
                                    !validate_range(Validator::Range{min: Some(min),
                                                    max: Some(max)}, num) {
                                    global_err = true;
                                    local_err = true;
                                    let msg = format!(
                                        "Number {} is out of range (min={} <> max={}).",
                                        num, min, max);
                                    attrs.error = Self::accumula_err(&attrs, &msg).unwrap();
                                }
                                // Insert result
                                // -----------------------------------------------------------------
                                if !local_err && !ignore_fields.contains(field_name) {
                                    attrs.value = field_value.to_string();
                                    doc_res.insert(field_name.to_string(),
                                        Bson::Int64(field_value));
                                }
                            }
                            "InputCheckBoxF64" | "InputRadioF64" | "InputNumberF64"
                            | "InputRangeF64" | "SelectF64" => {
                                // Get field value for validation
                                // -----------------------------------------------------------------
                                let field_value: f64 = value_bson_pre.as_f64().unwrap();
                                // Validation of `unique`
                                // -----------------------------------------------------------------
                                Self::check_unique(is_update, attrs.unique,
                                    field_name.to_string(), value_bson_pre, "f64", &coll)
                                    .await.unwrap_or_else(|err| {
                                    global_err = true;
                                    local_err = true;
                                    attrs.error =
                                        Self::accumula_err(&attrs, &err.to_string())
                                            .unwrap();
                                });
                                // Validation of range (`min` <> `max`)
                                // -----------------------------------------------------------------
                                let min: f64 = attrs.min.parse().unwrap();
                                let max: f64 = attrs.max.parse().unwrap();
                                let num: f64 = field_value.clone();
                                if (min > 0_f64 || max > 0_f64) &&
                                    !validate_range(Validator::Range{min: Some(min),
                                                    max: Some(max)}, num) {
                                    global_err = true;
                                    local_err = true;
                                    let msg = format!(
                                        "Number {} is out of range (min={} <> max={}).",
                                        num, min, max);
                                    attrs.error = Self::accumula_err(&attrs, &msg).unwrap();
                                }
                                // Insert result
                                // -----------------------------------------------------------------
                                if !local_err && !ignore_fields.contains(field_name) {
                                    attrs.value = field_value.to_string();
                                    doc_res.insert(field_name.to_string(),
                                        Bson::Double(field_value));
                                }
                            }
                            "InputCheckBoxBool" => {
                                // Get field value for validation
                                // -----------------------------------------------------------------
                                let field_value: bool = value_bson_pre.as_bool().unwrap();
                                // Validation of `unique`
                                // -----------------------------------------------------------------
                                Self::check_unique(is_update, attrs.unique,
                                    field_name.to_string(), value_bson_pre, "bool", &coll)
                                    .await.unwrap_or_else(|err| {
                                    global_err = true;
                                    local_err = true;
                                    attrs.error =
                                        Self::accumula_err(&attrs, &err.to_string())
                                            .unwrap();
                                });
                                // Insert result
                                // -----------------------------------------------------------------
                                if !local_err && !ignore_fields.contains(field_name) {
                                    attrs.value = field_value.to_string();
                                    doc_res.insert(field_name.to_string(),
                                        Bson::Boolean(field_value));
                                }
                            }
                            _ => {
                                Err(format!("Model: `{}` -> Field: `{}` -> Method: \
                                            `check()` : Unsupported data type.",
                                    MODEL_NAME, field_name))?
                            }
                        }

                        // Insert or update fields for timestamps `created` and `updated`
                        // -------------------------------------------------------------------------
                        if !global_err {
                            let dt: DateTime<Utc> = Utc::now();
                            if !is_update {
                                doc_res.insert("created".to_string(), Bson::DateTime(dt));
                                doc_res.insert("updated".to_string(), Bson::DateTime(dt));
                            } else {
                                let value_from_db: Option<&Bson> = doc_from_db.get("created");
                                if value_from_db.is_some() {
                                    doc_res.insert("created".to_string(), value_from_db.unwrap());
                                    doc_res.insert("updated".to_string(), Bson::DateTime(dt));
                                } else {
                                    Err(format!("Model: `{}` -> Field: `created` -> Method: \
                                                `check()` : Can't get field value from database.",
                                    MODEL_NAME))?
                                }
                            }
                        }
                    }
                } else {
                    Err(format!("Model: `{}` -> Method: `check()` : \
                                Did not receive data from cache.",
                        MODEL_NAME))?
                }

                // Post processing
                // ---------------------------------------------------------------------------------
                let result: OutputData = match output_format {
                    // Get Hash-line
                    OutputType::Hash => {
                        let data: String = Self::to_hash(&attrs_map)?;
                        OutputData::Hash((data, !global_err, doc_res))
                    }
                    // Get Attribute Map
                    OutputType::Map => OutputData::Map((attrs_map, !global_err, doc_res)),
                    // Get Json-line
                    OutputType::Json => {
                        let data: String = Self::to_json(&attrs_map)?;
                        OutputData::Json((data, !global_err, doc_res))
                    }
                    // Get Html-line
                    OutputType::Html => {
                        let data: String = Self::to_html(attrs_map)?;
                        OutputData::Html((data, !global_err, doc_res))
                    }
                };

                Ok(result)
            }

            // Post processing database queries
            // *************************************************************************************
            // Get Hash-line
            pub fn to_hash(attrs_map: &HashMap<String, Transport>) ->
                Result<String, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let mut errors = String::new();
                for (field, trans) in attrs_map {
                    let tmp = if !errors.is_empty() {
                        format!("{} ; ", errors)
                    } else {
                        String::new()
                    };
                    if !trans.error.is_empty() {
                        errors = format!("{}Field: `{}` - {}", tmp, field, trans.error);
                    }
                }
                if errors.is_empty() {
                    Ok(attrs_map
                        .get(&"hash".to_string())
                        .unwrap()
                        .value
                        .clone())
                } else {
                    Err(errors.replace("<br>", " | "))?
                }
            }

            // Get Json-line
            pub fn to_json(attrs_map: &HashMap<String, Transport>) ->
                Result<String, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let mut json_text = String::new();
                for (field, trans) in attrs_map {
                    let tmp = serde_json::to_string(&trans).unwrap();
                    if !json_text.is_empty() {
                        json_text = format!("{},\"{}\":{}", json_text, field, tmp);
                    } else {
                        json_text = format!("\"{}\":{}", field, tmp);
                    }
                }
                Ok(format!("{{{}}}", json_text))
            }

            // Get Html-line
            pub fn to_html(attrs_map: HashMap<String, Transport>) ->
                Result<String, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let controls = Self::html(
                    attrs_map,
                    &stringify!($sname).to_lowercase(),
                    true
                )?;
                Ok(controls)
            }

            // Database Query API
            // *************************************************************************************
            // Save to database as a new document or
            // update an existing document.
            // (Returns the hash-line of the identifier)
            pub async fn save(& mut self, client: &Client, output_format: OutputType) ->
                Result<OutputData, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let verified_data: OutputData = self.check(client, OutputType::Map).await?;
                let mut attrs_map: HashMap<String, Transport> = verified_data.map();
                let meta: Meta = Self::metadata()?;
                let is_update: bool = !self.hash.is_empty();
                let coll: Collection = client.database(&meta.database).collection(&meta.collection);

                // Save to database
                // ---------------------------------------------------------------------------------
                if verified_data.bool() {
                    if !is_update {
                        let result: results::InsertOneResult =
                            coll.insert_one(verified_data.doc(), None).await?;
                        self.hash = result.inserted_id.as_object_id().unwrap().to_hex();
                    } else {
                        let object_id: ObjectId = ObjectId::with_string(&self.hash)
                            .unwrap_or_else(|err| { panic!("{}", err.to_string()) });
                        let query: Document = doc!{"_id": object_id};
                        coll.update_one(query, verified_data.doc(), None).await?;
                    }
                }

                // Add hash-line
                // ---------------------------------------------------------------------------------
                attrs_map.get_mut(&"hash".to_string()).unwrap().value = self.hash.clone();

                // Post processing
                // ---------------------------------------------------------------------------------
                let result: OutputData = match output_format {
                    // Get Hash-line
                    OutputType::Hash => {
                        let data: String = Self::to_hash(&attrs_map)?;
                        OutputData::Hash((data, verified_data.bool(), verified_data.doc()))
                    }
                    // Get Attribute Map
                    OutputType::Map => {
                        OutputData::Map((attrs_map, verified_data.bool(), verified_data.doc()))
                    }
                    // Get Json-line
                    OutputType::Json => {
                        let data: String = Self::to_json(&attrs_map)?;
                        OutputData::Json((data, verified_data.bool(), verified_data.doc()))
                    }
                    // Get Html-line
                    OutputType::Html => {
                        let data: String = Self::to_html(attrs_map)?;
                        OutputData::Html((data, verified_data.bool(), verified_data.doc()))
                    }
                };

                Ok(result)
            }
        }
    }
}
