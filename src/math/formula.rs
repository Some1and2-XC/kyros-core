#![allow(non_snake_case)]

use super::structs;

/*
# Purpose
This section of the code is for defining the different functions that are used
to generate images. This is the function that gets run on each pixel. 
# Definitions
SD
    SD stands for standard. This is the standard way of generating the mandelbrot set. 
*/

pub fn SD(c: structs::Complex, z: structs::Complex) -> structs::Complex {
    return z * z + c;
}


pub fn R(c: structs::Complex, z: structs::Complex) -> structs::Complex {
    let mut new_z = z * z + c;
    new_z.imaginary -= z.real;
    new_z.real -= z.imaginary;
    return new_z;
}

pub fn BS(c: structs::Complex, mut z: structs::Complex) -> structs::Complex {
    z = z * z;
    if z.imaginary > 0. {
        z.imaginary = z.imaginary * -1.;
    }
    return z + c;
}

pub fn SYM(c: structs::Complex, z: structs::Complex) -> structs::Complex {
    let mut new_z = z * z + c;
    new_z.imaginary -= z.imaginary;
    new_z.real -= z.real;
    return new_z;
}

