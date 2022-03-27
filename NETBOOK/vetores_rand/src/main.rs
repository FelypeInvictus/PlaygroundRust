use rand::Rng;
use text_io::read;
fn main() {
	let mut vetor = [0;10];

	println!("GERADOR DE NUMEROS RANDÔMICOS\n\n\n");
	
	
	println!("Você gostaria de saber os numeros gerados? S/N");
	let esc: String = read!();

	if esc == "S" ||  esc == "s"{
		
		println!("Os numeros nos vetores são:");
		
			
			for i in 0..10{
				vetor[i] = rand::thread_rng().gen_range(1..100);
				print!("{},",vetor[i]);
			}
			
	} else {
		println!("ENCERRANDO PROGRAMA...");
	}
	
	
print!("\n");

}
