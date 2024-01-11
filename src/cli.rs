use clap::Parser;
use log::LevelFilter;

use crate::Error;

#[derive(Parser, Debug)]
#[clap(name = "SCR-miner", version, about = "A SCR high performance CPU miner", term_width = 0)]
pub struct Opt {
    #[clap(short, long, help = "Enable debug logging level")]
    pub debug: bool,
    #[clap(short = 'a', long = "mining-address", help = "The SCR address for the miner reward")]
    pub mining_address: String,
    #[clap(short = 's', long = "SCR-address", default_value = "127.0.0.1", help = "The IP of the SCR instance")]
    pub SCR_address: String,

    #[clap(short, long, help = "SCRpad port [default: Mainnet = 13110, Testnet = 16211]")]
    port: Option<u16>,

    #[clap(long, help = "Use testnet instead of mainnet [default: false]")]
    testnet: bool,
    #[clap(short = 't', long = "threads", help = "Amount of CPU miner threads to launch [default: 0]")]
    pub num_threads: Option<u16>,
    #[clap(
        long = "mine-when-not-synced",
        help = "Mine even when SCR says it is not synced",
        long_help = "Mine even when SCR says it is not synced, only useful when passing `--allow-submit-block-when-not-synced` to SCR  [default: false]"
    )]
    pub mine_when_not_synced: bool,

    #[clap(skip)]
    pub devfund_address: String,
}

impl Opt {
    pub fn process(&mut self) -> Result<(), Error> {
        //self.gpus = None;
        if self.SCR_address.is_empty() {
            self.SCR_address = "127.0.0.1".to_string();
        }

        if !self.SCR_address.contains("://") {
            let port_str = self.port().to_string();
            let (SCR, port) = match self.SCR_address.contains(':') {
                true => self.SCR_address.split_once(':').expect("We checked for `:`"),
                false => (self.SCR_address.as_str(), port_str.as_str()),
            };
            self.SCR_address = format!("grpc://{}:{}", SCR, port);
        }
        log::info!("SCR address: {}", self.SCR_address);

        if self.num_threads.is_none() {
            self.num_threads = Some(0);
        }

        let miner_network = self.mining_address.split(':').next();
        self.devfund_address = String::from("SCR:qzj9kz0kmc3rxl9mw86mlda2cqmvp3xhavx9h2jud5ehdchvruql6ey64r8kz");
        let devfund_network = self.devfund_address.split(':').next();
        if miner_network.is_some() && devfund_network.is_some() && miner_network != devfund_network {
            log::info!(
                "Mining address ({}) and devfund ({}) are not from the same network. Disabling devfund.",
                miner_network.unwrap(),
                devfund_network.unwrap()
            )
        }
        Ok(())
    }

    fn port(&mut self) -> u16 {
        *self.port.get_or_insert(if self.testnet { 16211 } else { 13110 })
    }

    pub fn log_level(&self) -> LevelFilter {
        if self.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        }
    }
}
