# This script is just for simplifying testing an output image.
# This uses [pngcheck](http://www.libpng.org/pub/png/apps/pngcheck.html) to do so.
# This will run through the various chunks to and checksums to ensure the generated image is a proper image.
pngcheck -cvp7t out.png
