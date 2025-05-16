// src/database/schema.rs

use rusqlite::{Connection, Result as RusqliteResult};

/// Initializes the database schema.
pub fn initialize_database(conn: &mut Connection) -> RusqliteResult<()> {
    println!("Initializing database schema...");

    let tx = conn.transaction()?;

    // Salespeople table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS salespeople (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            employee_id TEXT,
            email TEXT,
            is_active INTEGER DEFAULT 1 NOT NULL
        )",
        [],
    )?;
    println!("✅ Table 'salespeople' checked/created.");

    // Products table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sku TEXT UNIQUE,
            name TEXT NOT NULL,
            brand TEXT NOT NULL,
            category TEXT,
            cost_price REAL NOT NULL,
            unit_price REAL NOT NULL,
            description TEXT
        )",
        [],
    )?;
    println!("✅ Table 'products' checked/created.");

    // Sales entry table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS salesentry (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            salesperson_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            product_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            unit_type TEXT NOT NULL,
            FOREIGN KEY (salesperson_id) REFERENCES salespeople(id),
            FOREIGN KEY (product_id) REFERENCES products(id)
        )",
        [],
    )?;
    println!("✅ Table 'salesentry' checked/created.");

    // Target campaigns table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS target_campaigns (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            brand TEXT NOT NULL,
            start_date TEXT NOT NULL,
            end_date TEXT NOT NULL,
            is_active INTEGER DEFAULT 1 NOT NULL
        )",
        [],
    )?;
    println!("✅ Table 'target_campaigns' checked/created.");

    // Target campaign products table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS target_campaign_products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            campaign_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            FOREIGN KEY (campaign_id) REFERENCES target_campaigns(id),
            FOREIGN KEY (product_id) REFERENCES products(id)
        )",
        [],
    )?;
    println!("✅ Table 'target_campaign_products' checked/created.");

    // Target allocations table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS target_allocations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            campaign_id INTEGER NOT NULL,
            salesperson_id INTEGER NOT NULL,
            target_quantity INTEGER NOT NULL,
            base_reward REAL NOT NULL DEFAULT 0,
            FOREIGN KEY (campaign_id) REFERENCES target_campaigns(id),
            FOREIGN KEY (salesperson_id) REFERENCES salespeople(id)
        )",
        [],
    )?;
    println!("✅ Table 'target_allocations' checked/created.");

    // Target tiers table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS target_tiers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            campaign_id INTEGER NOT NULL,
            min_quantity INTEGER NOT NULL,
            multiplier REAL NOT NULL,
            reward_per_unit REAL NOT NULL,
            notes TEXT,
            FOREIGN KEY (campaign_id) REFERENCES target_campaigns(id)
        )",
        [],
    )?;
    println!("✅ Table 'target_tiers' checked/created.");

    tx.commit()?;
    println!("✅ Database schema initialization complete.");

    Ok(())
}
