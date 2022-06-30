#![allow(dead_code)]

#[derive(Clone, Copy, PartialEq, Debug)]
/// A type representing the type of an item.
pub enum ItemType {
    ///the item must be a file
    File,
    ///the item must be a directory
    Directory,
    ///the item may be a directory or a file
    Either,
}
///Allows direct access to the `async` execution of `find_item`
pub mod async_search {
    use crate::search::ItemType;
    use async_recursion::async_recursion;
    use async_std::path::PathBuf;
    use futures::{future::select_ok, TryStreamExt};
    use std::io::Result;

    /// Asynchronously finds the first item (folder or file) in the given directory that matches the given name and type.
    ///
    /// `child` is the name+extension of the file or folder to find.
    /// # Errors
    /// An error will be returned in the following situations:
    /// * `path` does not point to an existing directory.
    /// * the `item` does not exist.
    /// * any other IO error.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use std::env;
    /// use std::error::Error;
    /// use search_dir::search::async_search::find_item;
    /// use search_dir::search::ItemType;
    ///
    /// fn main()->Result<(), Box<dyn Error>> {
    ///     //creates directory we want to search
    ///     fs::create_dir_all("some/awesome/really/cool/dir")?;
    ///     let mut current_dir = env::current_dir()?;
    ///
    ///     //searches for a directory called `dir`
    ///     let found_path = futures::executor::block_on(find_item(
    ///         current_dir.clone().into(),
    ///         "dir",
    ///         ItemType::Directory,
    ///     ))?;
    ///
    ///     println!("{:?}", found_path);
    ///     current_dir.push("some");
    ///     fs::remove_dir_all(current_dir)?;
    ///     Ok(())
    /// }
    /// ```
    #[async_recursion]
    pub async fn find_item(dir: PathBuf, child: &str, item_type: ItemType) -> Result<String> {
        let mut files = async_std::fs::read_dir(dir).await?.into_stream();
        //all the found folders in the current directory
        let mut folders = Vec::new();
        while let Some(path) = files.try_next().await? {
            let path = async_std::fs::DirEntry::path(&path);
            //checks for match
            if path.iter().last().unwrap().to_str().unwrap() == child {
                match item_type {
                    ItemType::File => {
                        if path.as_path().is_file().await {
                            return Ok(path.to_str().unwrap().to_string());
                        }
                    }
                    ItemType::Directory => {
                        if path.as_path().is_dir().await {
                            return Ok(path.to_str().unwrap().to_string());
                        }
                    }
                    ItemType::Either => {
                        return Ok(path.to_str().unwrap().to_string());
                    }
                }
            }
            if path.is_dir().await {
                //recursively adds all the folders inside the folder to `folders`
                folders.push(find_item(path.clone(), &child, item_type));
            }
        }
        if folders.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("item `{}` not found", child),
            ));
        }
        let result = select_ok(folders).await?.0;
        Ok(result)
    }
}

use futures::executor::block_on;
use std::io::Result;
/// Same as `search_dir::search::async_search::find_item` but executed on the current thread for convenience.
///
/// # Example
///
/// ```
/// use std::fs;
/// use std::env;
/// use std::error::Error;
/// use search_dir::search::{ItemType, find_item};
///
/// fn main()->Result<(), Box<dyn Error>> {
///    //creates directory we want to search
///    fs::create_dir_all("./some/awesome/really/cool/")?;
///    fs::write("./some/awesome/really/cool/hello.txt", "this is a file")?;
///    let mut current_dir = env::current_dir()?;
///
///    //searches for a file called `hello.txt`
///    let found_path = find_item(&current_dir, "hello.txt", ItemType::File)?;
///
///    println!("{:?}", found_path);
///    current_dir.push("some");
///    fs::remove_dir_all(current_dir)?;
///    Ok(())
/// }
/// ```
pub fn find_item(dir: &std::path::Path, child: &str, item_type: ItemType) -> Result<String> {
    block_on(crate::search::async_search::find_item(
        dir.into(),
        &child,
        item_type,
    ))
}
