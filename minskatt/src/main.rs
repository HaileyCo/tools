use anyhow::Context;
use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, error, info};
use structopt::StructOpt;

use minskatt::sru_parser;

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "minskatt",
    setting = structopt::clap::AppSettings::ColoredHelp,
    about = " "
)]
struct MinSkatt {
    #[structopt(short = "v", long = "verbose", help = "turn on verbosity")]
    verbose: bool,

    #[structopt(short = "q", long = "quiet", help = "turn off all logs")]
    quiet: bool,

    #[structopt(subcommand)]
    cmd: Goal,
}

impl MinSkatt {
    fn run(self) {
        self.setup_logging();
        match self.cmd.run() {
            Ok(()) => (),
            Err(err) => error!("{:?}", &err),
        }
    }

    fn setup_logging(&self) {
        let colors_line = ColoredLevelConfig::new()
            .error(Color::Red)
            .warn(Color::Yellow)
            .info(Color::White)
            .debug(Color::BrightBlack)
            .trace(Color::BrightBlack);
        let colors_level = colors_line.clone().info(Color::Green);
        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{color_line}{date} {level}{color_line} :: {message}\x1B[0m",
                    color_line = format_args!(
                        "\x1B[{}m",
                        colors_line.get_color(&record.level()).to_fg_str()
                    ),
                    date = chrono::Local::now().format("%H:%M:%S"),
                    level = colors_level.color(record.level()),
                    message = message,
                ));
            })
            .level(match (self.verbose, self.quiet) {
                (_, true) => log::LevelFilter::Off,
                (true, false) => log::LevelFilter::Debug,
                (false, false) => log::LevelFilter::Info,
            })
            .level_for("pretty_colored", log::LevelFilter::Trace)
            .chain(std::io::stderr())
            .apply()
            .unwrap();
    }
}

#[derive(StructOpt, Debug, Clone)]
enum Goal {
    Validate(ValidateOpt),
}

impl Goal {
    fn run(self) -> Result<(), anyhow::Error> {
        match self {
            Goal::Validate(opts) => opts.validate(),
        }
    }
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "validate", about = "validate SRU file")]
struct ValidateOpt {
    #[structopt(default_value = "./INFO.SRU")]
    file: String,
}

impl ValidateOpt {
    fn validate(self) -> Result<(), anyhow::Error> {
        let contents = std::fs::read_to_string(self.file).context("Could not open file!")?;
        let sru = sru_parser::parse(contents)?;

        info!("File is valid! Here's the contents:");
        debug!("Internal data structure: {:?}", &sru);
        print!("{}", sru);

        Ok(())
    }
}

fn main() {
    MinSkatt::from_args().run();
}
