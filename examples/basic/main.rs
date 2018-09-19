extern crate jens;

use jens::block::Block;
use jens::grammar::File;

fn main() {
    let emoji: Vec<(&str, &str)> = vec![
        ("smile", "🙂"),
        ("frown", "☹️"),
        ("scream", "️😱"),
        ("robot", "🤖"),
    ];

    let file = File::parse(include_str!("./emoji.jens"));

    //////////////
    // The following boilerplatey functions could disappear if we used procedural macros
    let emoji_entry = |(key, value): (&str, &str)| -> Block {
        let mut t = file.get_template_block("key_value").unwrap();
        t.set_placeholder("key", &Block::from(key));
        t.set_placeholder("value", &Block::from(value));
        t
    };

    let map = |name: &str, entries: Block| {
        let mut t = file.get_template_block("map").unwrap();
        t.set_placeholder("name", &Block::from(name));
        t.set_placeholder("entries", &entries);
        t
    };

    let template = |map: Block, logger: Block| {
        let mut t = file.get_template_block("main").unwrap();
        t.set_placeholder("map", &map);
        t.set_placeholder("logger", &logger);
        t
    };

    let logger = |functions: Block| {
        let mut t = file.get_template_block("logger").unwrap();
        t.set_placeholder("functions", &functions);
        t
    };

    let log_function = |key: &str, map_name: &str| {
        let mut t = file.get_template_block("log_function").unwrap();
        t.set_placeholder("key", &Block::from(key));
        t.set_placeholder("map", &Block::from(map_name));
        t
    };
    //////////////

    let output = template(
        map(
            "EMOJI_MAP",
            Block::join(emoji.clone().into_iter().map(emoji_entry).collect()),
        ),
        logger(Block::join(
            emoji
                .into_iter()
                .map(|(key, _)| log_function(key, "EMOJI_MAP"))
                .collect(),
        )),
    );

    println!("{}", output);
}
