use std::collections::HashMap;
use std::env;
use std::process;
use std::process::Command;

// The program depends on env variables being set by PAM to generate logs

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //  Enter details:
    let server = "";
    let access_token = "";
    let room_id = "";

    let status = env::var("PAM_TYPE").unwrap();
    if status.trim() != "open_session" {
        process::exit(0);
    }

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
    let host = Command::new("/bin/hostname")
        .arg("--fqdn")
        .output()
        .expect("couldn't get date, at /bin/hostname");

    let service = env::var("PAM_SERVICE").expect("Cant read PAM_SERVICE");
    let user: String = env::var("PAM_USER").expect("Can't read PAM_USER");
    let r_host: String = env::var("PAM_RHOST").expect("Can't read PAM_RHOST");
    let tty: String = env::var("PAM_TTY").expect("Can't read PAM_TTY");

    let message: String = format!(
        "{} login on {} for account {}\nUser: {}\nRemote Host: {}\nTTY: {}\nDate: {}\nServer: {}",
        service.trim(),
        String::from_utf8_lossy(&host.stdout).trim(),
        &user,
        &user,
        &r_host,
        &tty,
        String::from_utf8_lossy(&date.stdout),
        String::from_utf8_lossy(&sys_info.stdout),
    );
    let formatted_message: String = format!(
        "<strong>{}: {} login</strong>
        <ul>
            <li>
                <strong>User:</strong> {}
            </li>
            <li>
                <strong>Remote Host:</strong> {}
            </li>
            <li>
                <strong>TTY:</strong> {}
            </li>
            <li>
                <strong>Date:</strong> {}
            </li>
            <li>
                <strong>Server:</strong> {}
            </li>
        </ul>",
        String::from_utf8_lossy(&host.stdout).trim(),
        service.trim(),
        &user,
        &r_host,
        &tty,
        String::from_utf8_lossy(&date.stdout),
        String::from_utf8_lossy(&sys_info.stdout),
    );

    let mut map = HashMap::new();
    map.insert("msgtype", "m.text");
    map.insert("body", &message);
    map.insert("formatted_body", &formatted_message);
    map.insert("format", "org.matrix.custom.html");

    let client = reqwest::Client::new();
    client.post(&url).json(&map).send().await?;
    Ok(())
}
