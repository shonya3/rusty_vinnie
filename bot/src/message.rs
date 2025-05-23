use poise::serenity_prelude::{ChannelId, Context as SerenityContext, CreateMessage, CreateThread};

pub struct MessageWithThreadedDetails {
    pub message: CreateMessage,
    pub thread_name: String,
    pub thread_message: Option<CreateMessage>,
}

impl MessageWithThreadedDetails {
    pub async fn send(self, ctx: &SerenityContext, channel_id: ChannelId) {
        match channel_id.send_message(ctx, self.message).await {
            Ok(initial_message) => {
                if let Some(thread_message) = self.thread_message {
                    match initial_message
                        .channel_id
                        .create_thread_from_message(
                            ctx,
                            initial_message.id,
                            CreateThread::new(self.thread_name.clone()),
                        )
                        .await
                    {
                        Ok(thread_channel) => {
                            if let Err(e) = thread_channel.send_message(ctx, thread_message).await {
                                eprintln!(
                                    "Failed to send content embed to new Discord thread '{}': {:?}",
                                    self.thread_name, e
                                );
                            } else {
                                println!("Successfully created Discord thread '{}' from message {} and sent full embed.", self.thread_name, initial_message.id);
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "Failed to create Discord thread '{}' from message {}: {:?}",
                                self.thread_name, initial_message.id, e
                            );
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to send initial embed to dev channel {}: {:?}",
                    channel_id, e
                );
            }
        }
    }
}
