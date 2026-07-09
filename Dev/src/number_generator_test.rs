use rand::Rng;

// pub struct NumberGeneratorTest {
//     pub random_number: i32,
// }

// https://doc.rust-lang.org/std/keyword.impl.html
// I'm not exactly sure what impl does just yet.
// impl NumberGeneratorTest {
//     fn test() {
//         println!("NumberGeneratorTest test");
//     }
// //
// }

// I did not know I could just use a public function like this, at least until I figure out how impl works.
pub fn generate_random_number() -> i32 {
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    return secret_number;
    // println!("NumberGeneratorTest test");
}

