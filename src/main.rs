use clap::Parser;
use rcli::{
    generate_password, process_csv, process_decode, process_encode, Base64SubCommand, Opts,
    SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, &opts.format)?
        }
        SubCommand::Passgen(opts) => generate_password(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.digits,
            opts.symbols,
        )?,
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }

    Ok(())
}
