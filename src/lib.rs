use pyo3::prelude::*;

mod structures;
mod parser;
mod polars_convert;

pub use structures::*;
pub use parser::*;
pub use polars_convert::*;

/// Parse STDF file and return a Polars DataFrame
#[pyfunction]
fn parse_to_dataframe(path: String) -> PyResult<PyObject> {
    let records = parse_stdf_file(&path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Parse error: {}", e)))?;
    
    let df = records_to_dataframe(records)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Conversion error: {}", e)))?;
    
    Python::with_gil(|py| {
        Ok(df.to_object(py))
    })
}

/// Python module definition
#[pymodule]
fn stdf_wafer_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_to_dataframe, m)?)?;
    m.add_class::<WaferData>()?;
    Ok(())
}
