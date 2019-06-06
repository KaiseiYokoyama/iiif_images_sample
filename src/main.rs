#![feature(custom_attribute)]
#[macro_use]
extern crate serde_derive;

mod iif_manifest;

fn main() -> reqwest::Result<()> {
    // 実行時引数
    let args: Vec<String> = std::env::args().collect();

    // 引数を指定されているとき
    if let Some(url) = args.get(1) {
        let resp = reqwest::get(url)?.text()?;
        let manifest: iif_manifest::Manifest = serde_json::from_str(&resp).unwrap();
        println!("{:?}", manifest.get_images());
    }

    Ok(())
}
