extern crate testmark;
use testmark::Testmark;
use testmark::Timer;

use tinystring::{ generate_dictionary, tiny_string_deflate, tiny_string_inflate };
use std::fs;

fn main() {
    let mut cbench: Testmark = Timer::new("TinyString Compression Test");

    let contents: String = fs::read_to_string("sample.txt").unwrap();
    let data: String = "I just spent about $3000 surgically removing a big ball of WTF from my Maine coon! Came home with a dozen staples down his belly and immediately started trying to eat the plastic wrap I just pulled off his medication bottles. Moron. I'm sorry your kitty didn't make it. Being stupidly suicidal seems to be a breed characteristic!".to_string();
    let dict: Vec<String> = generate_dictionary(contents, 5);

    cbench.start();
    let result: String = tiny_string_deflate(data, dict.clone());
    let inflated: String = tiny_string_inflate(result.to_string(), dict.clone());
    cbench.stop();

    println!("{} {}", result.to_string(), result.len());
    println!("{} {}", inflated, inflated.len());
    fs::write("result.txt", result).unwrap();

    cbench.print();
}