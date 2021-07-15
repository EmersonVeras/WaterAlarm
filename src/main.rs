use std::{io};
mod  tempo;
fn main(){

    println!("------------------");
    println!("   WaterAlarm");
    println!("------------------");
    let mut peso = String::new();
    
    println!("Digite seu peso:");
    io::stdin().read_line(&mut peso).expect("Falha ao ler entrada");
    let peso: f32 = peso.trim().parse().expect("Por favor, digite seu peso:");
    
    let water:f32 = peso * 0.045;
    println!("Você tem que beber {}L de água.", water);
    
    let mut water_division:f32 = (water/8.0)*1000.0;
    water_division = water_division.round();
    println!("Você deve beber beber {}ml de água a cada 3h.", water_division);
    
    // print!("\x1B[2J\x1B[1;1H");
    println!("Digite a hora que você hora_acorda!");
    let mut hora_acorda: String = String::new();
    io::stdin().read_line(&mut hora_acorda).expect("Falha ao ler Horário de acordar!");
    let mut hora_acorda: u32 = hora_acorda.trim().parse().expect("Por favor digite o horário que você hora_acorda!");
    
    let mut vetor: Vec<u32> = Vec::new();
    let mut count = 0;

    loop{
        count = count + 1;
        hora_acorda = hora_acorda + 2;
        if hora_acorda < 24{
            vetor.push(hora_acorda);
        }
        
        else if hora_acorda >= 24{
            let time = hora_acorda - 24;
            vetor.push(time);
        }
        if count == 8{
            break;
        }
    }

    let mut controle: bool = true;
    let mut tempo = 0;

    loop{
        
        for n in 0..vetor.len(){
            if controle == true{
                if tempo::tempo() == vetor[n]{
                    println!("Você deve beber {}ml de água!", water_division);
                    controle = false;
                    tempo = tempo::tempo();
                }
            }

            
            if tempo::tempo() != vetor[n] && tempo::tempo() != tempo {
                
                    controle = true;
                
            }

        }

    }
    
        
}
