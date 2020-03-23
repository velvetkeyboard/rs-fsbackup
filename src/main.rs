extern crate yaml_rust;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate shellexpand;
extern crate walkdir;

use walkdir::WalkDir;
// use walkdir::DirEntry;
// use std::path::Path;
use std::fs::read_to_string;
use yaml_rust::YamlLoader;
// use rusoto_core::region::Region;
use rusoto_credential::ProfileProvider;
// use rusoto_s3::S3Client;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let config_path = ;
    // println!("Reading config from: {}", config_path);
    let contents = read_to_string(args[1].clone()).expect("Unable to read file");
    let config = YamlLoader::load_from_str(&contents).unwrap();
    let backups = &config[0]["backups"].as_hash().unwrap();    

    for device_name in backups.keys() {
        println!("Backing up for device {:?}", device_name.as_str().unwrap());
        // println!("{:?}", backups[device_name].as_vec().unwrap());
        for entry in backups[device_name].as_vec().unwrap().iter() {
            println!("\tProcessing {:?}", entry.as_str().unwrap());
            // let source = &entry["source"];
            let file_path = entry.as_str().unwrap();

            // Check if it's a file or not
            let mut expanded_source = String::from("");
            expanded_source.push_str(
                &shellexpand::tilde(file_path).to_string()
            );

            // if Path::new(&expanded_source).is_file() {
            //     println!("      File!");
            // }
            // else if Path::new(&expanded_source).is_dir() {
            //     println!("      Dir!");
            // }

            for entry in WalkDir::new(&expanded_source).into_iter() {
                let option_full_file_path = entry.ok();
                if !option_full_file_path.is_none() {
                    let full_file_path = option_full_file_path.unwrap().into_path().to_str().unwrap().to_owned();
                    println!("{:?}", full_file_path);
                    let mut s3_key = String::from("");
                    s3_key.push_str(device_name.as_str().unwrap());
                    s3_key.push_str("/");
                    s3_key.push_str(&full_file_path);
                    println!("\t\tUploading {} to s3://{}", file_path, s3_key);
                    let pp = ProfileProvider::new();
                    pp.set_profile("default")
                    // let mut client = S3Client::new(Region::UsEast1);
                    let mut client = S3Client::new_with()
                }
                // let full_file_path = entry.ok().unwrap();//.into_path().to_str().unwrap().to_owned();
                // println!("{:}?", full_file_path);
                // println!("{:?}", full_file_path);
            }
        }
    }
}
