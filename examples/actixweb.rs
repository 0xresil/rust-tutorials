use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result, middleware::Logger};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
  name: String,
  age: i32
}

async fn greet(req: HttpRequest) -> Result<impl Responder> {
  let name = req.match_info().get("name").unwrap_or("World");
  if name == "Error" {
    Err(actix_web::error::ErrorInternalServerError("an error"))
  } else {
    Ok(web::Json(Person { name: name.to_string(), age: 28}))
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
  HttpServer::new(|| {
    App::new()
      .wrap(Logger::new("%a ${User-Agent}i"))
      .route("/{name}", web::get().to(greet))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::{test, web, App};
  use actix_web::web::Bytes;

  #[actix_rt::test]
  async fn test_index_get() {
    let mut app = test::init_service(
      App::new()
        .route("/{name}", web::get().to(greet)),
    )
    .await;

    let req = test::TestRequest::get().uri("/Jack-Poon").to_request();
    let resp = test::read_response(&mut app, req).await;

    assert_eq!(resp, serde_json::to_vec(&Person { name: "Jack-Poon".to_string(), age: 28}).unwrap());
  }
}
