fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("--gui")) {
        gui::run();
    } else {
        cli::run();
    }
}
