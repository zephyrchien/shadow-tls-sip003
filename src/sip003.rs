use super::{Args, Commands};
use anyhow::Result;
use std::env;

const OPT_THREADS: &str = "threads";
const OPT_TLS_NAME: &str = "tls_name";
const OPT_TLS_ADDR: &str = "tls_addr";

macro_rules! getopt {
    ($it: expr, $name: expr) => {
        $it.find(|kv| kv.starts_with($name))
            .and_then(|kv| kv.split_once("="))
            .map(|(_, v)| v.trim())
            .and_then(|v| if v.is_empty() { None } else { Some(v) })
            .and_then(|v| v.parse().ok())
    };
    ($s: expr => $name: expr) => {
        $crate::get_opt!($s.split(';').map(|x| x.trim()), $name)
    };
}

#[rustfmt::skip]
pub fn parse_env() -> Result<Args> {
    let local_host = env::var("SS_LOCAL_HOST")?;
    let local_port = env::var("SS_LOCAL_PORT")?;
    let remote_host = env::var("SS_REMOTE_HOST")?;
    let remote_port = env::var("SS_REMOTE_PORT")?;
    let plugin_opts = env::var("SS_PLUGIN_OPTIONS")?;

    let local = format!("{}:{}", local_host, local_port);
    let remote = format!("{}:{}", remote_host, remote_port);

    // handle options
    let it = plugin_opts.split(';').map(str::trim);
    let threads = getopt!(it.clone(), OPT_THREADS);
    let tls_name = getopt!(it.clone(), OPT_TLS_NAME);
    let tls_addr = getopt!(it.clone(), OPT_TLS_ADDR);

    Ok(match (tls_name, tls_addr) {
        (None, None) | (Some(_), Some(_)) => todo!(),
        (Some(tls_name), None) => Args {
            threads,
            cmd: Commands::Client { listen: local, server_addr: remote, tls_name },
        },
        (None, Some(tls_addr)) => Args {
            threads,
            cmd: Commands::Server { listen: remote, server_addr: local, tls_addr }
        },
    })
}
