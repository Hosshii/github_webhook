// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

use github_webhook::event::{self, Event};
use serde_json;
use std::fs;
fn main() {
    let event = std::env::args().nth(1).unwrap();
    let filename = format!("data/{}.json", event);
    let content = fs::read_to_string(filename).unwrap();
    let patched = event::patch_payload_json(&event, &content);
    let data = serde_json::from_str::<Event>(&patched);
    println!("{:#?}", data);
}
