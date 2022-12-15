use async_trait::async_trait;

#[async_trait]
pub trait BaseDataSource {
    async fn insert<T>(&self, models: Vec<T>) -> Result<Vec<i32>, String>;
    async fn delete<T>(&self, models: Vec<T>) -> Result<Vec<i32>, String>;
    async fn update<T>(&self, models: Vec<T>) -> Result<Vec<i32>, String>;
    async fn select_one<T>(&self, ftr: String) -> Result<T, String>;
    async fn select_many<T>(&self, ftr: String) -> Result<Vec<T>, String>;
    fn schema(&self) -> &'static str;
    fn collection(&self) -> &'static str;
}