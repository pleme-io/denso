use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "denso", about = "Wireless ADB manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Discover devices on the network via mDNS
    Discover,
    /// Pair with a device using a pairing code
    Pair,
    /// Connect to a wireless ADB device
    Connect,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Discover => {
            println!("denso: discovering devices");
        }
        Commands::Pair => {
            println!("denso: pairing with device");
        }
        Commands::Connect => {
            println!("denso: connecting to device");
        }
    }
}
