use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::process;
use std::process::Command;

use reqwest::ClientBuilder;

// The program depends on env variables being set by PAM to generate logs

const USER_AGENT: &str = "ids-matrix-bot";

fn prep_message() -> HashMap<String, String> {
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
    map.insert("msgtype".into(), "m.text".into());
    map.insert("body".into(), message);
    map.insert("formatted_body".into(), formatted_message);
    map.insert("format".into(), "org.matrix.custom.html".into());
    map
}

fn get_url() -> String {
    //    let server = env::var("SERVER").unwrap();
    //    let access_token = env::var("ACCESS_TOKEN").unwrap();
    //    let room_id = env::var("ROOM_ID").unwrap();

    let server = env!("SERVER");
    let access_token = env!("ACCESS_TOKEN");
    let room_id = env!("ROOM_ID");

    let url = format!(
        "{}/_matrix/client/r0/rooms/{}/send/m.room.message?access_token={}",
        server, room_id, access_token
    );

    url
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let status = env::var("PAM_TYPE").unwrap();

    if status.trim() != "open_session" {
        let date = Command::new("/bin/date")
            .output()
            .expect("couldn't get date, at /bin/date");
        let date = String::from_utf8_lossy(&date.stdout);
        eprintln!("[{}] PAM_TYPE: {}", &date[0..date.len() - 1], status);
        process::exit(0);
    }

    let client = ClientBuilder::default()
        .user_agent(crate::USER_AGENT)
        .use_rustls_tls()
        .build()
        .unwrap();

    client.post(&get_url()).json(&prep_message()).send().await?;
    Ok(())
}
