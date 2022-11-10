mod voucher;
mod error;

use actix_web::{
    dev::ServiceRequest, get, delete,
    middleware::{Logger, NormalizePath, TrailingSlash}, post,
    web::{self, Data, Json, Query}, App,
    FromRequest,
    HttpResponse, HttpResponseBuilder, HttpServer, Responder, HttpRequest};
use actix_web::web::Path;
use mongodb::{Client, Database};
use mongodb::bson::oid::ObjectId;
use std::str::FromStr;

use crate::voucher::{VoucherName, VoucherNameInput};
use crate::error::{Error, ErrorKind, Result};

#[post("/create")]
async fn create(
    db: Data<Database>,
    data: Json<VoucherNameInput>,
) -> Result<HttpResponse> {
    println!("create1");
    let res =
        VoucherName::create_voucher_name(&db.into_inner(), data.into_inner())
            .await?;
    Ok(HttpResponse::Ok().json(res))
}
#[get("/list")]
async fn list(db: Data<Database>) -> Result<HttpResponse> {
    let res =
        VoucherName::list_voucher_name(&db.into_inner())
            .await?;
    Ok(HttpResponse::Ok().json(res))
}
#[get("/get/{id}")]
async fn get(db: Data<Database>, id: Path<String>) -> Result<HttpResponse> {
    println!("get1");
    let id = id.into_inner();
    let id = ObjectId::from_str(&id).unwrap();
    let res =
        VoucherName::get_voucher_name(&db.into_inner(), id)
            .await?;
    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let listen_port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT not set");
    println!("listen_port:{}",&listen_port);
    let test_db = std::env::var("TEST_DB").expect("TEST_DB not set");
    println!("testdb:{test_db}");
    let client = Client::with_uri_str(test_db).await.unwrap();
    let db = client.default_database().expect("Default database not set");
    println!("db name:{}",&db.name());
    println!("Server started successfully on port {}", listen_port);
    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(create)
            .service(list)
            .service(get)
            .app_data(Data::new(db.clone()))
    })
        .bind(format!("0.0.0.0:{}", listen_port))?
        .run()
        .await
}
