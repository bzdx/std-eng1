use actix_files::{Files, NamedFile};
use actix_web::{web, get, App, HttpResponse, HttpServer, Responder, Result};

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::TemporaryRedirect()
  .header("Content-Type", "text/html; charset=utf-8")
  .header("Location", "/index.html")
  .body("")
}

async fn baidu_seo() -> Result<NamedFile> {
  Ok(NamedFile::open("baidu_verify_code-CNu82SU367.html")?)
}
async fn google_seo() -> Result<NamedFile> {
  Ok(NamedFile::open("google415cd2d0b77f2294.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let path = "127.0.0.1:9901";
  println!("server at http://{}", path);
  HttpServer::new(|| {
    App::new()
      .service(index)
      .route("/baidu_verify_code-CNu82SU367.html", web::get().to(baidu_seo)) // baidu SEO
      .route("/google415cd2d0b77f2294.html", web::get().to(google_seo)) // google SEO
      .service(Files::new("/", "dist").show_files_listing())
  })
  .bind(path)?
  .run()
  .await
}
