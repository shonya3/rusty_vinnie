use poise::serenity_prelude::{
    ChannelId, Context as SerenityContext, CreateEmbed, CreateMessage, CreateThread,
};

pub struct MessageWithThreadedDetails {
    pub message: CreateMessage,
    pub thread_name: String,
    pub details_content: Option<String>,
}

impl MessageWithThreadedDetails {
    pub async fn send(self, ctx: &SerenityContext, channel_id: ChannelId) {
        match channel_id.send_message(ctx, self.message).await {
            Ok(initial_message) => {
                if let Some(details_content) = self.details_content {
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
                            let detail_messages = create_details_message(&details_content);
                            let total_parts = detail_messages.len();
                            for (index, thread_message_part) in
                                detail_messages.into_iter().enumerate()
                            {
                                match thread_channel.send_message(ctx, thread_message_part).await {
                                    Ok(_) => {
                                        println!(
                                            "Successfully sent detail part {}/{} to thread '{}' (from initial message {}).",
                                            index + 1,
                                            total_parts,
                                            self.thread_name,
                                            initial_message.id
                                        );
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "Failed to send detail part {}/{} to thread '{}': {:?}",
                                            index + 1,
                                            total_parts,
                                            self.thread_name,
                                            e
                                        );
                                    }
                                }
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

pub fn create_details_message(content: &str) -> Vec<CreateMessage> {
    let mut text_fragments: Vec<String> = Vec::new();
    let mut current_fragment = String::new();

    for ch in content.chars() {
        if current_fragment.len() == crate::EMBED_DESCRIPTION_MAX_CHARS {
            text_fragments.push(current_fragment);
            current_fragment = String::new();
        }
        current_fragment.push(ch);
    }

    // After the loop, add the last fragment if it has content
    if !current_fragment.is_empty() {
        text_fragments.push(current_fragment);
    }

    text_fragments
        .into_iter()
        .map(|content| CreateMessage::new().embed(CreateEmbed::new().description(content)))
        .collect()
}
