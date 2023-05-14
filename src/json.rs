use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use serde::{de::DeserializeOwned, Serialize};

/// Trait for types that can be converted to and from JSON.
pub trait Jsonable: Serialize + DeserializeOwned {
    /// Encodes the type as JSON.
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Decodes the type from JSON.
    fn from_json(path: &PathBuf) -> Result<Self, Error> {
        let storage_json = match fs::read_to_string(&path) {
            Ok(json) => json,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Could not load storage. err:{e:?}, file_name: {path:?}"),
                ))
            }
        };

        match serde_json::from_str(&storage_json) {
            Ok(storage) => Ok(storage),
            Err(e) => return Err(Error::new(ErrorKind::Unsupported, e.to_string())),
        }
    }
}
