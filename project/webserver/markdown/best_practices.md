# Best practices
A collection of Rust best practices.

[Load a file async (non-blocking)](#load-a-file-async-non-blocking)

## Load a file async (non-blocking)
Also a good example for constructing paths.
```rust
let mut file_path = PathBuf::new();
file_path.push("subfolder");
file_path.push("textfile");
file_path.set_extension("txt");
let Ok(content) = tokio::fs::read_to_string(file_path).await else {
    // Handle file could not be read
};
// Do something with with content here
```