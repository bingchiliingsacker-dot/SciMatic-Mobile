use pyo3::prelude::*;

#[pymodule]
fn scimatic(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
			 // Wrapping flicker_module
    let flicker = PyModule::new(py, "flicker")?;
    flicker.add_function(
        wrap_pyfunction!(flicker_module::flick, &flicker)?
    )?;
    m.add_submodule(&flicker)?;
    
    // Wrapping rs_converters
    let conversion = PyModule::new(py, "conversion")?;
    conversion.add_function(
        wrap_pyfunction!(rs_converters::convert_binary, &conversion)?
    )?;
    m.add_submodule(&conversion)?;

    Ok(())
}