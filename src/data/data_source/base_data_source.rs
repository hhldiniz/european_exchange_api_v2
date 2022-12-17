use async_trait::async_trait;
use mongodb::{Database};
use serde::Serialize;

use crate::data::model::base_model::BaseModel;

#[async_trait]
pub trait BaseDataSource {
    async fn insert<T: BaseModel + Serialize>(&self, models: Vec<T>, database: &Database) -> Result<(), String>;
    async fn delete<T: BaseModel + Serialize>(&self, models: Vec<T>, &database: &Database) -> Result<(), String>;
    async fn update<T: BaseModel + Serialize>(&self, models: Vec<T>, &database: &Database) -> Result<(), String>;
    async fn select_one<T: BaseModel + Serialize>(&self, ftr: String, &database: &Database) -> Result<T, String>;
    async fn select_many<T: BaseModel + Serialize>(&self, ftr: String, &database: &Database) -> Result<Vec<T>, String>;
    fn schema(&self) -> &'static str;
    fn collection(&self) -> &'static str;
}