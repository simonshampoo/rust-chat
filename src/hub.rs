const OUTPUT_CHANNEL_SIZE: usize = 16;

#[derive(Clone, Copy, Default)]
pub struct HubOptions {
    pub alive_internal: Option<Duration>,
}

pub struct Hub {
    alive_internal: Option<Duration>,
    output_sender: broadcast::Sender<OutputParcel>,
    users: RwLock<HashMap<Uuid, User>>,
    feed: RwLock<Feed>,
}

impl Hub {
    pub fn new(options: HubOptions) -> Self {
        let (output_sender, _) = broadcast::channel(OUTPUT_CHANNEL_SIZE);
        Hub {
            alive_internal: options.alive_internal,
            output_sender,
            users: Default::default(),
            feed: Default::default(),
        }
    }

    async fn send(&self, output: Output) {
        if self.output_sender.receiver_count() == 0 {
            return; 
        }
        self.users.read().await.keys().for_each(|user_id| {
            self.output_sender.send(OutputParcel::new(*user_id, output.clone())).unwrap(); 
        })
    }
}
