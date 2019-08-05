use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
enum Opt {
    #[structopt(name = "set", about = "Set the value of a string key to a string")]
    Set {
        #[structopt(name = "KEY", help = "A string key", required = true)]
        key: String,
        #[structopt(name = "VALUE", help = "The string value of the key", required = true)]
        value: String,
    },
    #[structopt(name = "get", about = "Get the string value of a given string key")]
    Get {
        #[structopt(name = "KEY", help = "A string key", required = true)]
        key: String,
    },
    #[structopt(name = "rm", about = "Remove a given key")]
    Rm {
        #[structopt(name = "KEY", help = "A string key", required = true)]
        key: String,
    },
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Set { key: _, value: _ } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Opt::Get { key: _ } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Opt::Rm { key: _ } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
