use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use crate::json::Jsonable;

/// Trait for types that can be stored.
///
/// Contains logic to create or get the path to the storage file and storage dir.
pub trait StoragePath {
    /// The name of the storage file.
    fn storage_file_name() -> &'static str;

    /// The name of the storage directory.
    fn storage_dir_name() -> &'static str;

    /// The base directory path.
    fn base_dir_path() -> PathBuf {
        dirs::home_dir().unwrap()
    }

    /// Gets or creates the storage directory.
    ///
    /// If its creation fails. an [`ErrorKind::Unsupported`] error is returned.
    fn get_or_create_storage_path() -> Result<PathBuf, Error> {
        let mut storage_dir_path = PathBuf::new();
        storage_dir_path.push(Self::base_dir_path());
        storage_dir_path.push(Self::storage_dir_name());
        if storage_dir_path.exists() {
            return Ok(storage_dir_path);
        }
        match fs::create_dir_all(&storage_dir_path) {
            Ok(_) => Ok(storage_dir_path),
            Err(_) => Err(Error::new(
                ErrorKind::Unsupported,
                "Could not create storage directory",
            )),
        }
    }

    /// Gets or creates the storage file path.
    fn get_or_create_storage_file_path() -> Result<PathBuf, Error> {
        let storage_dir_path = match Self::get_or_create_storage_path() {
            Ok(path) => path,
            Err(e) => return Err(e),
        };
        let storage_file_path = storage_dir_path.join(Self::storage_file_name());

        Ok(storage_file_path)
    }
}

/// Trait for types that can be stored in a file.
///
/// If implemented, the type can be loaded from and saved to a file.
/// The file is located in the user's home directory.
/// It requieres the type to implement the [`Jsonable`] and [`StoragePath`] traits.
pub trait Storable: Jsonable + StoragePath {
    /// Loads the type from the storage file.
    fn load() -> Result<Self, Error> {
        let storage_file_path = match Self::get_or_create_storage_file_path() {
            Ok(path) => path,
            Err(e) => return Err(e),
        };

        Self::from_json(&storage_file_path)
    }

    /// Saves the type to the storage file.
    fn save(&self) -> Result<(), Error> {
        let storage_file_path = match Self::get_or_create_storage_file_path() {
            Ok(path) => path,
            Err(e) => return Err(e),
        };

        match fs::write(&storage_file_path, self.to_json()) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(
                ErrorKind::Other,
                format!("Could not save storage. {err:?}, file_name: {storage_file_path:?}"),
            )),
        }
    }
}
