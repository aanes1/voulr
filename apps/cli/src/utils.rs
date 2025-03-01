use crossterm::style::Stylize;
use inquire::ui::{Attributes, Color, ErrorMessageRenderConfig, RenderConfig, StyleSheet, Styled};

const ERROR_PREFIX: &str = "✘ ";
const PROMPT_PREFIX: &str = "?";
const ANSWER_PREFIX: &str = ">";

pub fn log_err(err: &anyhow::Error) {
    let err_msg = format!("{} {}", ERROR_PREFIX, err).red().bold();
    println!("{}", err_msg);
}

pub fn get_rcfg() -> RenderConfig<'static> {
    RenderConfig::empty()
        // input
        .with_default_value(StyleSheet::new().with_fg(Color::DarkGrey))
        .with_prompt_prefix(Styled::new(PROMPT_PREFIX).with_fg(Color::DarkMagenta))
        // answer
        .with_answer(StyleSheet::new().with_attr(Attributes::BOLD))
        .with_answered_prompt_prefix(Styled::new(ANSWER_PREFIX).with_fg(Color::DarkMagenta))
        // error
        .with_error_message(
            ErrorMessageRenderConfig::empty()
                .with_prefix(Styled::new(ERROR_PREFIX).with_fg(Color::LightRed))
                .with_message(
                    StyleSheet::new()
                        .with_fg(Color::LightRed)
                        .with_attr(Attributes::BOLD),
                ),
        )
        // select
        .with_selected_option(Some(StyleSheet::new().with_attr(Attributes::BOLD)))
        .with_highlighted_option_prefix(Styled::new(ANSWER_PREFIX).with_fg(Color::DarkMagenta))
}
