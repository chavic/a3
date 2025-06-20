use acter::ActerModel;
use anyhow::{bail, Result};
use tokio_retry::{
    strategy::{jitter, FibonacciBackoff},
    Retry,
};

use crate::utils::{random_users_with_random_space, random_users_with_random_space_under_template};

const TMPL: &str = r#"
version = "0.1"
name = "Pin Notifications Setup Template"

[inputs]
main = { type = "user", is-default = true, required = true, description = "The starting user" }
space = { type = "space", is-default = true, required = true, description = "The main user" }

[objects.acter-website-pin]
type = "pin"
title = "Acter Website"
url = "https://acter.global"

"#;

#[tokio::test]
async fn pins_creation_notification() -> Result<()> {
    let _ = env_logger::try_init();
    let (users, room_id) = random_users_with_random_space("pins_creation_notifications", 1).await?;

    let mut user = users[0].clone();
    let mut second = users[1].clone();

    second.install_default_acter_push_rules().await?;

    let sync_state1 = user.start_sync();
    sync_state1.await_has_synced_history().await?;

    let sync_state2 = second.start_sync();
    sync_state2.await_has_synced_history().await?;

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(10);
    let main_space = Retry::spawn(retry_strategy, || async {
        let spaces = user.spaces().await?;
        if spaces.len() != 1 {
            bail!("space not found");
        }
        Ok(spaces.first().cloned().expect("space found"))
    })
    .await?;

    let space_on_second = second.room(main_space.room_id_str()).await?;
    space_on_second
        .set_notification_mode(Some("all".to_owned()))
        .await?; // we want to see push for everything;

    let title = "Acter Website";
    let event_id = main_space
        .pin_draft()?
        .title(title.to_owned())
        .send()
        .await?;
    tracing::trace!("draft sent event id: {}", event_id);

    let notifications = second
        .get_notification_item(room_id.to_string(), event_id.to_string())
        .await?;

    assert_eq!(notifications.push_style(), "creation");
    assert_eq!(notifications.target_url(), format!("/pins/{event_id}"));
    let parent = notifications.parent().expect("parent should be available");
    assert_eq!(parent.type_str(), "pin");
    assert_eq!(parent.title().as_deref(), Some(title));
    assert_eq!(parent.emoji(), "📌"); // pin icon
    assert_eq!(parent.object_id_str(), event_id);

    Ok(())
}

#[tokio::test]
async fn pin_title_update() -> Result<()> {
    let (users, _sync_states, space_id, _engine) =
        random_users_with_random_space_under_template("pinTitleUpdate", 1, TMPL).await?;

    let first = users.first().expect("exists");
    let second_user = &users[1];

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(30);
    let obj_entry = Retry::spawn(retry_strategy, || async {
        let entries = second_user.pins().await?;
        if entries.is_empty() {
            bail!("entries not found");
        }
        Ok(entries[0].clone())
    })
    .await?;

    // we want to see push for everything;
    first
        .room(obj_entry.room_id_str())
        .await?
        .set_notification_mode(Some("all".to_owned()))
        .await?;

    let title = "Renamed Pin";
    let notification_ev = obj_entry
        .update_builder()?
        .title(title.to_owned())
        .send()
        .await?;

    let notification_item = first
        .get_notification_item(space_id.to_string(), notification_ev.to_string())
        .await?;
    assert_eq!(notification_item.push_style(), "titleChange");
    assert_eq!(
        notification_item
            .parent_id_str()
            .expect("parent is in change"),
        obj_entry.event_id_str(),
    );

    let obj_id = obj_entry.event_id_str();

    assert_eq!(notification_item.title(), title); // old title
    let parent = notification_item.parent().expect("parent was found");
    assert_eq!(notification_item.target_url(), format!("/pins/{}", obj_id));
    assert_eq!(parent.type_str(), "pin");
    // assert_eq!(parent.title().as_deref(), Some("Acter Website"));
    assert_eq!(parent.emoji(), "📌"); // pin icon
    assert_eq!(parent.object_id_str(), obj_id);

    Ok(())
}

#[tokio::test]
async fn pin_desc_update() -> Result<()> {
    let (users, _sync_states, space_id, _engine) =
        random_users_with_random_space_under_template("pinDescUpdate", 1, TMPL).await?;

    let first = users.first().expect("exists");
    let second_user = &users[1];

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(30);
    let obj_entry = Retry::spawn(retry_strategy, || async {
        let entries = second_user.pins().await?;
        if entries.is_empty() {
            bail!("entries not found");
        }
        Ok(entries[0].clone())
    })
    .await?;

    // we want to see push for everything;
    first
        .room(obj_entry.room_id_str())
        .await?
        .set_notification_mode(Some("all".to_owned()))
        .await?;

    let body = "Added description";
    let notification_ev = obj_entry
        .update_builder()?
        .content_text(body.to_owned())
        .send()
        .await?;

    let notification_item = first
        .get_notification_item(space_id.to_string(), notification_ev.to_string())
        .await?;
    assert_eq!(notification_item.push_style(), "descriptionChange");
    assert_eq!(
        notification_item.parent_id_str().expect("parent is in pin"),
        obj_entry.event_id_str(),
    );

    let obj_id = obj_entry.event_id_str();

    let content = notification_item.body().expect("found content");
    assert_eq!(content.body(), body); // new description
    let parent = notification_item.parent().expect("parent was found");
    assert_eq!(notification_item.target_url(), format!("/pins/{}", obj_id));
    assert_eq!(parent.type_str(), "pin");
    assert_eq!(parent.title().as_deref(), Some("Acter Website"));
    assert_eq!(parent.emoji(), "📌"); // pin icon
    assert_eq!(parent.object_id_str(), obj_id);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn pin_redaction() -> Result<()> {
    let (users, _sync_states, space_id, _engine) =
        random_users_with_random_space_under_template("pinRedaction", 1, TMPL).await?;

    let first = users.first().expect("exists");
    let second_user = &users[1];

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(30);
    let pin = Retry::spawn(retry_strategy, || async {
        let entries = first.pins().await?;
        if entries.is_empty() {
            bail!("entries not found");
        }
        Ok(entries[0].clone())
    })
    .await?;

    // we want to see push for everything;
    second_user
        .room(pin.room_id_str())
        .await?
        .set_notification_mode(Some("all".to_owned()))
        .await?;

    let space = first.space(pin.room_id().to_string()).await?;
    let notification_ev = space.redact(pin.event_id(), None, None).await?.event_id;

    let notification_item = second_user
        .get_notification_item(space_id.to_string(), notification_ev.to_string())
        .await?;
    assert_eq!(notification_item.push_style(), "redaction");
    assert_eq!(
        notification_item
            .parent_id_str()
            .expect("parent is in redaction"),
        *pin.event_id()
    );

    let parent = notification_item.parent().expect("parent was found");
    assert_eq!(notification_item.target_url(), "/pins/");
    assert_eq!(parent.type_str(), "pin");
    assert_eq!(parent.title().as_deref(), Some("Acter Website"));
    assert_eq!(parent.emoji(), "📌"); // pin icon
    assert_eq!(parent.object_id_str(), *pin.event_id());

    Ok(())
}
