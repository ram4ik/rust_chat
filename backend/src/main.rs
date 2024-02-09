use rocket::futures::{SinkExt, StreamExt};
use rocket_ws::{WebSocket, Channel};

#[rocket::get("/")]
fn chat(ws: WebSocket) -> Channel<'static> {
    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(mesage) = stream.next().await {
            let _ = stream.send(mesage?).await;
        }
        Ok(())
    }))
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            chat
        ])
        .launch()
        .await;
}
