use chrono::prelude::*;

pub mod config {
    use super::*;

    use std::{env::current_dir, fs, io};

    // This collects the config data, calling all config functions
    pub fn all() -> Config {
        // The `Config` `struct` is defined at the end of the module
        Config {
            token: token(),
            channel: channel(),
            date: date(),
            quantity: quantity(),
            path: path(),
        }
    }

    // All config functions call this default one to run a specified prompt and
    // return the user's input, before interpreting it themselves
    fn input(prompt: &str) -> String {
        println!("\n{}\n", prompt);

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input!");

        // All inputs are trimmed of starting and ending whitespace in this
        // functions as this would be done in all cases anyway
        choice.trim().to_string()
    }

    fn token() -> String {
        loop {
            let input = input("What's your bot's token?");

            // Bot Tokens are definitely longer than 10 characters, so this will
            // be a sloppy validation before I implement an actual check of the
            // access token being real
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

            // All Channel IDs are Snowflakes, so they should be able to parse
            // to an integer if they are valid
            match input.parse::<u64>() {
                // They are then saved as `String`s anyway, as there's no need
                // for them to be `u64`s if they're simply part of a URL
                Ok(_) => break input,
                Err(_) => {
                    println!("\nInvalid input!");
                    println!("Discord Channel IDs only contain numerical characters.");
                    continue;
                }
            }
        }
    }

    // While I initially saved the Start Date as an instance of
    // `chrono::DateTime`, I opted for a Snowflake instead, this time saved as a
    // `u64`, so that I could compare its value to the IDs of the collected
    // messages to make sure that they were sent within the specified time range
    fn date() -> u64 {
        loop {
            // This is split in two lines as the standrd formatter (rustfmt)
            // doesn't like long lines
            let input =
                input("How far back should we search?\nLeave blank to download all images.");

            // As dumb as it sounds, `Default` is a recognized value because I
            // liked how it looked in a screenshot of the program's interface I
            // sent a friend while writing it
            if input.is_empty() || input.to_lowercase() == "default" {
                // Zero is used to represent no limit, as Rust doesn't have
                // `null` values and I thought that using `Option<u64>::None`
                // was unnecessary
                break 0;
            } else {
                // The slash here is represented as a `char` instead of a `&str`
                // as our Lord and Savior Clippy said that single characters
                // should be saved that way and I do as it says
                let date: Vec<&str> = input.split('/').collect();

                // Checking the length of the input and making sure that it
                // contains two slashes works well enough to validate the date
                if date.len() == 3 && input.len() == 8 | 10 {
                    let day: u32 = match date[0].parse() {
                        Ok(num) => {
                            // No month has less than 1 day or more than 31, but
                            // I'm too lazy to check the exact range for each
                            // month, so please use real dates or the program
                            // will quit and piss you off
                            if num > 0 && num < 32 {
                                num
                            } else {
                                println!("\nNo month has that many days!");
                                continue;
                            }
                        }

                        // And there's obviously something wrong with the date
                        // if it can't even parse to an integer
                        Err(_) => {
                            println!("\nInvalid Day Input!");
                            println!("Please use the following format: `DD/MM/YY`!");
                            continue;
                        }
                    };

                    // Same goes for the other... "time units"?
                    let month: u32 = match date[1].parse() {
                        Ok(num) => {
                            if num > 0 && num < 13 {
                                num
                            } else {
                                println!("\nThere aren't that many months!");
                                continue;
                            }
                        }
                        Err(_) => {
                            println!("\nInvalid Month Input!");
                            println!("Please use the following format: `DD/MM/YY`!");
                            continue;
                        }
                    };

                    // This check allows the program to support the
                    // `DD/MM/YYYY`datr format as well as `DD/MM/YY`
                    let year = if input.len() == 8 {
                        format!("20{}", date[2]).parse()
                    } else {
                        date[2].parse()
                    };

                    let year: i32 = match year {
                        Ok(num) => {
                            if num > 2014 {
                                num
                            } else {
                                println!("\nDiscord didn't even exist at the time!");
                                println!("Type `Default`, `0`, or nothing, if you don't wont to select a time range.");
                                continue;
                            }
                        }
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

                    // I might as well remove this check since it's already
                    // validated above
                    /*
                    } else if date < Utc.yo(2015, 1).and_hms(0, 0, 0) {
                        println!("Discord didn't even exist at the time!");
                        println!("If you want to include all messages, simply click `Enter`.");
                        continue;
                    */
                    } else {
                        // This converts the UNIX timestamp to a Snowflake
                        break (date.timestamp_millis() as u64 - 1420070400000) << 22;
                    }
                } else {
                    println!("Invalid input! Please write a date as `DD/MM/YY`.");
                    continue;
                }
            }
        }
    }

    fn quantity() -> u32 {
        loop {
            // While I normally separate multiple-line messages into more than
            // one `println!()`, I thought it was simpler to only include one
            // `&str` as the `prompt` parameter rather than a tuple or vector
            let input = input(
                "How many images should be downloaded at most?\nLeave blank for an unlimited amount.",
            );

            // The checks here are fairly similar to the previous ones and
            // rather self-explanatory
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

    // Returning `&'static Path`s gave me so many issues I ended up saving the
    // selected path as a `String`, since Rust lifetimes can be great but are
    // definitely a double-edged sword
    fn path() -> String {
        loop {
            let input =
                input("Where should the images be saved?\nLeave blank to use the default path.");

            if input.is_empty() || input.to_lowercase() == "default" {
                // This was the most obvious way to create a folder in the
                // current directory, and I'll probably keep it this way as it
                // works and I'm too dumb to figure out a more idiomatic method
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
                        println!("Selected Filepath might be invalid, please use the following format: `foo/bar`.");
                        continue;
                    }
                }
            }
        }
    }

    // The resoning behind the types used in the `struct` were all mentioned in
    // the functions that generate them, so I won't repeat them
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

    use serenity::model::channel::Message;
    use std::{collections::HashMap, fs::write, path::Path};

    // Jut like in the `config` module, the `all()` function calls its own
    // functions and handles the whole task. I could have simply written two
    // functions, `config()` and `run()`, outside of modules, but I only
    // intended for them to be used, and as far as I know hey wouldn't be able
    // to use private functions from the modules otherwise
    pub fn all(selected: config::Config) {
        // This prints an empty sline to separate he download messages from the
        // user's last input, once again for cosmetic reasons (this is a tool
        // for Sneaker Twitter Designers, after all)
        println!();

        // Since the path was saved as a String, as I explained earlier, the
        // real path has to be created here
        let path = Path::new(&selected.path);

        // This `HashMap` is used to keep track of the number of images
        // downloaded and make sure they don't exceed the specified limit, while
        // also keeping track of the amount of times each one is downloaded,
        // ensuring that images aren't downloaded more than once
        let mut images: HashMap<String, u32> = HashMap::new();

        // `after` is initialized as the Start Date, as all images should have
        // be sent after it
        let mut after = selected.date;

        loop {
            // After getting migraines due to my completely unnecessary efforts to
            // serialize Discord's Message API JSON responses, I decided to simply
            // use the ones defined in the `serenity` crate, importing their
            // `Message` struct (and making me cry for wasting so much time)
            let res: Vec<Message> = get(&selected, after);

            // Once all messages are requested, there will be no new ones and
            // the program will be done
            if res.is_empty() {
                break;
            } else {
                // The first message returned by the API is the most recent one,
                // so after is updated to ts ID so that the next requests only
                // includes messages sent after it
                after = format!("{}", res.first().expect("Failed to find last message!").id)
                    .parse()
                    .expect("Failed to parse Message ID!");

                // Since the API's response is simply an array of messages, I iterate
                // through each one
                for msg in res {
                    // I immediately extract the Message ID so that I can check
                    // if the image has been downloaded already
                    let id = format!("{}", msg.id);

                    // The program only continues if the image limit hasn't been
                    // reached and the image hasn't been previously downloaded
                    if images.len() > selected.quantity as usize
                        || selected.quantity != 0 && images.contains_key(id.as_str())
                    {
                        break;
                    } else {
                        // Not all messages have attatchments, but not all attatchments are
                        // images either, so each one must be checked
                        for att in msg.attachments {
                            // This checks that the attatchment is an image by checking
                            // if a `width` property is specified
                            if att.width.is_some() {
                                // If it is, the image's url is accessed and the file is
                                // saved using the `save()` function, defined below
                                let url = att.url.to_string();
                                save(&url, path);

                                // The image's Message ID is added to `images`
                                // if it isn't part of it already
                                /*
                                let count =
                                */
                                images.entry(format!("{}", msg.id)).or_insert(0);

                                // The number of downloads could be checked in
                                // the future to verify if an image has indeed
                                // only been saved once, however that
                                // functionality hasn't been implemented yet
                                /*
                                 *count += 1;
                                 */
                            }
                        }
                    }
                }
            }
        }

        // Once all images have been downloaded, the number found is dispayed
        println!("\nSuccessfully saved {} images!", images.len());
    }

    #[tokio::main]
    async fn get(selected: &config::Config, after: u64) -> Vec<Message> {
        // The API is extremely simple, as shown below
        let mut url = format!(
            "https://discordapp.com/api/channels/{}/messages?limit=100",
            selected.channel
        );

        if after > 0 {
            url = format!("{}&after={}", url, after);
        }

        // The authorization in the API is as basic as adding a header with the
        // Bot Token, formatted this way
        let auth = format!("Bot {}", selected.token);
        reqwest::Client::new()
            .get(url)
            .header("Authorization", auth)
            .send()
            .await
            .expect("Failed to send GET Request!")
            // The response is serialized as a `Vec<Message>` as explained
            // in the `all()` function
            .json::<Vec<Message>>()
            .await
            .expect("Failed to parse Response!")
    }

    #[tokio::main]
    async fn save(url: &String, path: &Path) {
        // Although the Message ID is specified in the `Message` `struct`, it's
        // easier to extract it from the Image URL
        let name = url.split('/').nth(5).expect("Failed to get image name!");

        // The filetype is also extracted from the URL
        let ext = url
            .split('.')
            .last()
            .expect("Failed to get image filetype!");

        // The image's file name is formed using those two values
        let path = path.join(format!("{}.{}", name, ext));

        // The image is then downloaded by reqeuesting its URL
        let img = reqwest::get(url)
            .await
            .expect("Failed to GET image!")
            // It's then converted to bytes so it can be written on the new file
            .bytes()
            .await
            .expect("Gailed to convert image to bytes!");

        // And saved to storage
        write(path, img).expect("Failed to save image!");

        // The task's completion is then logged to display the program's
        // progress and showcase it's speed
        println!("Saved {}.{}!", name, ext);
    }
}
