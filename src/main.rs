mod spongecore;
use spongecore::{sponge, config};
use std::string::String;
use clap::{Arg, App};

fn main() -> sponge::BoxedResult<()> {

    let matches = App::new("SpongeBab")
                    .about("Who lives in a pineapple under the sea... and wants to trash your network?")
                    .arg(Arg::with_name("interface")
                        .short("i")
                        .long("interface")
                        .value_name("INTERFACE_NAME")
                        .help("The name of the interface on which to listen for packets.")
                        .takes_value(true)
                        .required(true))
                    .get_matches();



    let s = sponge::Sponge::new(config::Config{
        iface: String::from(matches.value_of("interface").unwrap()),
    })?;

    s.run()
}
