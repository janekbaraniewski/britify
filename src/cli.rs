use clap::{App, Arg};

pub fn run() {
    let version = env!("VERSION");
    let matches = App::new("Britify")
        .version(version)
        .author("https://baraniew.ski/")
        .author("Your Name")
        .about("Transforms text into British styles")
        .arg(
            Arg::with_name("TEXT")
                .help("Input text to transform")
                .required(false),
        )
        .arg(
            Arg::with_name("style")
                .short("s")
                .long("style")
                .help("Sets the style: slang, formal, scottish")
                .takes_value(true),
        )
        .get_matches();

    if matches.value_of("TEXT").is_none() {
        App::new("Britify")
            .version("0.1.0")
            .author("https://baraniew.ski/")
            .about("Transform yer text intae Scottish, laddie! Britify tak's yer words an' turns 'em intae somethin' pure bonnie. Choose frae slang, formal, or Scottish styles, an' watch as yer text is reborn!")
            .arg(
                Arg::with_name("TEXT")
                    .help("Input text to transform")
                    .required(false),
            )
            .arg(
                Arg::with_name("style")
                    .short("s")
                    .long("style")
                    .help("Sets the style: slang, formal, scottish")
                    .takes_value(true),
            )
            .print_help()
            .unwrap();
        println!(); // Print a newline after the help message
        return;
    }

    let text = matches.value_of("TEXT").unwrap();
    let style = matches.value_of("style").unwrap_or("slang");

    // Call your translation function here, e.g.
    // let result = translator::translate(text, style);
    // println!("{}", result);
}
