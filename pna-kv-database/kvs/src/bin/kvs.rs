use clap::{command, Arg, Command};
use kvs::{KvStore, err::Result};
use std::{env::current_dir, process::exit};

fn main() -> Result<()>{
    let matches = command!()
        .name("kvs")
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("set")
                .about("set the value of a string key to a string")
                .arg(Arg::new("KEY").help("A string key").required(true))
                .arg(
                    Arg::new("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("get the string value of a given string key")
                .arg(Arg::new("KEY").help("A string key").required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("remove a given key")
                .arg(Arg::new("KEY").help("a string key").required(true)),
        )
        .get_matches();

    match matches.subcommand().unwrap() {
        ("set", matches) => {
            let key = matches.get_one::<String>("KEY").unwrap();
            let value = matches.get_one::<String>("VALUE").unwrap();

            let mut kv = KvStore::open(current_dir()?)?;
            kv.set(String::from(key), String::from(value))?;
        }

        ("get", _matches) => {
            eprintln!("unimplemented");
            exit(1);
        }
        
        ("rm", matches) => {
            let key = matches.get_one::<String>("KEY").unwrap(); 
            let mut kv = KvStore::open(current_dir()?)?;
            match kv.remove(String::from(key)) {
                Ok(()) => {}
                Err(_err) => {
                    println!("Key not found");
                    exit(1);
                }
            }
        }

        _ => unreachable!()
    }

    Ok(())
}
