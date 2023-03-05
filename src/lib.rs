pub mod enums;
pub mod field_enums;

use pyo3::prelude::*;

use crate::enums::calculate_day;
use crate::enums::schedule;
use crate::enums::Weekday;
use crate::field_enums::print_int;
use crate::field_enums::print_prop;
use crate::field_enums::print_str;
use crate::field_enums::Int;
use crate::field_enums::Str;

#[pymodule]
fn rust_field_enums_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Weekday>()?;
    m.add_class::<Str>()?;
    m.add_class::<Int>()?;
    m.add_function(wrap_pyfunction!(schedule, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_day, m)?)?;
    m.add_function(wrap_pyfunction!(print_prop, m)?)?;
    m.add_function(wrap_pyfunction!(print_str, m)?)?;
    m.add_function(wrap_pyfunction!(print_int, m)?)?;
    Ok(())
}
