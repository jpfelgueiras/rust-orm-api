// api/src/post_handler.rs

use shared::response_models::{Response, ResponseBody};
use application::post::{create, read, publish, delete}; 
use domain::models::{Post, NewPost};  
use rocket::{get, post, put, delete}; 
use rocket::response::status::{NotFound, Created}; 
use rocket::serde::json::Json;

use revolt_rocket_okapi::openapi;


#[openapi]
#[get("/post")]
pub fn list_posts_handler() -> String {
    let posts: Vec<Post> = read::list_posts();
    let response = Response { body: ResponseBody::Posts(posts) };

    serde_json::to_string(&response).unwrap()
}

#[openapi]
#[get("/post/<post_id>")]
pub fn list_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post = read::list_post(post_id)?;
    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[openapi]
#[post("/post", format = "application/json", data = "<post>")]
pub fn create_post_handler(post: Json<NewPost>) -> Created<String> {
    create::create_post(post)
}

#[openapi]
#[put("/post/<post_id>/publish")]
pub fn publish_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post = publish::publish_post(post_id)?; 
    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[openapi]
#[delete("/post/<post_id>")]
pub fn delete_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let posts = delete::delete_post(post_id)?;
    let response = Response { body: ResponseBody::Posts(posts) };

    Ok(serde_json::to_string(&response).unwrap())
}