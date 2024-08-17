use std::{path::PathBuf, str::FromStr};

use crate::{
    get_content, get_reader, process_text_key_generate, process_text_sign, process_text_verify,
    CmdExecuter,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use tokio::fs;

use super::{verify_file, verify_path};

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Black3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "black3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

impl CmdExecuter for TextSignOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let sig = process_text_sign(&mut reader, &key, self.format)?;
        // base64 output
        let encoded = URL_SAFE_NO_PAD.encode(sig);
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExecuter for TextVerifyOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let decoded = URL_SAFE_NO_PAD.decode(&self.sig)?;
        let verified = process_text_verify(&mut reader, &key, &decoded, self.format)?;
        if verified {
            println!("Signature verified");
        } else {
            println!("Signature not verified");
        }
        Ok(())
    }
}

impl CmdExecuter for KeyGenerateOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let key = process_text_key_generate(self.format)?;
        for (k, v) in key {
            fs::write(self.output_path.join(k), v).await?;
        }
        Ok(())
    }
}

impl CmdExecuter for TextSubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            Self::Sign(opts) => opts.execute().await,
            Self::Verify(opts) => opts.execute().await,
            Self::Generate(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "black3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(short, long, default_value = "black3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signature with a public/session key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a random black3 key or ed25519 keypair")]
    Generate(KeyGenerateOpts),
}

fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "black3" => Ok(Self::Black3),
            "ed25519" => Ok(Self::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
