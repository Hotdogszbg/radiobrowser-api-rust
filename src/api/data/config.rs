use crate::config::Config;
use std::error::Error;

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiConfig {
    pub check_enabled: bool,
    pub prometheus_exporter_enabled: bool,
    pub pull_servers: Vec<String>,
    pub tcp_timeout_seconds: u64,
    pub broken_stations_never_working_timeout_seconds: u64,
    pub broken_stations_timeout_seconds: u64,
    pub checks_timeout_seconds: u64,
    pub click_valid_timeout_seconds: u64,
    pub clicks_timeout_seconds: u64,
    pub mirror_pull_interval_seconds: u64,
    pub update_caches_interval_seconds: u64,
    pub server_name: String,
    pub check_retries: u8,
    pub check_batchsize: u32,
    pub check_pause_seconds: u64,
    pub api_threads: usize,
}

impl ApiConfig {
    pub fn serialize_config(config: ApiConfig) -> std::io::Result<String> {
        let mut xml = xml_writer::XmlWriter::new(Vec::new());
        xml.begin_elem("config")?;
        xml.elem_text("check_enabled", &config.check_enabled.to_string())?;
        xml.elem_text(
            "prometheus_exporter_enabled",
            &config.prometheus_exporter_enabled.to_string(),
        )?;
        {
            xml.begin_elem("pull_servers")?;
            for server in config.pull_servers {
                xml.elem_text("url", &server)?;
            }
            xml.end_elem()?;
        }
        xml.elem_text(
            "tcp_timeout_seconds",
            &config.tcp_timeout_seconds.to_string(),
        )?;
        xml.elem_text(
            "broken_stations_never_working_timeout_seconds",
            &config
                .broken_stations_never_working_timeout_seconds
                .to_string(),
        )?;
        xml.elem_text(
            "broken_stations_timeout_seconds",
            &config.broken_stations_timeout_seconds.to_string(),
        )?;
        xml.elem_text(
            "checks_timeout_seconds",
            &config.checks_timeout_seconds.to_string(),
        )?;
        xml.elem_text(
            "click_valid_timeout_seconds",
            &config.click_valid_timeout_seconds.to_string(),
        )?;
        xml.elem_text(
            "clicks_timeout_seconds",
            &config.clicks_timeout_seconds.to_string(),
        )?;
        xml.elem_text(
            "mirror_pull_interval_seconds",
            &config.mirror_pull_interval_seconds.to_string(),
        )?;
        xml.elem_text(
            "update_caches_interval_seconds",
            &config.update_caches_interval_seconds.to_string(),
        )?;
        xml.elem_text(
            "server_name",
            &config.server_name,
        )?;

        xml.elem_text("check_retries", &config.check_retries.to_string())?;
        xml.elem_text("check_batchsize", &config.check_batchsize.to_string())?;
        xml.elem_text("check_pause_seconds", &config.check_pause_seconds.to_string())?;
        xml.elem_text("api_threads", &config.api_threads.to_string())?;
        xml.end_elem()?;
        xml.close()?;
        xml.flush()?;
        Ok(String::from_utf8(xml.into_inner()).unwrap_or("encoding error".to_string()))
    }

    pub fn get_response(
        config: ApiConfig,
        format: &str,
    ) -> Result<rouille::Response, Box<dyn Error>> {
        Ok(match format {
            "json" => {
                let j = serde_json::to_string(&config)?;
                rouille::Response::text(j)
                    .with_no_cache()
                    .with_unique_header("Content-Type", "application/json")
            }
            "xml" => {
                let j = ApiConfig::serialize_config(config)?;
                rouille::Response::text(j)
                    .with_no_cache()
                    .with_unique_header("Content-Type", "text/xml")
            }
            _ => rouille::Response::empty_406(),
        })
    }
}

impl From<Config> for ApiConfig {
    fn from(item: Config) -> Self {
        ApiConfig {
            check_enabled: item.enable_check,
            prometheus_exporter_enabled: item.prometheus_exporter,
            pull_servers: item.servers_pull,
            tcp_timeout_seconds: item.tcp_timeout.as_secs(),
            broken_stations_never_working_timeout_seconds: item
                .broken_stations_never_working_timeout
                .as_secs(),
            broken_stations_timeout_seconds: item.broken_stations_timeout.as_secs(),
            checks_timeout_seconds: item.checks_timeout.as_secs(),
            click_valid_timeout_seconds: item.click_valid_timeout.as_secs(),
            clicks_timeout_seconds: item.clicks_timeout.as_secs(),
            mirror_pull_interval_seconds: item.mirror_pull_interval.as_secs(),
            update_caches_interval_seconds: item.update_caches_interval.as_secs(),
            server_name: item.source,
            check_retries: item.retries,
            check_batchsize: item.check_stations,
            check_pause_seconds: item.pause.as_secs(),
            api_threads: item.threads,
        }
    }
}
