use api::db::users;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // users::insert_user(
    //     String::from("Artsem"),
    //     String::from("Artsem@gmail.com"),
    //     String::from("Artem'as secret"),
    // )
    // .await?;

    let users = users::get_users().await?;
    println!("{:#?}", users);

    Ok(())
}
