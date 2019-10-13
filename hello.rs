use std::env;
fn main(){
	for s in env::args().collect::<Vec<String>>() {
		println!("{}", s);
	}
}
