mod cli;
mod process;
mod utils;

pub use cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand, CsvOpts, HttpServeOpts,
    HttpSubCommand, KeyGenerateOpts, Opts, OutputFormat, PassgenOpts, SubCommand, TextSignFormat,
    TextSignOpts, TextSubCommand, TextVerifyOpts,
};
use enum_dispatch::enum_dispatch;
pub use process::{
    generate_password, process_csv, process_decode, process_encode, process_http_serve,
    process_text_key_generate, process_text_sign, process_text_verify,
};
pub use utils::{get_content, get_reader};

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecuter {
    async fn execute(&self) -> anyhow::Result<()>;
}
