use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::{Expense, ExpenseInput};
use uuid::Uuid;
use chrono::Utc;
use validator::Validate;
use serde_json::json;  // ← add this

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/expenses").route(web::post().to(create_expense)).route(web::get().to(get_expenses)))
        .service(web::resource("/expenses/summary").route(web::get().to(get_summary))) // ← move up
        .service(web::resource("/expenses/{id}")
            .route(web::get().to(get_expense))
            .route(web::put().to(update_expense))
            .route(web::delete().to(delete_expense)));
}

// POST /expenses
async fn create_expense(pool: web::Data<SqlitePool>, item: web::Json<ExpenseInput>) -> impl Responder {
    if let Err(e) = item.validate() {
        return HttpResponse::BadRequest().json(e);
    }
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    if let Err(e) = sqlx::query!(
        r#"
        INSERT INTO expenses (id, amount, description, category, expense_date, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        id, item.amount, item.description, item.category, item.expense_date, now, now
    )
    .execute(pool.get_ref())
    .await
    {
        return HttpResponse::InternalServerError().json(json!({ "error": format!("DB error: {}", e) }));
    }
    HttpResponse::Created().json(json!({ "id": id })) // ← 201 Created
}

// GET /expenses/summary
async fn get_summary(pool: web::Data<SqlitePool>) -> impl Responder {
    let row = sqlx::query!(
        r#"SELECT SUM(amount) as total 
           FROM expenses 
           WHERE strftime('%Y-%m', expense_date) = strftime('%Y-%m', date('now'))"# // ← date('now')
    )
    .fetch_one(pool.get_ref())
    .await;
    match row {
        Ok(r) => HttpResponse::Ok().json(json!({ "total": r.total })),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": format!("DB error: {}", e) })),
    }
}
