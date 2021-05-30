use chrono::prelude::*;

pub mod config {
    use super::*;
    use std::{env::current_dir, fs, io};

    pub fn all() -> Config {
        Config {
            token: token(),
            channel: channel(),
            date: date(),
            quantity: quantity(),
            path: path(),
        }
    }

    fn input(prompt: &str) -> String {
        println!("\n{}\n", prompt);

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input!");

        choice.trim().to_string()
    }

    fn token() -> String {
        loop {
            let input = input("What's your bot's token?");
            if input.len() > 10 {
                break input;
            } else {
                println!("\nNo bot token is that short!");
                continue;
            }
        }
    }

    fn channel() -> String {
        loop {
            let input = input("What channel are the images in?");
            match input.parse::<u64>() {
                Ok(_) => break input,
                Err(_) => {
                    println!("\nInvalid input!");
                    println!("Discord Channel IDs only contain numerical characters.");
                    continue;
                }
            }
        }
    }

    fn date() -> u64 {
        loop {
            let input =
                input("How far back should we search?\nLeave blank to download all images.");
            if input.is_empty() || input.to_lowercase() == "default" {
                break 0;
            } else {
                let date: Vec<&str> = input.split("/").collect();
                if input.len() == 8 && date.len() == 3 {
                    let day: u32 = match date[0].parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("\nInvalid Day Input!");
                            println!("Please use the following format: `DD/MM/YY`!");
                            continue;
                        }
                    };
                    let month: u32 = match date[1].parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("\nInvalid Month Input!");
                            println!("Please use the following format: `DD/MM/YY`!");
                            continue;
                        }
                    };
                    let year: i32 = match format!("20{}", date[2]).parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("\nInvalid Year Input!");
                            println!("Please use the following format: `DD/MM/YY`!");
                            continue;
                        }
                    };
                    let date = Utc.ymd(year, month, day).and_hms(0, 0, 0);
                    if date > Utc::now() {
                        println!("You can't select a future date!");
                        continue;
                    } else if date < Utc.yo(2015, 1).and_hms(0, 0, 0) {
                        println!("Discord didn't even exist at the time!");
                        println!("If you want to include all messages, simply click `Enter`.");
                        continue;
                    } else {
                        break (date.timestamp_millis() as u64 - 1420070400000) << 22;
                    }
                } else {
                    println!("Invalid format! Please write a date as `DD/MM/YY`.");
                    continue;
                }
            }
        }
    }

    fn quantity() -> u32 {
        loop {
            let input = input(
                "How many images should be downloaded at most?\nLeave blank for an unlimited amount.",
            );
            match input.parse::<u32>() {
                Ok(num) => break num,
                Err(_) => {
                    if input.is_empty() || input.to_lowercase() == "default" {
                        break 0;
                    } else {
                        println!("\nInvalid input!");
                        println!("Make sure to either select a positive integer or to hit `Enter` immediately.");
                        continue;
                    }
                }
            }
        }
    }

    fn path() -> String {
        loop {
            let input =
                input("Where should the images be saved?\nLeave blank to use the default path.");
            if input.is_empty() || input.to_lowercase() == "default" {
                let path = format!(
                    "{}/Discord Images",
                    current_dir()
                        .expect("Failed to get current directory!")
                        .to_str()
                        .expect("Failed to get current directory!")
                );
                fs::create_dir_all(&path).expect("Failed to create directory!");
                break path;
            } else {
                match fs::create_dir_all(&input) {
                    Ok(_) => break input,
                    Err(_) => {
                        println!("\nFailed to create directory!");
                        println!("Selected Filepath might be invalid, please use the following format: `/foo/bar`.");
                        continue;
                    }
                }
            }
        }
    }

    pub struct Config {
        pub token: String,
        pub channel: String,
        pub date: u64,
        pub quantity: u32,
        pub path: String,
    }
}

pub mod run {
    use super::*;

    use reqwest;
    use serenity::model::channel::Message;
    use std::{fs::write, path::Path};

    pub fn all(selected: config::Config) {
        println!("");
        let res: Vec<Message> = get(&selected);
        let path = Path::new(&selected.path);
        let mut images: u32 = 0;
        for i in 0..res.len() {
            for j in 0..res[i].attachments.len() {
                let att = &res[i].attachments[j];
                if !att.width.is_none() {
                    save(&att.url, path);
                    images += 1;
                }
            }
        }
        println!("\nSuccessfully saved {} images!", images);
    }

    #[tokio::main]
    async fn get(selected: &config::Config) -> Vec<Message> {
        let mut url = format!(
            "https://discordapp.com/api/channels/{}/messages",
            selected.channel
        );
        if selected.date > 0 {
            url.push_str(format!("?after={}", selected.date).as_str());
        }
        let auth = format!("Bot {}", selected.token);
        reqwest::Client::new()
            .get(url)
            .header("Authorization", auth)
            .send()
            .await
            .expect("Failed to send GET Request!")
            .json::<Vec<Message>>()
            .await
            .expect("Failed to parse Response!")
    }

    #[tokio::main]
    async fn save(url: &String, path: &Path) {
        let name = url.split("/").nth(5).expect("Failed to get image name!");
        let ext = url
            .split(".")
            .last()
            .expect("Failed to get image filetype!");
        let path = path.join(format!("{}.{}", name, ext));
        let img = reqwest::get(url)
            .await
            .expect("Failed to GET image!")
            .bytes()
            .await
            .expect("Gailed to convert image to bytes!");
        write(path, img).expect("Failed to save image!");
        println!("Saved {}.{}!", name, ext);
    }
}
