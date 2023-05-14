use std::fs;

use locus_lib::{storage::{Storable, StoragePath}, json::Jsonable};
use serde::{Serialize, Deserialize};


#[test]
fn test_storage_path() {
    #[derive(Serialize, Deserialize)]
    struct TestObj {
        name: String,
    }

    impl StoragePath for TestObj {
        fn storage_file_name() -> &'static str {
            "test.json"
        }

        fn storage_dir_name() -> &'static str {
            ".locus"
        }
    }

    let storage_path = TestObj::get_or_create_storage_file_path().unwrap();
    assert!(storage_path.parent().unwrap().exists());
    fs::remove_dir_all(storage_path.parent().unwrap()).unwrap();
}

#[test]
fn test_storable() {
    #[derive(Serialize, Deserialize)]
    struct TestObj {
        name: String,
    }

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

    let obj = TestObj {
        name: "Test".to_string(),
    };

    obj.save().unwrap();
    let loaded_obj = TestObj::load().unwrap();
    assert_eq!(obj.name, loaded_obj.name);
    fs::remove_dir_all(TestObj::get_or_create_storage_path().unwrap()).unwrap();
}