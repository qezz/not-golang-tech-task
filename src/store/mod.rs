mod inmem;
mod postgres;

pub use inmem::InMem;

use actix_web::Result;
use crate::models::{Buff, CreateBuff};

#[derive(Clone, Debug)]
pub enum Error {
    BuffNotFound(usize),
    FailedToCreateBuffer(CreateBuff),
}

pub trait Store {
    fn get_buff(&self, id: usize) -> Result<Buff, Error>;
    fn add_buff(&mut self, cb: CreateBuff) -> Result<Buff, Error>;
}
