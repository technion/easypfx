use openssl::pkcs12::Pkcs12;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;
use std::ffi::OsStr;
use anyhow::{Result, anyhow};

const EASYPFX_VERSION: &str = "v1.0";

fn validate_filename(filename: &str) -> Result<()>{
    let path = match Path::new(filename).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            return Err(anyhow!(e.to_string()));
        }
    };

    match path.extension().and_then(OsStr::to_str) {
        None =>  Err(anyhow!("Missing extension - must be a pfx file")),
        Some("pfx") =>  Ok(()),
        _ => Err(anyhow!("Invalid extension - must be a pfx file"))

    }
}

fn get_pfx_contents(filename: &str) -> Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut content = vec![];
    file.read_to_end(&mut content)?;
    Ok(content)

}

fn main() {

    println!("EasyPFX: https://github.com/technion/easypfx");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: easypfx.exe <filename.pfx>");
        return;
    }
    if args[1] == "--version" {
        println!(
            "EasyPFX Version: {}\n",
            EASYPFX_VERSION
        );
        return;
    }

    if let Err(x) =  validate_filename(&args[1]) {
            println!("Filename error: {}", x);
            return;
    }

    let content = match get_pfx_contents(&args[1]) {
        Ok(buf) => buf,
        Err(x) =>  {
            println!("Error with provided file: {}", x);
            return;
        }           
    };

    let pkcs12 = Pkcs12::from_der(&content).unwrap();

    let password = rpassword::prompt_password("PFX Import Key: ").unwrap();
    let x509 = pkcs12.parse(&password).unwrap();
   
    let cert = x509.cert;
    let pemcert = &cert.to_pem().unwrap();
    
    println!("{}", std::str::from_utf8(&pemcert).unwrap() );
    let key = x509.pkey;
    let pemkey = &key.private_key_to_pem_pkcs8().unwrap();
    println!("{}", std::str::from_utf8(&pemkey).unwrap() );
}
