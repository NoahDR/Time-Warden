use rusqlite::{params, Connection};

pub struct DatabaseReader;

impl DatabaseReader{
    pub fn read_setting_setup_complete(conn: &Connection, user_id: i32) -> bool {
        conn.query_row(
            "SELECT setup_complete FROM settings WHERE user_id = ?1",
            params![user_id],
            |row| row.get(0),
        ).unwrap_or(false)
    }
    
    pub fn read_setting_work_day(conn: &Connection, user_id: i32) -> i32 {
        conn.query_row(
            "SELECT work_day FROM settings WHERE user_id = ?1",
            params![user_id],
            |row| row.get(0),
        ).unwrap_or(-1)
    }
    
    pub fn read_setting_hours_week(conn: &Connection, user_id: i32) -> i32 {
        conn.query_row(
            "SELECT hours_week FROM settings WHERE user_id = ?1",
            params![user_id],
            |row| row.get(0),
        ).unwrap_or(-1)
    }

    pub fn read_work_days_sum_minutes_worked(conn: &Connection, date: &str) -> i32 {
        conn.query_row(
            "SELECT sum_minutes_worked FROM work_days WHERE date = 01-01-2222",
            params![date],
            |row| row.get(0),
        ).unwrap_or(-1)
    }

    pub fn read_work_days_overtime_change(conn: &Connection, date: &str) -> i32 {
        conn.query_row(
            "SELECT overtime_change FROM work_days WHERE date = ?1",
            params![date],
            |row| row.get(0),
        ).unwrap_or(-1)
    }
}