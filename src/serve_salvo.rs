#[allow(dead_code)]
use std::collections::HashMap;
use std::env;
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};

use handlebars::Handlebars;
use salvo::prelude::*;
use salvo::prelude::TcpListener;
use salvo::server::ServerHandle;
use tokio::signal;

use website::*;

pub static DEFAULT_STYLE: &'static str = include_str!("../style/main.css");
pub static INDEX_TEMPLATE: &'static str = include_str!("../templates/index.html");
pub static THREE_PANE_LAYOUT_TEMPLATE: &'static str = include_str!("../templates/three_pane_layout.html");
pub static SITE_DB: OnceLock<Arc<Mutex<Website>>> = OnceLock::new(); // Arc::new(Mutex::new(Website::new(Some("website.db"))));
//pub static SITE_DB: Lazy<Website> = Lazy::new(|| Website::new(Some("website.db")));
pub static TEMPLATE_ENGINE: OnceLock<Arc<Mutex<Handlebars>>> = OnceLock::new();

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
async fn full_hello(_req: &mut Request, _depot: &mut Depot, res: &mut Response, _ctrl: &mut FlowCtrl) {
	res.render("Hello world");
}

#[handler]
async fn hello(res: &mut Response) {
	let mut db_lock = SITE_DB.get().expect("Failed to get OnceLock -> ARC for DB.").lock().expect("Lock DB failed.");
	res.render(Text::Plain("Hello World"));
}

#[handler]
async fn get_style(res: &mut Response) {
	res.render(Text::Css(DEFAULT_STYLE));
}

#[handler]
async fn index(req: &mut Request, res: &mut Response) {
	if let Some(id) = req.param::<i64>("p") {  // Legacy page_id.  Forward to blog=.
		res.render(Redirect::permanent(format!("/blog/{}", &id)));
	} else if let Some(id) = req.query::<i64>("p") {
		res.render(Redirect::permanent(format!("/blog/{}", &id)));
	} else {
		res.render(Text::Html("Blog page unspecified."));
	}
}

#[handler]
async fn find_blog_page(req: &mut Request, res: &mut Response) {
	if let Some(query) = req.param::<String>("q") {

	}
}

#[handler]
async fn get_blog_page(req: &mut Request, res: &mut Response) {
	let id: Option<i64> = req.param::<i64>("id");
	let mut handlebars = TEMPLATE_ENGINE.get().expect("Failed to acquire template engine.").lock().expect("Failed to lock template engine.");
	let mut data = HashMap::new();
	if let Some(id) = id {
		let mut db_lock = SITE_DB.get().expect("Failed to get OnceLock -> ARC for DB.").lock().expect("Lock DB failed.");
		let page = db_lock.get_page_by_id(id);
		if let Some(page_data) = page {
			data.insert("left_content", page_data.title);
			data.insert("main_content", page_data.body_html);
		} else {
			//data.insert("title", "Page Not Found".into());
			data.insert("main_content", "Couldn't find a blog with the specified ID.".into());
			res.status_code = Some(StatusCode::from_u16(404).unwrap());
		}
	} else {
		data.insert("main_content", "No Blog ID specified.".into());
		res.status_code = Some(StatusCode::from_u16(400).unwrap());
	}
	res.render(Text::Html(handlebars.render("three_pane_layout", &data).unwrap()));
}

#[tokio::main]
async fn main() {
	let site = Arc::new(Mutex::new(Website::new_from_filepath("website.db")));
	SITE_DB.set(site).expect("Unable to set static reference to site DB.");
	let mut handlebars = Handlebars::new();
	//handlebars.register_template_string("blog", BLOG_TEMPLATE).unwrap();
	handlebars.register_template_file("three_pane_layout", "./templates/three_pane_layout.html").unwrap();
	handlebars.register_template_string("index", INDEX_TEMPLATE).unwrap();
	TEMPLATE_ENGINE.set(Arc::new(Mutex::new(handlebars))).expect("Failed to set static reference to template engine.");

	let mut router = Router::with_path("/")
		.get(index)
		.push(Router::with_path("style/main.css").get(get_style))
		.push(Router::with_path("blog/<id>").get(get_blog_page))
		.push(Router::with_path("search").get(find_blog_page));
	/*
		//.post(create_writer)
		.push(
			Router::with_path("blog")
				.get()

		)
		.push(
			Router::with_path("<id>")
				.get(show_writer)
				.patch(edit_writer)
				.delete(delete_writer)
				.push(Router::with_path("articles").get(list_writer_articles)),
		);
	*/
	//Service::new(router).catcher(Catcher::default().hoop(handle404));
	let listener = TcpListener::new("0.0.0.0:443")
		.acme()
		.add_domain("josephcatrambone.com")
		.http01_challenge(&mut router)
		.quinn("0.0.0.0:443");
	let acceptor = listener.join(TcpListener::new("0.0.0.0:80")).bind().await;

	let server = Server::new(acceptor);
	let handle = server.handle();

	// Listen Shutdown Signal
	tokio::spawn(listen_shutdown_signal(handle));

	server.serve(router).await;
}

async fn listen_shutdown_signal(handle: ServerHandle) {
	// Wait Shutdown Signal
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(windows)]
	let terminate = async {
		signal::windows::ctrl_c()
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	tokio::select! {
        _ = ctrl_c => println!("ctrl_c signal received"),
        _ = terminate => println!("terminate signal received"),
    };

	// Graceful Shutdown Server
	handle.stop_graceful(None);
}
