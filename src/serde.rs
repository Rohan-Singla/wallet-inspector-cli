use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Dog {
    name: String,
    age: u32,
    owner : Owner
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    name : String,
    age : u32
}

// Convert Rust struct -> JSON string
pub fn convert_dog() -> String {

    let owner = Owner {
        name : "Rohan".to_string(),
        age : 18
    };

    let dog = Dog {
        name: "Tommy".to_string(),
        age: 2,
        owner : owner
    };

    let json = to_string_pretty(&dog).unwrap();

    return json;
}

// Convert JSON string -> Rust struct
pub fn back_to_dog(json: &str) -> Dog {

    let dog: Dog = from_str(json).unwrap();

    return dog;
}