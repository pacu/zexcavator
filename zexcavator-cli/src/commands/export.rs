//! `parse` subcommand - example of how to write a subcommand

use std::{path::PathBuf, str::FromStr};

use crate::{config::ZexCavatorCliConfig, prelude::APP};
use abscissa_core::{Application, Command, FrameworkError, Runnable, config};
use bc_envelope::Envelope;

/// `export` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(clap::Parser, Command, Debug)]
pub struct ExportCmd {
    /// A wallet file. Currently only ZecWallet and YWallet are supported.
    #[arg(required = true, value_name = "INPUT_FILE")]
    input_file: String,

    /// Where to save the ZeWIF file.
    #[arg(value_name = "OUTPUT_FILE")]
    output_file: Option<String>,
}

impl Runnable for ExportCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();

        let _output = config.output_file.to_str().unwrap();

        let erp_envelope =
            Envelope::new("<seed>").add_assertion("generates", "<emergency_recovery_phrase>");

        let zc_object: Envelope = Envelope::new("wallet").add_assertion("hasSeed", erp_envelope);

        // Extension version
        let zc_extension =
            zc_object.add_assertion(bc_envelope::known_values::VERSION_VALUE, "0.0.1");

        let sample_envelope = Envelope::new("Alice")
            .add_assertion("Knows", "Bob")
            .add_attachment(
            zc_extension,
            "org.zingolabs",
            Some("https://github.com/zingolabs/zexcavator/blob/main/docs/zewif-extension-spec.md"),
        );

        println!("{:}", Envelope::format_flat(&sample_envelope));
        todo!()
    }
}

impl config::Override<ZexCavatorCliConfig> for ExportCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: ZexCavatorCliConfig,
    ) -> Result<ZexCavatorCliConfig, FrameworkError> {
        config.input_file = PathBuf::from_str(&self.input_file).unwrap();

        if let Some(output_file) = &self.output_file {
            config.output_file = PathBuf::from_str(output_file).unwrap();
        }

        Ok(config)
    }
}
