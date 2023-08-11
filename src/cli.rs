use clap::{App, Arg};

pub fn run() {
    let matches = App::new("Britify")
        .version("0.1.0")
        .author("Your Name")
        .about("Transforms text into British styles")
        .arg(
            Arg::with_name("TEXT")
                .help("Input text to transform")
                .required(true),
        )
        .arg(
            Arg::with_name("style")
                .short("s")
                .long("style")
                .help("Sets the style: slang, formal, scottish")
                .takes_value(true),
        )
        .get_matches();

    let text = matches.value_of("TEXT").unwrap();
    let style = matches.value_of("style").unwrap_or("slang");

    // Call your translation function here, e.g.
    // let result = translator::translate(text, style);
    // println!("{}", result);
}
