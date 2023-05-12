use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web};

    #[actix_web::test]
    async fn test_index_ok() {
        let app = test::init_service(App::new().route("/", web::get().to(index))).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }
}
