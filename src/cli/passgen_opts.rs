use crate::CmdExecuter;
use clap::Parser;
use zxcvbn::zxcvbn;

use crate::generate_password;

#[derive(Debug, Parser)]
pub struct PassgenOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub digits: bool,

    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}

impl CmdExecuter for PassgenOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let password = generate_password(
            self.length,
            self.uppercase,
            self.lowercase,
            self.digits,
            self.symbols,
        )?;
        println!("{}", password);
        let estimate = zxcvbn(&password, &[]);
        println!("Estimated strength: {}", estimate.score());
        Ok(())
    }
}
