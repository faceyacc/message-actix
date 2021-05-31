#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::usize;

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

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

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
	let request_count = state.request_count.get() + 1;
	state.request_count.set(request_count);
	let ms = state.messages.lock().unwrap();
	
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
		println!("Starting http server: 127.0.0.1:{}", self.port);
		HttpServer::new(move || {
			App::new()
				.wrap(middleware::Logger::default())
				.service(index)
		})
		.bind(("127.0.0.1", self.port))?
		.workers(8)
		.run()
	}
}
