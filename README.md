<p align="center">
    <br>
    <img src="https://github.com/BackyardML/dithering/blob/main/resources/dithering.png?raw=true" width="600"/>
    <br>
<p>

![CI](https://github.com/BackyardML/dithering/actions/workflows/CI.yml/badge.svg)

## Overview
Welcome to Dithering, a project that brings efficient Rust implementation of various image dithering methods to be used in Python!

## Installation
```
pip install dithering
```

## Usage
```python
from PIL import Image
from dithering import ordered_dither
import numpy as np

input_image = np.array(Image.open('path/to/image.jpeg').convert('L'))

dither_matrix = np.array([[0, 128], [192, 64]], dtype=np.uint8)

output = ordered_dither(input_image, dither_matrix)


```
