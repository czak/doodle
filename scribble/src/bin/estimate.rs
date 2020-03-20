use scraper::{Html, Selector};

fn main() {
    let username = std::env::args().nth(1).expect("must specify username");

    let current = get_current(&username);

    println!("{}", current);
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
