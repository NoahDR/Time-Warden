use rusqlite::{Connection, Result, Transaction};
use std::path::{Path, PathBuf};

pub struct DatabaseInitializer;

impl DatabaseInitializer {
    pub fn does_db_exist(documents_path: &str) -> bool {
        DatabaseInitializer::get_database_path(documents_path).exists()
    }

    pub fn create_db(documents_path: &str) -> Result<Connection> {
        let db_path = DatabaseInitializer::get_database_path(documents_path);
        let mut connection = Connection::open(db_path)?;
        let mut transaction = connection.transaction()?;
        
        DatabaseInitializer::create_table(&mut transaction, "settings", SETTINGS_TABLE_SCHEMA)?;
        DatabaseInitializer::create_table(&mut transaction, "work_days", WORK_DAYS_TABLE_SCHEMA)?;
        DatabaseInitializer::create_table(&mut transaction, "work_sessions", WORK_SESSIONS_TABLE_SCHEMA)?;

        transaction.commit()?;

        Ok(connection)
    }
    
    fn get_database_path(documents_path: &str) -> PathBuf {
        Path::new(documents_path).join(DATABASE_NAME)
    }
    
    fn create_table(transaction: &mut Transaction, table_name: &str, schema: &str) -> Result<()> {
        transaction.execute(&format!("CREATE TABLE IF NOT EXISTS {} {}", table_name, schema), [])?;
        Ok(())
    }
}

pub const DATABASE_NAME: &str = "time_data.db";

const SETTINGS_TABLE_SCHEMA: &str = "
    (
        user_id INTEGER PRIMARY KEY,
        setup_complete BOOLEAN NOT NULL,
        work_day INTEGER NOT NULL,
        hours_week INTEGER NOT NULL
    )";

const WORK_DAYS_TABLE_SCHEMA: &str = "
    (
        date TEXT PRIMARY KEY,
        sum_minutes_worked INTEGER NOT NULL,
        overtime_change INTEGER NOT NULL
    )";

const WORK_SESSIONS_TABLE_SCHEMA: &str = "
    (
        id INTEGER PRIMARY KEY,
        date TEXT NOT NULL,
        time_stamp_start TEXT NOT NULL,
        time_stamp_end TEXT NOT NULL,
        FOREIGN KEY (date) REFERENCES work_days(date)
    )";
