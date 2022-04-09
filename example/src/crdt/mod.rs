use std::{cell::RefCell, rc::Rc};

use crate::db::{Database, Tag, Client};

pub mod client;
pub mod tag;

#[derive(Debug)]
pub struct Ctx {
    pub database: Rc<RefCell<Database>>,
    pub id: u64,
}

pub trait CRDTRecord {
    type Create: Clone;
    type Update: Clone;

    fn create(data: Self::Create, ctx: Ctx)
    where
        Self: Sized;

    fn update(data: Self::Update, ctx: Ctx)
    where
        Self: Sized;

    fn delete(ctx: Ctx)
    where
        Self: Sized;
}

#[derive(Clone)]
pub enum Operation<T: CRDTRecord + Clone> {
    Create(T::Create),
    Update(T::Update),
}

impl<T: CRDTRecord + Clone> Operation<T> {
    pub fn apply(self, ctx: Ctx) {
        match self {
            Self::Create(data) => T::create(data, ctx),
            Self::Update(data) => T::update(data, ctx),
        }
    }
}

#[derive(Clone)]
pub enum RecordOperation {
    Tag(Operation<Tag>),
    Client(Operation<Client>)
}
