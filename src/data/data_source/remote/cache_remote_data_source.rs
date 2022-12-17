use async_trait::async_trait;
use mongodb::{Database};
use serde::Serialize;
use crate::data::{data_source::base_data_source::BaseDataSource, model::base_model::BaseModel};

pub struct CacheRemoteDataSource {

}

#[async_trait]
impl BaseDataSource for CacheRemoteDataSource {
    
    async fn insert<T: BaseModel + Serialize>(&self, models: Vec<T>, database: &Database) -> Result<(), String>{
        let collection = database.collection(self.collection());
        match(collection.insert_many(models, None)).await {
            Ok(result) => Ok(()),
            Err(e) =>  Err(e.to_string())
        }
    }
    async fn delete<T: BaseModel + Serialize>(&self, models: Vec<T>, database: &Database) -> Result<(), String>{
        
    }
    async fn update<T: BaseModel + Serialize>(&self, models: Vec<T>, database: &Database) -> Result<(), String>{
        
    }
    async fn select_one<T: BaseModel + Serialize>(&self, ftr: String, database: &Database) -> Result<T, String> {
        
    }
    async fn select_many<T: BaseModel + Serialize>(&self, ftr: String, database: &Database) -> Result<Vec<T>, String>{
        
    }
    fn schema(&self) -> &'static str {
        ""
    }
    fn collection(&self) -> &'static str {
        "cache"
    }
}