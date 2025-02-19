use crate::db::models::*;
use crate::db::schema::posts::dsl::*;
use crate::models::common::*;
use actix_web::{get, post, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[get("/query")]
pub async fn query(data: web::Data<AppState>) -> HttpResponse {
    // 从连接池获取连接
    let pool = &data.pg_db;

    let conn = &mut pool
        .get()
        .map_err(|e| {
            eprintln!("Failed to get database connection: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    match posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(conn)
    {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateNewPost {
    pub title: String,
    pub body: String,
}

#[post("/create_post")]
pub async fn create_post(req: web::Json<CreateNewPost>, data: web::Data<AppState>) -> HttpResponse {
    let pool = &data.pg_db;
    let conn = &mut pool
        .get()
        .map_err(|e| {
            eprintln!("Failed to get database connection: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    let new_post = NewPost {
        title: &req.title,
        body: &req.body,
    };

    match diesel::insert_into(posts)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
    {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
struct BaseId {
    id: i32,
}

#[post("/update_post")]
pub async fn update_post(req: web::Json<BaseId>, data: web::Data<AppState>) -> HttpResponse {
    let pool = &data.pg_db;
    let conn = &mut pool
        .get()
        .map_err(|e| {
            eprintln!("Failed to get database connection: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    match diesel::update(posts.find(req.id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(conn)
    {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}
#[post("/delete_post")]
pub async fn delete_post(req: web::Json<BaseId>, data: web::Data<AppState>) -> HttpResponse {
    let pool = &data.pg_db;
    let conn = &mut pool
        .get()
        .map_err(|e| {
            eprintln!("Failed to get database connection: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    match diesel::delete(posts.filter(id.eq(req.id))).execute(conn) {
        Ok(reponse) => HttpResponse::Ok().json(ApiResponse::success(reponse)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}
