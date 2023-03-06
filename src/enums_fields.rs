use pyo3::prelude::*;

#[derive(Debug)]
enum RProp {
    Str(String),
    Int(usize),
}

fn process_rprop(rprop: RProp) -> RProp {
    match rprop {
        RProp::Str(v) => RProp::Str(v),
        RProp::Int(v) => RProp::Int(v),
    }
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

    fn delegate_call(&self) {}
}

#[pyclass(extends=Prop, subclass)]
#[derive(Clone, Debug)]
pub struct Str {
    value: String,
}

#[pymethods]
impl Str {
    #[new]
    fn new(value: String) -> (Self, Prop) {
        (Str { value }, Prop::new())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Str({})", self.value))
    }

    fn delegate_call(&self, py: Python<'_>) -> PyResult<Py<Str>> {
        let s: Str = process_rprop(self.clone().into()).into();
        Py::new(py, (s, Prop::new()))
    }
}

impl From<Str> for RProp {
    fn from(v: Str) -> Self {
        RProp::Str(v.value)
    }
}

impl From<RProp> for Str {
    fn from(value: RProp) -> Self {
        match value {
            RProp::Str(value) => Str { value },
            RProp::Int(_) => panic!(),
        }
    }
}

#[pyclass(extends=Prop, subclass)]
#[derive(Clone, Debug)]
pub struct Int {
    value: usize,
}

#[pymethods]
impl Int {
    #[new]
    fn new(value: usize) -> (Self, Prop) {
        (Int { value }, Prop::new())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Int({})", self.value))
    }

    fn delegate_call(&self, py: Python<'_>) -> PyResult<Py<Int>> {
        let i: Int = process_rprop(self.clone().into()).into();
        Py::new(py, (i, Prop::new()))
    }
}

impl From<Int> for RProp {
    fn from(v: Int) -> Self {
        RProp::Int(v.value)
    }
}

impl From<RProp> for Int {
    fn from(value: RProp) -> Self {
        match value {
            RProp::Str(_) => panic!(),
            RProp::Int(value) => Int { value },
        }
    }
}

#[pyfunction]
pub fn process_prop(s: &PyCell<Prop>) -> PyResult<&PyAny> {
    s.call_method0("delegate_call")
}

#[pyfunction]
pub fn print_str(v: Str) {
    let s: Str = process_rprop(v.into()).into();
    println!("{:?}", s);
}

#[pyfunction]
pub fn print_int(v: Int) {
    let i: Int = process_rprop(v.into()).into();
    println!("{:?}", i);
}
