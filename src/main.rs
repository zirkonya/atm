use atm::{Atm, Denomination};
use error::AtmError;

mod atm;
mod error;

/*
        l'ATM tu as un stock de départ (tant de billets de 5, tant de 10, etc).
        Tu dois pouvoir prendre des inputs successifs où l'utilisateur demande à retirer un montant,
        tu gères si c'est possible (genre 22 c'est pas possible si plus petite coupure = 5,
        mais aussi peut-être qu'au bout d'un moment il y a rupture d'un certain type de billets),
        et tu gères ce que ça retire du stock.

    ATM :
        - : nombres de coupures
        - : vérifier si c'est possible
            - multiple de la plus petite coupure disponible
        - : vérifier le stock
            - <= au maximum disponible
            - ou possible d'atteindre la valeur avec les coupures disponible
        - : mettre à jour le stock
*/

const FIVE: Denomination = Denomination::new(5, 10);
const TEN: Denomination = Denomination::new(10, 10);
const TWENTY: Denomination = Denomination::new(20, 10);

fn main() {
    let mut atm = Atm::new([FIVE, TEN, TWENTY]);
    println!("How much do you want to withdraw ?");
    for line in std::io::stdin().lines() {
        let Ok(line) = line else {
            eprintln!("{}", line.err().unwrap());
            continue;
        };
        match u32::from_str_radix(&line, 10) {
            Ok(amount) => {
                match atm.withdraw(amount) {
                    Ok(denomination) => {
                        println!("You got : {}", denomination.iter().map(|denomination| {
                            format!("{}x{}€", denomination.amount, denomination.value)
                        }).collect::<Vec<String>>().join(" and "))
                    },
                    Err(err) if err == AtmError::AtmIsEmpty => {
                        eprintln!("You can't withdraw : {}", err);
                        break;
                    }
                    Err(err) => eprintln!("You can't withdraw : {}", err),
                }
            },
            Err(_) => eprintln!("\x1b[31mPlease enter a number !\x1b[m"),
        }
        println!("How much do you want to withdraw ?");
    }
}
