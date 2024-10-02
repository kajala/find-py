use walkdir::{WalkDir, DirEntry};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <str>", args[0]);
        std::process::exit(1);
    }
    let mut include_site_packages = false;
    let mut term: String = "".to_owned();
    for arg in args {
        if arg.starts_with("--") {
            if arg == "--include-site-packages" {
                include_site_packages = true;
            }
        }
        else {
            if term != "" {
                term.push_str(" ");
            }
            term = arg;
        }
    }

    let dir = ".";
    for entry in WalkDir::new(dir).into_iter().filter_entry(|entry| !is_skipped_dir(entry, include_site_packages)).filter_map(|e| e.ok()) {
        let file_path = entry.path();
        
        // If it's a file, you can process it here
        if file_path.is_file() {
            if let Some(basename) = entry.file_name().to_str() {
                if basename.ends_with(".py") {
                    //println!("File: {}", file_path.display());
                    let file = File::open(file_path)?;
                    let reader = BufReader::new(file);
                    let mut linenum = 1;
                    for line in reader.lines() {
                        let line = line?; // Handle potential I/O errors
                        //println!("{}", line);
                        if line.contains(&term) {
                            println!("{}({}): {}", file_path.display(), linenum, line);
                        }
                        linenum += 1;
                    }
                }
            }
        }
        else if file_path.is_dir() {
            //println!("Directory: {}", file_path.display());
        }
    }
    Ok(())
}

fn is_skipped_dir(entry: &DirEntry, include_site_packages: bool) -> bool {
    if let Some(basename) = entry.file_name().to_str() {
        let mut dirs_to_ignore = vec![".pytype", "__pycache__", ".mypy_cache", "downloads", "uploads", "tmp", ".git", "migrations", "logs"];
        if !include_site_packages {
            dirs_to_ignore.push("site-packages");
        }
        return dirs_to_ignore.contains(&basename);
    }
    return true;
}

