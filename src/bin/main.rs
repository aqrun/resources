extern crate resources;
extern crate json;

use resources::{
    get_old_data,
    get_new_data,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let _v = get_old_data()?;
    get_new_data();
    Ok(())
}