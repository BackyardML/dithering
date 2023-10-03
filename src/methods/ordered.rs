use pyo3::prelude::*;

#[pyfunction]
pub fn ordered_dither<'py>(
    _py: Python<'py>,
    image: Vec<Vec<u8>>,
    matrix: Vec<Vec<u8>>,
) -> Vec<Vec<u8>> {

    let img_height = image.len();
    let img_width = image[0].len();

    let dither_height = matrix.len();
    let dither_width = matrix[0].len();

    let mut result: Vec<Vec<u8>> = Vec::with_capacity(img_height);

    (0..img_height).for_each(|y| {
        let mut row: Vec<u8> = Vec::with_capacity(img_width);
        (0..img_width).for_each(|x| {
            let old_pixel = image[y][x]; // Fix the order of indices
            let d_val = matrix[y % dither_height][x % dither_width]; // Fix the order of indices
            row.push(if old_pixel < d_val { 0 } else { 255 });
        });
        result.push(row);
    });

    result
}

#[cfg(test)]
mod tests {
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
        let expected_result = vec![
            vec![255, 255, 255],
            vec![0, 0, 255],
            vec![255, 255, 0],
        ];

        // Call the function
        let result = ordered_dither(py, image, matrix);
        
        // Assert the result
        assert_eq!(result, expected_result);
        });
        
    }
}