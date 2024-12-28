
#[tokio::main]
async fn main() {
    pretty_env_logger::init_custom_env("SZFIT_LOG");

    log::info!("Starting command bot...");

    let db = szfit_domain::store::new_db_pool().await.unwrap();

     let db_bot = db.clone();
     tokio::spawn(async move {
         telegram_bot::TelegramBotService::new(db_bot)
             .start()
             .await
             .unwrap();

     });


    web::serve(db).await
}
