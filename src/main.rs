use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::error::Error;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mypath = &args[1];

    // Set flle extensions to keep
    let file_extensions = vec![OsStr::new("flac"), OsStr::new("mp3"), OsStr::new("webm")];

    println!("Found:");

    // Scan files and folders in directory
    let (_files, folders) = match scan_path(mypath) {

        Ok((fi, fo)) => {
        
            println!("  => {} files", fi.len());
            // for f in &fi {
            //     println!("{:?}", f.file_name());
            // }

            println!("  => {} folders", fo.len());
            // for f in &fo {
            //     println!("{:?}", f.file_name());
            // }

            (fi, fo)
        },

        Err(e) => panic!("ERROR: {}", e)

    };

    // Recursively scan folders for flacs (UNSAFE)
    let mut deep_files: Vec<fs::DirEntry> = Vec::new();
    recursive_find(&folders, &mut deep_files);
    println!("  => {} files nested in folders", deep_files.len());
    // for f in &deep_files {
    //     println!("{:?}", f.file_name());
    // }

    println!("\nExtracting...");
    if let Err(e) = extract_music(&deep_files, &file_extensions, &mut mypath.clone()) {
        panic!("ERROR: {}", e);
    }

    // Remove folders
    println!("Removing folders...");
    for folder in folders {
        if !folder.file_name().into_string().unwrap().starts_with(".") {
            fs::remove_dir_all(folder.path()).unwrap();
        }
    }

    println!("Complete!");
}

fn scan_path(directory: &str) -> Result<(Vec<fs::DirEntry>, Vec<fs::DirEntry>), std::io::Error> {

    let paths = fs::read_dir(directory)?;

    let mut files = Vec::new();
    let mut folders = Vec::new();

    for p in paths {
        
        let path = p?;
        let metadata = path.metadata()?;

        if metadata.is_file() {
            files.push(path);
        } else {
            folders.push(path);
        }
    }

    Ok((files, folders))
}

fn recursive_find(folders: &Vec<fs::DirEntry>, found_files: &mut Vec<fs::DirEntry>) {

    // println!();
    if folders.len() == 0 {
        return
    }
    for folder in folders {
        
        let (deep_files, deep_folders) = scan_path(folder.path().to_str().unwrap()).unwrap();
        for file in deep_files {
            found_files.push(file);
            // println!("{:?}", file.file_name());
        }
        recursive_find(&deep_folders, found_files);
    }

}

fn extract_music(files: &Vec<fs::DirEntry>, file_extensions: &Vec<&OsStr>, origin: &mut String) -> Result<(), Box<dyn Error>> {

    if !(origin.ends_with("/") || origin.ends_with("\\")) {
        origin.push('/');
    }

    for file in files {
        let path = file.path();
        let path = Path::new(&path);
        match path.extension() {
            Some(val) => {
                if file_extensions.contains(&val) {
                    // println!("{}", file.file_name().into_string().unwrap());
                    let destination = format!("{}{}", origin, file.file_name().into_string().unwrap());
                    // Copy over
                    fs::copy(file.path(), destination)?;
                }
            },
            None => ()
        };
    }

    Ok(())
}