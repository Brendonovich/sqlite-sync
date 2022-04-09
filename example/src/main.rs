use std::{cell::RefCell, rc::Rc};

use crdt::{Ctx, RecordOperation};
use db::{Clock, Database};

use crate::crdt::{client::ClientCreate, tag::{TagCreate, TagUpdate}, Operation};

mod crdt;
mod db;

#[derive(Debug)]
struct Core {
    id: u64,
    clock: Clock,
    database: Rc<RefCell<Database>>,
}

impl Core {
    pub fn new(id: u64) -> Self {
        let database = Database::new();

        Self {
            id,
            clock: Clock::new(id),
            database: Rc::new(RefCell::new(database)),
        }
    }

    pub fn from(core: &Core, id: u64) -> Self {
        let mut new_core = Self::new(id);

        new_core.database = core.database.clone();

        new_core
    }

    pub fn ingest_message(&self, msg: Message) {
        let Message {
            record_id,
            data,
            clock,
            client_id,
        } = msg;

        let ctx = Ctx {
            id: record_id,
            database: Rc::new(RefCell::new(self.database.borrow().clone())),
        };

        self.perform_operation(data, ctx);
    }

    fn perform_operation(&self, op: RecordOperation, ctx: Ctx) {
        match op {
            RecordOperation::Tag(o) => o.apply(ctx),
            RecordOperation::Client(o) => o.apply(ctx),
        }
    }

    pub fn process_operation(&self, id: u64, op: RecordOperation) -> Message {
        self.perform_operation(
            op.clone(),
            Ctx {
                database: self.database.clone(),
                id,
            },
        );

        let mut database = self.database.borrow_mut();

        database
            .clients
            .iter_mut()
            .find(|c| c.id == self.id)
            .unwrap()
            .clock
            .value += 1;

        Message {
            record_id: id,
            data: op,
            clock: database.clients.iter().map(|c| c.clock.clone()).collect(),
            client_id: self.id,
        }
    }
}

#[derive(Clone)]
struct Message {
    record_id: u64,
    data: RecordOperation,
    clock: Vec<Clock>,
    client_id: u64,
}

#[tokio::main]
async fn main() {
    let core_0 = Core::new(0);
    let core_1 = Core::from(&core_0, 1);

    core_0.process_operation(
        0,
        RecordOperation::Client(Operation::Create(ClientCreate {})),
    );

    core_0.ingest_message(core_1.process_operation(
        1,
        RecordOperation::Client(Operation::Create(ClientCreate {})),
    ));

    core_1.ingest_message(core_0.process_operation(
        0,
        RecordOperation::Tag(Operation::Create(TagCreate {
            name: "test".to_string(),
        })),
    ));

    core_0.ingest_message(core_1.process_operation(
        0,
        RecordOperation::Tag(Operation::Update(TagUpdate::Name("lmao".to_string()))),
    ));

    dbg!(core_0);
    dbg!(core_1);
}
