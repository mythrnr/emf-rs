use std::{
    fs::File,
    io::{Read, Write},
};

use clap::Parser;
use tracing_subscriber::{
    fmt::{format::FmtSpan, time::UtcTime},
    EnvFilter,
};

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The EMF file path to convert to SVG
    #[arg(short, long)]
    input: String,
    /// The destination file path to save converted SVG
    #[arg(short, long, default_value = "output.svg")]
    output: String,
    /// Omit logs except error log
    #[arg(short, long)]
    quiet: bool,
    /// Print debug logs
    #[arg(long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let env_filter = {
        let level = if cli.verbose {
            "debug"
        } else if cli.quiet {
            "error"
        } else {
            "info"
        };

        EnvFilter::from_default_env()
            .add_directive(
                format!("emf_core={level}").parse().expect("should be parsed"),
            )
            .add_directive(
                format!("emf_converter={level}")
                    .parse()
                    .expect("should be parsed"),
            )
            .add_directive(
                format!("wmf_core={level}").parse().expect("should be parsed"),
            )
            .add_directive(
                format!("wmf_converter={level}")
                    .parse()
                    .expect("should be parsed"),
            )
    };

    tracing_subscriber::fmt::fmt()
        // .pretty()
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true)
        .with_timer(UtcTime::rfc_3339())
        .with_env_filter(
            env_filter
        )
        .init();

    let _span = tracing::info_span!("main", input = %cli.input).entered();

    let Ok(mut input) = File::open(cli.input.clone()).inspect_err(|err| {
        tracing::error!(%err);
    }) else {
        std::process::exit(1);
    };

    let Ok(mut output) = File::create(cli.output.clone()).inspect_err(|err| {
        tracing::error!(%err);
    }) else {
        std::process::exit(1);
    };

    let mut buffer = vec![];
    if let Err(err) = input.read_to_end(&mut buffer) {
        tracing::error!(%err);
        std::process::exit(1);
    }

    // let bytes = buffer
    //     .iter()
    //     .map(|v| format!("{v:02X}"))
    //     .collect::<Vec<_>>()
    //     .chunks(4)
    //     .into_iter()
    //     .map(|v| v.join(" "))
    //     .collect::<Vec<_>>()
    //     .join("\n");
    // println!("{bytes}");

    let wmf_player = wmf_core::converter::SVGPlayer::new();
    let emf_player = emf_core::converter::SVGPlayer::new();
    let converter = emf_core::converter::EMFConverter::new(
        buffer.as_slice(),
        emf_player,
        wmf_player,
    );

    match converter.run() {
        Ok(bytes) => {
            if let Err(err) = output.write_all(&bytes) {
                tracing::error!(%err);

                // ignore error.
                let _ = std::fs::remove_file(cli.output)
                    .inspect_err(|err| tracing::error!(%err));

                std::process::exit(1);
            }
        }
        Err(err) => {
            tracing::error!(%err);

            // ignore error.
            let _ = std::fs::remove_file(cli.output)
                .inspect_err(|err| tracing::error!(%err));

            std::process::exit(1);
        }
    };

    tracing::info!("Converted successfully.");
}
