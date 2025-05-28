//Implement the endpoints 
use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::{Expense, ExpenseInput};
use uuid::Uuid;
use chrono::Utc;
use validator::Validate;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/expenses").route(web::post().to(create_expense)).route(web::get().to(get_expenses)))
        .service(web::resource("/expenses/{id}").route(web::get().to(get_expense)).route(web::put().to(update_expense)).route(web::delete().to(delete_expense)))
        .service(web::resource("/expenses/summary").route(web::get().to(get_summary)));
}

// POST /expenses
async fn create_expense(pool: web::Data<SqlitePool>, item: web::Json<ExpenseInput>) -> impl Responder {
    if let Err(e) = item.validate() {
        return HttpResponse::BadRequest().json(e);
    }
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let query = sqlx::query!(
        r#"
        INSERT INTO expenses (id, amount, description, category, expense_date, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        id,
        item.amount,
        item.description,
        item.category,
        item.expense_date,
        now,
        now
    );
    if let Err(e) = query.execute(pool.get_ref()).await {
        return HttpResponse::InternalServerError().body(format!("DB error: {}", e));
    }
    HttpResponse::Ok().json(serde_json::json!({ "id": id }))
}

// GET /expenses
async fn get_expenses(pool: web::Data<SqlitePool>) -> impl Responder {
    let rows = sqlx::query_as!(
        Expense,
        r#"SELECT * FROM expenses ORDER BY expense_date DESC"#
    )
    .fetch_all(pool.get_ref())
    .await;
    match rows {
        Ok(expenses) => HttpResponse::Ok().json(expenses),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}

// GET /expenses/{id}
async fn get_expense(pool: web::Data<SqlitePool>, id: web::Path<String>) -> impl Responder {
    let row = sqlx::query_as!(
        Expense,
        r#"SELECT * FROM expenses WHERE id = ?"#,
        id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await;
    match row {
        Ok(Some(expense)) => HttpResponse::Ok().json(expense),
        Ok(None) => HttpResponse::NotFound().body("Expense not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}

// PUT /expenses/{id}
async fn update_expense(
    pool: web::Data<SqlitePool>,
    id: web::Path<String>,
    item: web::Json<ExpenseInput>,
) -> impl Responder {
    if let Err(e) = item.validate() {
        return HttpResponse::BadRequest().json(e);
    }
    let now = Utc::now().to_rfc3339();
    let result = sqlx::query!(
        r#"
        UPDATE expenses SET amount = ?, description = ?, category = ?, expense_date = ?, updated_at = ?
        WHERE id = ?
        "#,
        item.amount,
        item.description,
        item.category,
        item.expense_date,
        now,
        id.into_inner()
    )
    .execute(pool.get_ref())
    .await;
    match result {
        Ok(r) if r.rows_affected() > 0 => HttpResponse::Ok().body("Updated"),
        Ok(_) => HttpResponse::NotFound().body("Expense not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}

// DELETE /expenses/{id}
async fn delete_expense(pool: web::Data<SqlitePool>, id: web::Path<String>) -> impl Responder {
    let result = sqlx::query!(
        r#"DELETE FROM expenses WHERE id = ?"#,
        id.into_inner()
    )
    .execute(pool.get_ref())
    .await;
    match result {
        Ok(r) if r.rows_affected() > 0 => HttpResponse::Ok().body("Deleted"),
        Ok(_) => HttpResponse::NotFound().body("Expense not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}

// GET /expenses/summary
async fn get_summary(pool: web::Data<SqlitePool>) -> impl Responder {
    let row = sqlx::query!(
        r#"SELECT SUM(amount) as total FROM expenses WHERE strftime('%Y-%m', expense_date) = strftime('%Y-%m', 'now')"#
    )
    .fetch_one(pool.get_ref())
    .await;
    match row {
        Ok(r) => HttpResponse::Ok().json(serde_json::json!({ "total": r.total })),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}