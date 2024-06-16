fn match_max(config_max: &Option<u8>) {
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}

fn if_let_max(config_max: &Option<u8>) {
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

fn main() {
    let config_max = Some(3u8);
    match_max(&config_max);
    if_let_max(&config_max);
}
