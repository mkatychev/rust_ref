use std::error::Error;

fn return_dyn_error(error_return: &str) -> Result<(), Box<dyn Error>> {
    match error_return {
        "str" => return Err("str error".into()), // .into() converts the string slice to a box
        _ => return Ok(()),
    }
}

trait Errr {}

// fn return_impl_error(error_return: &str) -> Result<(), impl Errr> {
//     match error_return {
//         "str" => return Err(Box::new("str error")), // .into() converts the string slice to a box
//         _ => return Ok(()),
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    return_dyn_error("str")
}
