fn main() {
    let config_max: Option<u8> = Some(3u8);

    // match max {
    //     Some(max) => println!("Max is: {max}"),
    //     _ => (),
    // }

    if let Some(max) = config_max {
        println!("Max is: {max}");
    } else {
        println!("Max is't defined!");
    }
}
