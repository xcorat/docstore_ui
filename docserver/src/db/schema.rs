use rusqlite::{Connection, Result, params, OptionalExtension};
use serde_json;
use chrono::Utc;

use super::models::{Client, TaxReturn};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn init(&self) -> Result<()> {
        let schema = include_str!("schema.sql");
        self.conn.execute_batch(schema)?;
        Ok(())
    }

    // Client operations
    pub fn create_client(&self, client: &Client) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO clients (
                first_name, last_name, social_security_number,
                address, phone_number, email
            ) VALUES (?, ?, ?, ?, ?, ?)"
        )?;

        stmt.execute(params![
            client.first_name,
            client.last_name,
            client.social_security_number,
            client.address,
            client.phone_number,
            client.email,
        ])?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_client(&self, client_id: i64) -> Result<Option<Client>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM clients WHERE client_id = ?"
        )?;

        let client = stmt.query_row([client_id], |row| {
            Ok(Client {
                client_id: Some(row.get(0)?),
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                social_security_number: row.get(3)?,
                address: row.get(4)?,
                phone_number: row.get(5)?,
                email: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        }).optional()?;

        Ok(client)
    }

    // Tax Return operations
    pub fn create_tax_return(&self, tax_return: &TaxReturn) -> Result<i64> {
        let income_sources_json = serde_json::to_string(&tax_return.income_sources)
            .map_err(|_| rusqlite::Error::InvalidParameterName("Serialization error".to_string()))?;
        let deductions_json = serde_json::to_string(&tax_return.deductions)
            .map_err(|_| rusqlite::Error::InvalidParameterName("Serialization error".to_string()))?;
        let credits_json = serde_json::to_string(&tax_return.credits)
            .map_err(|_| rusqlite::Error::InvalidParameterName("Serialization error".to_string()))?;

        let mut stmt = self.conn.prepare(
            "INSERT INTO tax_returns (
                client_id, tax_year, filing_status, income_sources,
                deductions, credits, taxes_paid, tax_liability,
                refund_or_amount_due
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )?;

        stmt.execute(params![
            tax_return.client_id,
            tax_return.tax_year,
            tax_return.filing_status,
            income_sources_json,
            deductions_json,
            credits_json,
            tax_return.taxes_paid,
            tax_return.tax_liability,
            tax_return.refund_or_amount_due,
        ])?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_tax_return(&self, tax_return_id: i64) -> Result<Option<TaxReturn>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM tax_returns WHERE tax_return_id = ?"
        )?;

        let tax_return = stmt.query_row([tax_return_id], |row| {
            Ok(TaxReturn {
                tax_return_id: Some(row.get(0)?),
                client_id: row.get(1)?,
                tax_year: row.get(2)?,
                filing_status: row.get(3)?,
                income_sources: serde_json::from_str(&row.get::<_, String>(4)?).unwrap(),
                deductions: serde_json::from_str(&row.get::<_, String>(5)?).unwrap(),
                credits: serde_json::from_str(&row.get::<_, String>(6)?).unwrap(),
                taxes_paid: row.get(7)?,
                tax_liability: row.get(8)?,
                refund_or_amount_due: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        }).optional()?;

        Ok(tax_return)
    }

    pub fn get_client_tax_returns(&self, client_id: i64) -> Result<Vec<TaxReturn>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM tax_returns WHERE client_id = ? ORDER BY tax_year DESC"
        )?;

        let tax_returns = stmt.query_map([client_id], |row| {
            Ok(TaxReturn {
                tax_return_id: Some(row.get(0)?),
                client_id: row.get(1)?,
                tax_year: row.get(2)?,
                filing_status: row.get(3)?,
                income_sources: serde_json::from_str(&row.get::<_, String>(4)?).unwrap(),
                deductions: serde_json::from_str(&row.get::<_, String>(5)?).unwrap(),
                credits: serde_json::from_str(&row.get::<_, String>(6)?).unwrap(),
                taxes_paid: row.get(7)?,
                tax_liability: row.get(8)?,
                refund_or_amount_due: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })?.collect::<Result<Vec<_>>>()?;

        Ok(tax_returns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::collections::HashMap;

    fn create_test_db() -> (Database, tempfile::TempDir) {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(db_path.to_str().unwrap()).unwrap();
        db.init().unwrap();
        (db, temp_dir)
    }

    #[test]
    fn test_create_and_get_client() {
        let (db, _temp) = create_test_db();
        
        let client = Client {
            client_id: None,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            social_security_number: "123-45-6789".to_string(),
            address: "123 Test St".to_string(),
            phone_number: "(555) 555-5555".to_string(),
            email: "test@example.com".to_string(),
            created_at: None,
            updated_at: None,
        };

        let client_id = db.create_client(&client).unwrap();
        assert!(client_id > 0);

        let retrieved_client = db.get_client(client_id).unwrap().unwrap();
        assert_eq!(retrieved_client.first_name, client.first_name);
        assert_eq!(retrieved_client.last_name, client.last_name);
        assert_eq!(retrieved_client.email, client.email);
    }

    #[test]
    fn test_create_and_get_tax_return() {
        let (db, _temp) = create_test_db();
        
        // First create a client
        let client = Client {
            client_id: None,
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            social_security_number: "987-65-4321".to_string(),
            address: "456 Test Ave".to_string(),
            phone_number: "(555) 555-1234".to_string(),
            email: "jane@example.com".to_string(),
            created_at: None,
            updated_at: None,
        };

        let client_id = db.create_client(&client).unwrap();

        // Create a tax return
        let mut income_sources = HashMap::new();
        income_sources.insert("wages".to_string(), 50000.0);
        
        let mut deductions = HashMap::new();
        deductions.insert("standard_deduction".to_string(), 12950.0);
        
        let mut credits = HashMap::new();
        credits.insert("child_tax_credit".to_string(), 2000.0);

        let tax_return = TaxReturn {
            tax_return_id: None,
            client_id,
            tax_year: 2023,
            filing_status: "Single".to_string(),
            income_sources,
            deductions,
            credits,
            taxes_paid: 8000.0,
            tax_liability: 7000.0,
            refund_or_amount_due: 1000.0,
            created_at: None,
            updated_at: None,
        };

        let tax_return_id = db.create_tax_return(&tax_return).unwrap();
        assert!(tax_return_id > 0);

        let retrieved_return = db.get_tax_return(tax_return_id).unwrap().unwrap();
        assert_eq!(retrieved_return.client_id, client_id);
        assert_eq!(retrieved_return.tax_year, 2023);
        assert_eq!(retrieved_return.filing_status, "Single");
        assert_eq!(retrieved_return.income_sources.get("wages"), Some(&50000.0));
    }

    #[test]
    fn test_get_client_tax_returns() {
        let (db, _temp) = create_test_db();
        
        // Create a client
        let client = Client {
            client_id: None,
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            social_security_number: "111-22-3333".to_string(),
            address: "789 Test Rd".to_string(),
            phone_number: "(555) 555-9999".to_string(),
            email: "test.user@example.com".to_string(),
            created_at: None,
            updated_at: None,
        };

        let client_id = db.create_client(&client).unwrap();

        // Create multiple tax returns for different years
        for year in [2021, 2022, 2023] {
            let mut income_sources = HashMap::new();
            income_sources.insert("wages".to_string(), 50000.0 * (year - 2020) as f64);
            
            let tax_return = TaxReturn {
                tax_return_id: None,
                client_id,
                tax_year: year,
                filing_status: "Single".to_string(),
                income_sources,
                deductions: HashMap::new(),
                credits: HashMap::new(),
                taxes_paid: 5000.0,
                tax_liability: 4500.0,
                refund_or_amount_due: 500.0,
                created_at: None,
                updated_at: None,
            };

            db.create_tax_return(&tax_return).unwrap();
        }

        let returns = db.get_client_tax_returns(client_id).unwrap();
        assert_eq!(returns.len(), 3);
        assert_eq!(returns[0].tax_year, 2023); // Should be ordered by year DESC
        assert_eq!(returns[1].tax_year, 2022);
        assert_eq!(returns[2].tax_year, 2021);
    }
}
