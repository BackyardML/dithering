use pyo3::prelude::*;
use numpy::PyArray2;

#[pyfunction]
pub fn ordered_dither<'py>(
    py: Python<'py>,
    image: Vec<Vec<u8>>,
    matrix: Vec<Vec<u8>>,
) -> PyResult<&'py PyArray2<u8>> {

    let img_height = image.len();
    let img_width = image[0].len();

    let dither_height = matrix.len();
    let dither_width = matrix[0].len();

    let mut result: Vec<Vec<u8>> = Vec::with_capacity(img_height);

    (0..img_height).for_each(|y| {
        let mut row: Vec<u8> = Vec::with_capacity(img_width);
        (0..img_width).for_each(|x| {
            let old_pixel = image[y][x];
            let d_val = matrix[y % dither_height][x % dither_width];
            row.push(if old_pixel < d_val { 0 } else { 255 });
        });
        result.push(row);
    });
    let d = PyArray2::from_vec2(py, &result).unwrap();

    Ok(d)
}

#[cfg(test)]
mod tests {

    use numpy::array;

    use super::*;

    #[test]
    fn test_ordered_dither() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
        // Test input data
        let image = vec![
            vec![100, 200, 150],
            vec![50, 75, 125],
            vec![225, 175, 50],
        ];
        let matrix = vec![
            vec![100, 50],
            vec![75, 125],
        ];

        // Expected output
        let expected_result = array![
            [255, 255, 255],
            [0, 0, 255],
            [255, 255, 0],
        ];

        // Call the function
        let result = ordered_dither(py, image, matrix).unwrap();
        
        // Assert the result
        assert_eq!(&result.readonly().as_array(), expected_result);
        });
        
    }
}