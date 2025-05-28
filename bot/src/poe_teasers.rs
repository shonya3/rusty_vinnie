use crate::{channel::AppChannel, Data};
use poe_teasers::{Teaser, TeasersForumThread};
use poise::serenity_prelude::{
    ChannelId, Context as SerenityContext, CreateEmbed, CreateEmbedAuthor, CreateMessage,
};
use std::{collections::HashSet, time::Duration};

pub async fn watch_teasers_threads(
    ctx: &SerenityContext,
    data: &Data,
    forum_threads: &[TeasersForumThread],
) {
    let mut interval = tokio::time::interval(Duration::from_secs(360));
    let channel_id = AppChannel::Poe.id();
    loop {
        interval.tick().await;
        for forum_thread in forum_threads {
            send_new_teasers(ctx, data, *forum_thread, channel_id).await;
        }
    }
}

async fn send_new_teasers(
    ctx: &SerenityContext,
    _data: &Data,
    forum_thread: TeasersForumThread,
    channel_id: ChannelId,
) {
    let thread_teasers = match poe_teasers::download_teasers_from_thread(forum_thread).await {
        Ok(teas) => teas,
        Err(err) => {
            println!("Could not download thread teasers. {err}");
            return;
        }
    };
    let published_teasers = load_published_teasers();

    for teaser in &thread_teasers {
        if !published_teasers.contains(teaser) {
            send_teaser(ctx, channel_id, teaser)
                .await
                .unwrap_or_else(|err| eprintln!("publish_new_teasers Error:{err}"))
        };
    }

    let mut set = HashSet::<Teaser>::from_iter(published_teasers);
    set.extend(thread_teasers);

    let _unique_teasers: Vec<Teaser> = set.into_iter().collect();

    if let Err(err) = save_published_teasers() {
        println!("Could not persist thread teasers: {err}");
    };
}

// TODO Use the actual storage
fn load_published_teasers() -> Vec<Teaser> {
    Vec::new()
}

// TODO Use the actual storage
fn save_published_teasers() -> Result<(), String> {
    Ok(())
}

async fn send_teaser(
    ctx: &SerenityContext,
    channel_id: ChannelId,
    teaser: &Teaser,
) -> Result<(), String> {
    let message = CreateMessage::new().embeds(
        std::iter::once(
            CreateEmbed::new()
                .title(teaser.forum_thread.title())
                .url(teaser.forum_thread.url())
                .author(create_vinnie_bot_author_embed())
                .description(&teaser.heading),
        )
        .chain(teaser.images_urls.iter().map(|image_url| {
            CreateEmbed::new()
                .image(image_url)
                .url(teaser.forum_thread.url())
        }))
        .collect(),
    );

    if let Err(err) = channel_id.send_message(&ctx, message).await {
        return Err(format!("Could not send teaser to {channel_id}. {err}"));
    }

    if !teaser.videos_urls.is_empty() {
        if let Err(err) = channel_id
            .send_message(
                &ctx,
                CreateMessage::new().content(teaser.videos_urls.join(" ")),
            )
            .await
        {
            return Err(format!("Could not send teaser to {channel_id}. {err}"));
        }
    }

    Ok(())
}

fn create_vinnie_bot_author_embed() -> CreateEmbedAuthor {
    CreateEmbedAuthor::new("Rusty Vinnie")
        .icon_url("https://discord.com/assets/ca24969f2fd7a9fb03d5.png")
        .url("https://github.com/shonya3/rusty_vinnie")
}

#[allow(unused)]
mod db_layer {
    use crate::DbClient;
    use libsql_client::{Statement, Value};
    use std::collections::HashSet;

    type LibsqlError = Box<dyn std::error::Error>;

    pub const CREATE_IF_NOT_EXISTS: &str = r#"
    CREATE TABLE IF NOT EXISTS published_poe_teasers (
        thread_url TEXT NOT NULL,
        teaser_heading TEXT NOT NULL,
        published_at TEXT DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY (thread_url, teaser_heading)
    ) STRICT;"#;

    pub async fn ensure_schema_exists(db: &DbClient) -> Result<(), LibsqlError> {
        db.execute(CREATE_IF_NOT_EXISTS).await?;

        Ok(())
    }

    pub async fn load_published_teaser_heading(
        db: &DbClient,
        thread_url: &str,
    ) -> Result<HashSet<String>, String> {
        let stmt = Statement::with_args(
            "SELECT
                teaser_heading 
            FROM 
                published_poe_teasers
            WHERE
                thread_url = ?
                ",
            &[Value::from(thread_url)],
        );

        let rows = db.execute(stmt).await.unwrap();

        let mut headings = HashSet::new();
        for row in rows.rows {
            if row.values.is_empty() {
                return Err(format!(
                    "DB row has no columns for thread_url: {}",
                    thread_url
                ));
            }
            match &row.values[0] {
                Value::Text {
                    value: heading_text,
                } => {
                    headings.insert(heading_text.clone());
                }
                _ => {
                    return Err(format!(
                        "Unexpected data type for teaser_heading in DB row for thread_url: {}",
                        thread_url
                    ))
                }
            }
        }

        Ok(headings)
    }

    pub async fn save_newly_published_teaser_headings(
        db: &DbClient,
        thread_url: &str,
        headings: &[String],
    ) -> Result<(), String> {
        if headings.is_empty() {
            return Ok(());
        }

        let mut stmts = Vec::new();
        for heading in headings {
            stmts.push(Statement::with_args(
                "INSERT OR IGNORE INTO
                     published_poe_teasers (thread_url, teaser_heading) 
                     VALUES 
                        (?, ?)",
                &[Value::from(thread_url), Value::from(heading.as_str())],
            ));
        }

        db.batch(stmts).await.map_err(|e| {
            format!(
                "DB batch insert failed for save_newly_published_teaser_headings [{}]: {}",
                thread_url, e
            )
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod db_layer_tests {
    use crate::poe_teasers::db_layer::*;
    use libsql_client::Client;
    use std::collections::HashSet;

    async fn memory_db_client() -> Client {
        Client::in_memory().unwrap()
    }

    #[tokio::test]
    async fn test_ensure_schema_exists() {
        let db = memory_db_client().await;

        // First run
        let result = ensure_schema_exists(&db).await;
        assert!(result.is_ok(), "Schema creation failed: {:?}", result.err());

        // Second run (idempotency check)
        let result_idempotent = ensure_schema_exists(&db).await;
        assert!(
            result_idempotent.is_ok(),
            "Schema creation on existing table failed: {:?}",
            result_idempotent.err()
        );

        // Optional: Try a simple insert to confirm table structure (basic check)
        let insert_result = db.execute(
            "INSERT INTO published_poe_teasers (thread_url, teaser_heading) VALUES ('test_url', 'test_heading')"
        ).await;
        assert!(
            insert_result.is_ok(),
            "Insert after schema creation failed: {:?}",
            insert_result.err()
        );
    }

    #[tokio::test]
    async fn test_save_and_load_published_teaser_headings() {
        let db = memory_db_client().await;
        ensure_schema_exists(&db).await.unwrap();

        let thread_url1 = "https://example.com/thread1";
        let thread_url2 = "https://example.com/thread2";

        // 1. Load from empty table
        let initial_headings = load_published_teaser_heading(&db, thread_url1)
            .await
            .unwrap();
        assert!(
            initial_headings.is_empty(),
            "Expected no headings initially for thread1"
        );

        // 2. Save some new headings for thread_url1
        let headings_to_save1 = vec!["Teaser A".to_string(), "Teaser B".to_string()];
        let save_result1 =
            save_newly_published_teaser_headings(&db, thread_url1, &headings_to_save1).await;
        assert!(
            save_result1.is_ok(),
            "Failed to save headings for thread1: {:?}",
            save_result1.err()
        );

        // 3. Load them back for thread_url1
        let loaded_headings1 = load_published_teaser_heading(&db, thread_url1)
            .await
            .unwrap();
        let expected_headings1: HashSet<String> = headings_to_save1.into_iter().collect();
        assert_eq!(
            loaded_headings1, expected_headings1,
            "Loaded headings do not match saved ones for thread1"
        );

        // 4. Try to save the same headings again + a new one for thread_url1 (OR IGNORE should handle duplicates)
        let headings_to_save_again1 = vec!["Teaser A".to_string(), "Teaser C".to_string()];
        let save_again_result1 =
            save_newly_published_teaser_headings(&db, thread_url1, &headings_to_save_again1).await;
        assert!(
            save_again_result1.is_ok(),
            "Failed to save headings again for thread1: {:?}",
            save_again_result1.err()
        );

        // 5. Load again for thread_url1, should include "Teaser C" and not duplicate "Teaser A"
        let final_loaded_headings1 = load_published_teaser_heading(&db, thread_url1)
            .await
            .unwrap();
        let mut final_expected_headings1 = expected_headings1.clone();
        final_expected_headings1.insert("Teaser C".to_string());
        assert_eq!(
            final_loaded_headings1, final_expected_headings1,
            "Final loaded headings for thread1 are incorrect after re-save"
        );

        // 6. Save headings for a different thread_url2
        let headings_to_save2 = vec!["Teaser X".to_string()];
        let save_result2 =
            save_newly_published_teaser_headings(&db, thread_url2, &headings_to_save2).await;
        assert!(
            save_result2.is_ok(),
            "Failed to save headings for thread2: {:?}",
            save_result2.err()
        );

        // 7. Load for thread_url2
        let loaded_headings2 = load_published_teaser_heading(&db, thread_url2)
            .await
            .unwrap();
        let expected_headings2: HashSet<String> = headings_to_save2.into_iter().collect();
        assert_eq!(
            loaded_headings2, expected_headings2,
            "Loaded headings for thread2 are incorrect"
        );

        // 8. Ensure loading thread_url1 again still gives its correct set
        let re_loaded_headings1 = load_published_teaser_heading(&db, thread_url1)
            .await
            .unwrap();
        assert_eq!(
            re_loaded_headings1, final_expected_headings1,
            "Re-loaded headings for thread1 are incorrect after thread2 operations"
        );

        // 9. Test saving empty list
        let empty_headings: Vec<String> = Vec::new();
        let save_empty_result =
            save_newly_published_teaser_headings(&db, thread_url1, &empty_headings).await;
        assert!(
            save_empty_result.is_ok(),
            "Saving empty headings list failed"
        );
        let headings_after_empty_save = load_published_teaser_heading(&db, thread_url1)
            .await
            .unwrap();
        assert_eq!(
            headings_after_empty_save, final_expected_headings1,
            "Saving empty list affected existing headings"
        );
    }

    // TODO: Add tests for error cases in load_published_teaser_heading,
    // e.g., what happens if the DB connection fails mid-operation (harder to simulate without mocking),
    // or if data is malformed (though STRICT table should prevent some of this).
}
