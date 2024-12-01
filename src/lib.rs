#[allow(dead_code)]

use anyhow::Result as AResult;
use pulldown_cmark;
use rusqlite::{Connection, Result, Row};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Website {
	db: Arc<Mutex<Connection>>,
}

// Maybe consider https://crates.io/crates/rusqlite-from-row
#[derive(Clone, Debug)]
pub struct Post {
	id: i64,
	pub title: String,
	pub permalink: String,
	body_markdown: String,
	pub body_html: String,
	pub tags: String,
	pub date_uploaded: u64, // Seconds since Unix epoch.
}

#[derive(Clone, Debug)]
pub struct SearchResult {
	score: f64,
	pub post_id: i64,
	pub title: String,
	pub body_html_excerpt: String,
	pub tags: String,
}

fn post_from_row(row: &Row) -> AResult<Post> {
	Ok(Post {
		id: row.get(0)?,
		title: row.get(1)?,
		permalink: row.get(2)?,
		body_markdown: row.get(3)?,
		body_html: row.get(4)?,
		tags: row.get(5)?,
		date_uploaded: row.get(6)?,
	})
}

impl Website {
	fn new_from_db(db: Connection) -> Self {
		// Check for tables and maybe create them.
		db.execute(
			"CREATE TABLE IF NOT EXISTS posts (
				id INTEGER PRIMARY KEY, -- If AUTOINCREMENT is specified a slightly different algorithm is used to pick ID.
				title TEXT NOT NULL,
				permalink TEXT NOT NULL,
				body_markdown TEXT NOT NULL,
				body_html TEXT NOT NULL,
				tags TEXT NOT NULL,
				-- data BLOB
				date_uploaded INTEGER -- UTC epoch
			);",
			(), // empty list of parameters.
		).expect("Failed to create posts table in database.");

		// Should we use trigram search for better code matching?
		// CREATE VIRTUAL TABLE search_index USING fts5(..., tokenize="trigram"); ?
		db.execute(
			"CREATE VIRTUAL TABLE search_index USING fts5(post_id, title, body_html, tags);",
			()
		);

		Website {
			db: Arc::new(Mutex::new(db))
		}
	}

	pub fn new_in_memory() -> Self {
		Self::new_from_db(Connection::open_in_memory().unwrap())
	}

	pub fn new_from_filepath<T>(filepath: T) -> Self where T: AsRef<Path> {
		Self::new_from_db(Connection::open::<T>(filepath.into()).unwrap())
	}

	// Get:
	pub fn get_recent(&self, count: u8) -> Vec<Post> {
		let mut db_ref = self.db.lock().expect("RWLock poisoned.");
		let mut stmt = db_ref.prepare("SELECT id, title, permalink, body_markdown, body_html, tags, date_uploaded FROM posts ORDER BY date_uploaded DESC LIMIT ?1;").expect("Failed to execute SQL in get_recent.");
		let post_iter = stmt.query_map([count,], |row| {
			Ok(Post {
				id: row.get(0).unwrap(),
				title: row.get(1).unwrap(),
				permalink: row.get(2).unwrap(),
				body_markdown: row.get(3).unwrap(),
				body_html: row.get(4).unwrap(),
				tags: row.get(5).unwrap(),
				date_uploaded: row.get(6).unwrap(),
			})
		}).expect("Failed to fetch recent queries from DB.");

		let mut posts: Vec<Post> = vec![];
		for post in post_iter {
			posts.push(post.unwrap().clone());
		}
		posts
	}

	pub fn get_all(&self) -> Vec<Post> {
		vec![]
	}
	
	pub fn get_page_by_id(&self, id: i64) -> Option<Post> {
		let db_ref = self.db.lock().expect("RWLock poisoned.");
		let mut stmt = db_ref.prepare("SELECT id, title, permalink, body_markdown, body_html, tags, date_uploaded FROM posts WHERE id=?1;").expect("Failed to execute SQL in get_page_by_id");
		// TODO: Let us have drafts and multiple versions of blog posts.
		// We could perhaps use query_row for this if we didn't care about the 404 case.
		let post_iter = stmt.query_map([id,], |row| {
			Ok(Post {
				id: row.get(0).unwrap(),
				title: row.get(1).unwrap(),
				permalink: row.get(2).unwrap(),
				body_markdown: row.get(3).unwrap(),
				body_html: row.get(4).unwrap(),
				//tags: row.get::<usize, String>(5).unwrap().split("\t").map(|t| String::from(t)).collect::<Vec<String>>(),
				tags: row.get(5).unwrap(),
				date_uploaded: row.get(6).unwrap(),
			})
		}).expect("Failed to get iterator over posts.");

		for post in post_iter {
			return Some(post.unwrap().clone());
		}
		return None;
	}

	pub fn search_pages_basic(&self, query: &String, limit: u8) -> Vec<SearchResult> {
		let db_ref = self.db.lock().expect("RWLock poisoned.");
		let mut stmt = db_ref.prepare(
			"SELECT rank, post_id, posts.title, posts.body_html, posts.tags FROM search_index JOIN posts ON posts.id = post_id WHERE search_index MATCH ?1 ORDER BY RANK LIMIT ?2"
		).expect("Failed to execute SQL in get_page_by_id");
		let post_iter = stmt.query_map([query, &limit.to_string()], |row| {
			Ok(SearchResult {
				score: row.get(0).unwrap(),
				post_id: row.get(1).unwrap(),
				title: row.get(2).unwrap(),
				body_html_excerpt: row.get(3).unwrap(),
				tags: row.get(4).unwrap(),
			})
		}).expect("Failed to get iterator over posts.");

		post_iter.flat_map(|r| if let Ok(res) = r { Some(res) } else { None }).collect::<Vec<SearchResult>>()
	}

	// Update/Create:
	pub fn create_page(&mut self, title: String, permalink: String, body_markdown: String, tags: String, save: bool) -> Post {
		let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
			Ok(n) => n.as_secs(),
			Err(_) => panic!("SystemTime before UNIX EPOCH!"),
		};

		// If save is False, we're previewing, so don't create the page in the DB.
		let mut post = Post {
			id: -1,
			title,
			permalink,
			body_markdown,
			body_html: String::new(),
			tags,
			//date_uploaded: Instant::now(),
			date_uploaded: now,
		};

		// Create parser with example Markdown text.
		let parser = pulldown_cmark::Parser::new(&post.body_markdown);
		pulldown_cmark::html::push_html(&mut post.body_html, parser);
		//assert_eq!(&post.body_html, "<p>hello world</p>\n");

		if save {
			let mut db_ref = self.db.lock().expect("Failed to grab write lock from RwLock. RwLock poisoned!?");
			db_ref.execute(
				"INSERT INTO posts (title, permalink, body_markdown, body_html, tags, date_uploaded) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
				(&post.title, &post.permalink, &post.body_markdown, &post.body_html, &post.tags, &post.date_uploaded),
			).expect("Failed to insert POST.");
			post.id = db_ref.last_insert_rowid();
		}
		post
	}

	pub fn reindex_search(&mut self) {
		let mut db_ref = self.db.lock().expect("Failed to get DB write lock. Lock poisoned.");
		db_ref.execute("DELETE FROM search_index;", ()).expect("Failed to clear previous search index.");
		db_ref.execute("INSERT INTO search_index (post_id, title, body_html, tags) SELECT posts.id, posts.title, posts.body_html, posts.tags FROM posts;", ()).expect("Failed to recompute search index.");
	}

	pub fn update_page(&mut self, post: Post) {
		let mut db_ref = self.db.lock().expect("Failed to get DB write lock. Lock poisoned.");
		db_ref.execute(
			"UPDATE posts SET title=?2, body_markdown=?3, body_html=?4, tags=?5 WHERE id=?1;", 
			(&post.id, &post.title, &post.body_markdown, &post.body_html, &post.tags)
		).expect("Failed to update post.");
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sanity() {
		let website = Website::new_in_memory();
	}

	#[test]
	fn test_create_post() {
		let mut website = Website::new_in_memory();
		for save in [false, true] {
			let post = website.create_page("Hi".to_string(), "test".to_string(), "**MARKDOWN!**".to_string(), "".to_string(), save);
			assert_eq!(&post.body_html, "<p><strong>MARKDOWN!</strong></p>\n");
		}
	}

	#[test]
	fn test_load_post() {
		let mut website = Website::new_in_memory();
		let post1 = website.create_page("Hi".to_string(), "asdf".to_string(), "asdf".to_string(), "".to_string(), true);
		let post2 = website.get_page_by_id(post1.id);
		assert!(post2.is_some());
		assert_eq!(&post1.id, &post2.unwrap().id);
	}

	#[test]
	fn test_basic_search() {
		let mut website = Website::new_in_memory();
		_ = website.create_page("Hi".to_string(), "test".to_string(), "**MARKDOWN!**".to_string(), "".to_string(), true);
		_ = website.create_page("What is up?".to_string(), "whazzaaaap".to_string(), "whazzaaaap".to_string(), "".to_string(), true);
		let test_post = website.create_page("test".to_string(), "test".to_string(), "test".to_string(), "".to_string(), true);
		website.reindex_search();
		let search_results = website.search_pages_basic(&"test".to_string(), 2);
		assert_eq!(search_results[0].post_id, test_post.id);
		println!("Best match for 'test': {}", &search_results[0].title);
	}
}
