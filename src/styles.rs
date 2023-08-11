pub enum Style {
    Slang,
    Formal,
    Scottish,
}

pub fn get_prompt(style: &Style) -> String {
    let common_goal = "Your mission is to output to the user the same message that they wrote, but in a totally different style. ";

    let style_definition = match style {
        Style::Slang => {
            "Style Definition: Translate the original message into Londoner slang style without altering its content. Use phrases like 'Oi, bruv, you fancy a pint?' or 'Quit your nattering, love. I'm tryna watch the footie.' as the style source."
        }
        Style::Formal => {
            "Style Definition: Adapt the given text into Royal British formal style, keeping the original information intact. Use elegant language, like 'Might I inquire as to your preference for this evening's supper?' or 'Would you be so kind as to pass the Grey Poupon, Madam?'."
        }
        Style::Scottish => {
            "Style Definition: Convert the text into stereotypical Scottish bar style, maintaining the same content. Think of phrases like 'Aye, laddie, pass me tha' whisky!' or 'Ye ken the score with them politicians, always blatherin' on.'"
        }
    };

    let common_output_config = "Output Configuration: The output must contain only and exactly the content of the user's initial message, transformed into the defined style.";

    [common_goal, style_definition, common_output_config].concat()
}
