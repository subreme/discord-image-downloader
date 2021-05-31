# Discord Image Downloader

Although I haven't made a Success Collage in ages *(and haven't designed much in the last months)*, I know that downloading all images from a Discord Server's success channel can be a hassle, so I decided to make a simple tool to give back to the Sneaker Twitter Designer Community.

Similar projects already exist, such as [@tfich](https://github.com/tfich)'s [discord-downloader](https://github.com/tfich/discord-downloader), however not everyone (who doesn't code) has NodeJS installed, and a compiled project might *[slightly]* benefit users with tight deadlines.

The whole reason I wrote this was because I saw this [tweet](https://twitter.com/clippedbypass/status/1398608052442574857) by [Cornelius](https://twitter.com/clippedbypass) yesterday and thought that rewriting [his repository](https://github.com/VX8888/Discord-Picture-Downloader) in Rust could be a good distraction from the homework I didn't want to do.

To be honest, Rust is **not** the most appropriate language to achieve this task, as it probably took me three times as long to write this than it would have if I used JavaScript, however I'm currently learning the language and could use some practice.

## Installation

The ideal way to use this tool is to install a [pre-compiled](https://github.com/subreme/discord-image-downloader/releases) version, as having to build the program yourself defeats the whole purpose of using it instead of the [NodeJS Alternative](https://github.com/tfich/discord-downloader).

Due to this reason, I'll make sure to find someone to compile a MacOS compatible version for me and publish a new [release](https://github.com/subreme/discord-image-downloader/releases) as soon as possible.

If the binaries won't run on your machine, however, you must install [Rust](https://www.rust-lang.org/learn/get-started).

Once Rust is installed, you can clone the repository, either by downloading the [ZIP](https://github.com/subreme/discord-image-downloader/archive/refs/heads/main.zip) and uncompressing the file
or  by running the following command in your terminal, using [GitHub CLI](https://cli.github.com/):

`gh repo clone subreme/password-generator`.

Finally, you can navigate to the directory where you cloned the repo and run it using [Cargo](https://doc.rust-lang.org/cargo/), which should have been installed along with the Rust Language, by using the command `cargo run`, or compiling the project using `cargo build --release`. The generated binaries will be named `discord_image_downloader.exe` and can be found in `discord-image-downloader\target\release`.

## Usage

Once the program is installed, usage should be rather straightforward, as input validation is explained in the prompts and error messages, however I'll also cover it here.

### Bot Token

The first thing the tool will ask for is a valid bot token.

The tool requires the Authorization Token to a Discord Bot that has access to the channel you want to download the images from in order to have the proper Authentication to access Discord's Message API. I'm fairly sure it would be easy to modify the code to use User Access Tokens, which can be obtained using the scripts in my [other repository](https://github.com/subreme/discord-self-xss), however in case of improper usage, this could be identified as "Self-Botting" and lead to your account getting banned, therefore I'm not comfortable with releasing that alternative version.

Bot Tokens can be obtained by going to the [Discord Developer Portal](https://discord.com/developers/applications) and selecting the bot in question, navigating to the `Bot` section under `Settings`, and clicking on the `Copy` button.

Make sure not to share the bot's token with anyone you don't trust, as it can give full control over your bot *(until you generate a new one on the same page where you copied it)*.

### Channel ID

As every element of the Discord app, every channel has a unique numerical ID known as a Snowflake, which can be found in two ways:

- The most user-friendly way is to access Discord through a browser, go to the channel you want to download the images from, and check your URL, which should be formatted as follows:

```none
https://discord.com/channels/@me/123456789012345678/246802468024680246
                                                    ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯
```

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; The number following the final slash, underlined in the code block above, is your channel's ID.

- Alternatively, if you enable `Developer Mode` by navigating to Discord's `User Settings/App Settings/Advanced`, and then right-click on the channel, selecting the `Copy ID` option at the bottom, however if you're reading this section of the instructions you most likely don't have `Developer Mode` enabled.

### Start Date *(Optional)*

*This setting is fully optional, however I decided to include it as other scripts did.*

The program allows the user to select a start date for the messages to check, which might be useful to make sure that only Success Images for a specific release are downloaded.

Although supporting multiple date formats would have been trivial, the tool currently only supports the `DD/MM/YY` format, as it's the format used by the other tools.

I will most likely implement other formats, such as `DD/MM/YYYY`, in the future, as there's no reason for only one standard to be supported.

Since this feature won't be needed by all users, the field can be left blank

### Maximum Image Number *(Optional)*

In order to prevent the program from accidentally downloading too many images and filling the user's storage, the program allows for a Maximum Number of Images to be specified, if necessary.

If `0` or a blank line is returned, no limit will be enforced.

### Image Directory *(Optional)*

By default, the tool saves the downloaded images in the following path: `./Discord Images`, creating a new folder in the same directory where the binaries are located.

Alternatively, a custom path can be selected by inputting it when prompted.

### Exit

Exiting the program, as explained in the console, is as simple as hitting `Enter` once the downloads are complete.

## Notes

### Image Names

When saved, all images are currently named using their corresponding `Message ID`, ensuring that each name is unique *(to avoid an image being overwritten)* and allowing for them to be sorted chronologically by default.

I'm considering adding the option to select an alternative naming format, so any suggestions in that regard are welcome.

### Input Validation

~~The script currently doesn't fully validate the selected parameters until it attempts to download the images, therefore it might quit unexpectedly if incorrect information is provided.~~

*Update: The program now sends requests to Discord's API and uses the Response Status to determine if the Bot Token is valid and if it can access the selected Channel.*

I will make sure to improve upon this soon to ensure an ideal user experience.

## Credits

Special thanks to:

- [tfich](https://github.com/tfich), as I essentially copied [his NodeJS project](https://github.com/tfich/discord-downloader) and used the same logic
- [Cornelius](https://github.com/VX8888) for sharing the [Python script](https://github.com/VX8888/Discord-Picture-Downloader) that convinced me to make this
- [Notifees](https://twitter.com/notifees) for being a great rubber ducky and pretty cute, ngl
