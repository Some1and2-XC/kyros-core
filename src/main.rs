#![allow(non_snake_case)]

use std::{fs};

fn CreateArr3D(SizeX: u32, SizeY: u32) -> Vec<Vec<Vec<f64>>> {
    // Creates 3D Array of size "SizeX" & "SizeY"
    // Full of Mathematical Values
    
    fn GetMathVal(Value: u32, MaxRef: u32) -> f64 {
        // Function for getting a mathematical space
        // Value from "Value"
        4f64 * (Value as f64) / (MaxRef as f64 - 1f64) - 2f64
    }    

    // Sets Up Itterator
    (0..SizeY)
        // Maps all the SizeY Values to SizeX Vectors of Imaginary & Real Values
        .map(|y| (0..SizeY)
            // Maps All the X Values to Imaginary and Real Values
            .map(|x| vec![GetMathVal(x, SizeX), GetMathVal(y, SizeY)])
            .collect::<Vec<Vec<f64>>>()
        ).collect::<Vec<Vec<Vec<f64>>>>()

}

fn CreateArr2D(SizeX: u32, SizeY: u32) -> Vec<Vec<u32>>{
    // Creates 2D Array of size "SizeX" & "SizeY"
    // Full of 0's

    // Sets Up Itterator
    (0..SizeY)
    // Maps all the SizeY Values to SizeX Vectors
        .map(|_y|
            // Maps All the X Values to 0's
            (0..SizeX)
            .map(|_x| 0)
            .collect::<Vec<u32>>()
        ).collect::<Vec<Vec<u32>>>()
}

fn FindMandelbrotValues(Values: Vec<Vec<Vec<f64>>>, MaxI: u32) -> Vec<Vec<u32>> {
    // Function for getting the mandelbrot values from vectors
    
    let mut OutValues = CreateArr2D(Values.len() as u32, Values[0].len() as u32);
    let mut ci: f64;
    let mut cj: f64;
    let mut zi: f64;
    let mut zj: f64;

    let ImageY = Values.len();

    for y in 0..Values.len() {
        for x in 0..Values[0].len() {

            // Splits Values into real and imaginary parts
            (cj, ci) = (Values[y][x][0], Values[y][x][1]);
            (zj, zi) = (cj, ci);

            for _Itter in 0..MaxI {
                
                let distance = f64::powf(zi, 2.) * f64::powf(zj, 2.);

                if distance > 4f64 {
                    break
                }

                OutValues[y][x] += 1;

                let New_zi = 2. * zi * zj + ci;
                zj = f64::powf(zj, 2.) - f64::powf(zi, 2.) + cj;
                zi = New_zi;

            }
        }
        print!("\t{} / {} | {}%\r", y + 1, ImageY, 100. * ((y + 1) as f64 / ImageY as f64) );
    }

    println!();
    
    OutValues
}

fn GenerateMandelbrot(SizeX: u32, SizeY: u32, MaxI: u32) -> Vec<Vec<u32>> {
    // Function for Generating the Mandelbrot Set

    println!("Creating Array");
    let Values = CreateArr3D(SizeX, SizeY);

    println!("Generating Image");
    FindMandelbrotValues(Values, MaxI)
}



fn main() {

    // Sets the dimensions of the Image
    let SizeX = 84;
    let SizeY = 84;

    // Setsup answer string
    let mut v = String::new();

    println!("Are you sure you want to continue with the size {} x {} [ y | n ]?", SizeX, SizeY);
    let _ = std::io::stdin().read_line(&mut v).unwrap();
    assert_eq!(v.to_lowercase().chars().nth(0).unwrap(), 'y');

    println!("Started!");

    // Generates Mandelbrot shape in type Vec<Vec<u32>>
    let Image = GenerateMandelbrot(SizeX, SizeY, 1000);
    
    // Uses the format!() macro
    println!("Converting Image to String");
    let ImageString: String = format!("{:?}", Image);

    // Writes to file
    println!("Writing to File");
    let _ = fs::write("Outvalues.kyros", &ImageString);

    println!("FINISHED!");
    let _ = std::io::stdin().read_line(&mut String::new());
}

