//setup database pool
use actix_web::{App, HttpServer, web};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use actix_cors::Cors;

mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "sqlite://e:/Expense_Tracker/expense-tracker/backend/expense_tracker.db";
    let pool = SqlitePoolOptions::new()
        .connect(db_url)
        .await
        .expect("Failed to connect to DB");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::init)
    })
    .bind(("127.0.0.1",8081))?
    .run()
    .await
}