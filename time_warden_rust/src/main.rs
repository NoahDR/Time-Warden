pub mod db;
pub mod models;

use db::data_base_initializer::DatabaseInitializer;
use db::data_base_reader::DatabaseReader;

use std::error::Error;
use rusqlite::{Connection, Result};

fn main() -> Result<(), Box<dyn Error>> {
    let db_path = r"C:\Software Development\Projects\Time Warden";
    let connection = initialize_database(db_path)?;
    
    let erg = read_setting_setup_complete(&connection, 1);
    
    if erg{
        println!("App initialisiert.");
    }else{
        println!("App noch nicht initialisiert.");
    }
    Ok(())
}

pub fn initialize_database(db_path: &str) -> Result<Connection, Box<dyn Error>> {
    let connection: Connection;
    if !DatabaseInitializer::does_db_exist(db_path) {
        connection = DatabaseInitializer::create_db(db_path)?;
        println!("Datenbank erfolgreich erstellt.");
    } else {
        connection = Connection::open(db_path)?;
        println!("Die Datenbank existiert bereits.");
    }
    Ok(connection)
}

pub fn read_setting_setup_complete(connection: &Connection, user_id: i32) -> bool {
    DatabaseReader::read_setting_setup_complete(connection, user_id)
}

pub fn read_setting_work_day(connection: &Connection, user_id: i32) -> i32 {
    DatabaseReader::read_setting_work_day(connection, user_id)
}

pub fn read_setting_hours_week(connection: &Connection, user_id: i32) -> i32 {
    DatabaseReader::read_setting_hours_week(connection, user_id)
}

pub fn read_work_day(connection: &Connection, date: &str) -> i32 {
    let sum_minutes_worked = DatabaseReader::read_work_days_sum_minutes_worked(connection, date);
    let overtime_changes = DatabaseReader::read_work_days_sum_minutes_worked(connection, date);

    sum_minutes_worked + overtime_changes
}