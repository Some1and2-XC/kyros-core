#![allow(non_snake_case)]

use super::structs;

/*
# Definitions
SD
    SD stands for standard. This is the standard way of generating the mandelbrot set. 
*/

pub trait Generator {
    // The generator formula returns a floating point number to allow for TD values
    fn formula(&self, max_i: u64, C: structs::Complex, Z: structs::Complex) -> f64;
}

impl<F> Generator for F where
    F: Fn(u64, structs::Complex, structs::Complex) -> f64, {
    fn formula(&self, max_i: u64, C: structs::Complex, Z: structs::Complex) -> f64 {
        return self(max_i, C, Z);
    }
}

pub fn SD(max_i: u64, mut c: structs::Complex, mut z: structs::Complex) -> f64 {
    /*
       Function for getting the value of a particular point
       Returns the amount of iterations completed
       */

    // Sets Initial Value
    let c = z.clone();

    // Gets Mandelbrot Value
    for iteration in 0..max_i {
        if z.is_greater(2f64) {
            return iteration as f64;
        }
        z = z * z + c
    }
    return max_i as f64;
}
