use serde::Serialize;
use warp::{http::StatusCode, reply, Rejection, Reply};
use std::convert::Infallible;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Value not of type '{0}'")]
	XValueNotOfType(&'static str),

	#[error("Generic Surrealdb error")]
	Surreal(surrealdb::err::Error),

	#[error(transparent)]
	IO(#[from] std::io::Error),

	/// Can not execute SELECT query using the specified value
	#[error("Can not execute SELECT query using value '{value}'")]
	SelectStatement {
		value: String,
	},

	/// Can not execute DELETE query using the specified value
	#[error("Can not execute DELETE query using value '{value}'")]
	DeleteStatement {
		value: String,
	},

	/// Can not execute CREATE query using the specified value
	#[error("Can not execute CREATE query using value '{value}'")]
	CreateStatement {
		value: String,
	},

	/// Can not execute UPDATE query using the specified value
	#[error("Can not execute UPDATE query using value '{value}'")]
	UpdateStatement {
		value: String,
	},
}

impl From<surrealdb::err::Error> for Error {
	fn from(val: surrealdb::err::Error) -> Self {
		Error::Surreal(val)
	}
}

#[derive(Serialize)]
struct ErrorResponse {
	message: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<Box<dyn Reply>, Infallible> {
	let code: StatusCode;
	let message: &str;

	if err.is_not_found() {
		code = StatusCode::NOT_FOUND;
		message = "Not Found";
	} else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
		code = StatusCode::BAD_REQUEST;
		message = "Invalid Body";
	} else if let Some(e) = err.find::<Error>() {
		match e {
			_ => {
				eprintln!("Unhandled application error: {:?}", err);
				code = StatusCode::INTERNAL_SERVER_ERROR;
				message = "Internal Server Error";
			}
		}
	} else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
		code = StatusCode::METHOD_NOT_ALLOWED;
		message = "Message Not Allowed";
	} else {
		eprintln!("Unhandled error: {:?}", err);
		code = StatusCode::INTERNAL_SERVER_ERROR;
		message = "Internal Server Error";
	}
	
	let json = reply::json(&ErrorResponse{
		message: message.into(),
	});

	Ok(Box::new(reply::with_status(json, code)))
}
