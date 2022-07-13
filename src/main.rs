#![allow(unused)]
use std::fs;
use scraper::{Html, Selector};

fn main() {
    let html = get_html();
    println!("{html}");
    let fragment = Html::parse_fragment(&html);
    let word = get_word(&fragment);
    let shorts = get_all_short(&fragment);
    for short in shorts {
        println!("{:#?}", short);
    }
    println!("word: {word}");
}

fn get_word(fragment: &Html) -> String {
    let selector = Selector::parse(".HWD").unwrap();
    if let Some(word) = fragment.select(&selector).next() {
        String::from(word.text().next().unwrap_or("unknown"))
    } else {
        String::from("unknown")
    }
}

#[derive(Debug)]
struct Explain {
    id: i32,
    short: Option<String>,
    detail: Option<String>,
}

fn get_all_short(fragment: &Html) -> Vec<Explain> {
    let selector = Selector::parse("a .Section").unwrap();
    fragment
        .select(&selector)
        .map(|element| element.text().collect::<Vec<&str>>())
        .enumerate()
        .map(|(i, list)| Explain {
            id: (i + 1) as i32,
            short: list
                .get(1)
                .map_or(Some(String::from("unknown")), |str| Some(str.to_string())),
            detail: None,
        })
        .collect()
}

fn get_html() -> String {
    return fs::read_to_string("./hello.html").unwrap();
}
