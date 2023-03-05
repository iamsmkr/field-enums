pub mod enums;

use pyo3::prelude::*;

use crate::enums::calculate_day;
use crate::enums::schedule;
use crate::enums::Weekday;

#[pymodule]
fn rust_field_enums_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Weekday>()?;
    m.add_function(wrap_pyfunction!(schedule, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_day, m)?)?;
    Ok(())
}
