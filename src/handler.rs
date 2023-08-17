use crate::{db::SurrealDB, WebResult, error::Error};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRequest {
	pub name: String,
	pub author: String,
	pub num_pages: i32,
	pub tags: Vec<String>,
}

pub async fn books_list_handler(db: SurrealDB) -> WebResult<impl Reply> {
    let books: Vec<surrealdb::sql::Object> = db
        .fetch_books()
        .await
        .map_err(|e| reject::custom(Error::SelectStatement { value: e.to_string() }))?;

    Ok(json(&books))
}

pub async fn create_book_handler(body: BookRequest, db: SurrealDB) -> WebResult<impl Reply> {
	db
		.create_book(&body)
		.await
		.map_err(|e| reject::custom(Error::CreateStatement { value: e.to_string() }))?;

	Ok(StatusCode::CREATED)
}

pub async fn get_book_handler(tid: String, db: SurrealDB) -> WebResult<impl Reply> {
    let book: surrealdb::sql::Object = db
        .get_book(&tid)
        .await?;

    Ok(json(&book))
}

pub async fn delete_book_handler(tid: String, db: SurrealDB) -> WebResult<impl Reply> {
	db.delete_book(&tid).await?;
	Ok(StatusCode::OK)
}
