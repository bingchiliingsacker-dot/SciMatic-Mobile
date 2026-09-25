mod conversion;

use pyo3::prelude::*;

#[pymodule]
fn rs_converters(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(conversion::convert_binary, m)?)?;
    Ok(())
}
