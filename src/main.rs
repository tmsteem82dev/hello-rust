#![feature(proc_macro_hygiene, decl_macro)]
extern crate reqwest;
extern crate select;
#[macro_use] extern crate text_io; 
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;


use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::time::{Duration, Instant};
use std::mem;
use std::collections::hash_map::{HashMap};
use rocket::response::content::Html;


fn main() {
    // rocket webserver, routes map to functions below
    rocket::ignite().mount("/", routes![index, farkerize]).launch();
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html(r"<html><h1>Welcome to this Rustaceous page!</h1><br/>Go to desired fark category like /farkerize/categoryname/, case sensitive (Cool, Murica, Florida, News, etc, * for all)...<br/><a href='/farkerize/Cool'>Try it!</a><br/><footer><img src='https://mir-s3-cdn-cf.behance.net/project_modules/disp/7df0bd42774743.57ee5f32bd76e.gif'></footer></html>")
}

#[get("/farkerize/<filter>")]
fn farkerize(filter: &rocket::http::RawStr) -> Html<String> {

    let stuff = crawlfark(String::from(filter.as_str())).unwrap();
    
    Html(format!("<html><h2>Summarized results from fark.com...</h2>{}</html>", stuff))
}

fn crawlfark(filtercategory: String) -> Result<String, String> {
    
    let url = String::from("https://www.fark.com");

    // crawl web stuff, very specific for fark.com because css selectors
    let start = Instant::now();
    let resp = reqwest::get(url.as_str()).unwrap();
    let document = Document::from_read(resp).unwrap();

    // we will parse crawl results into this table
    let mut vectable = vec![];

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
            
            vectable.push(format!("<tr><b>{}</b></tr>",category_n_title));
            vectable.push(format!("<tr><u>{}</u></tr>",url_trim));
            vectable.push(format!("<tr><i>{}</i></tr>",smmry));
        }
    }

    let mut output = String::new();

    for val in vectable
    {
        output.push_str("<table>");
        output.push_str(val.as_str());
        output.push_str("</table>")
    }

    // simple stats
    let duration = start.elapsed().subsec_millis() as f64;
    let sz = mem::size_of_val(&document) as f64;
    let throughput = (sz / duration) * 1000.0;

    // stats_table.add_row(row![FdBgbl->format!("Duration: {:.*}/ms | Size: {:.*}/bytes | Throughput: {:.*}/kbs", 0, duration, 2, sz, 2, throughput)]);
    // for val in stats_table.row_iter()
    // {
    //     output.push_str(format!("{:?}\r\n", val));
    // }

    Ok(output)
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