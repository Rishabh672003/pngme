use clap::{Parser, Subcommand};

/// Simple program to hide a secret message in a png file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,

}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ///  Encode the png file
    Encode {
        file: String,
        chunktype: String,
        message: String,
        output_path: Option<String>
    },
    Decode {
        file: String,
        chunktype: String,
    },
    Remove {
        file: String,
        chunktype: String,
    },
    Print {
        file: String,
    },
}
