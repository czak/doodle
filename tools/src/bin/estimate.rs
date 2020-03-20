use chrono::prelude::*;
use scraper::{Html, Selector};

// 12 weeks
const PATTERN_LEN: i64 = 84;
const PATTERN: [i64; 12] = [0, 8, 16, 24, 32, 40, 48, 54, 60, 66, 72, 78];

const TARGET_LOW: u32 = 4;
const TARGET_HIGH: u32 = 10;

fn main() {
    let username = std::env::args().nth(1).expect("must specify username");

    let current = get_current(&username);
    let target = get_target();

    println!("{}", target.saturating_sub(current));
}

fn get_current(username: &str) -> u32 {
    let document = Html::parse_document(&get_profile(username));
    let selector = Selector::parse("g g:last-of-type rect:last-child").unwrap();

    let rect = document.select(&selector).next().expect("rect not found");
    rect.value()
        .attr("data-count")
        .expect("data-count attribute missing")
        .parse()
        .expect("unable to parse data-count to u32")
}

fn get_target() -> u32 {
    let start = Utc.ymd(2020, 1, 5);
    let now = Utc::today();

    let offset = now.signed_duration_since(start).num_days() % PATTERN_LEN;

    if PATTERN.contains(&offset) {
        TARGET_HIGH
    } else {
        TARGET_LOW
    }
}

#[cfg(debug_assertions)]
fn get_profile(_username: &str) -> String {
    String::from(include_str!("../../data/czak.html"))
}

#[cfg(not(debug_assertions))]
fn get_profile(username: &str) -> String {
    let url = format!("https://github.com/{}", username);
    let resp = reqwest::blocking::get(&url).expect("failed to get url");
    resp.text().unwrap()
}
