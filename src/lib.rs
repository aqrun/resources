extern crate serde_json;
extern crate visdom;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap as Map;
use visdom::Vis;

/// 读取本地 old.json 内容并生成 字符串数组
pub fn get_old_data() -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file = File::open("old.json")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // 使用 serde_json 将文本转为结构化数据
    let v = serde_json::from_str::<Vec<String>>(&content)?;

    Ok(v)
}

/// 从 HTML 内容获取指定内容生成 字符串数据
pub fn get_new_data() -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file = File::open("dom.txt")?;
    let mut res = String::new();
    file.read_to_string(&mut res);

    // HTML DOM 对象
    let root = Vis::load(&res)?;
    // 根据 css 选择器 获取指定元素
    let td_list = root.find("tbody td.entry");

    let mut data: Vec<String> = Vec::new();

    for td in td_list {
        // 获取当前元素的文本内容
        let item = td.text();
        // 根据 字符串 前缀筛选
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

