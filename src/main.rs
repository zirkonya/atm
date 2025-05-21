use atm::{Atm, Denomination};
use error::AtmError;

mod atm;
mod error;

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
