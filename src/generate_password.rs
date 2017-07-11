extern crate rand;

use self::rand::{Rng, thread_rng};


pub fn random_password( length:usize ) -> String {


    let mut password = String::new();
    let mut rng = thread_rng();


    for _ in 0..length {

        match rng.gen_range(0, 5) {

            // Generate special characters from '!' to '/'
            0 => password.push(rng.gen_range(33, 48) as u8 as char),

            // Generate Numbers from 0 to 9
            1 => password.push(rng.gen_range(48, 58) as u8 as char),

            // Generate special characters from ':' to '@'
            2 => password.push(rng.gen_range(58, 65) as u8 as char),

            // Generate uppercase letters from A to Z
            3 => password.push(rng.gen_range(65, 91) as u8 as char),

            // Generate lowercase letters from a to z
            4 => password.push(rng.gen_range(97, 123) as u8 as char),

            _ => println!("Unreachable state!"),
        }
    }


    password

}
