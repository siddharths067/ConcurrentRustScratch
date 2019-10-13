use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex};
use std::env;
use std::thread;
use std::time::Duration;

fn main(){
	let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
	for arg in env::args().collect::<Vec<String>>(){
		let packet = arg.clone();
		let transmitter = tx.clone();
		thread::spawn(move || {
			transmitter.send(packet);
		}).join().expect(&format!("A thread panicked while sending {}", arg)[..]);
	}
	
	
	let receiver = Arc::new(Mutex::new(rx));
	let numrecvs: u32 = 3;
	let mut threads = Vec::with_capacity(numrecvs as usize);
	for i in 0..numrecvs{
		let rec = receiver.clone();
		threads.push(
			thread::spawn(move || {
				loop {
						{
							let val = rec.lock().unwrap().recv_timeout(Duration::from_millis(5000));
							match val {
								Ok(value) => println!("Argument Received {} by {}", value, i),
								_ => {
									println!("Timeout Reached {}", i);
									break;
								}
							}
							thread::sleep(Duration::from_millis(1000));
						}
				}
			})
		);
	}
	for i in threads{
		i.join().expect(&format!("Thread failed to join")[..]);
	}

}
