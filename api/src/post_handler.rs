// api/src/post_handler.rs

use application::post::{create, read, publish, delete}; 
use domain::models::{Post, NewPost};  
use rocket::{get, post, put, delete, http::Status, serde::json::Json}; 
use rocket::response::status::{NotFound, Created}; 
use serde::Serialize;
use revolt_rocket_okapi::openapi;
use revolt_rocket_okapi::JsonSchema;

#[derive(Serialize, JsonSchema)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}


#[openapi]
#[get("/post")]
pub fn list_posts_handler() -> Result<Json<Vec<Post>>, Status> {
    let posts: Vec<Post> = read::list_posts();
    Ok(Json(posts))
}
#[openapi]
#[get("/healthchecker")]
pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

#[openapi]
#[get("/post/<post_id>")]
pub fn list_post_handler(post_id: i32) -> Result<Json<Post>, NotFound<String>> {
    let post = read::list_post(post_id)?;
    Ok(Json(post))
}

#[openapi]
#[post("/post", format = "application/json", data = "<post>")]
pub fn create_post_handler(post: Json<NewPost>) -> Created<String> {
    create::create_post(post)
}

#[openapi]
#[put("/post/<post_id>/publish")]
pub fn publish_post_handler(post_id: i32) -> Result<Json<Post>, NotFound<String>> {
    let post = publish::publish_post(post_id)?; 
    Ok(Json(post))
}

#[openapi]
#[delete("/post/<post_id>")]
pub fn delete_post_handler(post_id: i32) -> Result<Json<Vec<Post>>, NotFound<String>> {
    let posts = delete::delete_post(post_id)?;
    Ok(Json(posts))
}