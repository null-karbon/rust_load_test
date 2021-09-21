extern crate reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //env_logger::init();

    println!("GET https://www.rust-lang.org");

    let res = reqwest::blocking::get("https://www.rust-lang.org/")?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());

    // copy the response body directly to stdout
    //res.copy_to(&mut std::io::stdout())?;

    println!("\n\nDone.");
    Ok(())
}