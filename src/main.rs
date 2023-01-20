use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::error::{TRANSIENT_TRANSACTION_ERROR, UNKNOWN_TRANSACTION_COMMIT_RESULT};
use mongodb::options::{
    Acknowledgment, ClientOptions, FindOneAndUpdateOptions, ReadConcern, ReturnDocument,
    TransactionOptions, WriteConcern,
};
use mongodb::{Client, ClientSession, Database};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stock {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    stock: f64,
}

#[get("/looptest")]
async fn looptest() -> impl Responder {
    println!("1looptest");
    // let a = loop {
    //     println!("loop");
    // break "a".to_string()
    //
    // };
    format!("test")
}

#[get("/hello/{qty}")]
async fn greet(param: web::Path<f64>, client: web::Data<Client>) -> impl Responder {
    // format!("Hello {}!", client.database("test").name())
    let db = client.database("test");
    let data = param.into_inner();

    let mut session = client.start_session(None).await.unwrap();
    let options = TransactionOptions::builder()
        .read_concern(ReadConcern::majority())
        .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
        .build();
    session.start_transaction(options).await.unwrap();
    let mut upd_stock = 0.0;

    while let Err(error) = execute_transaction(&db, &mut upd_stock, data, &mut session).await {
        if !error.contains_label(TRANSIENT_TRANSACTION_ERROR) {
            break;
        }
    }

    async fn execute_transaction(
        db: &Database,
        upd_stock: &mut f64,
        param: f64,
        session: &mut ClientSession,
    ) -> mongodb::error::Result<()> {
        let stock = db
            .collection::<Stock>("test1")
            .find_one_with_session(doc! {"name": "a"}, None, session)
            .await
            .unwrap()
            .unwrap();
        // sleep(Duration::from_secs(30)).await;
        println!("data");
        if stock.stock >= param {
            println!("no data");
            let find_one_update_option = FindOneAndUpdateOptions::builder()
                .upsert(true)
                .return_document(Some(ReturnDocument::After))
                .build();
            let out = db
                .collection::<Stock>("test1")
                .find_one_and_update_with_session(
                    doc! {"name": "a"},
                    doc! {"$inc": {"stock": - param}},
                    find_one_update_option,
                    session,
                )
                .await
                .unwrap()
                .unwrap();
            *upd_stock = out.stock;

            loop {
                println!("loop");
                let result = session.commit_transaction().await;
                println!("result");
                if let Err(ref error) = result {
                    println!("error");
                    if error.contains_label(UNKNOWN_TRANSACTION_COMMIT_RESULT) {
                        continue;
                    }
                }
                println!("test");
                break result?;
            }
        }
        Ok(())
    }

    format!("{:?}", upd_stock)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut client_options = ClientOptions::parse(
        "mongodb+srv://testadmin:rootroot@auditplus-test.dqqxs.mongodb.net/?retryWrites=true",
    )
    .await
    .unwrap();
    let client = Client::with_options(client_options).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(looptest)
            .service(greet)
    })
    .bind(("127.0.0.1", 8050))?
    .run()
    .await
}
