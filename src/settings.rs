use std::{env, net::IpAddr};
use config::{ConfigError, Config, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Service {
	pub host: IpAddr,
	pub port: u16,
//	tls: bool,
//	tls_key: String,
//	tls_crt: String,
	pub homedir: String,
	pub idxfile: String,
}

/*#[derive(Debug, Deserialize)]
pub struct Cache {
	pub size: usize, //B
	pub maxfilesize: usize, //B
}*/

#[derive(Debug, Deserialize)]
pub struct Settings {
	pub service: Service,
//	pub cache: Cache,
}

impl Settings {
	pub fn new() -> Result<Self, ConfigError> {
		let mut s = Config::new();
		env::set_var("RUST_APP_LOG", "warn");
		
		s.set("service.host", "::")?;
		s.set("service.port", 80)?;
		s.set("service.homedir", "/var/www")?;
		s.set("service.idxfile", "index.html")?;
/*		s.set("cache.size", 536_870_912)?;
		s.set("cache.maxfilesize", 2_097_152)?;*/
		
		s.merge(File::with_name("/etc/gws.toml").required(false))?;
		
		s.merge(File::with_name("config/gws.toml").required(false))?;
		
		s.merge(Environment::with_prefix("gws"))?;
		
		s.try_into()
	}
}
