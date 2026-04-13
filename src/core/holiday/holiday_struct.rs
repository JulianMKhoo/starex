use chrono::NaiveDate;

#[derive(Debug)]
pub struct Holiday {
    pub name: String,
    pub date: NaiveDate,
    pub icon: String,
}
