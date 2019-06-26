#[macro_use] extern crate prettytable;
extern crate reqwest;
extern crate select;
#[macro_use] extern crate text_io; 
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;


use ferris_says::say;
use reqwest::{Client};
use std::io::{stdout, BufWriter};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use prettytable::Table;
use std::time::{Duration, Instant};
use std::mem;
use std::collections::hash_map::{HashMap};


fn main() {
    println!("Wish this were a Rustaceous intro!");
    println!("Input desired fark category, case sensitive (Cool, Murica, Florida, News, etc, * for all)...");

    let filtercategory: String = read!();
    
    //crawl url but here for ferris_says scope
    let url = String::from("https://www.fark.com");

    let stdout = stdout();
    let out = format!("^^---  Rustacean crawled: {}  ---^^", url);
    let width = 53;
    let mut writer = BufWriter::new(stdout.lock());

    // crab say, somehow happens after the crawl below
    say(out.as_bytes(), width, &mut writer).unwrap();

    // crawl web stuff, very specific for fark.com because css selectors
    let start = Instant::now();
    let resp = reqwest::get(url.as_str()).unwrap();
    let document = Document::from_read(resp).unwrap();

    let mut table = Table::new();
    let mut stats_table = Table::new();

    // specific crawler for fark.com
    // uses CSS selector method to find headline text and headline title from the anchor tag
    // using select package
    for node in document.find(Class("headlineText")) {
        let category = node.parent().unwrap().find(Class("headlineTopic").descendant(Name("a"))).next().unwrap().attr("title").unwrap();
        let title = node.find(Class("headline").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        let url = node.find(Class("headline").descendant(Name("a")))
            .next()
            .unwrap();
        let url_txt = url.attr("href").unwrap();

        // shorten strings to make table aesthetically appealing
        // otherwise table will look mangled by long URLs
        // trim fark specific things (fark.com/goto/########/ redirection scheme), and prepend https because the python API expects it
        let url_trim = format!("https://{}", url_txt.trim_left_matches("https://www.fark.com/goto/").split_at(9).1);
        let category_n_title = format!("{} | {}", category, title);

        // FdBybl-> is formatting syntax specific to prettytable
        // filter out undesired categories
        if filtercategory == "*" || filtercategory == category
        {
            // Summarize the linked article via API
            let smmry = summarizeweb(String::from(url_trim.as_str())).unwrap();
            
            table.add_row(row![FdBybl->category_n_title]);
            table.add_row(row![Fy->url_trim]);
            table.add_row(row![FdBgul->String::from("Summarized results:")]);
            table.add_row(row![Fgl->smmry]);
        }
    }
    table.printstd();

    // simple stats
    let duration = start.elapsed().subsec_millis() as f64;
    let sz = mem::size_of_val(&document) as f64;
    let throughput = (sz / duration) * 1000.0;

    stats_table.add_row(row![FdBgbl->format!("Duration: {:.*}/ms | Size: {:.*}/bytes | Throughput: {:.*}/kbs", 0, duration, 2, sz, 2, throughput)]);
    stats_table.printstd();

}

fn summarizeweb(url: String) -> Result<String, reqwest::Error> {
    // json mapping
    let mut map = HashMap::new();
    map.insert("message",url);

    // python api cloud function to call summry api
    let request_url = "https://us-central1-rare-shadow-181117.cloudfunctions.net/function-4";
    
    let mut response = reqwest::Client::builder()
        .timeout(Duration::from_secs(240))
        .build()?
            .post(request_url)
            .json(&map)
            .send()
            .unwrap();

    // python api currently returning text, no parsing needed
    Ok(response.text().unwrap())
}