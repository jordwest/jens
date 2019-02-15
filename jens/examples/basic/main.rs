extern crate jens;

use jens::{Block, File};

fn main() {
    let emoji: Vec<(&str, &str)> = vec![
        ("smile", "🙂"),
        ("frown", "☹️"),
        ("scream", "️😱"),
        ("robot", "🤖"),
    ];

    let f = File::parse(include_str!("./emoji.jens")).unwrap();

    let output = f
        .template("main")
        .set(
            "map",
            f.template("map").set("name", "EMOJI_MAP").set(
                "entries",
                Block::join_map(&emoji, |&(key, val), _| {
                    f.template("key_value").set("key", key).set("value", val)
                }),
            ),
        )
        .set(
            "logger",
            f.template("logger").set(
                "functions",
                Block::join_map(&emoji, |&(key, _), _| {
                    f.template("log_function")
                        .set("key", key)
                        .set("map", "EMOJI_MAP")
                }),
            ),
        );

    println!("{}", output);
}
