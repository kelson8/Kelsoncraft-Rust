use rand::RngExt;

/// Generate a random number
pub fn generate_random_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();

    rng.random_range(min..max)
}