#![cfg(test)]
use crate::search::find_item;
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

    let _data_path = block_on(find_item(path.into(), "".to_string(), ItemType::Either));
}
#[test]
fn finds_dir() {
    let path = std::env::current_dir().unwrap();
    //wether in debug or build, cargo should produce a folder called `build`
    let _data_path = block_on(find_item(
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
    let _data_path = block_on(find_item(
        path.into(),
        ".cargo-lock".to_string(),
        ItemType::File,
    ))
    .unwrap();
}
#[test]
fn fails_to_find_file() {
    let path = std::env::current_dir().unwrap();

    let result = block_on(find_item(
        path.into(),
        "not_a_file".to_string(),
        ItemType::File,
    ))
    .unwrap();
    assert!(result.is_none());
}
