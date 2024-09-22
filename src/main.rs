use rpstree;
fn main() {
    match rpstree::run() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
