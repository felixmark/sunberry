# Best practices
A collection of Rust best practices.

## Load a file async (non-blocking)
Also a good example for constructing paths.
```rust
let mut file_path = PathBuf::new();
file_path.push("subfolder");
file_path.push("notes");
file_path.set_extension("txt");

if tokio::fs::metadata(&file_path).await.is_ok() {
    let md_content = tokio::fs::read_to_string(file_path).await?;
}
```