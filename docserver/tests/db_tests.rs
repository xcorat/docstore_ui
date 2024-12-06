use docserver::{Database, Client, TaxReturn};
use std::collections::HashMap;
use tempfile::tempdir;

#[test]
fn test_database_workflow() {
    // Create a temporary database for testing
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("integration_test.db");
    let db = Database::new(db_path.to_str().unwrap()).unwrap();
    db.init().unwrap();

    // Test creating a client
    let client = Client {
        client_id: None,
        first_name: "Integration".to_string(),
        last_name: "Test".to_string(),
        social_security_number: "444-55-6666".to_string(),
        address: "1 Integration St".to_string(),
        phone_number: "(555) 123-4567".to_string(),
        email: "integration@test.com".to_string(),
        created_at: None,
        updated_at: None,
    };

    let client_id = db.create_client(&client).unwrap();

    // Test creating a tax return for the client
    let mut income_sources = HashMap::new();
    income_sources.insert("wages".to_string(), 75000.0);
    income_sources.insert("interest".to_string(), 1000.0);

    let mut deductions = HashMap::new();
    deductions.insert("standard_deduction".to_string(), 12950.0);
    deductions.insert("student_loan_interest".to_string(), 2500.0);

    let tax_return = TaxReturn {
        tax_return_id: None,
        client_id,
        tax_year: 2023,
        filing_status: "Single".to_string(),
        income_sources,
        deductions,
        credits: HashMap::new(),
        taxes_paid: 15000.0,
        tax_liability: 14000.0,
        refund_or_amount_due: 1000.0,
        created_at: None,
        updated_at: None,
    };

    let tax_return_id = db.create_tax_return(&tax_return).unwrap();

    // Test retrieving all tax returns for the client
    let client_returns = db.get_client_tax_returns(client_id).unwrap();
    assert_eq!(client_returns.len(), 1);
    
    let retrieved_return = &client_returns[0];
    assert_eq!(retrieved_return.tax_return_id, Some(tax_return_id));
    assert_eq!(retrieved_return.client_id, client_id);
    assert_eq!(retrieved_return.tax_year, 2023);
    assert_eq!(retrieved_return.income_sources.get("wages"), Some(&75000.0));
    assert_eq!(retrieved_return.income_sources.get("interest"), Some(&1000.0));
    assert_eq!(retrieved_return.deductions.get("standard_deduction"), Some(&12950.0));
}
