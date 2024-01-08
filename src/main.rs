mod tz;
mod qalc;

use devzat_rs;
use tokio::try_join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let instance_host = match std::env::var("PLUGIN_HOST") {
        Ok(host) => host,
        Err(_) => "https://devzat.hackclub.com:5556".to_string(),
    };

    let auth_token = match std::env::var("PLUGIN_TOKEN") {
        Ok(token) => token,
        Err(_) => panic!("Missing PLUGIN_TOKEN"),
    };

    let login_room = match std::env::var("LOGIN_ROOM") {
        Ok(room) => room,
        Err(_) => "#bots".to_string(),
    };

    let dev_nick = match std::env::var("DEV_NICK") {
        Ok(nick) => nick,
        Err(_) => "Arkaeriit".to_string(),
    };

    let client = devzat_rs::Client::new(
        instance_host,
        auth_token,
    ).await?;

    login_notify(&client, &get_bot_name(), "Hi! I just logged in.", &login_room, &dev_nick).await;

   let time_at_cmd = client.register_cmd("time_at", "Tell the time at a given timezone.", "<time zone>", |event| async move {
       let name = get_bot_name();
       match tz::time_at_tz(&event.args) {
           Some(time) => format!("{}: At the timezone {}, it is {}.", name, &event.args, time),
           None => format!("{}: Error, {} is not a valid time zone.", name, &event.args),
       }
    });

   let rpn_cmd = client.register_cmd("rpn", "Perform a computation with a RPN calculator.", "<computation>", |event| async move {
       qalc::rpn_calc(&event.args)
   });

   let calc_cmd = client.register_cmd("calc", "Perform a computation with an infix calculator.", "<computation>", |event| async move {
       qalc::infix_calc(&event.args)
   });

   let _ = try_join!(time_at_cmd, rpn_cmd, calc_cmd);

    Ok(())

}

/// Get the name of the bot.
fn get_bot_name() -> String {
    match std::env::var("BOT_NAME") {
        Ok(name) => name,
        Err(_) => "Time-teller".to_string(),
    }
}

/// Try to tell a message to the room login_msg_room. then, try to
/// send a message to login_msg_target on #main. If this fails, give up.
async fn login_notify(client: &devzat_rs::Client ,name: &str, msg: &str, login_msg_room: &str, login_msg_target: &str) {
    match client.send_message( login_msg_room.to_string(), Some(name.to_string()), msg.to_string(), None).await {
        Ok(()) => {},
        Err(_) => {},
    }
    match client.send_message( "#main".to_string(), Some(name.to_string()), msg.to_string(), Some(login_msg_target.to_string())).await {
        Ok(()) => {},
        Err(_) => {},
    }
}

