//! Formatting utilities for encyclopedia display

/// Format a stat bar (e.g., health bar, experience bar)
pub fn format_bar(current: i32, max: i32, width: usize) -> String {
    let percentage = if max > 0 {
        (current as f32 / max as f32).min(1.0)
    } else {
        0.0
    };

    let filled = (width as f32 * percentage) as usize;
    let empty = width - filled;

    format!(
        "[{}{}] {}/{}",
        "█".repeat(filled),
        "░".repeat(empty),
        current,
        max
    )
}

/// Format a percentage bar
pub fn format_percentage_bar(percentage: f32, width: usize) -> String {
    let filled = (width as f32 * percentage.min(1.0)) as usize;
    let empty = width - filled;

    format!(
        "[{}{}] {:.0}%",
        "█".repeat(filled),
        "░".repeat(empty),
        percentage * 100.0
    )
}

/// Create a divider line
pub fn divider(width: usize) -> String {
    "═".repeat(width)
}

/// Create a box border
pub fn box_line(content: &str, width: usize) -> String {
    format!("║ {:<width$} ║", content, width = width - 4)
}

/// Wrap text to fit within a specified width
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.len() + word.len() + 1 > width {
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            current_line = word.to_string();
        } else {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

/// Format a table row
pub fn table_row(columns: &[&str], widths: &[usize]) -> String {
    let mut row = String::from("║ ");
    for (i, col) in columns.iter().enumerate() {
        let width = widths.get(i).copied().unwrap_or(20);
        row.push_str(&format!("{:<width$}", col, width = width));
        if i < columns.len() - 1 {
            row.push_str(" │ ");
        }
    }
    row.push_str(" ║");
    row
}

/// Create a section header
pub fn section_header(title: &str, width: usize) -> String {
    let _padding = width.saturating_sub(title.len() + 2) / 2;
    format!(
        "╠{}╣\n║{:^width$}║",
        "═".repeat(width - 2),
        title,
        width = width - 2
    )
}
