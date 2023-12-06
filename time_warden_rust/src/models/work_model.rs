use chrono;

pub struct Workday {
    pub date: chrono::NaiveDate,
    pub sum_hours_worked: chrono::Duration,
    pub overtime_change: chrono::Duration,
    pub sessions: Vec<Worksession>,
}

pub struct Worksession {
    pub id: i32,
    pub time_stamp_start: chrono::NaiveDateTime,
    pub time_stamp_end: chrono::NaiveDateTime,
}