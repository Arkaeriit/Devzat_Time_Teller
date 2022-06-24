mod tz;

use devzat_rs;

use tokio::try_join;

const LOGIN_MSG_ROOM: &str = "#main";
const LOGIN_MSG_TARGET: &str = "Arkaeriit";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", tz::time_at_tz("GB").expect("nono"));

    let client = devzat_rs::Client::new(
        "https://localhost:5556",
        "dvz@Xsi67V8WLyMQ/uyF5Uk+UmeoCu8sVX9xHH2GZ/D2SwM=", // Token valid on a local instance, no need to try to use it on the main Devzat server.
    ).await?;

    println!("Hello, world!");

    login_notify(&client, "Time-teller", "Hi!", LOGIN_MSG_ROOM, LOGIN_MSG_TARGET).await;

    Ok(())

}

/// Try to tell a message to the room login_msg_room. If this fails, try to
/// send a message to login_msg_target on #main. If this fails, give up.
async fn login_notify(client: &devzat_rs::Client ,name: &str, msg: &str, login_msg_room: &str, login_msg_target: &str) {
    match client.send_message( login_msg_room.to_string(), Some(name.to_string()), msg.to_string(), None).await {
        Ok(()) => {return},
        Err(_) => {},
    }
    match client.send_message( "#main".to_string(), Some(name.to_string()), msg.to_string(), Some(login_msg_target.to_string())).await {
        Ok(()) => {},
        Err(_) => {},
    }
}

