mod methods;

use methods::ordered_dither;
use pyo3::prelude::*;

#[pymodule]
fn dithering(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ordered_dither, m)?)?;
    Ok(())
}
