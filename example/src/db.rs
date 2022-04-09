#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub id: u64,
}

#[derive(Debug, Clone)]
pub struct Clock {
    pub client_id: u64,
    pub value: u64,
}

impl Clock {
    pub fn new(client_id: u64) -> Self {
        Self {
            client_id,
            value: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub clock: Clock,
    pub id: u64,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub tags: Vec<Tag>,
    pub clients: Vec<Client>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tags: Vec::new(),
            clients: Vec::new(),
        }
    }

    pub fn create_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }

    pub fn create_client(&mut self, client: Client) {
        self.clients.push(client);
    }
}
