use std::fmt::Display;

#[derive(PartialEq)]
pub enum AtmError {
    InsuficientBalance {
        amount: u32,
        balance: u32
    },
    DenominationNotAvailable {
        amount: u32,
        denomination: u32
    },
    AtmIsEmpty,
}

impl Display for AtmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtmError::InsuficientBalance { amount, balance } => write!(f, "Insuficient balance you tried {amount}€ but {balance}€ avaiable"),
            AtmError::DenominationNotAvailable { amount, denomination } => write!(f, "No denomination available to give you {amount}€ the smallest denomination available is {denomination}€."),
            AtmError::AtmIsEmpty => write!(f, "Atm is empty"),
        }
    }
}