# search_dir

A Rust library for finding the first matching item in a directory.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
search_dir = "0.1.2"
```

## Example

```rust
use std::fs;
use std::env;
use std::error::Error;
use search_dir::search::{ItemType, find_item};

fn main()->Result<(), Box<dyn Error>> {
   //creates directory we want to search
   fs::create_dir_all("./some/awesome/really/cool/")?;
   fs::write("./some/awesome/really/cool/hello.txt", "this is a file")?;
   let mut current_dir = env::current_dir()?;

   //searches for a file called `hello.txt`
   let found_path = find_item(&current_dir, "hello.txt", ItemType::File)?;

   println!("{:?}", found_path);
   current_dir.push("some");
   fs::remove_dir_all(current_dir)?;
   Ok(())
}
```

## License

This project is licensed under [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
