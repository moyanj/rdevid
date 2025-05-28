use pyo3::prelude::*;

#[pyfunction]
fn device_id() -> PyResult<String> {
    Ok(rdevid::device_id())
}

#[pyfunction]
fn device_info() -> PyResult<String> {
    Ok(rdevid::device_info())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "rdevid")]
fn binding(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(device_id, m)?)?;
    m.add_function(wrap_pyfunction!(device_info, m)?)?;
    Ok(())
}
