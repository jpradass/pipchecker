// use termimad::{minimad::TextTemplate, MadSkin};

use colored::Colorize;

pub fn display_pkg_info(pkg_info: serde_json::Value) {
    // let mut skin = MadSkin::default();
    // skin.bold.set_fg(ratatui::crossterm::style::Color::White);
    // let text_template = TextTemplate::from(
    //     r#"
    //     ┌───────────────────────────────────────────────────────┐
    //       **package name**      -> ${pkg_name}
    //       **author**            -> ${auth}
    //       **version**           -> ${ver}
    //       **summary**           -> ${summ}
    //       **required python**   -> ${min_python}
    //     └───────────────────────────────────────────────────────┘
    //     "#,
    // );
    // let mut expander = text_template.expander();
    let name = pkg_info["name"].as_str().unwrap_or("");
    let author = pkg_info["author"].as_str().unwrap_or("");
    let version = pkg_info["version"].as_str().unwrap_or("");
    let summary = pkg_info["summary"].as_str().unwrap_or("");
    let req_python = pkg_info["requires_python"].as_str().unwrap_or("");

    let strings = [
        name.to_string(),
        author.to_string(),
        version.to_string(),
        summary.to_string(),
        req_python.to_string(),
    ];
    let longest = longest_line(&strings);
    let total_width = calculate_columns(longest.as_str()) + 24;

    let horizontal_line = "─".repeat(total_width);

    println!("╭{}╮", horizontal_line);
    println!(
        "│  📦 package name: {:<width$}  │",
        name.bold(),
        width = total_width - 21
    );
    println!(
        "│  👤 author: {:<width$}  │",
        author.bold(),
        width = total_width - 15
    );
    println!(
        "│  🛠️version: {:<width$}  │",
        version.bold(),
        width = total_width - 16
    );
    println!(
        "│  📝 summary: {:<width$}  │",
        summary.bold(),
        width = total_width - 16
    );
    println!(
        "│  🐍 min required python: {:<width$}  │",
        req_python.bold(),
        width = total_width - 28
    );
    println!("╰{}╯", horizontal_line);
    // expander.set("pkg_name", name);
    // expander.set("auth", author);
    // expander.set("ver", version);
    // expander.set("summ", summary);
    // expander.set("min_python", req_python);
    //
    // skin.print_expander(expander);
}

fn calculate_columns(text: &str) -> usize {
    let text_length = text.lines().map(|line| line.len()).max().unwrap_or(0);

    text_length + 4
}

fn longest_line(strings: &[String]) -> &String {
    strings.iter().max_by_key(|s| s.len()).unwrap()
}
