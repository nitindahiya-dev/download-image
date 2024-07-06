use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn  main() -> Result<()> {
    let temp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("temp.bin");

            println!("File name: '{}'", fname);

            let fname = temp_dir.path().join(fname);
    
            println!("File location: '{}'", fname.to_string_lossy());
            File::create(fname)?
        };
        let content = response.bytes().await?;
        copy(&mut content.as_ref(), &mut dest)?;
    
        Ok(())
    }