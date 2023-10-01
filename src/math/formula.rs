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

/*
pub fn SD(max_i: u64, _c: structs::Complex, mut z: structs::Complex) -> f64 {
    let c: structs::Complex = z.clone();
    for iteration in 0..max_i {
        if z.is_greater(2.0) {
            return iteration as f64;
        }
        z = z * z + c;
    }
    return max_i as f64;
}

pub fn R(max_i: u64, c: structs::Complex, mut z: structs::Complex) -> f64 {
    let mut old_z: structs::Complex;
    for iteration in 0..max_i {    
        if z.is_greater(2f64) {
            return iteration as f64;
        }
        old_z = z.clone();
        z = z * z + c;
        z.imaginary -= old_z.real;
        z.real -= old_z.imaginary;
    }
    return max_i as f64;
}

pub fn BS(max_i: u64, c: structs::Complex, mut z: structs::Complex) -> f64 {
    let new_c: structs::Complex = z;
    z = c;
    for iteration in 0..max_i {
        if z.is_greater(2f64) {
            return iteration as f64;
        }
        z = z * z;
        if z.imaginary > 0. {
            z.imaginary = z.imaginary * -1.;
        }
        z = z + new_c;
    }
    return max_i as f64;
}

pub fn SYM(max_i: u64, c: structs::Complex, mut z: structs::Complex) -> f64 {
    let mut old_z: structs::Complex;
    for iteration in 0..max_i {    
        if z.is_greater(2f64) {
            return iteration as f64;
        }
        old_z = z.clone();
        z = z * z + c;
        z.imaginary -= old_z.imaginary;
        z.real -= old_z.real;
    }
    return max_i as f64;
}
*/
