//! # APPLICATION SETTINGS

pub use application_state::*;
pub use corrective_functions::*;
pub use default_settings::*;

// DEFAULT SETTINGS ================================================================================
pub mod default_settings {
    pub static DEBUG: bool = true;
    pub static PROJECT_NAME: &str = "Example";
    pub static LOCAL_DOMAIN: &str = "127.0.0.1";
    pub static PORT: u16 = 8088;
    pub static SITE_DOMAIN: &str = "www.site-name.net";
    // 2.016 mb
    pub static MAX_UPLOAD_SIZE: usize = (2.016 * 1024.0 * 1024.0) as usize;
    // http://www.miniwebtool.com/django-secret-key-generator/
    pub static SECRET_KEY: &str = "hf@$%#-ftw(ia4jualowaqlejtm17h*98pzqk18bd65um5_xnx";
    pub static SESSION_KEY: &[u8] = SECRET_KEY.as_bytes();
    pub static MEDIA_URL: &str = "/media/";
    pub static MEDIA_ROOT: &str = "./media/";
    pub static STATIC_URL: &str = "/static/";
    pub static STATIC_ROOT: &str = "./static/";
    pub static TEMPLATES: &str = "./templates/";
}

// CORRECTIVE FUNCTIONS ============================================================================
pub mod corrective_functions {
    use super::*;

    pub fn local_domain() -> String {
        format!("{}:{}", LOCAL_DOMAIN, PORT)
    }

    pub fn local_url() -> String {
        format!("http://{}:{}", LOCAL_DOMAIN, PORT)
    }

    pub fn site_domain(debug: bool) -> String {
        match debug {
            true => local_domain(),
            false => SITE_DOMAIN.to_string(),
        }
    }

    pub fn site_url(debug: bool) -> String {
        match debug {
            true => local_url(),
            false => format!("https://{}", SITE_DOMAIN),
        }
    }

    pub fn session_name(project_name: &str) -> String {
        format!("{}_session", project_name.to_lowercase().replace(" ", "_"))
    }
}

// APPLICATION STATE ===============================================================================
pub mod application_state {
    use super::*;

    #[derive(Clone)]
    pub struct AppState {
        debug: bool,
        app_name: String,
        media_url: String,
        media_root: String,
        static_url: String,
        static_root: String,
        templates: String,
    }

    impl AppState {
        pub fn new() -> Self {
            Self {
                debug: DEBUG,
                app_name: PROJECT_NAME.to_string(),
                media_url: MEDIA_URL.to_string(),
                media_root: MEDIA_ROOT.to_string(),
                static_url: STATIC_URL.to_string(),
                static_root: STATIC_ROOT.to_string(),
                templates: TEMPLATES.to_string(),
            }
        }
        // Get status debug
        pub fn get_debug(&self) -> bool {
            self.debug
        }
        // Get App name
        pub fn get_app_name(&self) -> String {
            self.app_name.clone()
        }
        // Get media file path
        pub fn get_media_url(&self, inner_path: &str) -> String {
            format!("{}{}", self.media_url, inner_path)
        }
        pub fn get_media_root(&self, inner_path: &str) -> String {
            format!("{}{}", self.media_root, inner_path)
        }
        // Get static file path
        pub fn get_static_url(&self, inner_path: &str) -> String {
            format!("{}{}", self.static_url, inner_path)
        }
        pub fn get_static_root(&self, inner_path: &str) -> String {
            format!("{}{}", self.static_root, inner_path)
        }
        // Get template file path
        pub fn get_template(&self, inner_path: &str) -> String {
            format!("{}{}", self.templates, inner_path)
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Corrective functions ------------------------------------------------------------------------
    #[test]
    fn test_local_domain() {
        assert_eq!(local_domain(), "127.0.0.1:8088".to_string());
    }

    #[test]
    fn test_local_url() {
        assert_eq!(local_url(), "http://127.0.0.1:8088".to_string());
    }

    #[test]
    fn test_site_domain() {
        // DEBUG = true
        assert_eq!(site_domain(true), "127.0.0.1:8088".to_string());
        assert_ne!(site_domain(true), "www.site-name.net".to_string());
        // DEBUG = false
        assert_eq!(site_domain(false), "www.site-name.net".to_string());
        assert_ne!(site_domain(false), "127.0.0.1:8088".to_string());
    }

    #[test]
    fn test_site_url() {
        // DEBUG = true
        assert_eq!(site_url(true), "http://127.0.0.1:8088".to_string());
        assert_ne!(site_url(true), "https://www.site-name.net".to_string());
        // DEBUG = false
        assert_eq!(site_url(false), "https://www.site-name.net".to_string());
        assert_ne!(site_url(false), "http://127.0.0.1:8088".to_string());
    }

    #[test]
    fn test_session_name() {
        assert_eq!(session_name("Example"), "example_session");
        assert_eq!(session_name("Example Two"), "example_two_session");
        assert_eq!(session_name(PROJECT_NAME), "example_session");
    }

    // app state ----------------------------------------------------------------------------------
    #[test]
    fn test_app_state() {
        let app_state = AppState::new();
        // Testing of methods
        assert_eq!(app_state.get_debug(), DEBUG);
        assert_eq!(app_state.get_app_name(), "Example".to_string());
        assert_eq!(app_state.get_media_url("img.jpg"), "/media/img.jpg");
        assert_eq!(app_state.get_media_root("img.jpg"), "./media/img.jpg");
        assert_eq!(
            app_state.get_static_url("css/style.css"),
            "/static/css/style.css"
        );
        assert_eq!(
            app_state.get_static_root("css/style.css"),
            "./static/css/style.css"
        );
        assert_eq!(
            app_state.get_template("index.html"),
            "./templates/index.html"
        );
    }
}
