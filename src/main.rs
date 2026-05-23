
use crate::serde::{back_to_dog, convert_dog};

mod serde;
mod http_reqwest;
mod clap;

fn main() {
    // Serialize
    let json = convert_dog();

    // println!("JSON: {}", json);

    // Deserialize
    let dog = back_to_dog(&json);

    // println!("Dog Struct: {:?}", dog);

    // let result = http_reqwest::send_request();

    clap::try_clap();

}