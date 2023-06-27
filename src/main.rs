use std::{env, fs, path::Path};

use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [src, dst] => copy_files(Path::new(src), Path::new(dst)),
        _ => println!("Invalid arguments"),
    }
}

fn copy_files(src: &Path, dst: &Path) {
    for file in WalkDir::new(src) {
        let file = file.unwrap();
        let path = file.path();

        if path.is_file() {
            let new_path = path.strip_prefix(src).unwrap();
            let new_path = dst.join(new_path);

            if new_path.exists() {
                let meta = path.metadata().unwrap().modified().unwrap();
                let new_meta = new_path.metadata().unwrap().modified().unwrap();

                if new_meta > meta {
                    println!("Skipping {}", path.to_str().unwrap());
                    continue;
                } else {
                    fs::copy(path, new_path).expect("Error updating file");
                    println!("Updating {}", path.to_str().unwrap());
                }
            } else {
                fs::create_dir_all(new_path.parent().unwrap()).expect("Error creating directory");
                fs::copy(path, new_path).expect("Error copying files");
                println!("Copying {}", path.to_str().unwrap());
            }
        }
    }
}
