use console::{style, Term};

/// Display a success message
pub fn display_success(message: &str) {
    let term = Term::stdout();
    term.write_line(&format!("{} {}", style("✓").green().bold(), message))
        .ok();
}

/// Display an error message
pub fn display_error(message: &str) {
    let term = Term::stderr();
    term.write_line(&format!("{} {}", style("✗").red().bold(), message))
        .ok();
}

/// Display an info message
pub fn display_info(message: &str) {
    let term = Term::stdout();
    term.write_line(&format!("{} {}", style("ℹ").cyan(), message))
        .ok();
}

/// Display a warning message
pub fn display_warning(message: &str) {
    let term = Term::stdout();
    term.write_line(&format!("{} {}", style("⚠").yellow(), message))
        .ok();
}

/// Display a section header
pub fn display_header(title: &str) {
    println!("\n{}", style(title).bold().underlined());
}

/// Display a separator line
pub fn display_separator() {
    println!("{}", style("━".repeat(60)).dim());
}

/// Display a key-value pair
pub fn display_kv(key: &str, value: &str) {
    println!("  {}: {}", style(key).bold(), value);
}
