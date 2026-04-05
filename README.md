# USB 2.0 Bulk File Transfer CLI (Rust)

A high-performance CLI tool built with **Rust** (zero-framework) specifically designed to handle mass file transfers (thousands of files) to legacy USB 2.0 hardware formatted with **FAT32**.

## 🚀 Why Use This Tool?
Transferring 20GB+ consisting of thousands of small files to a USB 2.0 drive often causes standard OS File Explorers to hang or overheat the drive controller. This tool optimizes the process by:
- **Low-Level I/O**: Direct file streaming using Rust's standard library for minimal overhead.
- **Throttling Control**: Implements `SLEEP_MS` to prevent the USB controller from throttling or disconnecting during long 20GB+ sessions.
- **Stability**: Better handle on deep directory trees and mass metadata operations on legacy interfaces.

## 🛠 Configuration (.env)
The tool reads path configurations from a `.env` file in the root directory.

```env
# Source directory (Local)
SOURCE_PATH=/Users/username/Documents/YOUR_SOURCE_PATH

# Destination directory (USB/Flashdisk)
DESTINATION_PATH=/Volumes/USB_BACKUP/YOUR_DESTINATION_PATH

# Delay between file transfers (in milliseconds) 
# Recommended: 100-300ms to maintain USB 2.0 stability
SLEEP_MS=300

# How to run
git clone https://github.com
cd usb20-filetransfer
cargo run 

# Or if you want build 
cargo build --release
