use base58::ToBase58;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

fn main() {
    // The name of the wallet generated while anchor build
    let key_pair_file_name = env::var("CARGO_PKG_NAME")
        .unwrap()
        .to_owned()
        .replace("-", "_")
        + ("-keypair.json");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let key_pair_file_path = Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target")
        .join("deploy")
        .join(key_pair_file_name);

    // Read keypare file
    let file = File::open(Path::new(&key_pair_file_path)).unwrap();

    let bytes: Vec<u8> = serde_json::from_reader(&file).unwrap();
    let keypair = ed25519_dalek::Keypair::from_bytes(&bytes)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        .unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("program_id_local.rs");
    fs::write(
        &dest_path,
        String::from("declare_id!(\"") 
        + keypair.public.as_ref()
        .to_base58()
        .as_ref() 
        + String::from("\");")
        .as_ref()
    ).unwrap();
}
