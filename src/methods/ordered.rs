use numpy::{PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;

#[pyfunction]
pub fn ordered_dither<'py>(
    py: Python<'py>,
    image: PyReadonlyArray2<'py, u8>,
    matrix: PyReadonlyArray2<'py, u8>,
) -> PyResult<&'py PyArray2<u8>> {
    let shape: &[usize] = image.shape();
    let dims = matrix.dims();
    let dither_rows = dims[0];
    let dither_cols = dims[1];

    let mut result: Vec<Vec<u8>> = Vec::with_capacity(shape[0]);

    for x in 0..shape[0] {
        let mut row: Vec<u8> = Vec::with_capacity(shape[1]);
        for y in 0..shape[1] {
            let old_pixel = image.get([x, y]);
            let d_val = matrix.get([x % dither_rows, y % dither_cols]);
            let new_pixel = if old_pixel < d_val { 0 } else { 255 };
            row.push(new_pixel);
        }
        result.push(row);
    }
    let d = PyArray2::from_vec2(py, &result).unwrap();
    Ok(d)
}
