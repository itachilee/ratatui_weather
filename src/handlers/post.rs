use crate::db::models::*;
use crate::db::schema::posts::dsl::*;
use crate::models::common::*;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;

#[get("/query")]
pub async fn test(data: web::Data<AppState>) -> impl Responder {
    // 从连接池获取连接
    let pool = &data.pg_db;
    let conn = &mut pool
        .get()
        .map_err(|e| {
            eprintln!("Failed to get database connection: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.title);
    //     println!("-----------\n");
    //     println!("{}", post.body);
    // }
    // HttpResponse::Ok().body("asd");

    serde_json::to_string(&results)
}
