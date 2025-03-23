use std::{env, fs};

fn main() {
    let config_content = fs::read_to_string("../configs/configuration.nix")
        .expect("Failed to read configuration file");
    
    // Parse the file content into an AST.
    let parsed = rnix::Root::parse(&config_content);

    // The AST preserves the exact formatting and span info.
    println!("{:#?}", parsed.syntax());
}
