/// download html：the origin spider

mod htmldownload;

fn main()  {
    let url = "https://www.rust-lang.org";
    let file_path = "resource/news.html";
    htmldownload::write_html(&htmldownload::get_html(url), file_path, 'a');
}
