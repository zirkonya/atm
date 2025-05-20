use crate::error::AtmError;

#[derive(Debug, Default, Clone)]
pub struct Denomination {
    pub value: u32,
    pub amount: u32,
}

impl Denomination {
    pub const fn new(value: u32, amount: u32) -> Self {
        Self { value, amount }
    }
}

pub struct Atm {
    denominations: Vec<Denomination>,
}

impl Atm {
    pub fn new<const SIZE: usize>(mut denominations: [Denomination; SIZE]) -> Self {
        denominations.sort_by_key(|item| item.value);
        Self { denominations: denominations.to_vec() }
    }

    pub fn balance(&self) -> u32 {
        self.denominations.iter().map(|Denomination { value, amount }| {
            value * amount
        }).sum()
    }

    pub fn min_denomination(&self) -> Denomination {
        self.denominations
            .iter()
            .find(|Denomination { amount, .. }| *amount != 0)
            .cloned()
            .unwrap_or_default()
    }

    pub fn can_withdraw(&self, amount: u32) -> Result<(), AtmError> {
        let balance = self.balance();
        let min_denomination = self.min_denomination();
        if balance == 0 {
            Err(AtmError::AtmIsEmpty)
        } else if balance < amount {
            Err(AtmError::InsuficientBalance { amount, balance })
        } else if amount % min_denomination.value != 0 {
            Err(AtmError::DenominationNotAvailable { amount, denomination: min_denomination.value })
        } else {
            Ok(())
        }
    }

    fn apply_withdraw(&mut self, denominations: &Vec<Denomination>) {
        self.denominations.iter_mut().zip(denominations).for_each(|(lhs, rhs)| {
            lhs.amount -= rhs.amount;
        });
    }

    pub fn withdraw(&mut self, mut amount: u32) -> Result<Vec<Denomination>, AtmError> {
        self.can_withdraw(amount)?;
        let mut denominations = Vec::with_capacity(3);
        
        for &Denomination { value, amount: denominaiton_amount } in self.denominations.iter().rev() {
            if denominaiton_amount == 0 {
                continue;
            }
            let needed = amount / value;
            if needed <= denominaiton_amount {
                amount %= value;
                denominations.push(Denomination::new(value, needed));
            } else {
                let available = value * denominaiton_amount;
                amount -= available;
                denominations.push(Denomination::new(value, denominaiton_amount));
            }
        }

        denominations.sort_by_key(|item| item.value);
        self.apply_withdraw(&denominations);
        Ok(denominations)
    }
}