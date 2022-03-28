use openssl::pkcs12::Pkcs12;
use std::io::prelude::*;
use std::fs::File;


fn main() {

    let mut file = File::open("identity.pfx").unwrap();
    let mut pkcs12 = vec![];
    file.read_to_end(&mut pkcs12).unwrap();
    let pkcs12 = Pkcs12::from_der(&pkcs12).unwrap();

    let password = rpassword::prompt_password("PFX Import Key: ").unwrap();
    let x509 = pkcs12.parse(&password).unwrap();
   
    let cert = x509.cert;
    let pemcert = &cert.to_pem().unwrap();
    
    println!("{}", std::str::from_utf8(&pemcert).unwrap() );
    let key = x509.pkey;
    let pemkey = &key.private_key_to_pem_pkcs8().unwrap();
    println!("{}", std::str::from_utf8(&pemkey).unwrap() );
}
