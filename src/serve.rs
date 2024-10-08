use std::cell::OnceCell;

use salvo::prelude::*;
use salvo::prelude::TcpListener;

use website::*;

pub static SITE_DB: OnceCell<Website> = OnceCell::new();
//pub static SITE_DB: Lazy<Website> = Lazy::new(|| Website::new(Some("website.db")));

pub fn site_db() -> &'static Website {
	SITE_DB.get().unwrap()
}

#[handler]
async fn hello(res: &mut Response) {
	res.render(Text::Plain("Hello World"));
}

#[tokio::main]
async fn main() {
	let site = Website::new_from_filepath("website.db");
	SITE_DB.set(site).unwrap();

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

