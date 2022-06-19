#![allow(dead_code)]
use std::io::Result;

use async_recursion::async_recursion;
use async_std::path::PathBuf;
use futures::{future::select_ok, TryStreamExt};

#[derive(Clone, Copy)]
pub enum ItemType {
    File,
    Directory,
    Either,
}
//super fancy async recursion for finding the `data` folder
#[async_recursion]
pub async fn find_folder(path: PathBuf, child: String, item_type: ItemType) -> Result<String> {
    let mut files = async_std::fs::read_dir(path).await?.into_stream();
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
            folders.push(find_folder(path.clone(), child.clone(), item_type));
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
