use actix_files::{Files, NamedFile};
use actix_web::{web, get, App, HttpResponse, HttpServer, Responder, Result};

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::TemporaryRedirect()
  .header("Content-Type", "text/html; charset=utf-8")
  .header("Location", "/index.html")
  .body("")
}

// #[get("/baidu_verify_code-abGIytVVhs.html")]
async fn baidu() -> Result<NamedFile> {
  println!("{}", 123123132);
  Ok(NamedFile::open("baidu_verify_code-CNu82SU367.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let path = "127.0.0.1:9901";
  println!("server at http://{}", path);
  HttpServer::new(|| {
    App::new()
      .service(index)
      .route("/baidu_verify_code-CNu82SU367.html", web::get().to(baidu)) // baidu SEO
      .service(Files::new("/", "dist").show_files_listing())
  })
  .bind(path)?
  .run()
  .await
}