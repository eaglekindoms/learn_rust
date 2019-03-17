extern crate reqwest;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

/// Remember url and file_path are both string slice which don't need to reference again.
/// so when use them, we can ignore the "&".
/// when match the Err(_) should contain "panic" marco, or return same type,
/// otherwise the variable will have two possibilities.

// request html text.
pub fn get_html(url: &str) -> String {

    // get the text from url's path and deal with error.
    let html_text = match reqwest::get(url).unwrap().text() {

        Ok(html_text) => html_text.to_string(),
        Err(_)  => panic!("sorry, can't get html page!"),

    };
    return html_text;
}
// store text into file.
pub fn write_html(text: &str, file_path: &str, model: char) {

    // create file which doesn't exist.
    if  !Path::exists(Path::new(file_path)) {
        File::create(file_path).unwrap();
    }

    match model {
        'a' => {
            // append text into file, but don't truncate it.
            let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
            file.write(text.as_bytes()).unwrap();
        },
        'w' => {
            // truncate file, and write text into file.
            let mut file = OpenOptions::new().write(true)
                                        .truncate(true).open(file_path).unwrap();
            file.write(text.as_bytes()).unwrap();
        },
         _  => panic!("invalid char"),
    }

}
