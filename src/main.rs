extern crate yaml_rust;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate shellexpand;
extern crate walkdir;

use walkdir::{DirEntry, WalkDir};
// use std::path::Path;
use std::fs::read_to_string;
use yaml_rust::YamlLoader;
use rusoto_core::region::Region;
use rusoto_s3::S3Client;

fn main() {
    let contents = read_to_string("config.yaml").expect("Unable to read file");
    let config = YamlLoader::load_from_str(&contents).unwrap();
    let backups = &config[0]["backups"].as_hash().unwrap();    

    for device_name in backups.keys() {
        println!("Backing up for device {:?}", device_name.as_str().unwrap());
        for entry in backups[device_name].as_vec().unwrap().iter() {
            println!("  Processing {:?}", entry["name"].as_str().unwrap());
            let source = &entry["source"];

            // Check if it's a file or not
            let mut expanded_source = String::from("");
            expanded_source.push_str(
                &shellexpand::tilde(source.as_str().unwrap()).to_string()
            );

            // if Path::new(&expanded_source).is_file() {
            //     println!("      File!");
            // }
            // else if Path::new(&expanded_source).is_dir() {
            //     println!("      Dir!");
            // }

            for entry in WalkDir::new(&expanded_source).into_iter() {
                let full_file_path = entry.ok().unwrap().into_path().to_str().unwrap().to_owned();
                let mut s3_key = String::from("");
                s3_key.push_str(device_name.as_str().unwrap());
                s3_key.push_str("/");
                s3_key.push_str(&full_file_path);
                println!("    Uploading {} to s3://{}", source.as_str().unwrap(), s3_key);
                let mut client = S3Client::new(Region::UsEast1);
            }
        }
    }
}