use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;
use dotenvy::dotenv; 
use std::env;

fn main() -> io::Result<()> {
    // 1. Initialize environment variables from .env file
    dotenv().ok(); 

    // --- Path Configuration ---
    let source_path_raw = env::var("SOURCE_PATH").expect("SOURCE_PATH must be defined in .env");
    let destination_path_raw = env::var("DESTINATION_PATH").expect("DESTINATION_PATH must be defined in .env");
    let sleep_ms_raw = env::var("SLEEP_MS").unwrap_or_else(|_| "100".to_string());

    let source_dir = Path::new(&source_path_raw);
    let destination_dir = Path::new(&destination_path_raw);

    // 2. Ensure the destination directory exists
    if !destination_dir.exists() {
        fs::create_dir_all(destination_dir)?;
        println!("Created destination folder: {:?}", destination_dir);
    }

    // 3. SCAN DESTINATION (Perform once at startup to avoid repeated disk reads)
    println!("🔍 Scanning existing files on destination drive...");
    let mut existing_files = HashSet::new();
    if let Ok(entries) = fs::read_dir(destination_dir) {
        for entry in entries {
            if let Ok(e) = entry {
                if let Ok(meta) = e.metadata() {
                    if meta.is_file() {
                        // Store File Name and Size (Bytes) in the HashSet for O(1) lookup
                        existing_files.insert((e.file_name(), meta.len()));
                    }
                }
            }
        }
    }
    println!("✅ Scan complete. Found {} files on destination.\n", existing_files.len());

    // 4. START MIGRATION PROCESS
    let mut count_skip = 0;
    let mut count_copy = 0;

    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_os_string();
            let source_metadata = fs::metadata(&path)?;
            let source_size = source_metadata.len();

            // CHECK CACHE (Very fast RAM-level lookup)
            if existing_files.contains(&(file_name.clone(), source_size)) {
                count_skip += 1;
                // No print here to keep the terminal clean
                continue;
            }

            // COPY PROCESS IF MISSING OR SIZE DIFFERS
            let target_path = destination_dir.join(&file_name);
            
            match copy_with_buffer(&path, &target_path) {
                Ok(_) => {
                    count_copy += 1;
                    println!("🚀 [{}] Successfully copied.", file_name.to_string_lossy());
                    
                    // Throttle the process to prevent overheating/ejection on USB 2.0
                    let sleep_duration = sleep_ms_raw.parse::<u64>().unwrap_or(100);
                    thread::sleep(Duration::from_millis(sleep_duration));
                }
                Err(e) => {
                    eprintln!("❌ Failed to copy {:?}: {}", file_name, e);
                    // Halt the program if an I/O error occurs (e.g., USB disconnected)
                    return Err(e);
                }
            }
        }
    }

    println!("\n--- SUMMARY ---");
    println!("Total Files Skipped (Already exists): {}", count_skip);
    println!("Total New Files Copied: {}", count_copy);
    println!("All processes completed successfully.");

    Ok(())
}

/// Copies a file using Buffered I/O to be gentle on legacy hardware like USB 2.0
fn copy_with_buffer(src: &Path, dst: &Path) -> io::Result<()> {
    let f_src = File::open(src)?;
    let mut reader = BufReader::new(f_src);

    let f_dst = File::create(dst)?;
    let mut writer = BufWriter::new(f_dst);

    // io::copy utilizes an internal buffer (usually 8KB - 64KB)
    // which is very stable for data transfers over USB
    io::copy(&mut reader, &mut writer)?;
    
    // Explicitly flush to ensure all data is written to physical disk before returning
    writer.flush()?; 
    Ok(())
}