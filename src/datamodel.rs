
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PostID(u64);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Post {
	pub id: PostID,
	pub title: String,
	pub markdown: String, // We don't store the final HTML of the site.  It's parsed and generated on command.  TODO: Cache in DB?
	pub tags: Vec<String>, // Stored as comma-separated string
	pub posted: u64,
	pub visible: bool,
	pub parent: Option<PostID>,
}

#[cfg(feature = "ssr")]
pub mod db_functions {
	// use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
	use leptos::ServerFnError;
	use rusqlite::{Connection, Result};

	/*
	pub async fn db() -> Result<SqliteConnection, ServerFnError> {
		Ok(SqliteConnection::connect("sqlite:Todos.db").await?)
	}
	*/
	
	//let conn = Connection::open(path.into())
	//c = Connection::open_in_memory()?:

	pub fn make_post_table(db: &mut Connection) -> Result<()> {
		conn.execute(
			"CREATE TABLE post (
				id INTEGER PRIMARY KEY,
				title TEXT NOT NULL,
				markdown TEXT NOT NULL,
				tags TEXT NOT NULL,
				data BLOB
			)",
			(), // empty list of parameters.
		)?;
		let me = datamodel::Post {
			id: 0,
			name: "Steven".to_string(),
			data: None,
		};
		conn.execute(
			"INSERT INTO person (name, data) VALUES (?1, ?2)",
			(&me.name, &me.data),
		)?;
	
		let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
		let person_iter = stmt.query_map([], |row| {
			Ok(Person {
				id: row.get(0)?,
				name: row.get(1)?,
				data: row.get(2)?,
			})
		})?;
	
		for person in person_iter {
			println!("Found person {:?}", person.unwrap());
		}
		Ok(())
	}

}