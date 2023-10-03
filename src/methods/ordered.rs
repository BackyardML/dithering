use numpy::ndarray::ArrayD;
use numpy::{IxDyn, PyArrayDyn, PyReadonlyArrayDyn, ToPyArray};
use pyo3::prelude::*;
use pyo3::{PyResult, Python};
use std::fmt::{Debug, Display as FmtDisplay, Formatter, Result as FmtResult};
use std::str::FromStr;
use std::string::ToString;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

const CHANNELS_3: usize = 3;
const CHANNELS_2: usize = 2;

#[derive(Debug, Display, EnumIter)]
pub enum DitherMatrix {
    Bayer2x2,
    Bayer4x4,
    Bayer8x8,
}

impl FromStr for DitherMatrix {
    type Err = DitherMatrixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Bayer2x2" => Ok(Self::Bayer2x2),
            "Bayer4x4" => Ok(Self::Bayer4x4),
            "Bayer8x8" => Ok(Self::Bayer8x8),
            _ => Err(DitherMatrixError),
        }
    }
}

impl DitherMatrix {
    fn get_matrix(&self) -> Vec<Vec<u8>> {
        match self {
            DitherMatrix::Bayer2x2 => vec![vec![0, 127], vec![191, 63]],
            DitherMatrix::Bayer4x4 => vec![
                vec![0, 32, 8, 40],
                vec![48, 16, 56, 24],
                vec![12, 44, 4, 36],
                vec![60, 28, 52, 20],
            ],
            DitherMatrix::Bayer8x8 => vec![
                vec![0, 32, 8, 40, 2, 34, 10, 42],
                vec![48, 16, 56, 24, 50, 18, 58, 26],
                vec![12, 44, 4, 36, 14, 46, 6, 38],
                vec![60, 28, 52, 20, 62, 30, 54, 22],
                vec![3, 35, 11, 43, 1, 33, 9, 41],
                vec![51, 19, 59, 27, 49, 17, 57, 25],
                vec![15, 47, 7, 39, 13, 45, 5, 37],
                vec![63, 31, 55, 23, 61, 29, 53, 21],
            ],
        }
    }
}

#[derive(Debug)]
pub struct DitherMatrixError;

impl FmtDisplay for DitherMatrixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Invalid matrix. Available values: {}",
            DitherMatrix::iter()
                .map(|mat| format!("'{}'", mat))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl std::error::Error for DitherMatrixError {}

impl From<DitherMatrixError> for PyErr {
    fn from(err: DitherMatrixError) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(err.to_string())
    }
}

#[pyfunction]
pub fn ordered_dither<'py>(
    py: Python<'py>,
    image: PyReadonlyArrayDyn<u8>,
    matrix: String,
) -> PyResult<&'py PyArrayDyn<u8>> {
    let matrix = match DitherMatrix::from_str(&matrix) {
        Ok(matrix) => matrix,
        Err(e) => panic!("{}", e),
    };

    let matrix_values = matrix.get_matrix();
    let img_shape = image.shape();
    let mat_rows = matrix_values.len();
    let mat_cols = matrix_values[0].len();
    let array = image.as_array();
    let mut result = ArrayD::<u8>::zeros(IxDyn(image.shape()));

    match img_shape.len() {
        CHANNELS_3 => {
            (0..img_shape[0]).for_each(|y| {
                (0..img_shape[1]).for_each(|x| {
                    (0..img_shape[2]).for_each(|channel| {
                        let old_pixel = &array[[y, x, channel]];
                        let d_val = &matrix_values[y % mat_rows][x % mat_rows];
                        let new_pixel = if old_pixel < d_val { 0 } else { 255 };
                        result[[y, x, channel]] = new_pixel;
                    });
                });
            });
        }
        CHANNELS_2 => {
            (0..img_shape[0]).for_each(|y| {
                (0..img_shape[1]).for_each(|x| {
                    let old_pixel = &array[[y, x]];
                    let d_val = &matrix_values[y % mat_rows][x % mat_cols];
                    let new_pixel = if old_pixel < d_val { 0 } else { 255 };
                    result[[y, x]] = new_pixel;
                });
            });
        }
        _ => {
            panic!("Invalid image dimensions. Make sure it is 3 (rgb) or 2 (grayscale).")
        }
    };
    return Ok(result.to_pyarray(py));
}
