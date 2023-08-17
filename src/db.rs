use crate::{handler::BookRequest, error::Error};
use crate::prelude::*;

use chrono::prelude::*;
use anyhow::Result;
use std::collections::BTreeMap;
use surrealdb::dbs::{Response, Session};
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Object, Value, Array, thing};
use std::sync::Arc;
use std::fmt;

const DB_NAME: &str = "books";
const ID: &str = "id";
const NAME: &str = "name";
const AUTHOR: &str = "author";
const NUM_PAGES: &str = "num_pages";
const CREATED_AT: &str = "created_at";
const TAGS: &str = "tags";
const DATA: &str = "data";
const TH: &str = "th";

#[derive(Clone)]
pub struct SurrealDB {
	pub ds: Arc<Datastore>,
	pub ses: Session
}

impl fmt::Debug for SurrealDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl SurrealDB {
	pub async fn init() -> Result<Self, Error> {
		let ds: Arc<Datastore> = Arc::new(Datastore::new("memory").await?);
		let ses: Session = Session::for_db(DB_NAME, DB_NAME);
	
		Ok(Self { ds, ses })
	}

	pub async fn fetch_books(&self) -> Result<Vec<Object>, Error>{
		let SurrealDB { ds, ses } = self;
		let sql: &str = "SELECT * FROM book";

		let query_response: Vec<Response> = ds.execute(sql, ses, None, false).await?;
		let first_response: Response = query_response.into_iter().next().expect("Did not get a query response");

		let result_array: Array = W(first_response.result?).try_into()?;

		result_array
			.into_iter()
			.map(|value| W(value).try_into())
			.collect()

	}

	pub async fn get_book(&self, tid: &String) -> Result<Object, Error> {
		let SurrealDB { ds, ses } = self;
        let sql: &str = "SELECT * FROM $th";
		let tid: String = format!("book:{}", tid);

		let vars: BTreeMap<String, Value> = [
			(TH.into(), thing(&tid)?.into()),
		].into();

		let ress: Vec<Response> = ds.execute(sql, ses, Some(vars), true).await?;

		let first_res: Response = ress.into_iter().next().expect("Did not get a response");

		W(first_res.result?.first()).try_into()
    }

	pub async fn create_book(&self, entry: &BookRequest) -> Result<Vec<Response>, Error> {
		let SurrealDB { ds, ses } = self;
		let sql: &str = "CREATE book CONTENT $data";

		let data: BTreeMap<String, Value> = [
			(NAME.into(), SurrealDB::into_db_value(entry.name.clone())),
			(AUTHOR.into(), SurrealDB::into_db_value(entry.author.clone())),
			(NUM_PAGES.into(), SurrealDB::into_db_value(entry.num_pages.to_string())),
			(CREATED_AT.into(), SurrealDB::into_db_value(Utc::now().to_string())),
			(TAGS.into(), SurrealDB::into_db_value(entry.tags.clone())),
		].into();

		let vars: BTreeMap<String, Value> = [
			(DATA.into(), data.into())
		].into();

		let query_response: Vec<Response> = ds.execute(sql, ses, Some(vars), false).await?;

		Ok(query_response)
	}

    pub async fn delete_book(&self, tid: &str) -> Result<String, Error> {
		let SurrealDB { ds, ses } = self;
		let sql: &str = "DELETE $th RETURN *";
		let tid: String = format!("book:{}", tid);

		let vars: BTreeMap<String, Value> = [
			(TH.into(), thing(&tid)?.into()),
		].into();
	
	
		let ress: Vec<Response> = ds.execute(sql, ses, Some(vars), false).await?;
		let first_res: Response = ress.into_iter().next().expect("id not returned");
	
		first_res.result?;
	
		Ok(tid)
	}
	
	fn into_db_value<T: Into<Value> + Clone>(value: T) -> Value {
		value.clone().into()
	}

}
