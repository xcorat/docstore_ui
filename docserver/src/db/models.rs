use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub client_id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub social_security_number: String,
    pub address: String,
    pub phone_number: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxReturn {
    pub tax_return_id: Option<i64>,
    pub client_id: i64,
    pub tax_year: i32,
    pub filing_status: String,
    pub income_sources: HashMap<String, f64>,
    pub deductions: HashMap<String, f64>,
    pub credits: HashMap<String, f64>,
    pub taxes_paid: f64,
    pub tax_liability: f64,
    pub refund_or_amount_due: f64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
