use clap::{command, Arg, Command};
use std::process::exit;

fn main() {
    let matches = command!()
        .name("kvs")
        .author("Leslie Su <3530611790@qq.com>")
        .about("A key/value store cli program.")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("set")
                .about("set the value of a string key to a string")
                .arg(Arg::new("KEY").help("A string key").required(true))
                .arg(
                    Arg::new("VALUE")
                    .help("The string value of the key")
                    .required(true)
                )
        )
        .subcommand(
            Command::new("get")
                .about("get the string value of a given string key")
                .arg(Arg::new("KEY").help("A string key").required(true))
        )
        .subcommand(
            Command::new("rm")
                .about("remove a given key")
                .arg(Arg::new("KEY").help("a string key").required(true))   
        )
        .get_matches();

    match matches.subcommand().unwrap() {
        ("set", _matches) => {
            eprintln!("unimplemented");
            exit(1); 
        }
        ("get", _matches) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", _matches) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => { exit(1); }
    }
}
