use chrono::prelude::*;
use pyo3::{
    prelude::*,
    types::{PyDate, PyDateAccess},
};

#[pyclass]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[pyfunction]
pub fn schedule(wd: &Weekday) {
    match wd {
        Weekday::Friday => println!("Finally Friyay!!"),
        Weekday::Saturday | Weekday::Sunday => println!("See friends or take rest"),
        _ => println!("Back to work!"),
    }
}

#[pyfunction]
pub fn calculate_day(date: &PyDate) -> Weekday {
    let dt = NaiveDate::from_ymd_opt(
        date.get_year(),
        date.get_month().into(),
        date.get_day().into(),
    )
    .unwrap();
    match dt.weekday() {
        chrono::Weekday::Mon => Weekday::Monday,
        chrono::Weekday::Tue => Weekday::Tuesday,
        chrono::Weekday::Wed => Weekday::Wednesday,
        chrono::Weekday::Thu => Weekday::Thursday,
        chrono::Weekday::Fri => Weekday::Friday,
        chrono::Weekday::Sat => Weekday::Saturday,
        chrono::Weekday::Sun => Weekday::Sunday,
    }
}
