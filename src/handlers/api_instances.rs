use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{postgres::Postgres, redis::Redis, schema};

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::api_instances)]
struct PartialApiInstance {
    id: i32,
    module_id: i32,
    name: String,
}

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::api_instances)]
struct CompleteApiInstance {
    id: i32,
    name: String,
    description: String,
    protocol: String,
    host: String,
    port: i32,
    path: String,
    module_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

#[derive(Deserialize)]
pub struct NewApiInstanceRequest {
    name: String,
    description: String,
    protocol: String,
    host: String,
    port: i32,
    path: String,
    module_id: i32,
}

#[derive(Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::api_instances)]
struct NewApiInstance {
    name: String,
    description: String,
    protocol: String,
    host: String,
    port: i32,
    path: String,
    module_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ModifyApiInstanceRequest {
    name: String,
    description: String,
    protocol: String,
    host: String,
    port: i32,
    path: String,
}

#[derive(Serialize, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::api_instances)]
struct ModifyApiInstance {
    name: String,
    description: String,
    protocol: String,
    host: String,
    port: i32,
    path: String,
    updated_at: NaiveDateTime,
}

pub async fn list_api_instances(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::api_instances::table
        .select(PartialApiInstance::as_select())
        .load::<PartialApiInstance>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(api_instances)) => {
            return HttpResponse::Ok().json(api_instances);
        }
        Ok(None) => {
            return HttpResponse::Ok().json(Vec::<PartialApiInstance>::new());
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn list_api_instances_by_module_id(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::api_instances::table
        .select(PartialApiInstance::as_select())
        .filter(schema::api_instances::module_id.eq(search_id.into_inner()))
        .load::<PartialApiInstance>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(api_instances)) => {
            return HttpResponse::Ok().json(api_instances);
        }
        Ok(None) => {
            return HttpResponse::Ok().json(Vec::<PartialApiInstance>::new());
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_instance_by_id(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let result = schema::api_instances::table
        .select(CompleteApiInstance::as_select())
        .filter(schema::api_instances::id.eq(search_id.into_inner()))
        .first::<CompleteApiInstance>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(instance)) => {
            return HttpResponse::Ok().json(instance);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_instance_by_name(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_name: web::Path<String>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::api_instances::table
        .select(CompleteApiInstance::as_select())
        .filter(schema::api_instances::name.eq(&search_name.into_inner()))
        .first::<CompleteApiInstance>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(instance)) => {
            return HttpResponse::Ok().json(instance);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn create_instance(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    create_instance_request: web::Json<NewApiInstanceRequest>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let create_instance = NewApiInstance {
        name: create_instance_request.name.clone(),
        description: create_instance_request.description.clone(),
        protocol: create_instance_request.protocol.clone(),
        host: create_instance_request.host.clone(),
        port: create_instance_request.port,
        path: create_instance_request.path.clone(),
        module_id: create_instance_request.module_id,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let count = diesel::insert_into(schema::api_instances::table)
        .values(&create_instance)
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Created().finish();
        }
        Ok(None) => {
            return HttpResponse::InternalServerError().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn update_instance(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    update_id: web::Path<i32>,
    update_instance_request: web::Json<ModifyApiInstanceRequest>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let update_instance = ModifyApiInstance {
        name: update_instance_request.name.clone(),
        description: update_instance_request.description.clone(),
        protocol: update_instance_request.protocol.clone(),
        host: update_instance_request.host.clone(),
        port: update_instance_request.port,
        path: update_instance_request.path.clone(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let count = diesel::update(schema::api_instances::table)
        .set(&update_instance)
        .filter(schema::api_instances::id.eq(update_id.into_inner()))
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Ok().finish();
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn delete_instance(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    delete_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let count = diesel::delete(schema::api_instances::table)
        .filter(schema::api_instances::id.eq(delete_id.into_inner()))
        .returning((
            schema::api_instances::id,
            schema::api_instances::name,
            schema::api_instances::description,
        ))
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Ok().finish();
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}
