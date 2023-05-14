# Locus

Locus is a simple, light, library for helping in storing JSON data from objects in Rust. It aims to better handle and
manage how data is stored and retrieved from JSON files. This library is strongly recommended to be used with CLI tools
as it will help in storing data in a more efficient way and get rid of the need to write a lot of boilerplate code.

## Run tests

To run tests:
```bash
cargo test
```

To run the documentation:
```bash
cargo doc --open
```


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
locus = "0.1.0"
```

## Example

Short and sweet example on how to implement the traits on your struct:
```rust
/// The struct needs to implement serde_json Serialize and Deserialize
#[derive(Serialize, Deserialize)]
struct TestObj {
    name: String,
}

/// The struct needs to implement Storable and Jsonable
impl StoragePath for TestObj {
    fn storage_file_name() -> &'static str {
        "test.json"
    }

    fn storage_dir_name() -> &'static str {
        ".locus"
    }
}

impl Storable for TestObj {}
impl Jsonable for TestObj {}
```