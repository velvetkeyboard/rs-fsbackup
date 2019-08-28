extern crate yaml_rust;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate shellexpand;

use std::path::Path;
use std::fs::read_to_string;
use yaml_rust::YamlLoader;

fn main() {
    let contents = read_to_string("config.yaml").expect("Unable to read file");
    let config = YamlLoader::load_from_str(&contents).unwrap();
    let backups = &config[0]["backups"].as_hash().unwrap();    

    for device_name in backups.keys() {
        println!("Backing up for device {:?}", device_name.as_str().unwrap());
        for entry in backups[device_name].as_vec().unwrap().iter() {
            println!("  Processing {:?}", entry["name"].as_str().unwrap());
            let source = &entry["source"];
            let mut s3_key = String::from("");
            s3_key.push_str(device_name.as_str().unwrap());
            s3_key.push_str("/");
            s3_key.push_str(source.as_str().unwrap());
            println!("    Uploading {} to s3://{}", source.as_str().unwrap(), s3_key);
            // Check if it's a file or not
            let mut expanded_source = String::from("");
            expanded_source.push_str(
                &shellexpand::tilde(source.as_str().unwrap()).to_string()
            );
            if Path::new(&expanded_source).is_file() {
                println!("      File!");
            } else if Path::new(&expanded_source).is_dir() {
                println!("      Dir!");
            }
        }
    }
}