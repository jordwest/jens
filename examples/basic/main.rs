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
    template.replace("MAP", "EMOJI_MAP");
    template.repeat_template("MapEntry", &emoji, &emoji_entry);
    template.repeat_template("PrintFunction", &emoji, &emoji_entry);
    println!("{}", template.output());
}
