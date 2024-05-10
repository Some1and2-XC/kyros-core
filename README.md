<h1>
    <img src="https://raw.githubusercontent.com/Some1and2-XC/Kyros-in-rust/8474631c3133c2e1b6317daa9db659940828b447/logo.svg" height="200" width="200">
    - A Fractal Generator
</h1>

## Key Features
 - Output in multiple formats (png file, output of base-64 encoded PNG, etc.)
 - Many different configuration options from generation formula, color and shadow setting and more.
 - Comprehensive error messages & help menus.
 - Configuration for background & foreground color.

## Color Settings
Kyros uses [csscolorparser](https://docs.rs/csscolorparser/latest/csscolorparser/), a rust parser library for css colors. This means that any color parameter can be set the same way it would be in css.
Ex:
```
kyros.exe --background transparent --foreground "rgba(255, 0, 0, 1)" --rgba -y
```

## GPU
Kyros implements most of its arguments to work both on the CPU and GPU. The GPU acceleration is done with the Vulkan rust API called [Vulkano](https://vulkano.rs/) as well as using the [shaderc-rs](https://github.com/google/shaderc-rs) library for run time SPIR-V compilation. Because of the overhead of this implementation, generally images under 1000px x 1000px will be faster on CPU however this will vary by system.
To use the GPU implementation, the vulkan library must be installed first.
### Limitations and Implementation details
 - Because the minimum byte width of the GPU array is 4, the GPU always uses RGBA color even if this isn't specified in the CLI arguments.
 - The size of the image depends on the amount of memory available in GPU. Because of this, run-time errors can occur at higher resolutions (around 25kpx x 25kpx for a NVIDIA GeForce GTX 1060 6GB GPU.)
### Performance
 - At higher resolutions, the bottleneck for performance becomes saving images instead of generation. Using the GTX 1060 a 20kpx x 20kpx image can be generated in under 2s however saving that image can take up to 5 seconds (depending of course on hardware.)
 - Without GPU acceleration and with identical settings (except GPU flag) the same 20xpx x 20xpx image mentioned above (which took 6.61 seconds) takes 280.30s for generation meaning there is a 42x increase with the GPU flag.
 - This performance improvement is even more dramatic when higher max iterations are set and there are more pixels with higher iterations.

## Examples (with outputs)
 - `kyros.exe -y`
    - Generates basic mandelbrot image with reasonable defaults.
 - `kyros.exe -p 1024 -y`
    - Generates basic mandelbrot image with higher resolution.
 - `kyros.exe -i 1024 -y`
    - Generates basic mandelbrot image with more iterations per pixel.
 - `kyros.exe -f HELP -y`
    - Shows help menu to display different options for the -f command.
 - `kyros.exe -f R -y`
    - Changes the formula to generate the image with.
 - `kyros.exe --save-method HELP -y`
    - Displays options for the --save-method flag.
 - `kyros.exe --save-method B64 -y`
    - Outputs a base64 encoded version of the image to stdout.
 - `kyros.exe --logs INFO -y`
    - Shows progress and other generation information.
 - `kyros.exe --gpu -p 20000 --background transparent -y`
    - Generates a 20000px x 20000px image using the GPU.

<p>And there are many more different combinations of these flags to get unique outputs.</p>

## Current Maximums
Part of this project was to see how large of an image I could generate.
The current largest image size (which is a power of 2) I could hit is a 131072x131072 png image
on both the SD & R generation method. Both of which are ~500MB in filesize. By using rust for this project
I could cut down the amount of memory requried (by its python counterpart by saving to a PNG) by around half
because of the way its libraries are designed (except for certain save methods.) as well as other performance
improvements. In addition I don't have to worry about dependencies, python versions to fit with dependencies and
the cost of a virtual environment. 
