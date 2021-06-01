use chrono::prelude::*;

pub mod config {
    use {
        super::*,
        reqwest::Response,
        std::{env::current_dir, fs::create_dir_all, io},
    };

    // This collects the config data, calling all config functions
    pub async fn all() -> Config {
        let token = get_token().await;
        let channel = get_channel(&token).await;

        let mut date: u64 = 0;
        let mut quantity: u32 = 0;
        let mut path = default_path();

        // The program won't ask the user to configure the remaining settings if
        // the Default Settings were selected
        if custom_settings() {
            date = get_date();
            quantity = get_quantity();
            path = get_path();
        }

        // The `Config` `struct` is defined at the end of the module
        Config {
            token,
            channel,
            date,
            quantity,
            path,
        }
    }

    // All config functions call this default one to run a specified prompt and
    // return the user's input, before interpreting it themselves
    fn input(prompt: &[&str]) -> String {
        // `input()`'s prompt argument used to be a `&str`, however it was
        // changed to an slice of `&str`s in order to make multiline prompts
        // easier to read when called
        println!("\n{}\n", prompt.join("\n"));

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input!");

        // All inputs are trimmed of starting and ending whitespace in this
        // functions as this would be done in all cases anyway
        choice.trim().to_string()
    }

    // This function allows `get_token()` and `get_channel()` to use Discord-s API to
    // check if their value is valid
    async fn api(token: &str, path: &str) -> Response {
        reqwest::Client::new()
            .get(format!("https://discordapp.com/api/{}", path))
            .header("Authorization", format!("Bot {}", token))
            .send()
            .await
            .expect("Request Failed!")
    }

    async fn get_token() -> String {
        loop {
            let input = input(&["What's your bot's token?"]);

            // If the response's status is "OK", the Bot Token is valid,
            // otherwise it's not, and the user is prompted again
            if api(&input, "gateway/bot").await.status() == 200 {
                break input;
            } else {
                println!("\nInvalid Bot Token!");
                continue;
            }
        }
    }

    async fn get_channel(token: &str) -> String {
        loop {
            let input = input(&[
                "What channel are the images in?",
                "Input the Channel ID, not its name.",
            ]);

            // If the response's status is "OK", the Channel ID is valid and can
            // be accessed using the inputted Bot Token, and if it's not, the
            // user is prompted again
            if api(token, format!("channels/{}", input).as_str())
                .await
                .status()
                == 200
            {
                break input;
            } else {
                println!("\nInvalid Channel ID!");
                println!("The bot can't access this channel!");
                continue;
            }
        }
    }

    // The only reason this function is asynchronous is that, for reasons I
    // can't explain, the program would crash if I didn't
    fn custom_settings() -> bool {
        println!("\nAlthough the program allows for several customizations, most users use the default settings.\n");
        println!("The default settings are:");
        println!("- Download images of any age");
        println!("- Save an unlimited amount of photos");
        println!("- Store pictures in `./Discord Images`");

        loop {
            let input = input(&[
                "Which settings do you want to use?",
                "Write `Default`, `D`, or leave the line empty to use the tool's suggested settings.",
                "Input `Custom` or `C` to edit the settings yourself."
            ]).to_lowercase();

            if input == "default" || input == "d" || input.is_empty() {
                break false;
            } else if input == "custom" || input == "c" {
                break true;
            } else {
                println!("\nInvalid input!");
                continue;
            }
        }
    }

    // While I initially saved the Start Date as an instance of
    // `chrono::DateTime`, I opted for a Snowflake instead, this time saved as a
    // `u64`, so that I could compare its value to the IDs of the collected
    // messages to make sure that they were sent within the specified time range
    fn get_date() -> u64 {
        loop {
            // This is split in two lines as the standrd formatter (rustfmt)
            // doesn't like long lines
            let input = input(&[
                "How far back should we search?",
                "Leave blank to download all images.",
            ])
            .to_lowercase();

            // As dumb as it sounds, `Default` is a recognized value because I
            // liked how it looked in a screenshot of the program's interface I
            // sent a friend while writing it
            if input == "default" || input.is_empty() {
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
                    } else if date < Utc.yo(2015, 1).and_hms(0, 0, 0) {s
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

    fn get_quantity() -> u32 {
        loop {
            // While I normally separate multiline messages into more than
            // one `println!()`, I thought it was simpler to only include one
            // `&str` as the `prompt` parameter rather than a tuple or vector
            let input = input(&[
                "How many images should be downloaded at most?",
                "Leave blank for an unlimited amount.",
            ])
            .to_lowercase();

            // The checks here are fairly similar to the previous ones and
            // rather self-explanatory
            match input.parse::<u32>() {
                Ok(num) => break num,
                Err(_) => {
                    if input == "default" || input.is_empty() {
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
    fn get_path() -> String {
        loop {
            let input = input(&[
                "Where should the images be saved?",
                "Leave blank to use the default path.",
            ]);

            if input.to_lowercase() == "default" || input.is_empty() {
                break default_path();
            } else {
                match create_dir_all(&input) {
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

    fn default_path() -> String {
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

        create_dir_all(&path).expect("Failed to create directory!");

        path
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
    use {
        super::*,
        serenity::model::channel::Message,
        std::{collections::HashMap, fs::write, path::Path},
    };

    // Jut like in the `config` module, the `all()` function calls its own
    // functions and handles the whole task. I could have simply written two
    // functions, `config()` and `run()`, outside of modules, but I only
    // intended for them to be used, and as far as I know hey wouldn't be able
    // to use private functions from the modules otherwise
    pub async fn all(selected: config::Config) {
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
            let res: Vec<Message> = get(&selected, after).await;

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
                    if images.len() < selected.quantity as usize
                        || selected.quantity == 0 && !images.contains_key(id.as_str())
                    {
                        // Not all messages have attatchments, but not all attatchments are
                        // images either, so each one must be checked
                        for att in msg.attachments {
                            // This checks that the attatchment is an image by checking
                            // if a `width` property is specified
                            if att.width.is_some() {
                                // If it is, the image's url is accessed and the file is
                                // saved using the `save()` function, defined below
                                let url = att.url;
                                save(&url, path).await;

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
                    } else {
                        break;
                    }
                }
            }
        }

        // Once all images have been downloaded, the number found is dispayed

        // Conditional statements are used to customize the final message
        if images.is_empty() {
            println!(
                "The channel doesn't contain any images{}!",
                if selected.date == 0 {
                    ""
                } else {
                    " in the selected time range"
                }
            );
        } else {
            println!(
                "\nSuccessfully saved {} image{}!",
                images.len(),
                if images.len() == 1 { "" } else { "s" }
            );
        }
    }

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

    async fn save(url: &str, path: &Path) {
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

    // This function allows for unrecoverable errors to be displayed to the user
    // instead of quitting the program with no explanation
    /*
    fn error(message: &str) {
        println!("{}", message);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        exit(0);
    }
    */
}
