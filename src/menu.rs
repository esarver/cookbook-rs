use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use std::str::pattern::Pattern;

use crate::error;

use tracing::error;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Meal {
    pub name: String,
    pub tags: Vec<String>,
}

impl Meal {
    pub fn new(name: String, tags: Option<Vec<String>>) -> Self {
        Self {
            name,
            tags: tags.unwrap_or_else(Vec::new),
        }
    }
}

pub struct Cookbook {
    book: Vec<Meal>,
    path: PathBuf,
}

impl Cookbook {
    /// Connect to the file or database (JSON in this case).
    pub fn connect(path: &PathBuf) -> Result<Self, error::Error> {
        let file = File::open(path)?;
        let file = BufReader::new(file);

        let book = serde_json::from_reader(file)?;

        Ok(Self {
            book,
            path: path.to_path_buf(),
        })
    }

    /// Add a new meal to the [`Cookbook`].
    ///
    /// # Parameters
    /// * `meal`: [`Meal`] - the meal to add to the [`Cookbook`]
    ///
    /// # Returns
    ///
    /// The index of the new meal entry.
    ///
    /// # Errors
    ///
    /// If the meal already exists, returns [`crate::error::Error::AddError`]
    /// with the name of the meal that had a duplicate.
    pub fn add(&mut self, meal: Meal) -> Result<usize, error::Error> {
        if self.book.iter().any(|m| m.name == meal.name) {
            return Err(error::Error::AddError(meal.name));
        }
        self.book.push(meal);

        Ok(self.book.len())
    }

    /// Search the [`Cookbook`]
    ///
    /// # Parameters
    ///
    /// * `pattern`: [`String`] - the pattern to use to search the [`Cookbook`]
    ///
    /// # Errors
    ///
    /// If the search pattern is invalid, returns [`crate::error::Error::InvalidSearchPattern`]
    /// with the provided search string.
    pub fn search(&mut self, pattern: impl Pattern + Clone) -> Result<Vec<&Meal>, error::Error> {
        Ok(self
            .book
            .iter()
            .filter(|e| e.name.contains(pattern.clone()))
            .collect::<Vec<&Meal>>())
    }

    /// Get information on the [`Meal`] in the [`Cookbook`].
    ///
    /// # Parameters
    ///
    /// * `name`: [`String`] - the name of the [`Meal`] about which more information is desired
    ///
    /// # Errors
    ///
    /// If the meal does not exist in the [`Cookbook`], returns
    /// [`crate::error::Error::MealDoesNotExist`] with the provided name.
    pub fn info(&mut self, name: String) -> Result<Meal, error::Error> {
        match self.book.iter().find(|e| e.name == name) {
            Some(m) => Ok(m.clone()),
            None => Err(error::Error::MealDoesNotExist(name)),
        }
    }

    pub fn commit(&mut self) -> Result<(), error::Error> {
        let mut file = File::create(self.path.clone())?;
        file.write_all(serde_json::to_string_pretty(&self.book)?.as_bytes())?;
        Ok(())
    }

    pub fn list(&self) -> Vec<Meal> {
        self.book.clone()
    }
}

impl Drop for Cookbook {
    fn drop(&mut self) {
        if let Err(e) = self.commit() {
            error!("Error committing cookbook: {e}");
        }
    }
}
