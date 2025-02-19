use crate::models::{AppState, Document};
use actix_web::{get, post, Responder};
use actix_web::web;
use elasticsearch::{IndexParts, SearchParts};
use log::info;
use serde_json::{json, Value};

static INDEX_NAME: &str = "my_index";

async fn index() -> &'static str {
    "Hello World"
}

#[post("/create_document")]
pub async fn create_document(data: web::Data<AppState>) -> impl Responder {
    let client = data.es_client.lock().await;

    // 1. 创建索引
    let create_index_response = client
        .indices()
        .create(elasticsearch::indices::IndicesCreateParts::Index(
            INDEX_NAME,
        ))
        .body(json!({
            "settings": {
                "number_of_shards": 1,
                "number_of_replicas": 1
            },
            "mappings": {
                "properties": {
                    "id": { "type": "integer" },
                    "title": { "type": "text" },
                    "content": { "type": "text" }
                }
            }
        }))
        .send()
        .await
        .unwrap();

    info!("Index created: {:?}", create_index_response);
    "Index created"
}

#[get("/search_document/{query}")]
pub async fn search_document(
    data: web::Data<AppState>,
    query: web::Path<String>,
) -> impl Responder {
    let client = data.es_client.lock().await;
    let search_response = client
        .search(SearchParts::Index(&[INDEX_NAME]))
        .body(json!({
            "query": {
                "match": {
                    "content": query.into_inner()
                }
            }
        }))
        .send()
        .await
        .unwrap();

    let response_body = search_response.json::<Value>().await.unwrap();
    info!("Search results: {:?}", response_body);

    response_body.to_string()
}

#[get("/insert_document")]
pub async fn insert_document(
    data: web::Data<AppState>,
    doc: web::Json<Document>,
) -> impl Responder {
    let client = data.es_client.lock().await;
    let doc = doc.into_inner();

    let response = client
        .index(IndexParts::IndexId(INDEX_NAME, "1"))
        .body(&doc)
        .send()
        .await
        .unwrap();

    info!("Document indexed: {:?}", response);
    response.text().await.unwrap()
}

#[get("/update_document")]
pub async fn update_document(
    data: web::Data<AppState>,
    doc: web::Json<Document>,
) -> impl Responder {
    let client = data.es_client.lock().await;
    let doc = doc.into_inner();
    let response = client
        .index(IndexParts::IndexId(INDEX_NAME, doc.id.to_string().as_str()))
        .body(&doc)
        .send()
        .await
        .unwrap();
    response.text().await.unwrap()
}
