# Devzat Time Teller

On [Devzat](https://github.com/quackduck/devzat), people come from all around the time. It is sometime hard to know what time it is for other peoples. This plugin let you see the current time at any timezone with a command.

## In-chat usage

For the people in the chat-room, this plugin exposes a single command, `time_at`. This command takes as argument an IANA timezone.

Here is an example of in-chat use:

```
Arkaeriit: time_at CET
Time-teller: At the timezone CET, it is 14:29.
Arkaeriit: time_at Europe/Paris
Time-teller: At the timezone Europe/Paris, it is 14:29.
Arkaeriit: time_at NotATimeZone
Time-teller: Error, NotATimeZone is not a valid time zone.
```

## Admin usage

The plugin is made for a single-file executable. It is configured with the following environment variable.

|Variable name |Description                                                   |Default                                                                     |
|--------------|--------------------------------------------------------------|----------------------------------------------------------------------------|
|`PLUGIN_HOST` |URL of the chat-room interface                                |`https://devzat.hackclub.com:5556`                                          |
|`PLUGIN_TOKEN`|Authentication token                                          |Does not defaults to anything. The program panics if the token is not given.|
|`LOGIN_ROOM`  |Name of the room where the bot will tell when it is connected.|`#bots`                                                                     |
|`DEV_NICK`    |Nickname of the user the bot will tell when it is connected   |`Arkaeriit`                                                                 |
|`BOT_NAME`    |Name used by the bot to introduce itself.                     |`Time-teller`                                                               |

## Acknowledgment

Special thanks to [Tommy](https://github.com/TommyPujol06) for the [library](https://github.com/TommyPujol06/devzat-rs) he made to make Devzat plugins in Rust.

