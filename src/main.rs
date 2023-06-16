// Define unicode character gradient
//const LEVEL_CHARACTERS: &str = " ░▒▓█";
const LEVEL_CHARACTERS: &str = " ·›‹«»+×=¤zuw2a&4f¾O6TU9Sk§PZDF¥bÞd$8Gß€AÐXRKHENBWMØŒÆ";
const RESOLUTION: i32 = 256;
const ITERATIONS: i32 = 32;

fn main() {
    for y in 0..(RESOLUTION / 2) {
        // Calculate pixel y location based on y and resolution
        let oy = y as f64 / RESOLUTION as f64 * 6.0 - 1.5;
        for x in 0..RESOLUTION {
            // Calculate pixel x location based on x and resolution
            let ox = x as f64 / RESOLUTION as f64 * 3.0 - 2.0;
            // Create variables to iterate
            let mut ax = ox;
            let mut ay = oy;
            let mut i = ITERATIONS;
            // Iteration loop
            for z in 0..ITERATIONS {
                // Perform Mandelbrot set calculations
                let tax = (ax * ax) - (ay * ay) + ox as f64;
                ay = (2.0 * ax * ay) + oy as f64;
                ax = tax;
                // Check if current iteration is outside the set
                if (ax * ax + ay * ay) > 16.0 {
                    i = z;
                    break;
                }
            }
            // Display unicode character on screen
            print!("{}", LEVEL_CHARACTERS.chars().nth(((LEVEL_CHARACTERS.chars().count() - 1) as f64 / ITERATIONS as f64 * i as f64) as usize).unwrap());
        }
        // Line break after every row
        print!("\n");
    }
 }
