/// Representin' th' various British styles for text transformation, aye.
#[derive(PartialEq, Debug)]
pub enum Style {
    Slang,    // Londoner slang style, innit?
    Formal,   // Royal British formal style, if ye please.
    Scottish, // Scottish bar style, laddie.
}

/// Gettin' th' prompt based on th' selected style, aye.
///
/// This prompt helps in translatin' th' text accordin' tae th' chosen style, by th' bonnie banks.
pub fn get_prompt(style: &Style) -> String {
    let common_goal = "Your mission as a text transformation agent is to output to the user the same message they wrote, just in a totally different style.";
    let common_output_config = "Output only and ONLY the content of the user's initial message after applying the defined style.";
    let style_definition = match style {
        Style::Slang => "Translate the following message into Londoner slang style without altering its content. As a source for style, use phrases like ['Oi, bruv, you fancy a pint?', 'Quit your nattering, love. I'm tryna watch the footie.'].",
        Style::Formal => "Adapt the given text into Royal British formal style, keeping the original information intact. Use elegant language, like 'Might I inquire as to your preference for this evening's supper?' or 'Would you be so kind as to pass the Grey Poupon, Madam?'",
        Style::Scottish => "Convert the text into stereotypical Scottish bar style, maintaining the same content. Think of phrases like 'Aye, laddie, pass me tha' whisky!' or 'Ye ken the score with them politicians, always blatherin' on.'",
    };
    [common_goal, style_definition, common_output_config].join("\n")
}
