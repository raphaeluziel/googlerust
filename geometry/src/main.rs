// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

fn magnitude(v: &[f64; 3]) -> f64 {
    let mut result = v[0].abs().powf(2f64);
    result += v[1].abs().powf(2f64);
    result += v[2].abs().powf(2f64);
    result = result.sqrt();
    result
}


// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

fn normalize(v: &[f64; 3]) {
    let mag = magnitude(v);
    
}

// Use the following `main` to test your work.

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    //normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}