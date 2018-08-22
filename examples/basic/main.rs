extern crate jens;

use jens::Template;

fn emoji_entry((key, value): &(&str, &str), template: &mut Template) {
    template.replace("key", key);
    template.replace("value", value);
}

fn main() {
    let emoji: Vec<(&str, &str)> = vec![
        ("smile", "🙂"),
        ("frown", "☹️"),
        ("scream", "️😱"),
        ("robot", "🤖"),
    ];

    let mut template = Template::parse(include_str!("./emoji.js"));
    template.repeat_section("Entry", &emoji, &emoji_entry);
    println!("{}", template.output());
}
