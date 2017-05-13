extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);

    loop {
	    println!("Please input your guess.");

	    let mut guess = String::new(); 
	    // - let: binding statement, guess is now variable.
	    // - mut: mutation flag. Immutable default for a binding.
	    // - ::new(): associated function. static method in general.
	    //     * static method: not change its behavior between instances.

	    io::stdin().read_line(&mut guess)
	        .expect("Failed to read line");
	    // - io::stdin(): handling user input.
	    // - .readline(): method, only availble for an instance.
	    // - &mut guess: references, &, should &mut since references are immutable by default.
	    // - .expect(): method of Result object, which is the return value of .readline().
	    //     - expect(): takes a value it's called on, and if it isn't successful one then return panic!
	    // - panic!: indication of the expression may cause a cruhs of the program.
	    //     - need to handle possible error. write error handling to surpuss this.

	    let guess: u32 = match guess.trim().parse() {
	    	Ok(num) => num,       // parse() has Ok data, which indicate success
	    	Err(_)  => continue,  // Err indicate failure
	    };
	    // - re-use the guess namne, and use it as u32 type
	    // - trim(): remove any white spaces
	    // - parse(): parse a string to some kind of number
	    // - let guess: u32: (:) after guess tells Rust we're going to annotate its type.
	 

	    println!("You guessed: {}", guess);
	    // - {}: is the pace holder fo the variable

	    match guess.cmp(&secret_number) {
	    	Ordering::Less    => println!("Too small!"),
	    	Ordering::Greater => println!("Too big!"),
	    	Ordering::Equal   => { 
	    		println!("You win!");
	    		break;
	    	}
	    }
	    // - cmp(): takes a ref to the thing you want to compare it to
	    // - Ordering: a type used earlier
	    // - match: determine what kind of Ordering it is.
	    // - Ordering is enum 
	    // Orering has possible variants: Less, Equal and Greater
	    // match takes a val of a type and lets you create an 'arm' for each possible value
	}
}

