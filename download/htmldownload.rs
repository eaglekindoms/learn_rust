extern crate reqwest;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

/// Remember url and file_path are both string slice which don't need to reference again.
/// so when use them, we can ignore the "&".
/// this is a function to download html's text from http path
pub fn get_html(url: &str, file_path: &str) {

    //create file which doesn't exist.
    if  !Path::exists(Path::new(file_path)) {

        File::create(file_path).unwrap();

    }

    //open file as append
    let mut file = OpenOptions::new().append(true).open(file_path).unwrap();

    //get the text from url's path and deal with error
    let _html_error_result = match reqwest::get(url).unwrap().text() {

        Ok(html_text) => file.write(html_text.as_bytes()),
        Err(_)  => panic!("sorry, unknown error!"),

    };

}
