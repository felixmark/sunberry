# Book
Notes, best practices and links.

**Index**  
- [Book](#book)
- [Notes](#notes)
- [Best practices](#best-practices)
  - [Load a file async (non-blocking)](#load-a-file-async-non-blocking)
- [Links](#links)
  
  
# Notes
**ToDo**
- Read Photovoltaic Voltage, Current, Power
- Read Raspberry Pi Voltage, Current, Power
- Store read data in SQLite database
- Fetch stored data from database in webinterface and render it as graphs


# Best practices
A collection of Rust best practices.

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

# Links

| Title  | Link        |
| ------ | ----------- |
| 30min Rust | [https://fasterthanli.me/articles/a-half-hour-to-learn-rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) |
| Rust book | [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book)|
| Actix  | [https://actix.rs/docs/](https://actix.rs/docs/)|
| Askama | [https://djc.github.io/askama/](https://djc.github.io/askama/)|
| Ina219 | [https://github.com/scttnlsn/ina219](https://github.com/scttnlsn/ina219)|
| Ina226 | [https://github.com/Thiapalm/ina226](https://github.com/Thiapalm/ina226)|
