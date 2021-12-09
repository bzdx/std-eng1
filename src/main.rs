use actix_files::{Files};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::TemporaryRedirect()
  .header("Content-Type", "text/html; charset=utf-8")
  .header("Location", "/index.html")
  .body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let path = "127.0.0.1:9901";
  println!("server at http://{}", path);
  HttpServer::new(|| {
    App::new()
      .service(index)
      .service(Files::new("/", "dist").show_files_listing())
  })
  .bind(path)?
  .run()
  .await
}