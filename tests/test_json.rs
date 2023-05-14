use std::{fs::File, io::Write};

use locus::json::Jsonable;
use serde::{Serialize, Deserialize};
use tempfile::tempdir;

#[test]
fn test_encodes_to_json() {
    #[derive(Serialize, Deserialize)]
    struct TestObj {
        name: String,
        details: String,
    }
    impl Jsonable for TestObj {}
    let obj = TestObj {
        name: "Test".to_string(),
        details: "Details".to_string(),
    };

    let json = obj.to_json();
    assert_eq!(json, "{\"name\":\"Test\",\"details\":\"Details\"}");
}

#[test]
fn test_encodes_nested_to_json() {
    #[derive(Serialize, Deserialize)]
    struct NestedObj {
        name: String,
    }

    #[derive(Serialize, Deserialize)]
    struct TestObj {
        name: String,
        details: NestedObj,
    }

    impl Jsonable for TestObj {}

    let obj = TestObj {
        name: "Test".to_string(),
        details: NestedObj {
            name: "test".to_string(),
        },
    };

    let json = obj.to_json();
    assert_eq!(json, "{\"name\":\"Test\",\"details\":{\"name\":\"test\"}}");
}

#[test]
fn test_decodes_from_json() {
    #[derive(Serialize, Deserialize)]
    struct TestObj {
        name: String,
        details: String,
    }
    impl Jsonable for TestObj {}
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("my-temporary-note.txt");
    let mut file = File::create(&file_path).unwrap();
    let json = "{\"name\":\"Test\",\"details\":\"Details\"}";
    file.write(json.as_bytes()).unwrap();

    let obj = TestObj::from_json(&file_path).unwrap();
    assert_eq!(obj.name, "Test");
    assert_eq!(obj.details, "Details");

    drop(file);
    dir.close().unwrap();
}

#[test]
fn test_decodes_nested_from_json() {
    #[derive(Serialize, Deserialize)]
    struct NestedObj {
        name: String,
    }

    #[derive(Serialize, Deserialize)]
    struct TestObj {
        name: String,
        details: NestedObj,
    }

    impl Jsonable for TestObj {}

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("my-temporary-note.txt");
    let mut file = File::create(&file_path).unwrap();
    let json = "{\"name\":\"Test\",\"details\":{\"name\":\"test\"}}";
    file.write(json.as_bytes()).unwrap();

    let obj = TestObj::from_json(&file_path).unwrap();
    assert_eq!(obj.name, "Test");
    assert_eq!(obj.details.name, "test");

    drop(file);
    dir.close().unwrap();
}