pub fn cmd_echo(args: &Vec<String>) -> i8 {
    let text: String = args.join(" ");

    println!("{}", text);

    return 0;
}
