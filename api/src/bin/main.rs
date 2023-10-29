// api/src/bin/main.rs

#[macro_use] extern crate rocket;
use api::post_handler;

use revolt_rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use revolt_rocket_okapi::openapi_get_routes;


fn get_docs() -> SwaggerUIConfig {

    SwaggerUIConfig {
        url: "/api/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", openapi_get_routes![
            post_handler::list_posts_handler, 
            post_handler::list_post_handler,
            post_handler::create_post_handler,
            post_handler::publish_post_handler,
            post_handler::delete_post_handler,
            post_handler::health_checker_handler,
        ])
        .mount("/swagger", make_swagger_ui(&get_docs()))

}