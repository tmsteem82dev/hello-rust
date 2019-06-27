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
    rocket::ignite().mount("/", routes![index, farkerize, uniquestrings]).launch();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(all_chars_unique_hash(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_hash(&String::from("abcdefga")), false);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(all_chars_unique_bits(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_bits(&String::from("abcdefga")), false);
    }

}

#[get("/")]
fn index() -> Html<&'static str> {
    Html(r"<html><h1>Welcome to this Rustaceous page!</h1><br/>Go to desired fark category like /farkerize/categoryname/, case sensitive (Cool, Murica, Florida, News, etc, * for all)...<br/><a href='/farkerize/Cool'>Try it!</a><br/><br/><br/>From the Cracking the Coding Inteview v6 book: now with random API thing to do string inspection for unique chars @ /api/unique/chars/your_string<br/><a href='/api/unique/chars/zxcvbnmasdfghjklqwertyuiop'>Try that out...</a><footer><img src='https://mir-s3-cdn-cf.behance.net/project_modules/disp/7df0bd42774743.57ee5f32bd76e.gif'></footer></html>")
}

#[get("/api/unique/chars/<in_this_thing>")]
fn uniquestrings(in_this_thing: &rocket::http::RawStr) -> Html<String> {

    //time the method executions
    let start_method1 = Instant::now();
    let method1_parse = all_chars_unique_hash(in_this_thing);
    let duration_method1 = start_method1.elapsed().subsec_nanos();

    let start_method2 = Instant::now();
    let method2_parse = all_chars_unique_bits(in_this_thing);
    let duration_method2 = start_method2.elapsed().subsec_nanos();
    
    //measure the fastest result and do some string formatting
    let mut fastest_result = String::new();
    if duration_method1 < duration_method2 
        { 
            fastest_result = format!("{} returned in {:#} nanosecs by hash.<br/><br/>This was approximately {}% faster than bitfield method. (<i>{:#}</i>ns).", 
                method1_parse.to_string(), 
                duration_method1,
                (duration_method2 / duration_method1) * 100,
                duration_method2); 
        } 
    else 
        { 
            fastest_result = format!("{} returned in {:#} nanosecs by bitfield.<br/><br/>This was approximately {}% faster than hash method (<i>{:#}</i>ns).", 
                method2_parse.to_string(), 
                duration_method2,
                (duration_method1 / duration_method2) * 100,
                duration_method1); 
        } ;
    
     Html(format!("<html><h2>Results for checking if all characters in string are unique...</h2>{}</html>", fastest_result))
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

// grabbed from cracking the programming interview questions solved @ https://github.com/brndnmtthws/cracking-the-coding-interview-rust/tree/master/src/bin
fn all_chars_unique_hash(s: &str) -> bool {
    use std::collections::HashSet;
    let mut characters: HashSet<char> = HashSet::new();

    for c in s.chars() {
        if characters.contains(&c) {
            return false;
        }
        characters.insert(c);
    }
    true
}

// grabbed from cracking the programming interview questions solved @ https://github.com/brndnmtthws/cracking-the-coding-interview-rust/tree/master/src/bin
fn all_chars_unique_bits(s: &str) -> bool {
    let mut bitfield: i64 = 0;
    let a_int_char: i16 = 'a' as i16;

    for c in s.chars() {
        let mut int_char: i16 = c as i16;
        int_char -= a_int_char;

        if (1 << int_char) & bitfield != 0 {
            return false;
        }

        // set bit
        bitfield |= 1 << int_char;
    }

    true
}
