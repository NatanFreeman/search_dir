# search_dir

A Rust library for finding the first matching item in a directory.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
futures="^0.3.21"
search_dir = "0.1"
```

## Example

```rust
    use futures::executor::block_on;
    use search_dir::search::{find_item, ItemType};
    use std::env;
    use std::fs;

    fn main() -> std::io::Result<()> {
        //creates directory we want to search
        fs::create_dir_all("./some/dir")?;
        let current_dir = env::current_dir()?;

        //searches for a directory called `dir`
        let found_path = block_on(find_item(
            current_dir.into(),
            "dir".to_string(),
            ItemType::Directory,
        ))?
        .unwrap();

        println!("{:?}", found_path);
        Ok(())
    }
```
