use db::SurrealDB;
use anyhow::Result;

// This type is particularly useful in situations where you need to implement a trait that requires an error type
// but your implementation is guaranteed to never produce errors.
use std::convert::Infallible;

// lightweight web framework
use warp::{Filter, Rejection};

type WebResult<T> = Result<T, Rejection>;

mod db;
mod error;
mod handler;
mod try_froms;
mod prelude;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Book {
//     pub id: String,
//     pub name: String,
//     pub author: String,
//     pub num_pages: i32,
//     pub created_at: DateTime<Utc>,
//     pub tags: String
// }

#[tokio::main]
async fn main() -> Result<()> {
    let db: SurrealDB = SurrealDB::init().await?;
    let book_routes =
        books_list(db.clone())
            .or(books_create(db.clone()))
            .or(books_get(db.clone()))
            .or(books_delete(db));

    let routes = book_routes.recover(error::handle_rejection);
    println!("Started on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(db: SurrealDB) -> impl Filter<Extract = (SurrealDB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

/// GET /books
pub fn books_list(
    db: SurrealDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("books")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handler::books_list_handler)
}

/// POST /books with JSON body
pub fn books_create(
    db: SurrealDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("books")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handler::create_book_handler)
}

/// GET /books/:id
pub fn books_get(
    db: SurrealDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("books" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handler::get_book_handler)
}

/// DELETE /books/:id
pub fn books_delete(
    db: SurrealDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("books" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handler::delete_book_handler)
}
