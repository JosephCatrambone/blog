use website::*;

use salvo::prelude::*;
use salvo::prelude::TcpListener;

#[handler]
async fn hello(res: &mut Response) {
	res.render(Text::Plain("Hello World"));
}

#[tokio::main]
async fn main() {
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

