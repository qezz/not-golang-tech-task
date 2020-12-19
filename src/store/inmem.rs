use crate::store::Store;

use crate::models::{Buff, CreateBuff};
use crate::store::Error;

pub struct InMem {
    buffs: Vec<Buff>
}

impl InMem {
    pub fn new() -> Self {
        Self {
            buffs: Vec::new()
        }
    }
}

impl Store for InMem {
    fn get_buff(&self, id: usize) -> Result<Buff, Error> {
        if id < self.buffs.len() {
            return Ok(self.buffs[id].clone())
        }

        return Err(Error::BuffNotFound(id))
    }

    fn add_buff(&mut self, cb: CreateBuff) -> Result<Buff, Error> {
        let buff = Buff {
            id: self.buffs.len(),
            question: cb.question,
            answer: cb.answer,
        };
        self.buffs.push(buff.clone());
        Ok(buff)
    }
}
