extern crate serde_json;
extern crate visdom;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap as Map;
use visdom::Vis;

pub type ResourceMap = Map<String, Vec<String>>;

pub fn get_old_data() -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file = File::open("old.json")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let v = serde_json::from_str::<Vec<String>>(&content)?;

    Ok(v)
}

pub fn get_new_data() -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file = File::open("dom.txt")?;
    let mut res = String::new();
    file.read_to_string(&mut res);

    // html dom
    let root = Vis::load(&res)?;
    let td_list = root.find("tbody td.entry");

    let mut data: Vec<String> = Vec::new();

    for td in td_list {
        let item = td.text();
        if let Some(i) = item.find("ALIYUN") {
            let item_arr: Vec<&str> = item.split("::").collect();
            let product = item_arr[1].to_lowercase();
            let resource_type = item_arr[2].to_lowercase();

            data.push(format!("{}::{}", product, resource_type));
        }
    }
    // println!("{:?}",data);
    Ok(data)
}


#[test]
fn get_data() {
    let resv = get_old_data();

    match resv {
        Ok(v) => println!("v---{:?}", v),
        Err(e) => println!("err---11{:?}", e),
    }

    assert_eq!(1,1);
}