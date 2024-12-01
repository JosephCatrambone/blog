#[allow(dead_code)]
use std::collections::HashMap;
use std::env;
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};

use handlebars::Handlebars;
use salvo::prelude::*;
use salvo::prelude::TcpListener;

use website::*;

pub static INDEX_TEMPLATE: &'static str = include_str!("../templates/index.html");
pub static BLOG_TEMPLATE: &'static str = include_str!("../templates/blog.html");
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
			data.insert("title", page_data.title);
			data.insert("body", page_data.body_html);
		} else {
			data.insert("title", "Page Not Found".into());
			data.insert("body", "Couldn't find a blog post with the specified ID.".into());
			res.status_code = Some(StatusCode::from_u16(404).unwrap());
		}
	} else {
		data.insert("title", "Bad Request".into());
		data.insert("body", "No Blog ID specified.".into());
		res.status_code = Some(StatusCode::from_u16(400).unwrap());
	}
	res.render(Text::Html(handlebars.render("blog", &data).unwrap()));
}

#[tokio::main]
async fn main() {
	let site = Arc::new(Mutex::new(Website::new_from_filepath("website.db")));
	SITE_DB.set(site).expect("Unable to set static reference to site DB.");

	let mut handlebars = Handlebars::new();
	//handlebars.register_template_string("blog", BLOG_TEMPLATE).unwrap();
	handlebars.register_template_file("blog", "./templates/blog.html").unwrap();
	handlebars.register_template_string("index", INDEX_TEMPLATE).unwrap();
	TEMPLATE_ENGINE.set(Arc::new(Mutex::new(handlebars))).expect("Failed to set static reference to template engine.");

	let mut router = Router::with_path("/")
		.get(index)
		.push(
			Router::with_path("blog/<id>")
				.get(get_blog_page)
		)
		.push(
			Router::with_path("search")
				.get(find_blog_page)
		);
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
		.http01_challege(&mut router)
		.quinn("0.0.0.0:443");
	let acceptor = listener.join(TcpListener::new("0.0.0.0:80")).bind().await;
	Server::new(acceptor).serve(router).await;
}
