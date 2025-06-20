use anyhow::{bail, Result};
use tokio_retry::{
    strategy::{jitter, FibonacciBackoff},
    Retry,
};

use acter::{api::SubscriptionStatus, ActerModel};
use urlencoding::encode;

use crate::utils::random_users_with_random_space_under_template;

const TMPL: &str = r#"
version = "0.1"
name = "News Notifications Setup Template"

[inputs]
main = { type = "user", is-default = true, required = true, description = "The starting user" }
space = { type = "space", is-default = true, required = true, description = "The main user" }

[objects.example-news-one-image]
type = "news-entry"
slides = [
  { body = "This is the news section. Swipe down for more.", info = { size = 3264047, mimetype = "image/jpeg", thumbnail_info = { w = 400, h = 600, mimetype = "image/jpeg", size = 130511 }, w = 3840, h = 5760, "xyz.amorgan.blurhash" = "TQF=,g?uIo},={X5$c#+V@t2sRjF", thumbnail_url = "mxc://acter.global/aJhqfXrJRWXsFgWFRNlBlpnD" }, msgtype = "m.image", url = "mxc://acter.global/tVLtaQaErMyoXmcCroPZdfNG" }
]

[objects.acter-event-1]
type = "calendar-event"
title = "Onboarding on Acter"
utc_start = "{{ future(add_mins=1).as_rfc3339 }}"
utc_end = "{{ future(add_mins=60).as_rfc3339 }}"

[objects.acter-website-pin]
type = "pin"
title = "Acter Website"
url = "https://acter.global"

"#;

#[tokio::test]
async fn comment_on_news() -> Result<()> {
    let (users, _sync_states, space_id, _engine) =
        random_users_with_random_space_under_template("cOnboost", 1, TMPL).await?;

    let first = users.first().expect("exists");
    let second_user = &users[1];

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(30);
    let news_entry = Retry::spawn(retry_strategy.clone(), || async {
        let news_entries = second_user.latest_news_entries(1).await?;
        if news_entries.len() != 1 {
            bail!("news entries not found found");
        }
        Ok(news_entries[0].clone())
    })
    .await?;

    // ensure we are expected to see these notifications
    let notif_settings = first.notification_settings().await?;
    let obj_id = news_entry.event_id();

    notif_settings
        .subscribe_object_push(obj_id.to_string(), None)
        .await
        .expect("setting notifications subscription works");
    // ensure this has been locally synced
    Retry::spawn(retry_strategy, || async {
        let status = notif_settings
            .object_push_subscription_status(obj_id.to_string(), None)
            .await?;
        if status != SubscriptionStatus::Subscribed {
            bail!("not yet subscribed");
        }
        Ok(())
    })
    .await?;

    let comments = news_entry.comments().await?;
    let body = "this is great";
    let notification_ev = comments
        .comment_draft()?
        .content_text(body.to_owned())
        .send()
        .await?;

    let notification_item = first
        .get_notification_item(space_id.to_string(), notification_ev.to_string())
        .await?;
    assert_eq!(notification_item.push_style(), "comment");
    assert_eq!(
        notification_item
            .parent_id_str()
            .expect("parent is in comment"),
        news_entry.event_id()
    );

    let content = notification_item.body().expect("found content");
    assert_eq!(content.body(), body);
    let parent = notification_item.parent().expect("parent was found");
    assert_eq!(
        notification_item.target_url(),
        format!(
            "/updates/{}?section=comments&commentId={}",
            news_entry.event_id(),
            encode(notification_ev.as_str())
        )
    );
    assert_eq!(parent.type_str(), "news");
    assert_eq!(parent.title(), None);
    assert_eq!(parent.emoji(), "🚀"); // rocket
    assert_eq!(parent.object_id_str(), news_entry.event_id());

    Ok(())
}

#[tokio::test]
async fn comment_on_pin() -> Result<()> {
    let (users, _sync_states, space_id, _engine) =
        random_users_with_random_space_under_template("cOnpin", 1, TMPL).await?;

    let first = users.first().expect("exists");
    let second_user = &users[1];

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(30);
    let obj_entry = Retry::spawn(retry_strategy.clone(), || async {
        let entries = second_user.pins().await?;
        if entries.is_empty() {
            bail!("entries not found found");
        }
        Ok(entries[0].clone())
    })
    .await?;

    // ensure we are expected to see these notifications
    let notif_settings = first.notification_settings().await?;
    let obj_id = obj_entry.event_id().to_string();

    notif_settings
        .subscribe_object_push(obj_id.clone(), None)
        .await
        .expect("setting notifications subscription works");
    // ensure this has been locally synced
    Retry::spawn(retry_strategy, || async {
        let status = notif_settings
            .object_push_subscription_status(obj_id.clone(), None)
            .await?;
        if status != SubscriptionStatus::Subscribed {
            bail!("not yet subscribed");
        }
        Ok(())
    })
    .await?;

    let comments = obj_entry.comments().await?;
    let body = "now we just need to find dory";
    let notification_ev = comments
        .comment_draft()?
        .content_text(body.to_owned())
        .send()
        .await?;

    let notification_item = first
        .get_notification_item(space_id.to_string(), notification_ev.to_string())
        .await?;
    assert_eq!(notification_item.push_style(), "comment");
    assert_eq!(
        notification_item
            .parent_id_str()
            .expect("parent is in comment"),
        *obj_entry.event_id()
    );

    let content = notification_item.body().expect("found content");
    assert_eq!(content.body(), body);
    let parent = notification_item.parent().expect("parent was found");
    assert_eq!(
        notification_item.target_url(),
        format!(
            "/pins/{}?section=comments&commentId={}",
            obj_entry.event_id(),
            encode(notification_ev.as_str())
        )
    );
    assert_eq!(parent.type_str(), "pin");
    assert_eq!(parent.title().as_deref(), Some("Acter Website"));
    assert_eq!(parent.emoji(), "📌"); // pin
    assert_eq!(parent.object_id_str(), *obj_entry.event_id());

    Ok(())
}

#[tokio::test]
async fn comment_on_calendar_events() -> Result<()> {
    let (users, _sync_states, space_id, _engine) =
        random_users_with_random_space_under_template("cOnpin", 1, TMPL).await?;

    let first = users.first().expect("exists");
    let second_user = &users[1];

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(30);
    let obj_entry = Retry::spawn(retry_strategy.clone(), || async {
        let entries = second_user.calendar_events().await?;
        if entries.is_empty() {
            bail!("entries not found found");
        }
        Ok(entries[0].clone())
    })
    .await?;

    // ensure we are expected to see these notifications
    let notif_settings = first.notification_settings().await?;
    let obj_id = obj_entry.event_id().to_string();

    notif_settings
        .subscribe_object_push(obj_id.clone(), None)
        .await
        .expect("setting notifications subscription works");
    // ensure this has been locally synced
    Retry::spawn(retry_strategy, || async {
        let status = notif_settings
            .object_push_subscription_status(obj_id.clone(), None)
            .await?;
        if status != SubscriptionStatus::Subscribed {
            bail!("not yet subscribed");
        }
        Ok(())
    })
    .await?;

    let comments = obj_entry.comments().await?;
    let body = "looking forward to it";
    let notification_ev = comments
        .comment_draft()?
        .content_text(body.to_owned())
        .send()
        .await?;

    let notification_item = first
        .get_notification_item(space_id.to_string(), notification_ev.to_string())
        .await?;
    assert_eq!(notification_item.push_style(), "comment");
    assert_eq!(
        notification_item
            .parent_id_str()
            .expect("parent is in comment"),
        obj_entry.event_id()
    );

    let content = notification_item.body().expect("found content");
    assert_eq!(content.body(), body);
    let parent = notification_item.parent().expect("parent was found");
    assert_eq!(
        notification_item.target_url(),
        format!(
            "/events/{}?section=comments&commentId={}",
            obj_entry.event_id(),
            encode(notification_ev.as_str())
        )
    );
    assert_eq!(parent.type_str(), "event");
    assert_eq!(parent.title().as_deref(), Some("Onboarding on Acter"));
    assert_eq!(parent.emoji(), "🗓️"); // calendar icon
    assert_eq!(parent.object_id_str(), obj_entry.event_id());

    Ok(())
}
