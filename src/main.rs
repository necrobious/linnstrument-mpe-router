use std::error::Error;

use rmididings::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "linnstrument-mpe-router")]
struct Opt {
    #[structopt(short = "v", long)]
    verbose: bool,

    #[structopt(short = "i", long)]
    linnstrument: String,

    #[structopt(short = "l", long)]
    left: String,

    #[structopt(short = "r", long)]
    right: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    if opt.verbose { println!("Using options:\n{:?}", opt); }

    let linn = opt.linnstrument.as_ref();
    let lft = opt.left.as_ref();
    let rgt = opt.right.as_ref();

    let in_ports = &[["linnstrument", linn]];
    let out_ports = &[["left", lft],["right", rgt]];

    let mut md = RMididings::new()?;
    md.config(ConfigArguments {
        in_ports: in_ports,
        out_ports: out_ports,
        client_name: "LinnStrument MPE Router",
        ..ConfigArguments::default()
    })?;

    let patch = Process!(|ev: &Event| {
        Box::new(ev
            .channel()
            .map(|src| {
                Chain!(
                    Port(if src > 8 { 2 } else { 1 }),
                    Channel(if src > 8 { 17 - src } else { src })
                )
            })
            .unwrap_or(Chain!(Pass()))
        )
    });

    if opt.verbose { println!("Running..."); }

    md.run(RunArguments {
        patch: &patch,
        ..RunArguments::default()
    })?;

    Ok(())
}

