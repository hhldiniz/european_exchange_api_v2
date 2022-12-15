use async_trait::async_trait;
use crate::data::data_source::base_data_source::BaseDataSource;
use crate::util::database;

pub struct CacheRemoteDataSource {

}

#[async_trait]
impl BaseDataSource for CacheRemoteDataSource {
    async fn insert<T>(&self, models: Vec<T>) -> Result<Vec<i32>, String>{
        let database_con = database::get_database_con().await;
        match database_con {
            Ok(client) => {
                let database = client.database("some_database").collection(self.collection());
                Ok(vec![2])
            },
            Err(e) => Err(e.to_string())
        }
    }
    async fn delete<T>(&self, models: Vec<T>) -> Result<Vec<i32>, String>{

    }
    async fn update<T>(&self, models: Vec<T>) -> Result<Vec<i32>, String>{

    }
    async fn select_one<T>(&self, ftr: String) -> Result<T, String> {

    }
    async fn select_many<T>(&self, ftr: String) -> Result<Vec<T>, String>{

    }
    fn schema(&self) -> &'static str {
        ""
    }
    fn collection(&self) -> &'static str {
        "cache"
    }
}