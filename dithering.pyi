import numpy as np
from numpy.typing import ArrayLike
from typing import Union

def ordered_dither(
    image: Union[ArrayLike, np.ndarray],
    matrix: str
) -> np.ndarray:
    """
    Applies ordered dithering to the input image using the specified dither matrix.

    Parameters:
        image (Union[ArrayLike, np.ndarray]): The input image to be dithered.
        matrix (str): The name of the dither matrix to be used (e.g., 'Bayer2x2', 'Bayer4x4', 'Bayer8x8').

    Returns:
        np.ndarray: The dithered image as a NumPy array.
    """
    pass
