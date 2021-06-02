#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
// use serde::Serialize;
use std::cell::Cell;
use std::os::macos::raw::stat;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::usize;
use serde::{Deserialize, Serialize};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Deserialize)]
struct PostInput {
	message: String,
}

#[derive(Serialize)]
struct PostResponse {
	server_id: usize, 
	request_count: usize, 
	message: String,
}


// Holds the state of app.
struct AppState {
	server_id: usize, 
	request_count: Cell<usize>,
	messages: Arc<Mutex<Vec<String>>>,
}
pub struct MessageApp {
	port: u16,
}

// Makes IndexResponse struct derive the Serialize trait.
// The derive attribute allows you to implemnt traits for types

#[derive(Serialize)]
struct IndexResponse {
	server_id: usize, 
	request_count: usize, 
	messages: Vec<String>,
}


 
fn post(msg: web::Json<PostInput>, state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {
	let request_count = state.request_count.get() + 1;
	state.request_count.set(request_count);

	let mut ms = state.messages.lock().unwrap();
	ms.push(msg.message.clone());

	Ok(web::Json(PostResponse {
		server_id: state.server_id,
		request_count, 
		message: msg.message.clone(),

	}))

}



// Notice that handler takes state as a parameter.
#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
	let request_count = state.request_count.get() + 1;
	state.request_count.set(request_count);
	let ms = state.messages.lock().unwrap();
	
	// Returns JSON response.
	Ok(web::Json(IndexResponse {
		server_id: state.server_id,
		request_count, 
		messages: ms.clone(),
	} ))
}


impl MessageApp {
	pub fn new(port: u16) -> Self {
		MessageApp { port }
	}

	pub fn run(&self) -> std::io::Result<()> {
		let messages = Arc::new(Mutex::new(vec![]));
		println!("Starting http server: 127.0.0.1:{}", self.port);
		HttpServer::new(move || {
			App::new()
				.data(AppState {
					server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst), 
					request_count: Cell::new(0),
					messages:messages.clone(), 
				})
				.wrap(middleware::Logger::default())
				.service(index)
		})
		.bind(("127.0.0.1", self.port))?
		.workers(8)
		.run()
	}
}
