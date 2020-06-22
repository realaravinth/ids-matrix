use std::collections::HashMap;
use std::env;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //  Enter details:

    let server = "";
    let access_token = "";
    let room_id = "";

    let url = format!(
        "{}/_matrix/client/r0/rooms/{}/send/m.room.message?access_token={}",
        server, room_id, access_token
    );

    let date = Command::new("/bin/date")
        .output()
        .expect("couldn't get date, at /bin/date");

    let sys_info = Command::new("/bin/uname")
        .arg("-a")
        .output()
        .expect("Error at /bin/uname");

    let hostname_fqdn = Command::new("/bin/hostname")
        .arg("--fqdn")
        .output()
        .expect("couldn't get date, at /bin/hostname");

    let args: Vec<String> = env::args().collect();
    let message: String = format!(
        "{} login on {} for account {}\nUser: {}\nRemote Host: {}\nTTY: {}\nDate: {}Server: {}",
        &args[1],
        String::from_utf8_lossy(&hostname_fqdn.stdout),
        &args[2],
        &args[2],
        &args[3],
        &args[4],
        String::from_utf8_lossy(&date.stdout),
        String::from_utf8_lossy(&sys_info.stdout),
    );

    let mut map = HashMap::new();
    map.insert("msgtype", "m.text");
    map.insert("body", &message);

    let client = reqwest::Client::new();
    client.post(&url).json(&map).send().await?;
    Ok(())
}
