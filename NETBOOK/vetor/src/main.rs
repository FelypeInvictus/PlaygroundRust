//use text_io::read;

use std::io;

fn main() {

	let mut vet=[0;5];
	let mut read = i32::new();
	

	for i in 0..4 {
		vet[i] = io::stdin().read_line(&mut read).expect("A leitura falhou");
	}

	
    println!("Os dados no vetor são: {:?}",vet);
}
