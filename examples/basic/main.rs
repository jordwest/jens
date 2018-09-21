extern crate jens;

use jens::grammar::File;

fn main() {
    let emoji: Vec<(&str, &str)> = vec![
        ("smile", "🙂"),
        ("frown", "☹️"),
        ("scream", "️😱"),
        ("robot", "🤖"),
    ];

    let f = File::parse(include_str!("./emoji.jens"));

    let output = f
        .template("main")
        .set(
            "map",
            f.template("map").set("name", "EMOJI_MAP").set(
                "entries",
                f.template("key_value")
                    .for_each(&emoji, |&(key, val), block| {
                        block.set("key", key).set("value", val)
                    }),
            ),
        ).set(
            "logger",
            f.template("logger").set(
                "functions",
                f.template("log_function")
                    .for_each(&emoji, |&(key, _), block| {
                        block.set("key", key).set("map", "EMOJI_MAP")
                    }),
            ),
        );

    println!("{}", output);
}
