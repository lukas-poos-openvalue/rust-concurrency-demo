use std::sync::*;
use std::cell::*;
use std::rc::*;

fn main() {
	let value = 0;
	let value = RefCell::new(value);
	let value = Rc::new(value);
	let mut thread_vec = vec![];

	for _ in 0..10 {
		let mut thread_value = value.clone();
		let thread = std::thread::spawn(move || {
			for _ in 0..10000 {
				let thread_value_cell = *thread_value.borrow();
            	thread_value_cell = thread_value_cell + 1;
			}
		});
		thread_vec.push(thread);
	}

	for thread in thread_vec {
		thread.join().unwrap();
	}

	println!("Value of counter.value: {}", value.borrow());
}

