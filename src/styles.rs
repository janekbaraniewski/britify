use csv::Reader;
use serde::Deserialize;
use std::error::Error;

/// Representin' th' various British styles for text transformation, aye.
#[derive(PartialEq, Debug)]
pub enum Style {
    Slang,    // Londoner slang style, innit?
    Formal,   // Royal British formal style, if ye please.
    Scottish, // Scottish bar style, laddie.
}

#[derive(Debug, Deserialize)]
pub struct TextExample {
    id: u64,
    original: String,
    slang: String,
    formal: String,
    scottish: String,
    source: String,
}

pub fn get_texts() -> Result<Vec<TextExample>, Box<dyn Error>> {
    let mut reader = Reader::from_path("/Users/janbaraniewski/Workspace/priv/britify/texts.csv")?;
    let mut texts = Vec::new();

    for result in reader.deserialize() {
        let record: TextExample = result?;
        texts.push(record);
    }

    Ok(texts)
}


/// Gettin' th' prompt based on th' selected style, aye.
///
/// This prompt helps in translatin' th' text accordin' tae th' chosen style, by th' bonnie banks.
pub fn get_prompt(style: &Style, texts: &[TextExample]) -> String {
    let common_goal = "
    Your mission as a text transformation agent is to output to the user the same message they wrote, just in a totally different style.
    ";

    let common_output_config = "
    Output only and ONLY the content of the user's initial message after applying the defined style.
    ";

    let examples = texts.iter()
        .map(|text| format!("- Original: '{}'\n  Transformed: '{}'", text.original, match style {
            Style::Slang => &text.slang,
            Style::Formal => &text.formal,
            Style::Scottish => &text.scottish,
        }))
        .collect::<Vec<String>>()
        .join("\n");

    let style_definition = match style {
        Style::Slang => format!("
        Translate the following message into Londoner slang style without altering its content.\n{}", examples),
        Style::Formal => format!("\nAdapt the given text into Royal British formal style, keeping the original information intact.\n{}", examples),
        Style::Scottish => format!("\nConvert the text into stereotypical Scottish bar style, maintaining the same content.\n{}", examples),
    };
    [common_goal, &style_definition, common_output_config].join("\n")
}
