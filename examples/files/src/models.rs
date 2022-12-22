use green_barrel::*;
use metamorphose::Model;
use serde::{Deserialize, Serialize};

use crate::settings::{
    accounts::SERVICE_NAME, APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, UNIQUE_APP_KEY,
};

#[Model()]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: InputText,
    pub avatar: InputImage,
    pub resume: InputFile,
}

impl Control for User {
    fn custom_default() -> Self {
        Self {
            username: InputText {
                unique: true,
                required: true,
                maxlength: 150,
                ..Default::default()
            },
            avatar: InputImage {
                label: "Avatar".into(),
                default: Some(ImageData {
                    path: "./resources/media/default/no_image.png".into(),
                    url: "/media/default/no_image.png".into(),
                    ..Default::default()
                }),
                target_dir: "users/avatars".into(),
                thumbnails: vec![("xs".into(), 150), ("sm".into(), 300)], // all sizes: "xs","sm","md","lg"
                is_quality: false, // Create thumbnails - Fast=false or qualitatively=true ; Default = true.
                ..Default::default()
            },
            resume: InputFile {
                default: Some(FileData {
                    path: "./resources/media/default/no_file.odt".into(),
                    url: "/media/default/no_file.odt".into(),
                    ..Default::default()
                }),
                label: "Resume".into(),
                target_dir: "users/resume".into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
