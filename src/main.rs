use std::env;
use std::time::Duration;

use dotenv;
use futures::StreamExt;
use telegram_bot::*;
use tokio::timer::delay_for;

const DELAY_DURATION: Duration = Duration::from_secs(2);

async fn test(api: Api, message: Message) -> Result<(), Error> {
    let reply = api
        .send(
            message
                .location_reply(52.007721756068385, 4.706305009461701)
                .live_period(60),
        )
        .await?;

    delay_for(DELAY_DURATION).await;
    api.send(reply.edit_live_location(10.0, 10.0)).await?;

    delay_for(DELAY_DURATION).await;
    api.send(reply.edit_live_location(20.0, 20.0)).await?;

    delay_for(DELAY_DURATION).await;
    api.send(reply.edit_live_location(30.0, 30.0)).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);
    let mut stream = api.stream();

    println!("start");

    while let Some(update) = stream.next().await {
        let update = update?;

        match update.kind {
            UpdateKind::Message(message) => match message.kind {
                MessageKind::Text { ref data, .. } => match data.as_str() {
                    "/livelocation" => test(api.clone(), message.clone()).await?,
                    _ => println!("Text:{:?}", data),
                },
                MessageKind::Location { ref data, .. } => {
                    println!("long:{}, lat:{}", data.longitude, data.latitude);
                }
                _ => println!("Text {:?}", message.kind),
            },
            UpdateKind::EditedMessage(ref msg) => match msg.kind {
                MessageKind::Location { ref data, .. } => {
                    println!(
                        "Update long:{}, lat:{}, date:{}, user:{}",
                        data.longitude, data.latitude, msg.date, msg.from.first_name
                    );
                }
                _ => println!("Update {:?}", msg.kind),
            },
            _ => println!("other {:?}", update.kind),
        }
    }
    Ok(())
}
