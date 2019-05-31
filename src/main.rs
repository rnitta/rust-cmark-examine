use pulldown_cmark::{html, Options, Parser};
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Suite {
    markdown: String,
    html: String,
    start_line: usize,
    end_line: usize,
    section: String,
}

fn main() {
    let data = fs::read_to_string("./src/0_29_spec.json").unwrap();
    let suites: Vec<Suite> = serde_json::from_str(&data).unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let mut total_suites_count = 0;
    let mut error_suites_count = 0;

    for suite in &suites {
        let parser = Parser::new_ext(&suite.markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        total_suites_count += 1;
        if suite.html != html_output {
            error_suites_count += 1;
            println!("Error: {:#?}", suite);
            println!("Expected output:\n{}", suite.html);
            println!("Actual output:\n{}", html_output);
            println!("-----------------")
        }
    }
    println!(
        "total: {} specs examined, error: {} specs detected.",
        total_suites_count, error_suites_count
    );
}
