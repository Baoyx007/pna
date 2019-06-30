// use clap::App;
use clap::{App, Arg, SubCommand};
use kvs::{KvStore, KvsCommand, Result};
use structopt::StructOpt;
use std::path::Path;

fn get_from_clap() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("KEY").required(true))
                .arg(Arg::with_name("VALUE").required(true)),
        )
        .subcommand(SubCommand::with_name("get").arg(Arg::with_name("KEY").required(true)))
        .subcommand(SubCommand::with_name("rm").arg(Arg::with_name("KEY").required(true)))
        .get_matches();

    // println!("{:?}", matches);
    let mut store = KvStore::open(Path::new("test.db")).unwrap();

    match matches.subcommand() {
        ("set", Some(sub)) => {
            store.set(
                sub.value_of("KEY").unwrap().to_owned(),
                sub.value_of("VALUE").unwrap().to_owned(),
            );
        }
        ("get", Some(sub)) => {
            store.get(sub.value_of("KEY").unwrap().to_owned());
        }
        ("rm", Some(sub)) => {
            store.remove(sub.value_of("KEY").unwrap().to_owned());
        }
        _ => unreachable!(),
    }
}

fn main() -> Result<()> {
    let opt = KvsCommand::from_args();
    // println!("{:?}", opt);

    let mut store = KvStore::open(Path::new("test.db"))?;

    match opt {
        KvsCommand::Set { key, value } => {
            store.set(key, value)?;
        }
        KvsCommand::Get { key } => {
            store.get(key)?;
        }
        KvsCommand::Rm { key } => {
            store.remove(key)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
