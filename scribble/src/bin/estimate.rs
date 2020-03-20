use scraper::{Html, Selector};

fn main() {
    let username = match std::env::args().nth(1) {
        Some(s) => s,
        None => {
            eprintln!("must specify username");
            std::process::exit(1);
        }
    };

    // let html = {
    //     let url = format!("https://github.com/{}", username);
    //     let resp = reqwest::blocking::get(&url).unwrap();
    //     resp.text().unwrap()
    // };

    let html = include_str!("../../czak.html");

    let document = Html::parse_document(&html);
    let selector = Selector::parse("g g:last-of-type rect:last-child").unwrap();

    let rect = document.select(&selector).next().unwrap();
    println!("{}", rect.value().attr("data-count").unwrap());
}
