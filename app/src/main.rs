fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("--cli")) {
        cli::run();
    } else if args.contains(&String::from("--gui")) {
        gui::run();
    } else {
        eprintln!("Usage: spreadsheet_project [--cli | --gui]");
    }
}
