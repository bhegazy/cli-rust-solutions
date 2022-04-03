
fn main() {
    if let Err(e) = headr::get_args().and_then(headr::run) {
        eprintln!("{}", e); // prints to stderr
        std::process::exit(1);
    }
}
