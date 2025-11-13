use crate::utils::Result;
use dialoguer::{Confirm, Input, Password, Select};

/// Prompt for text input
pub fn prompt_text(prompt: &str, default: Option<&str>) -> Result<String> {
    let mut input = Input::<String>::new().with_prompt(prompt);

    if let Some(default_val) = default {
        input = input.with_initial_text(default_val);
    }

    Ok(input.interact_text()?)
}

/// Prompt for password/secret input
pub fn prompt_password(prompt: &str) -> Result<String> {
    Ok(Password::new().with_prompt(prompt).interact()?)
}

/// Prompt for confirmation
pub fn prompt_confirm(prompt: &str, default: bool) -> Result<bool> {
    Ok(Confirm::new()
        .with_prompt(prompt)
        .default(default)
        .interact()?)
}

/// Prompt for selection from a list
pub fn prompt_select<T: ToString>(prompt: &str, items: &[T]) -> Result<usize> {
    Ok(Select::new().with_prompt(prompt).items(items).interact()?)
}

/// Prompt for multiline text (workflow description)
pub fn prompt_multiline(prompt: &str) -> Result<String> {
    println!("{}", prompt);
    println!("(Enter a blank line to finish)");

    let mut lines = Vec::new();
    loop {
        let line: String = Input::new()
            .with_prompt(">")
            .allow_empty(true)
            .interact_text()?;

        if line.trim().is_empty() && !lines.is_empty() {
            break;
        }

        if !line.trim().is_empty() {
            lines.push(line);
        }
    }

    Ok(lines.join("\n"))
}
