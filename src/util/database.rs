use mongodb::bson::doc;
use mongodb::Client;
use mongodb::options::ClientOptions;

pub async fn get_database_con() ->  mongodb::error::Result<Client> {
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb+srv://<username>:<password>@<cluster-url>/test?w=majority")
            .await?;
    // Manually set an option
    client_options.app_name = Some("European Exchange Api V2".to_string());
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");
    Ok(client)
}