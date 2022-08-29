// use api::app_config::AppConfig;
use api::db::sqlite_client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let config = AppConfig::get().expect("something went wrong");
    // println!("{:#?}", config)
    let client = Client::new().await?;
    // let result = client
    //     .insert_user(
    //         String::from("Mike"),
    //         String::from("mike@mail.com"),
    //         String::from("mike's secret"),
    //     )
    //     .await?;

    let finded_user = client.get_user_by_id(2).await?;

    println!("{:#?}", finded_user);

    Ok(())
}
