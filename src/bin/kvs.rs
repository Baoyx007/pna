// use clap::App;
use clap::{App, Arg, SubCommand};
use kvs::KvStore;
use structopt::StructOpt;
// use std::process::exit;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab_case")]
enum Kvs {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

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
    let mut store = KvStore::new();

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

fn main() {
    let opt = Kvs::from_args();
    // println!("{:?}", opt);

    let mut store = KvStore::new();

    match opt {
        Kvs::Set { key, value } => {
            store.set(key, value);
        }
        Kvs::Get { key } => {
            store.get(key);
        }
        Kvs::Rm { key } => {
            store.remove(key);
        }
        _ => unreachable!(),
    }
}
