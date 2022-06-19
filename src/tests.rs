#![cfg(test)]
use crate::search::find_folder;
use crate::search::ItemType;
use futures::executor::block_on;

#[test]
fn compiles() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
#[test]
fn runs() {
    let path = std::env::current_dir().unwrap();

    let _data_path = block_on(find_folder(path.into(), "".to_string(), ItemType::Either));
}
#[test]
fn finds_dir() {
    let path = std::env::current_dir().unwrap();
    //wether in debug or build, cargo should produce a folder called `build`
    let _data_path = block_on(find_folder(
        path.into(),
        "build".to_string(),
        ItemType::Directory,
    ))
    .unwrap();
}
#[test]
fn finds_file() {
    let path = std::env::current_dir().unwrap();

    //wether in debug or build, cargo should produce a file called `.cargo-lock`
    let _data_path = block_on(find_folder(
        path.into(),
        ".cargo-lock".to_string(),
        ItemType::File,
    ))
    .unwrap();
}
#[test]
fn fails_to_find_file() {
    let path = std::env::current_dir().unwrap();

    let result = block_on(find_folder(
        path.into(),
        "not_a_file".to_string(),
        ItemType::File,
    ));
    assert!(result.is_err());
}
