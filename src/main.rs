use clap::Parser;
use rcli::{generate_password, process_csv, Opts, SubCommand};

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
    }

    Ok(())
}
