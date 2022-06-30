#![cfg(test)]
#![allow(dead_code)]
use crate::search::*;

use std::env;
use std::error::Error;
use std::fs;

use rand::distributions::Alphanumeric;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

///increments the seed
fn increment(seed: &mut ChaCha8Rng) {
    let current = seed.get_stream();
    seed.set_stream(current + 1);
}
///Generates a random file name for testing purposes
///Takes the original item if it exists to avoid giving the same name
fn generate_dir(seed: &mut ChaCha8Rng, item: Option<&Item>) -> String {
    loop {
        let created_dir = seed
            .clone()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();
        increment(seed);

        if let Some(item) = item {
            if created_dir != item.name {
                return created_dir;
            }
        } else {
            return created_dir;
        }
    }
}

///Generates a random file name for testing purposes
///Takes the original item if it exists to avoid giving the same name
fn generate_file(seed: &mut ChaCha8Rng, item: Option<&Item>) -> String {
    loop {
        let created_file: String = format!(
            "{}.{}",
            seed.clone()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>(),
            seed.clone()
                .sample_iter(&Alphanumeric)
                .take(4)
                .map(char::from)
                .collect::<String>()
        );
        increment(seed);

        if let Some(item) = item {
            if created_file != item.name {
                return created_file;
            }
        } else {
            return created_file;
        }
    }
}
#[derive(Clone)]
struct Item {
    name: String,
    item_type: ItemType,
}
struct Items {
    real_item: Item,
    fake_file: String,
    fake_dir: String,
}
///This is an internal function used for testing.
///It does the following:
///1. Creates a temporary random folder structure.
///2. Tests `find_item` in the created directory
///3. Deletes the created folder
fn simulate(
    current_dir: &mut std::path::PathBuf,
    root_dir: &std::path::PathBuf,
    seed: &mut ChaCha8Rng,
    items: Items,
) -> Result<(), Box<dyn Error>> {
    //creates the directory that we will be searching
    fs::create_dir_all(&root_dir)?;
    //the file that does not exist and should fail when searched for
    let fake_file = generate_file(seed, Some(&items.real_item));

    //*creates a random folder structure
    loop {
        //the item we will create
        let mut next_item = current_dir.clone();
        //decide whether to create a folder or file
        if seed.gen_bool(0.7) {
            let item = generate_dir(seed, Some(&items.real_item));

            next_item.push(&item);
            fs::create_dir(&next_item)?;
            if seed.gen_bool(0.6) {
                current_dir.push(item);
            }
        } else {
            let item = generate_file(seed, Some(&items.real_item));
            next_item.push(item);
            fs::write(&next_item, "")?;
            next_item.pop();
        }
        if seed.gen_bool(1.0 / 12.0) {
            next_item.push(&items.real_item.name);
            if items.real_item.item_type == ItemType::File {
                fs::write(next_item, "")?;
            } else if items.real_item.item_type == ItemType::Directory {
                fs::create_dir(&next_item)?;
            }
            //if not specified decides whether or not to create a file or directory randomly
            else {
                if seed.gen_bool(1.0 / 2.0) {
                    fs::write(next_item, "")?;
                } else {
                    fs::create_dir(&next_item)?;
                }
            }
            break;
        }
    }
    //*tests `find_item`
    //`Directory`
    let res = find_item(&root_dir, &items.real_item.name, ItemType::File);
    if items.real_item.item_type == ItemType::Directory {
        assert!(res.is_err());
    } else if items.real_item.item_type == ItemType::File {
        assert!(res.is_ok());
    }
    //`Either`
    find_item(&root_dir, &items.real_item.name, ItemType::Either)?;
    //`File`
    let res = find_item(&root_dir, &items.real_item.name, ItemType::Directory);
    if items.real_item.item_type == ItemType::File {
        assert!(res.is_err());
    } else if items.real_item.item_type == ItemType::Directory {
        assert!(res.is_ok());
    }

    let err = find_item(&root_dir, &fake_file, ItemType::Directory);
    assert!(err.is_err());
    let err = find_item(&root_dir, &fake_file, ItemType::File);
    assert!(err.is_err());
    let err = find_item(&root_dir, &fake_file, ItemType::Either);
    assert!(err.is_err());

    //deletes the test directory
    fs::remove_dir_all(&root_dir)?;

    //resets the working directory
    *current_dir = root_dir.clone();
    Ok(())
}

#[test]
fn deep_search() -> Result<(), Box<dyn Error>> {
    //the dir the algorithm is in
    let mut current_dir = env::current_dir()?;
    current_dir.push("test_dir");

    //the root dir that we will be testing in
    let test_dir = current_dir.clone();
    //makes sure the folder doesn't already exist
    let _ = fs::remove_dir_all(&test_dir);

    //the arbitrary number of times we want to test
    for i in 0..60 {
        //*searching for a folder
        let mut seed = ChaCha8Rng::seed_from_u64(i);
        let real_item = Item {
            name: generate_dir(&mut seed, None),
            item_type: ItemType::Directory,
        };
        let items = Items {
            real_item: real_item.clone(),
            fake_dir: generate_dir(&mut seed, Some(&real_item)),
            fake_file: generate_file(&mut seed, Some(&real_item)),
        };

        simulate(&mut current_dir, &test_dir, &mut seed, items)?;

        //*searching for a file
        let real_item = Item {
            name: generate_file(&mut seed, None),
            item_type: ItemType::File,
        };
        let items = Items {
            real_item: real_item.clone(),
            fake_dir: generate_dir(&mut seed, Some(&real_item)),
            fake_file: generate_file(&mut seed, Some(&real_item)),
        };

        simulate(&mut current_dir, &test_dir, &mut seed, items)?;

        //*searching for either
        let real_item = Item {
            name: generate_file(&mut seed, None),
            item_type: ItemType::Either,
        };
        let items = Items {
            real_item: real_item.clone(),
            fake_dir: generate_dir(&mut seed, Some(&real_item)),
            fake_file: generate_file(&mut seed, Some(&real_item)),
        };
        simulate(&mut current_dir, &test_dir, &mut seed, items)?;
    }

    Ok(())
}
