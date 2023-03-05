use pyo3::prelude::*;

enum RProp {
    Str(String),
    Int(i64),
}

#[pyclass(subclass)]
#[derive(Clone)]
pub struct Prop;

#[pymethods]
impl Prop {
    #[new]
    fn new() -> Self {
        Prop
    }

    fn convert(&self) {}
}

#[pyclass(extends=Prop, subclass)]
#[derive(Clone)]
pub struct Str {
    value: String,
}

#[pymethods]
impl Str {
    #[new]
    fn new(value: String) -> (Self, Prop) {
        (Str { value }, Prop::new())
    }

    fn convert(&self) {
        println!("value = {}", self.value)
    }
}

#[pyclass(extends=Prop, subclass)]
#[derive(Clone)]
pub struct Int {
    value: usize,
}

#[pymethods]
impl Int {
    #[new]
    fn new(value: usize) -> (Self, Prop) {
        (Int { value }, Prop::new())
    }

    fn convert(&self) {
        println!("value = {}", self.value)
    }
}

#[pyfunction]
pub fn print_prop(s: &PyCell<Prop>) -> PyResult<&PyAny> {
    s.call_method0("convert")
}

#[pyfunction]
pub fn print_str(v: Str) {
    println!("{}", v.value)
}

#[pyfunction]
pub fn print_int(v: Int) {
    println!("{}", v.value)
}
