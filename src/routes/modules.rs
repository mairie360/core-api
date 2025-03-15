use actix_web::web;
use crate::handlers::modules::{list_modules, get_module_by_id, get_module_by_name, create_module, update_module, delete_module};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/modules")
            .route("", web::get().to(list_modules))
            .route("/{id}", web::get().to(get_module_by_id))
            .route("/name/{name}", web::get().to(get_module_by_name))
            .route("", web::post().to(create_module))
            .route("/{id}", web::put().to(update_module))
            .route("/{id}", web::delete().to(delete_module))
    );
}