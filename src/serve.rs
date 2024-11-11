use std::sync::OnceLock;
use std::sync::{Arc, Mutex};
use std::env;

use salvo::prelude::*;
use salvo::prelude::TcpListener;

use website::*;

pub static SITE_DB: OnceLock<Arc<Mutex<Website>>> = OnceLock::new(); // Arc::new(Mutex::new(Website::new(Some("website.db"))));
//pub static SITE_DB: Lazy<Website> = Lazy::new(|| Website::new(Some("website.db")));

/*
pub fn site_db() -> &'static Website {
	SITE_DB.get().unwrap()
}
*/

pub struct ServerConfig {
	pub db_file_path: String,
	pub port: u16,
}

impl ServerConfig {
	pub fn from_env(args: &[String]) -> Result<Self, &'static str> {
		//let db_name = env::var("DB_FILE_PATH").is_ok();
		todo!()
	}
}

#[handler]
async fn hello(res: &mut Response) {
	let mut db_lock = SITE_DB.get().expect("Failed to get OnceLock -> ARC for DB.").lock().expect("Lock DB failed.");
	res.render(Text::Plain("Hello World"));
}

#[tokio::main]
async fn main() {
	let site = Arc::new(Mutex::new(Website::new(Some("website.db"))));
	SITE_DB.set(site).expect("Unable to set static reference to site DB.");

	let mut router = Router::new().get(hello);
	let listener = TcpListener::new("0.0.0.0:443")
		.acme()
		.add_domain("josephcatrambone.com")
		.http01_challege(&mut router)
		.quinn("0.0.0.0:443");
	let acceptor = listener.join(TcpListener::new("0.0.0.0:80")).bind().await;
	Server::new(acceptor).serve(router).await;
}

/*
pub fn main() {
	println!("What up?");
	let _site = Website::new();
}
*/

