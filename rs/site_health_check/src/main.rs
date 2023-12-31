use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://leighzer.com";
    let resp = reqwest::get(url).await?;
    
    println!("{}", resp.status());
    
    Ok(())
}