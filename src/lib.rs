use std::io::Result;

use async_recursion::async_recursion;
use async_std::path::PathBuf;
use futures::{future::select_ok, TryStreamExt};

//super fancy async recursion for finding the `data` folder
#[async_recursion]
pub async fn find_folder(path: PathBuf, folder_name: String) -> Result<String> {
    let mut files = async_std::fs::read_dir(path.clone()).await?.into_stream();
    //all the found folders in the current directory
    let mut folders = Vec::new();
    while let Some(path) = files.try_next().await? {
        let path = async_std::fs::DirEntry::path(&path);
        if path.is_dir().await {
            if path.file_name().unwrap().to_str().unwrap() == &folder_name {
                return Ok(path.to_str().unwrap().to_string());
            }
            //recursively adds all the folders inside the folder to `folders`
            folders.push(find_folder(path.clone(), folder_name.clone()));
        }
    }
    if folders.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "data folder not found",
        ));
    }
    let result: String = select_ok(folders).await?.0;
    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn compiles() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn runs() {
        use crate::find_folder;
        use futures::executor::block_on;
        let path = std::env::current_dir().unwrap();

        let _data_path = block_on(find_folder(path.into(), "".to_string()));
    }
    #[test]
    fn finds_folder() {
        use crate::find_folder;
        use futures::executor::block_on;
        let path = std::env::current_dir().unwrap();

        let _data_path = block_on(find_folder(path.into(), "build".to_string())).unwrap();
    }
}
