use clap::{App, Arg};
use crate::styles::Style;

use crate::translator::translate;

pub fn from_str(style: &str) -> Style {
    match style.to_lowercase().as_str() {
        "formal" => Style::Formal,
        "scottish" => Style::Scottish,
        _ => Style::Slang, // Default to slang if no match
    }
}

pub async fn run() {
    let version = env!("VERSION");
    let matches = App::new("Britify")
        .version(version)
        .author("https://baraniew.ski/")
        .author("Your Name")
        .about("Transforms text into British styles")
        .arg(
            Arg::with_name("TEXT")
                .help("Input text to transform")
                .index(1) // This makes it a positional argument
                .required(false),
        )
        .get_matches();

    let text = if let Some(input_text) = matches.value_of("TEXT") {
        input_text.to_string()
    } else {
        // Read from stdin if no text is provided as an argument
        use std::io::{self, Read};
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
        buffer
    };

    let style = Style::Slang; // You can modify this based on your needs

    let result = translate(&text, style).await;
    match result {
        Ok(translated_text) => println!("{}", translated_text),
        Err(error) => println!("An error occurred: {:?}", error),
    }
}
