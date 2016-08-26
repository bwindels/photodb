extern crate sqlite3;

use sqlite3::{
	DatabaseConnection,
	StatementUpdate
};

fn list_photos(conn: &mut DatabaseConnection) {
	let mut stmt = conn.prepare("SELECT id, filename, master_width, master_height FROM photos").unwrap();
	let mut rs = stmt.execute();

	let mut stop = false;
	while !stop {
		let row_option = rs.step().unwrap();
		match row_option {
			Some(ref row) => {
				let id = row.column_int64(0);
				let filename = row.column_str(1).unwrap();
				let width = row.column_int(2);
				let height = row.column_int(3);

				println!("photo, id: {}, filename: {:?}, {}x{}", id, filename, width, height);
			},
			None => {
				println!("end");
				stop = true;
			}
		}
	}
}

fn create_table(conn: &mut DatabaseConnection) {
	conn.exec("CREATE TABLE photos (
		id integer PRIMARY KEY AUTOINCREMENT,
		filename TEXT,
		lat REAL,
		long REAL,
		camera TEXT,
		master_height INTEGER,
		master_width INTEGER,
		thumbnail_height INTEGER,
		thumbnail_width INTEGER
	)").unwrap();
}

fn insert_photos(conn: &mut DatabaseConnection) {
	insert_photo(conn, "IMG_123.jpg", 800, 600);
	insert_photo(conn, "GREECE_1.jpg", 1280, 800);
	insert_photo(conn, "GREECE_2.jpg", 1440, 900);
	insert_photo(conn, "IMG_ITALY_23872.jpg", 640, 400);
}

pub fn insert_photo(conn: &mut DatabaseConnection, filename: &'static str, width: i64, height: i32) {
	let mut stmt = conn.prepare("INSERT INTO photos (filename, master_width, master_height) VALUES($1, $2, $3)").unwrap();
	let changes = stmt.update(&[&filename.as_bytes(), &width, &height]).unwrap();
	println!("insert caused {} changes", changes);
}
