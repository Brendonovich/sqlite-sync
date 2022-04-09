use crate::db::{Client, Clock};

use super::{CRDTRecord, Ctx};

#[derive(Clone)]
pub struct ClientCreate {}

impl CRDTRecord for Client {
    type Create = ClientCreate;
    type Update = ();

    fn create(data: ClientCreate, ctx: Ctx) {
        let mut db = ctx.database.borrow_mut();

        db.create_client(Client {
            id: ctx.id,
            clock: Clock {
                client_id: ctx.id,
                value: 0,
            },
        });
    }

    fn delete(ctx: Ctx) {
        let mut db = ctx.database.borrow_mut();

        db.clients
            .iter()
            .position(|client| client.id == ctx.id)
            .map(|i| {
                db.clients.swap_remove(i);
            });
    }

    fn update(_: (), _: Ctx) {}
}
