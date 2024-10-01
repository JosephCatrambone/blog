#[allow(dead_code)]

use pulldown_cmark;
use rusqlite::{Connection, Result};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Website {
	db: Connection,
}

// Maybe consider https://crates.io/crates/rusqlite-from-row
#[derive(Debug, Clone)]
pub struct Post {
	id: i64,
	pub title: String,
	pub permalink: String,
	body_markdown: String,
	pub body_html: String,
	pub tags: Vec<String>,
	pub date_uploaded: u64, // Seconds since Unix epoch.
}

impl Website {
	pub fn new(filepath: Option<&String>) -> Self {
		// Get DB ref.
		let mut db = if let Some(path) = filepath {
			Connection::open(path)
		} else {
			Connection::open_in_memory()
		}.unwrap();

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
			)",
			(), // empty list of parameters.
		).expect("Failed to create posts table in database.");

		Website {
			db
		}
	}

	// Get:
	pub fn get_recent(&self, count: u8) -> Vec<Post> {
		vec![]
	}

	pub fn get_all(&self) -> Vec<Post> {
		vec![]
	}
	
	pub fn get_page_by_id(&self, id: i64) -> Option<Post> {
		let mut stmt = self.db.prepare("SELECT id, title, permalink, body_markdown, body_html, tags, date_uploaded FROM posts WHERE ?1;").expect("Failed to execute SQL in get_page_by_id");
		// TODO: Let us have drafts and multiple versions of blog posts.
		let post_iter = stmt.query_map([id,], |row| {
			Ok(Post {
				id: row.get(0).unwrap(),
				title: row.get(1).unwrap(),
				permalink: row.get(2).unwrap(),
				body_markdown: row.get(3).unwrap(),
				body_html: row.get(4).unwrap(),
				tags: row.get::<usize, String>(5).unwrap().split("\t").map(|t| String::from(t)).collect::<Vec<String>>(),
				date_uploaded: row.get(6).unwrap(),
			})
		}).expect("Failed to get iterator over posts.");

		for post in post_iter {
			return Some(post.unwrap().clone());
		}
		return None;
	}

	pub fn search_pages(&self, query: &String) -> Vec<(f32, Post)> {
		todo!()
	}

	// Update/Create:
	pub fn create_page(&mut self, title: String, permalink: String, body_markdown: String, tags: Vec<String>, save: bool) -> Post {
		let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
			Ok(n) => n.as_secs(),
			Err(_) => panic!("SystemTime before UNIX EPOCH!"),
		};

		// If save is False, we're previewing, so don't create the page in the DB.
		let mut post = Post {
			id: 0,
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
			self.db.execute(
				"INSERT INTO posts (title, permalink, body_markdown, body_html, tags, date_uploaded) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
				(&post.title, &post.permalink, &post.body_markdown, &post.body_html, &post.tags.join("\t"), &post.date_uploaded),
			).expect("Failed to insert POST.");
			post.id = self.db.last_insert_rowid();
		}
		post
	}

	pub fn update_page(&mut self, post: Post) {
		self.db.execute(
			"UPDATE posts SET title=?2, body_markdown=?3, body_html=?4, tags=?5 WHERE id=?1;", 
			(&post.id, &post.title, &post.body_markdown, &post.body_html, &post.tags.join("\t"))
		);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sanity() {
		let website = Website::new(None);
	}

	#[test]
	fn test_create_post() {
		let mut website = Website::new(None);
		for save in [false, true] {
			let post = website.create_page("Hi".to_string(), "test".to_string(), "**MARKDOWN!**".to_string(), vec![], save);
			assert_eq!(&post.body_html, "<p><strong>MARKDOWN!</strong></p>\n");
			let p2 = website.get_page_by_id(post.id);
			assert!(p2.is_some());
			assert_eq!(&post.id, &p2.unwrap().id);
		}
	}
}
