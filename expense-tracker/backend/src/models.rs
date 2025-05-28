//Define the Expense Model
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ExpenseInput {
    #[validate(range(min = 0.01))]
    pub amount: f64,
    pub description: Option<String>,
    #[validate(custom = "validate_category")]
    pub category: String,
    pub expense_date: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Expense {
    pub id: String,
    pub amount: f64,
    pub description: Option<String>,
    pub category: String,
    pub expense_date: String,
    pub created_at: String,
    pub updated_at: String,
}

// Custom validator for category
fn validate_category(category: &str) -> Result<(), validator::ValidationError> {
    let allowed = ["Work", "Personal", "Food", "Transport", "Utilities", "Entertainment", "Others"];
    if allowed.contains(&category) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_category"))
    }
}