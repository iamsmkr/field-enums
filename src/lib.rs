pub mod enums;
pub mod enums_fields;

use pyo3::prelude::*;

use crate::enums::calculate_day;
use crate::enums::schedule;
use crate::enums::Weekday;
use crate::enums_fields::print_int;
use crate::enums_fields::print_str;
use crate::enums_fields::process_prop;
use crate::enums_fields::Int;
use crate::enums_fields::Str;

#[pymodule]
fn rust_field_enums_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Weekday>()?;
    m.add_class::<Str>()?;
    m.add_class::<Int>()?;
    m.add_function(wrap_pyfunction!(schedule, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_day, m)?)?;
    m.add_function(wrap_pyfunction!(process_prop, m)?)?;
    m.add_function(wrap_pyfunction!(print_str, m)?)?;
    m.add_function(wrap_pyfunction!(print_int, m)?)?;
    Ok(())
}
