extern crate resources;
extern crate visdom;

use std::fs::File;
use visdom::Vis;
use std::io::Read;
use resources::{get_old_data, get_new_data};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let old_data = get_old_data()?;
    let new_data = get_new_data()?;

    let mut data_exists: Vec<String> = Vec::new();
    let mut data: Vec<String> = Vec::new();

    for item in new_data {
        if !old_data.contains(&item) {
            data.push(item);
        } else {
            data_exists.push(item);
        }
    }

    println!("已存在数据: {:?}", data_exists);
    println!("新增数据: {:?}", data);
    Ok(())
}
