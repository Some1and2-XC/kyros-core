<h1>
    <img src="https://raw.githubusercontent.com/Some1and2-XC/Kyros-in-rust/8474631c3133c2e1b6317daa9db659940828b447/logo.svg" height="100" width="100">
     - A Fractal Generator
</h1>
## Key Features
 - Output in multiple formats (png file, output of base-64 encoded PNG, etc.)
 - Many different configuration options from generation formula, color and shadow setting and more.
 - Toggle enabling stdout progress reporting.
 - Comprehensive error messages & help menues.

## Examples (with outputs)
 - kyros.exe -y
    - Generates basic mandelbrot image with reasonable defaults.
 - kyros.exe -p 1024 -y
    - Generates basic mandelbrot image with higher resolution.
 - kyros.exe -i 1024 -y
    - Generates basic mandelbrot image with more iterations per pixel.
 - kyros.exe -f HELP -y
    - Shows help menu to display different options for the -f command.
 - kyros.exe -f R -y
    - Changes the formula to generate the image with.
 - kyros.exe --save-method HELP -y
    - Displays options for the --save-method flag.
 - kyros.exe --save-method B64 -y
    - Outputs a base64 encoded version of the image to stdout.
###### And there are many more different combinations of these flags to get unique outputs.

## Current Maximums
Part of this project was to see how large of an image I could generate.
The current largest image size (which is a power of 2) I could hit is a 131072x131072 png image
on both the SD & R generation method. Both of which are ~500MB in filesize. By using rust for this project
I could cut down the amount of memory requried (by its python counterpart by saving to a PNG) by around half
because of the way its libraries are designed (except for certain save methods.) as well as other performance
improvements. In addition I don't have to worry about dependencies, python versions to fit with dependencies and
the cost of a virtual environment. 

## Building to WASM (unavailable)
- wasm-pack build --target web
