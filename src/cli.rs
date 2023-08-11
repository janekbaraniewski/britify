use clap::{App, Arg};
use crate::styles::Style;

use crate::translator::translate;

/// 'Ere we be, matching style names to them fancy enum values, squire.
pub fn from_str(style: &str) -> Style {
    match style.to_lowercase().as_str() {
        "formal" => Style::Formal,
        "scottish" => Style::Scottish,
        _ => Style::Slang, // Default's a bit chav, innit? Slang it is!
    }
}

/// Run 'round the CLI like a cabbie in London, guv!
pub async fn run() {
    let version = env!("VERSION");
    let matches = App::new("Britify")
        .version(version)
        .author("https://baraniew.ski/")
        .author("Your Name")
        .about("Turns yer blather into pure British gold, it does!")
        .arg(
            Arg::with_name("TEXT")
                .help("The words to britify, innit?")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::with_name("style")
                .short("s")
                .long("style")
                .help("What's the cut of your jib, squire? Slang, formal, scottish, you choose!")
                .takes_value(true)
                .default_value("slang"), // Keep it chav if ya dunno what yer want
        )
        .arg(
            Arg::with_name("model")
                .short("m")
                .long("model")
                .help("Pick yer brain, guv! gpt-4 or sumthin' else?")
                .takes_value(true)
                .default_value("gpt-4"), // The good ol' default, like.
        )
        .get_matches();

    let text = if let Some(input_text) = matches.value_of("TEXT") {
        input_text.to_string()
    } else {
        use std::io::{self, Read};
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Couldn't hear ya, mate! Speak up!");
        buffer
    };

    let style_str = matches.value_of("style").unwrap_or("slang");
    let style = from_str(style_str);

    // That model thing, know what I mean? Right here!
    let model = matches.value_of("model").unwrap_or("gpt-4");

    let result = translate(&text, style, model).await;
    match result {
        Ok(translated_text) => println!("{}", translated_text),
        Err(error) => println!("Blimey! Something's gone pear-shaped: {:?}", error),
    }
}
