use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{postgres::Postgres, redis::Redis, schema};

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::modules)]
struct PartialModule {
    id: i32,
    name: String,
}

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::modules)]
struct CompleteModule {
    id: i32,
    name: String,
    description: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

#[derive(Deserialize)]
pub struct NewModuleRequest {
    name: String,
    description: String,
}

#[derive(Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::modules)]
struct NewModule {
    name: String,
    description: String,
}

#[derive(Deserialize)]
pub struct ModifyModuleRequest {
    name: String,
    description: String,
}

#[derive(Serialize, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::modules)]
struct ModifyModule {
    name: String,
    description: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

pub async fn list_modules(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::modules::table
        .select(PartialModule::as_select())
        .load::<PartialModule>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(modules)) => {
            return HttpResponse::Ok().json(modules);
        }
        Ok(None) => {
            return HttpResponse::Ok().json(Vec::<PartialModule>::new());
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_module_by_id(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let result = schema::modules::table
        .select(CompleteModule::as_select())
        .filter(schema::modules::id.eq(search_id.into_inner()))
        .first::<CompleteModule>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(module)) => {
            return HttpResponse::Ok().json(module);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_module_by_name(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_name: web::Path<String>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::modules::table
        .select(CompleteModule::as_select())
        .filter(schema::modules::name.eq(&search_name.into_inner()))
        .first::<CompleteModule>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(module)) => {
            return HttpResponse::Ok().json(module);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn create_module(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    create_module_request: web::Json<NewModuleRequest>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let create_module = NewModule {
        name: create_module_request.name.clone(),
        description: create_module_request.description.clone(),
    };

    let count = diesel::insert_into(schema::modules::table)
        .values(&create_module)
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

pub async fn update_module(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    update_id: web::Path<i32>,
    update_module_request: web::Json<ModifyModuleRequest>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let update_module = ModifyModule {
        name: update_module_request.name.clone(),
        description: update_module_request.description.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let count = diesel::update(schema::modules::table)
        .set(&update_module)
        .filter(schema::modules::id.eq(update_id.into_inner()))
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

pub async fn delete_module(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    delete_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let count = diesel::delete(schema::modules::table)
        .filter(schema::modules::id.eq(delete_id.into_inner()))
        .returning((
            schema::modules::id,
            schema::modules::name,
            schema::modules::description,
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
