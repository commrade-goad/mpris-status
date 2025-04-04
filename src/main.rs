use mpris::{PlaybackStatus, PlayerFinder};

enum ProgramMode {
    STATUS,
    METADATA,
}

const MAX_TITLE_CHAR:usize = 70;
const MAX_ARTIST_CHAR: usize = 25;

fn connect() -> Result<mpris::Player, ()> {
    let player = match PlayerFinder::new() {
        Err(_) => return Err(()),
        Ok(v) => v,
    }
    .find_active();
    if let Ok(player) = player {
        return Ok(player);
    } else {
        return Err(());
    }
}

fn get_metadata(player_name: &mpris::Player) -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    let metadata = match player_name.get_metadata() {
        Err(_) => {
            data.push("Not available".to_string()); // title will be `Not available`
            data.push("None".to_string()); // artist will be `None`
            return data;
        }
        Ok(v) => v,
    };
    if let Some(mpris::MetadataValue::String(title)) = metadata.get("xesam:title") {
        let title_len: usize = title.len();
        if title_len >= MAX_TITLE_CHAR {
            let mut title_cp: String = title
                .chars()
                .take(MAX_TITLE_CHAR - 1 - 3)
                .collect();
            title_cp.push_str("...");
            data.push(title_cp);

        } else {
            data.push(title.to_owned());
        }
    } else {
        data.push("Not available".to_string());
    };
    if let Some(mpris::MetadataValue::Array(artist)) = metadata.get("xesam:artist") {
        if artist.len() > 1 {
            let mut data_to_push: String = String::new();
            for n in 0..artist.len() {
                if let Some(mpris::MetadataValue::String(artist_str)) = artist.get(n) {
                    match n {
                        0 => data_to_push.push_str(&artist_str.to_string()),
                        _ => data_to_push.push_str(&format!(", {artist_str}")),
                    };
                } else {
                    data.push("None".to_string());
                }
            }
        } else {
            if let Some(mpris::MetadataValue::String(artist_str)) = artist.get(0) {
                data.push(artist_str.to_string());
            } else {
                data.push("None".to_string());
            }
        }
        if data[1].len() > MAX_ARTIST_CHAR {
            let mut artist_cp : String = data[1].clone()
                .chars()
                .take(MAX_ARTIST_CHAR - 1 - 3)
                .collect();
            artist_cp.push_str("...");
            data[1] = artist_cp;
        }
    };
    if data.len() < 2 {
        data.push("None".to_string());
    } else if data[1].chars().count() < 1 {
        data[1].push_str(&"None");
    }
    return data;
}

fn get_status(player: &mpris::Player) -> Result<(String, PlaybackStatus), ()> {
    let player_name : String = player.bus_name_player_name_part().to_string();
    let player_status= mpris::Player::get_playback_status(&player);
    if let Ok(value) = player_status {
        return Ok((player_name, value));
    } else {
        return Err(());
    };
}

fn get_args() -> Option<ProgramMode> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1{
        return None;
    }
    let mut return_value: ProgramMode = ProgramMode::METADATA;
    let mut counter: usize = 1;
    while counter < args.len(){
        match &args[counter][..]{
            "status" | "s" => {
                counter += 1;
                return_value = ProgramMode::STATUS;
            }
            "metadata" | "m" => {
                counter += 1;
                return_value = ProgramMode::METADATA;
            }
            _ => {
                return None;
            }
        }
        counter += 1;
    }
    return Some(return_value);
}

fn main() {
    let args: Option<ProgramMode> = get_args();
    let player = connect();
    match player {
        Err(_) => {
            // println!("No player available.");
            // eprintln!("ERR : Cant get the player!");
            std::process::exit(1);
        }
        Ok(v) => {
            match args {
                Some(ProgramMode::STATUS) => {
                    match get_status(&v) {
                        Ok((s, PlaybackStatus::Paused)) => {
                            println!("{} Currently Paused", s.to_uppercase());
                        }
                        Ok((s,PlaybackStatus::Playing)) => {
                            println!("{} Currently Playing", s.to_uppercase());
                        }
                        Ok((s,PlaybackStatus::Stopped)) => {
                            println!("{} Currently Stopped", s.to_uppercase());
                        }
                        Err(()) => {
                            eprintln!("ERR : Cant get the player current status");
                            std::process::exit(2);
                        }
                    };
                }
                Some(ProgramMode::METADATA) => {
                    let metadata: Vec<String> = get_metadata(&v);
                    println!("{} - {}", metadata[0], metadata[1]);
                }
                None => {
                    eprintln!("ERR : Did't get valid args.");
                    std::process::exit(3);
                }
            }
        }
    }
}
