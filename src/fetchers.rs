use error_chain::error_chain;
use std::fs::File;
use std::io::Cursor;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[allow(dead_code)]
pub async fn fetch_bytes(url: &str, file_name: &str) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

