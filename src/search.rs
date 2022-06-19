#![allow(dead_code)]
use std::io::Result;

use async_recursion::async_recursion;
use async_std::path::PathBuf;
use futures::{future::select_ok, TryStreamExt};

#[derive(Clone, Copy)]
/// A type representing the type of an item.
pub enum ItemType {
    ///the item must be a file
    File,
    ///the item must be a directory
    Directory,
    ///the item may be a directory or a file
    Either,
}
/// asynchronously finds the first item (folder or file) in the given directory that matches the given name and type.
///
/// `child` is the name+extension of the file or folder to find.
/// # Errors
/// An error will be returned in the following situations:
/// * `path` does not point to an existing directory.
/// * the `item` does not exist.
/// * any other IO error.
#[async_recursion]
pub async fn find_item(dir: PathBuf, child: String, item_type: ItemType) -> Result<String> {
    let mut files = async_std::fs::read_dir(dir).await?.into_stream();
    //all the found folders in the current directory
    let mut folders = Vec::new();
    while let Some(path) = files.try_next().await? {
        let path = async_std::fs::DirEntry::path(&path);
        //checks for match
        if path.iter().last().unwrap().to_str().unwrap() == &child {
            match item_type {
                ItemType::File => {
                    if path.is_file().await {
                        return Ok(path.to_str().unwrap().to_string());
                    }
                }
                ItemType::Directory => {
                    if path.is_dir().await {
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
            folders.push(find_item(path.clone(), child.clone(), item_type));
        }
    }
    if folders.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("item `{}` not found", child),
        ));
    }
    let result: String = select_ok(folders).await?.0;
    Ok(result)
}
