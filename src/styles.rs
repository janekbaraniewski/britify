pub enum Style {
    Slang,
    Formal,
    Scottish,
}

pub fn get_prompt(style: &Style) -> &'static str {
    match style {
        Style::Slang => "Can you respond using Londoner slang style? Imagine we're chatting by the Thames, using phrases like 'Oi, bruv, you fancy a pint?' or 'Quit your nattering, love. I'm tryna watch the footie.'",
        Style::Formal => "Please write the next text in the Royal British formal style, like we're at a royal dinner. Use elegant language, such as 'Might I inquire as to your preference for this evening's supper?' or 'Would you be so kind as to pass the Grey Poupon, Madam?'",
        Style::Scottish => "Could you reply in the stereotypical Scottish bar style? Think of a conversation in an Edinburgh pub, with phrases like 'Aye, laddie, pass me tha' whisky!' or 'Ye ken the score with them politicians, always blatherin' on.'",
    }
}
