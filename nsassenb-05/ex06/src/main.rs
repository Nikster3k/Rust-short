use core::task;
use std::thread::{spawn, JoinHandle};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc, RwLock};
use std::net::{TcpListener, SocketAddr};
use std::io::{Error, Read, Write};
use std::result::Result;

type Task = Box<dyn 'static + Send + FnOnce()>;

struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
    should_stop: Arc<RwLock<bool>>,
    task_sender: Sender<Task>,
}

struct ERROR;

impl ThreadPool {
    fn new(thread_count: usize) -> Self {
		let (sender, reciever) = channel();
		let mut tp = ThreadPool{threads: Vec::with_capacity(thread_count), should_stop: Arc::new(RwLock::new(false)), task_sender: sender};
		let wrapped_recv = Arc::new(reciever);
		for _ in 0..thread_count {
			let stop_condition = Arc::clone(&tp.should_stop);
			let task_reciever = Arc::clone(&wrapped_recv);
			tp.threads.push(spawn(move || {
				println!("waiting!");

				while !*stop_condition.read().unwrap() {
					if let Ok(task) = task_reciever.recv_timeout(std::time::Duration::from_secs(1)) {
						task();
					}
				}
			}));
		}
		tp
	}

    fn spawn_task<F>(self, task: F) -> Result<(), ERROR> 
    where
        F: 'static + Send + FnOnce() {
			self.task_sender.send(Box::new(task)).map_err(|_| return ERROR);
			Ok(())
		}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		todo!()
	}
}


fn main() {
	let mut pool = ThreadPool::new(10);
	let mut conn = match TcpListener::bind("127.0.0.1:8080") {
		Ok(connection) => connection,
		Err(_) => {
			println!("Failed to bind to ip 127.0.0.1:8080");
			return;
		}
	};

	loop {
		for stream in conn.incoming() {
			match stream {
				Ok(client) => {
					let pool_r = &pool;
					match pool_r.spawn_task(move || {
						client.write_all(b"This page does not exist :(").unwrap();
					}) {
						Ok(_) => (),
						Err(_) => println!("Error spawning task!")
					}
				},
				Err(_) => ()
			}
		}
	}
}
