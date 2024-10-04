#[allow(dead_code)]

use anyhow::Result as AResult;
use pulldown_cmark;
use rusqlite::{Connection, Result, Row};
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Website {
	db: Connection,
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
	pub fn new<T>(filepath: Option<T>) -> Self where T: AsRef<Path> {
		// Get DB ref.
		let db = if let Some(path) = filepath {
			Connection::open::<T>(path.into())
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
		let mut stmt = self.db.prepare("SELECT id, title, permalink, body_markdown, body_html, tags, date_uploaded FROM posts ORDER BY date_uploaded DESC LIMIT ?1;").expect("Failed to execute SQL in get_recent.");
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
		let mut stmt = self.db.prepare("SELECT id, title, permalink, body_markdown, body_html, tags, date_uploaded FROM posts WHERE ?1;").expect("Failed to execute SQL in get_page_by_id");
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

	pub fn search_pages(&self, query: &String) -> Vec<(f32, Post)> {
		todo!()
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
			self.db.execute(
				"INSERT INTO posts (title, permalink, body_markdown, body_html, tags, date_uploaded) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
				(&post.title, &post.permalink, &post.body_markdown, &post.body_html, &post.tags, &post.date_uploaded),
			).expect("Failed to insert POST.");
			post.id = self.db.last_insert_rowid();
		}
		post
	}

	pub fn update_page(&mut self, post: Post) {
		self.db.execute(
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
		let website = Website::new(None);
	}

	#[test]
	fn test_create_post() {
		let mut website = Website::new(None);
		for save in [false, true] {
			let post = website.create_page("Hi".to_string(), "test".to_string(), "**MARKDOWN!**".to_string(), "".to_string(), save);
			assert_eq!(&post.body_html, "<p><strong>MARKDOWN!</strong></p>\n");
		}
	}

	#[test]
	fn test_load_post() {
		let mut website = Website::new(None);
		let post1 = website.create_page("Hi".to_string(), "asdf".to_string(), "asdf".to_string(), "".to_string(), true);
		let post2 = website.get_page_by_id(post1.id);
		assert!(post2.is_some());
		assert_eq!(&post1.id, &post2.unwrap().id);
	}
}
