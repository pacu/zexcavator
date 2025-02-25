//! `parse` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::ZexCavatorCliConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use zexcavator::parser::WalletParserFactory;

/// `parse` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(clap::Parser, Command, Debug)]
pub struct ParseCmd {
    /// What wallet file are we parsing?
    #[arg(required = true)]
    wallet_path: String,

    /// Enable verbose mode. A flag `-v` or `--verbose` will enable verbose mode.
    #[arg(short('v'), long("verbose"))]
    verbose: bool,
}

impl Runnable for ParseCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        println!("Config: {:#?}", config);
        let wallet_parser = WalletParserFactory::read(config.input_file.to_str().unwrap()).unwrap();

        // println!("{:#?}", wallet_parser.parser.get_wallet_name());
        wallet_parser.parser.print_internal();
    }
}

impl config::Override<ZexCavatorCliConfig> for ParseCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: ZexCavatorCliConfig,
    ) -> Result<ZexCavatorCliConfig, FrameworkError> {
        if !self.wallet_path.is_empty() {
            config.input_file = self.wallet_path.to_string().into();
        }

        config.verbose = self.verbose;

        Ok(config)
    }
}
