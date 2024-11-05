use std::collections::HashSet;

use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Feeds {
    pub feeds: Vec<Feed>,
}

#[derive(Clone, Deserialize)]
pub struct Feed {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub allow: HashSet<String>,
    pub deny: String,
    pub matchers: Vec<Matcher>,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Matcher {
    #[serde(rename = "equal")]
    Equal { path: String, value: String },

    #[serde(rename = "prefix")]
    Prefix { path: String, value: String },

    #[serde(rename = "sequence")]
    Sequence { path: String, values: Vec<String> },
}

#[derive(Clone)]
pub struct HttpPort(u16);

#[derive(Clone)]
pub struct CertificateBundles(Vec<String>);

#[derive(Clone)]
pub struct TaskEnable(bool);

#[derive(Clone)]
pub struct Config {
    pub version: String,
    pub http_port: HttpPort,
    pub external_base: String,
    pub database_url: String,
    pub certificate_bundles: CertificateBundles,
    pub consumer_task_enable: TaskEnable,
    pub vmc_task_enable: TaskEnable,
    pub plc_hostname: String,
    pub user_agent: String,
    pub zstd_dictionary: String,
    pub jetstream_hostname: String,
    pub feeds: Feeds,
}

impl Config {
    pub fn new() -> Result<Self> {
        let http_port: HttpPort = default_env("HTTP_PORT", "4050").try_into()?;
        let external_base = require_env("EXTERNAL_BASE")?;

        let database_url = default_env("DATABASE_URL", "sqlite://development.db");

        let certificate_bundles: CertificateBundles =
            optional_env("CERTIFICATE_BUNDLES").try_into()?;

        let jetstream_hostname = require_env("JETSTREAM_HOSTNAME")?;
        let zstd_dictionary = require_env("ZSTD_DICTIONARY")?;

        let consumer_task_enable: TaskEnable =
            default_env("CONSUMER_TASK_ENABLE", "true").try_into()?;

        let vmc_task_enable: TaskEnable = default_env("VMC_TASK_ENABLE", "true").try_into()?;

        let plc_hostname = default_env("PLC_HOSTNAME", "plc.directory");

        let default_user_agent = format!(
            "supercell ({}; +https://github.com/astrenoxcoop/supercell)",
            version()?
        );

        let user_agent = default_env("USER_AGENT", &default_user_agent);

        let feeds: Feeds = require_env("FEEDS")?.try_into()?;

        Ok(Self {
            version: version()?,
            http_port,
            external_base,
            database_url,
            certificate_bundles,
            consumer_task_enable,
            vmc_task_enable,
            plc_hostname,
            user_agent,
            jetstream_hostname,
            zstd_dictionary,
            feeds,
        })
    }
}

fn require_env(name: &str) -> Result<String> {
    std::env::var(name)
        .map_err(|err| anyhow::Error::new(err).context(anyhow!("{} must be set", name)))
}

fn optional_env(name: &str) -> String {
    std::env::var(name).unwrap_or("".to_string())
}

fn default_env(name: &str, default_value: &str) -> String {
    std::env::var(name).unwrap_or(default_value.to_string())
}

pub fn version() -> Result<String> {
    option_env!("GIT_HASH")
        .or(option_env!("CARGO_PKG_VERSION"))
        .map(|val| val.to_string())
        .ok_or(anyhow!("one of GIT_HASH or CARGO_PKG_VERSION must be set"))
}

impl TryFrom<String> for HttpPort {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(Self(80))
        } else {
            value.parse::<u16>().map(Self).map_err(|err| {
                anyhow::Error::new(err).context(anyhow!("parsing PORT into u16 failed"))
            })
        }
    }
}

impl AsRef<u16> for HttpPort {
    fn as_ref(&self) -> &u16 {
        &self.0
    }
}

impl TryFrom<String> for CertificateBundles {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .split(';')
                .filter_map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.to_string())
                    }
                })
                .collect::<Vec<String>>(),
        ))
    }
}

impl AsRef<Vec<String>> for CertificateBundles {
    fn as_ref(&self) -> &Vec<String> {
        &self.0
    }
}

impl AsRef<bool> for TaskEnable {
    fn as_ref(&self) -> &bool {
        &self.0
    }
}

impl TryFrom<String> for TaskEnable {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.parse::<bool>().map_err(|err| {
            anyhow::Error::new(err).context(anyhow!("parsing task enable into bool failed"))
        })?;
        Ok(Self(value))
    }
}

impl TryFrom<String> for Feeds {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let content = std::fs::read(value).map_err(|err| {
            anyhow::Error::new(err).context(anyhow!("reading feed config file failed"))
        })?;

        serde_yaml::from_slice(&content).map_err(|err| {
            anyhow::Error::new(err).context(anyhow!("parsing feeds into Feeds failed"))
        })
    }
}
