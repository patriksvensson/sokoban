use std::env;
use std::fs;
use std::path::Path;

fn main() {
    
    // Define well known directories.
    let profile = env::var("PROFILE").unwrap();
    let target_path = Path::new("target").join(profile).join("data");

    // Does the target directory exist?
    if !target_path.exists() {
        // Create it.
        std::fs::create_dir(&target_path).unwrap();
    }

    // Copy all data to the output directory.
    let paths = fs::read_dir(".\\data").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let destination = target_path.join(path.file_name().unwrap());
        fs::copy(path, destination).unwrap();
    }
}
