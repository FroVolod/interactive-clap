use dialoguer::{theme::ColorfulTheme, Select};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = crate::common::ConnectionConfig)]
struct OnlineArgs {
    #[interactive_clap(subcommand)]
    submit: Submit,
}

#[derive(Debug, EnumDiscriminants, Clone, clap::Clap, interactive_clap_derive::ToCliArgs)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum Submit {
    #[strum_discriminants(strum(message = "I want to send the transaction to the network"))]
    Send,
    #[strum_discriminants(strum(
        message = "I only want to print base64-encoded transaction for JSON RPC input and exit"
    ))]
    Display,
}

impl Submit {
    fn choose_variant(connection_config: crate::common::ConnectionConfig) -> color_eyre::eyre::Result<Self> {
        let variants = SubmitDiscriminants::iter().collect::<Vec<_>>();
        let submits = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let select_submit = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("How would you like to proceed")
            .items(&submits)
            .default(0)
            .interact()
            .unwrap();
        match variants[select_submit] {
            SubmitDiscriminants::Send => Ok(Submit::Send),
            SubmitDiscriminants::Display => Ok(Submit::Display),
        }
    }

    fn from_cli(
        optional_clap_variant: Option<<Submit as interactive_clap::ToCli>::CliVariant>,
        context: crate::common::ConnectionConfig,
    ) -> color_eyre::eyre::Result<Self> {
        let submit: Option<Submit> = optional_clap_variant
            .clone();
        match submit {
            Some(_) => Ok(Submit::Send),
            None => Ok(Submit::Display)
        }
            
    }
}

impl interactive_clap::ToCli for Submit {
    type CliVariant = Submit;
}

fn main() {
    let cli_online_args = CliOnlineArgs::default();
    println!("cli_online_args: {:?}", cli_online_args);
    let online_args = OnlineArgs::from_cli(Some(cli_online_args), crate::common::ConnectionConfig::Testnet);
    println!("online_args: {:?}", online_args)
}