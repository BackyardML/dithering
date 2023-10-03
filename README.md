<p align="center">
    <br>
    <img src="https://raw.githubusercontent.com/BackyardML/dithering/72ea732b8c7372af6f427a5f8244af158d80d208/resources/dithering.svg" width="600"/>
    <br>
<p>

![CI](https://github.com/BackyardML/dithering/actions/workflows/CI.yml/badge.svg)

Dithering is an open-source image processing library written in Rust, designed to perform image dithering operations. It provides a collection of algorithms that can be used to convert images to a limited color palette, resulting in the illusion of additional colors. This library is licensed under the MIT license.

## Features

- Perform image dithering operations to reduce the number of colors in an image.
- Support for ordered dithering algorithm.
- Convert images to a limited color palette.
- Integration with Python through Rust's Foreign Function Interface (FFI).

## Installation
To install the library using pip, run the following command:
```commandLine
pip install dithering
```
## Usage

Here's an example usage of the Dithering library in Python:

```python
from PIL import Image
from dithering import ordered_dither
import numpy as np

input_image = np.array(Image.open('path/to/image.jpeg'))

output_image = ordered_dither(input_image, "Bayer2x2")
```

In the example above, we first import the necessary modules. Then, we load an image using the PIL library and convert it to a NumPy array. We pick a dither matrix ("`Dither2x2`") and pass it along with the input image to the `ordered_dither` function from the `dithering` module. The function returns the dithered output image as a NumPy array.

## Contributing

Contributions to the Dithering library are welcome! If you want to contribute, please follow these steps:

1. Fork the repository on GitHub.
2. Create a new branch from the `main` branch.
3. Make your changes and ensure the tests pass.
4. Add/update tests if applicable.
5. Commit your changes and push them to your forked repository.
6. Submit a pull request to the `main` branch of the original repository.

Please make sure to follow the code style and conventions used in the project.

## License

The Dithering library is licensed under the MIT license. See the [LICENSE](https://github.com/BackyardML/dithering/blob/main/LICENSE) file for more details.

## Contact

If you have any questions, suggestions, or feedback regarding the Dithering library, you can reach out to us by creating an issue on the GitHub repository or contacting us via email at [info@backyardml.se](mailto:info@backyardml.se).

Feel free to explore the repository, submit bug reports, or request new features. We appreciate your interest and support in advancing the Dithering library.