use rocket::{get, State};
use rocket::serde::json::Json;
use crate::config::AppState;
use crate::db::{Client, TaxReturn};

#[get("/clients")]
pub async fn list_clients(state: &State<AppState>) -> Json<Vec<Client>> {
    let db_lock = state.get_db().expect("Database connection should be available");
    let db = db_lock.as_ref().expect("Database should be initialized");
    
    let conn = db.conn.lock().expect("Failed to acquire database connection lock");
    // Execute query to get all clients
    let mut stmt = conn.prepare("
        SELECT client_id, first_name, last_name, social_security_number, 
               address, phone_number, email, created_at, updated_at 
        FROM clients
    ").expect("Failed to prepare statement");

    let clients = stmt.query_map([], |row| {
        Ok(Client {
            client_id: Some(row.get(0)?),
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            social_security_number: row.get(3)?,
            address: row.get(4)?,
            phone_number: row.get(5)?,
            email: row.get(6)?,
            created_at: Some(row.get(7)?),
            updated_at: Some(row.get(8)?),
        })
    }).expect("Failed to execute query")
    .filter_map(Result::ok)
    .collect();

    Json(clients)
}

#[get("/clients/<client_id>")]
pub async fn get_client(state: &State<AppState>, client_id: i64) -> Option<Json<Client>> {
    let db_lock = state.get_db().expect("Database connection should be available");
    let db = db_lock.as_ref().expect("Database should be initialized");
    
    let conn = db.conn.lock().expect("Failed to acquire database connection lock");
    // Execute query to get specific client
    let mut stmt = conn.prepare("
        SELECT client_id, first_name, last_name, social_security_number, 
               address, phone_number, email, created_at, updated_at 
        FROM clients 
        WHERE client_id = ?
    ").expect("Failed to prepare statement");

    let client = stmt.query_row([client_id], |row| {
        Ok(Client {
            client_id: Some(row.get(0)?),
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            social_security_number: row.get(3)?,
            address: row.get(4)?,
            phone_number: row.get(5)?,
            email: row.get(6)?,
            created_at: Some(row.get(7)?),
            updated_at: Some(row.get(8)?),
        })
    }).ok();

    client.map(Json)
}

#[get("/clients/<client_id>/files")]
pub async fn list_client_files(state: &State<AppState>, client_id: i64) -> Json<Vec<String>> {
    let root_path = state.get_root_path().unwrap();
    let client_path = root_path.join(client_id.to_string());
    
    let mut files = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&client_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Ok(file_name) = entry.file_name().into_string() {
                            files.push(file_name);
                        }
                    }
                }
            }
        }
    }
    Json(files)
}
fn map_tax_return(row: &rusqlite::Row) -> rusqlite::Result<TaxReturn> {
    Ok(TaxReturn {
        tax_return_id: Some(row.get(0)?),
        client_id: row.get(1)?,
        tax_year: row.get(2)?,
        filing_status: row.get(3)?,
        income_sources: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
        deductions: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
        credits: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
        taxes_paid: row.get(7)?,
        tax_liability: row.get(8)?,
        refund_or_amount_due: row.get(9)?,
        created_at: Some(row.get(10)?),
        updated_at: Some(row.get(11)?),
    })
}

#[get("/returns?<client_id>")]
pub async fn list_returns(state: &State<AppState>, client_id: Option<i64>) -> Json<Vec<TaxReturn>> {
    let db_lock = state.get_db().expect("Database connection should be available");
    let db = db_lock.as_ref().expect("Database should be initialized");
    let conn = db.conn.lock().expect("Failed to acquire database connection lock");

    let query = if let Some(_cid) = client_id {
        "SELECT tax_return_id, client_id, tax_year, filing_status, income_sources, 
                deductions, credits, taxes_paid, tax_liability, refund_or_amount_due,
                created_at, updated_at 
         FROM tax_returns 
         WHERE client_id = ?"
    } else {
        "SELECT tax_return_id, client_id, tax_year, filing_status, income_sources, 
                deductions, credits, taxes_paid, tax_liability, refund_or_amount_due,
                created_at, updated_at 
         FROM tax_returns"
    };

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");

    let returns_iter = if let Some(cid) = client_id {
        stmt.query_map([cid], map_tax_return)
    } else {
        stmt.query_map([], map_tax_return)
    }.expect("Failed to execute query");

    let returns: Vec<TaxReturn> = returns_iter
        .filter_map(Result::ok)
        .collect();

    Json(returns)
}

#[get("/returns/<tax_return_id>")]
pub async fn get_return(state: &State<AppState>, tax_return_id: i64) -> Option<Json<TaxReturn>> {
    let db_lock = state.get_db().expect("Database connection should be available");
    let db = db_lock.as_ref().expect("Database should be initialized");
    let conn = db.conn.lock().expect("Failed to acquire database connection lock");

    let mut stmt = conn.prepare("
        SELECT tax_return_id, client_id, tax_year, filing_status, income_sources, 
               deductions, credits, taxes_paid, tax_liability, refund_or_amount_due,
               created_at, updated_at 
        FROM tax_returns 
        WHERE tax_return_id = ?
    ").expect("Failed to prepare statement");

    let tax_return = stmt.query_row([tax_return_id], |row| {
        Ok(TaxReturn {
            tax_return_id: Some(row.get(0)?),
            client_id: row.get(1)?,
            tax_year: row.get(2)?,
            filing_status: row.get(3)?, income_sources: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
            deductions: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
            credits: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
            taxes_paid: row.get(7)?,
            tax_liability: row.get(8)?,
            refund_or_amount_due: row.get(9)?,
            created_at: Some(row.get(10)?),
            updated_at: Some(row.get(11)?),
        })
    }).ok();

    tax_return.map(Json)
}

