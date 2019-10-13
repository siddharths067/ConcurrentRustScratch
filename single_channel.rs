use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::env;
use std::thread;

fn main(){
	let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
	for arg in env::args().collect::<Vec<String>>(){
		let packet = arg.clone();
		let transmitter = tx.clone();
		thread::spawn(move || {
			transmitter.send(packet);
		}).join().expect(&format!("A thread panicked while sending {}", arg)[..]);
	}
	
	thread::spawn(move || {
		let mut iter = rx.iter();
		loop {
			let val = iter.next();
			if val != None {
				println!("{}", val.unwrap());
			}
		}
	}).join().expect("Receiver Panicked");

}
