
# file_report

**file_report** is a Rust-based tool that generates a disk usage report, identifying the largest directories, largest files, and cache directory usage on your system. The report is saved to a text file for easy reference.

## Features

- **Disk Usage Report**: Scans the root directory (`/`) and reports the size of each top-level directory.
- **Largest Files Report**: Identifies the top 20 largest files in the system.
- **Cache Files Report**: Provides a summary of the `.cache` directory usage for users in `/home`.

## Requirements

- Rust toolchain installed
- The `fs_extra` crate for directory size calculation
- The `walkdir` crate for recursive directory traversal

## Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/your-username/file_report.git
   cd file_report
   ```

2. **Build the project using Cargo**:

   ```bash
   cargo build --release
   ```

## Usage

Run the program to generate the disk space report:

```bash
./target/release/file_report
```

This will generate a detailed report and save it to `full_disk_space_report.txt`.

### Example of Output:

The report will include:

1. **Disk Usage Report** for `/`:

   ```
   Disk Usage Report for /
   /bin: 20 MB
   /lib: 100 MB
   /usr: 500 MB
   ...
   ```

2. **Largest Files Report**:

   ```
   Largest Files Report
   /usr/lib/libbigfile.so: 1024 MB
   /var/log/huge.log: 500 MB
   ...
   ```

3. **Cache Files Report** for `/home`:

   ```
   Cache Files Report for /home
   /home/user/.cache: 200 MB
   ```

## Customization

- **Disk Usage Report**: Scans the root directory (`/`) by default. You can modify the code to target a different directory.
- **Cache Report**: Scans the `/home` directory. Adjust it as needed if the home directories are located elsewhere.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
