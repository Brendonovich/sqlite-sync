use crate::db::Tag;
use serde::{Deserialize, Serialize};

use super::{CRDTRecord, Ctx};

#[derive(Serialize, Deserialize, Clone)]
pub struct TagCreate {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TagUpdate {
    Name(String),
}

impl CRDTRecord for Tag {
    type Create = TagCreate;
    type Update = TagUpdate;

    fn create(data: Self::Create, ctx: Ctx) {
        let mut db = ctx.database.borrow_mut();

        db.create_tag(Self {
            id: ctx.id,
            name: data.name,
        });
    }

    fn update(data: Self::Update, ctx: Ctx) {
        let mut db = ctx.database.borrow_mut();

        db.tags
            .iter_mut()
            .find(|tag| tag.id == ctx.id)
            .map(|tag| match data {
                TagUpdate::Name(name) => tag.name = name,
            });
    }

    fn delete(ctx: Ctx) {
        let mut db = ctx.database.borrow_mut();

        db.tags.iter().position(|tag| tag.id == ctx.id).map(|i| {
            db.tags.swap_remove(i);
        });
    }
}
