extern crate reqwest;
extern crate tokio;

use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://help.aliyun.com/document_detail/172061.html";
    let res = reqwest::get(url)
        .await?
        .text()
        .await?;

    // 获取的网页内容写入 dom.txt 文件
    let mut file = File::create("dom.txt")?;
    file.write(res.as_bytes())?;

    Ok(())
}