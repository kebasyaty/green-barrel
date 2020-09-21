use actix_web::{web, HttpResponse, Responder};
use tera::{Context, Tera};

use super::super::settings;

pub mod mango_models;

pub use configure_urls::*;
pub use request_handlers::*;

// CONFIGURE URLs ==================================================================================
pub mod configure_urls {
    use super::*;

    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/user").route(web::get().to(user)));
    }
}

// REQUEST HANDLERS ================================================================================
pub mod request_handlers {
    use super::*;

    // Home page
    pub async fn index(
        app_state: web::Data<settings::AppState>,
        tmpl: web::Data<Tera>,
        // form: web::Data<mango_models::User>,
    ) -> impl Responder {
        let mut ctx = Context::new();
        ctx.insert("title", &app_state.get_app_name());
        ctx.insert(
            "description",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        );
        ctx.insert("form", &mango_models::User::form_attrs());
        let rendered = tmpl.render("index.html", &ctx).unwrap();
        HttpResponse::Ok().content_type("text/html").body(rendered)
    }

    // User page -----------------------------------------------------------------------------------
    pub async fn user(
        app_state: web::Data<settings::AppState>,
        tmpl: web::Data<Tera>,
        form: web::Query<mango_models::User>,
    ) -> impl Responder {
        let mut ctx = Context::new();
        ctx.insert("title", &app_state.get_app_name());
        ctx.insert(
            "description",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        );
        ctx.insert("username", &form.username);
        ctx.insert("email", &form.email);
        let rendered = tmpl.render("user.html", &ctx).unwrap();
        HttpResponse::Ok().content_type("text/html").body(rendered)
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, web, App};

    // Handlers ------------------------------------------------------------------------------------
    #[actix_rt::test]
    async fn test_index_ok() {
        let app_state = web::Data::new(settings::AppState::new());
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        let mut app = test::init_service(
            App::new()
                .app_data(app_state)
                .data(tera)
                .route("/", web::get().to(index)),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
