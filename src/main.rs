use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use walkdir::{WalkDir, DirEntry};
use fs_extra::dir::get_size;

fn main() -> io::Result<()> {
    let output_file = "full_disk_space_report.txt";
    let mut file = File::create(output_file)?;

    // 1. Disk Usage Report
    let root_dir = "/";
    writeln!(file, "Disk Usage Report for {}", root_dir)?;
    generate_disk_usage_report(root_dir, &mut file)?;

    // 2. Largest Files Report
    writeln!(file, "\nLargest Files Report")?;
    generate_largest_files_report(root_dir, &mut file)?;

    // 3. Cache Files Report
    let home_dir = "/home";
    writeln!(file, "\nCache Files Report for {}", home_dir)?;
    generate_cache_files_report(home_dir, &mut file)?;

    writeln!(file, "\nReport generated successfully!")?;
    println!("Report saved to {}", output_file);

    Ok(())
}

fn generate_disk_usage_report<P: AsRef<Path>>(root: P, file: &mut File) -> io::Result<()> {
    let walker = WalkDir::new(root).max_depth(1);
    for entry in walker {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            match get_size(path) {
                Ok(size) => writeln!(file, "{}: {} MB", path.display(), size / (1024 * 1024))?,
                Err(e) => writeln!(file, "Error getting size for {}: {}", path.display(), e)?,
            }
        }
    }
    Ok(())
}

fn generate_largest_files_report<P: AsRef<Path>>(root: P, file: &mut File) -> io::Result<()> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, // Skip entries we cannot access or have issues with
        };
        if is_regular_file(&entry) {
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue, // Skip files where metadata cannot be accessed
            };
            let size = metadata.len();
            files.push((entry.path().to_owned(), size));
        }
    }
    files.sort_by(|a, b| b.1.cmp(&a.1));

    for (path, size) in files.iter().take(20) {
        writeln!(file, "{}: {} MB", path.display(), size / (1024 * 1024))?;
    }
    Ok(())
}

fn generate_cache_files_report<P: AsRef<Path>>(home_dir: P, file: &mut File) -> io::Result<()> {
    let cache_dir = home_dir.as_ref().join(".cache");
    if cache_dir.exists() && cache_dir.is_dir() {
        match get_size(&cache_dir) {
            Ok(size) => writeln!(file, "{}: {} MB", cache_dir.display(), size / (1024 * 1024))?,
            Err(e) => writeln!(file, "Error getting size for {}: {}", cache_dir.display(), e)?,
        }
    } else {
        writeln!(file, "Cache directory not found or not a directory.")?;
    }
    Ok(())
}

fn is_regular_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}

