-- Create clients table
CREATE TABLE IF NOT EXISTS clients (
    client_id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    social_security_number VARCHAR(11) NOT NULL,  -- Should be encrypted in production
    address VARCHAR(255) NOT NULL,
    phone_number VARCHAR(20) NOT NULL,
    email VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create tax returns table
CREATE TABLE IF NOT EXISTS tax_returns (
    tax_return_id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id INTEGER NOT NULL,
    tax_year INTEGER NOT NULL,
    filing_status VARCHAR(20) NOT NULL,
    income_sources TEXT NOT NULL,  -- Stored as JSON
    deductions TEXT NOT NULL,      -- Stored as JSON
    credits TEXT NOT NULL,         -- Stored as JSON
    taxes_paid DECIMAL(10,2) NOT NULL,
    tax_liability DECIMAL(10,2) NOT NULL,
    refund_or_amount_due DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES clients(client_id)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_clients_ssn ON clients(social_security_number);
CREATE INDEX IF NOT EXISTS idx_tax_returns_client_year ON tax_returns(client_id, tax_year);

-- Insert sample data
INSERT INTO clients (
    first_name, last_name, social_security_number, address, phone_number, email
) VALUES (
    'John', 'Doe', '123-45-6789', '123 Main St, Anytown, CA 12345', '(123) 456-7890', 'johndoe@email.com'
);

INSERT INTO tax_returns (
    client_id, tax_year, filing_status, income_sources, deductions, credits,
    taxes_paid, tax_liability, refund_or_amount_due
) VALUES (
    1, 2023, 'Single',
    '{"wages": 50000, "interest": 1000}',
    '{"standard_deduction": 13000}',
    '{"child_tax_credit": 2000}',
    10000.00, 8000.00, 2000.00
);
