use actix_web::web;
use crate::handlers::web_instances::{list_web_instances, list_web_instances_by_module_id, get_instance_by_id, get_instance_by_name, create_instance, update_instance, delete_instance};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/web_instances")
            .route("", web::get().to(list_web_instances))
            .route("/modules/{id}", web::get().to(list_web_instances_by_module_id))
            .route("/{id}", web::get().to(get_instance_by_id))
            .route("/name/{name}", web::get().to(get_instance_by_name))
            .route("", web::post().to(create_instance))
            .route("/{id}", web::put().to(update_instance))
            .route("/{id}", web::delete().to(delete_instance))
    );
}