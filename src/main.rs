#[macro_use] extern crate clap;
mod libtoygrep;
fn main() {
    let app = libtoygrep::create_app();
    let matches = app.get_matches();
    let task = libtoygrep::parse_search_task(matches);
    task.run();
}
