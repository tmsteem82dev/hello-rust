#[macro_use]
extern crate prettytable;
extern crate reqwest;
extern crate select;
#[macro_use] extern crate text_io; 


use ferris_says::say;
use std::io::{stdout, stdin, BufWriter};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use prettytable::Table;
use std::time::{Duration, Instant};
use std::mem;


fn main() {
    println!("Wish this were a rustacean intro!");
    println!("Input the number to count...");
    //io read using text_io package
    let count: i32 = read!();
    println!("Input desired fark category, case sensitive (Cool, Florida, etc, * for all)...");
    let filterCategory: String = read!();

    // NOTE: mut key words makes variable writable
    // Rust variables are immutable by default.
    let mut sum = 0;

    //crawl url but here for ferris_says scope
    let url = String::from("https://www.fark.com");

    for i in 0..count {
        let even_odd = if i % 2 == 0 {"even"} else if i % 3 == 0 {"heck"} else {"odd"};
        println!("{} {}", even_odd, i);
        sum += i;
    }
    println!("The sum total: {}", sum);
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

    //specific crawler for fark.com
    //using select package
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
        let url_trim = url_txt.trim_left_matches('/');
        let category_n_title = format!(" | {} | {}", category, title);
        // FdBybl-> is formatting syntax specific to prettytable
        // filter out undesired categories
        if (filterCategory == "*")
        {
            table.add_row(row![FdBybl->category_n_title]);
            table.add_row(row![Fy->url_trim]);
        }
        else if (filterCategory == category)
        {
            table.add_row(row![FdBybl->category_n_title]);
            table.add_row(row![Fy->url_trim]);
        }
    }
    table.printstd();

    //simple stats
    let duration = start.elapsed().subsec_millis() as f64;
    let sz = mem::size_of_val(&document) as f64;
    let throughput = (sz / duration) * 1000.0;

    stats_table.add_row(row![FdBybl->format!("Duration: {:.*}/ms | Size: {:.*}/bytes | Throughput: {:.*}/kbs", 0, duration, 2, sz, 2, throughput)]);
    stats_table.printstd();
}