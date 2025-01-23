use error_chain::error_chain;
use std::{future::Future, io::Read};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://httpbin.org/get").await?;
    println!("Satus:{}" , res.status());
    println!("Headers:\n{:#?}", res.headers());
    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}

// fn main () -> Result<()>  {
//     let mut res = reqwest::blocking::get("https://httpbin.org/get")?;
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;

//     println!("Status :{}", res.status());
//     println!("Header:\n{:#?}", res.headers());
//     println!("Body:\n{}" , body);
//     Ok(())
// }