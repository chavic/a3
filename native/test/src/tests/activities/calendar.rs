use anyhow::{bail, Result};
use tokio_retry::{
    strategy::{jitter, FibonacciBackoff},
    Retry,
};

use super::get_latest_activity;
use crate::{tests::activities::assert_latest_activity, utils::random_user_with_template};

const TMPL: &str = r#"
version = "0.1"
name = "Smoketest Template"

[inputs]
main = { type = "user", is-default = true, required = true, description = "The starting user" }

[objects]
main_space = { type = "space", is-default = true, name = "{{ main.display_name }}’s calendar event test space" }

[objects.acter-event-1]
type = "calendar-event"
title = "Onboarding on Acter"
utc_start = "{{ future(add_mins=1).as_rfc3339 }}"
utc_end = "{{ future(add_mins=60).as_rfc3339 }}"
"#;

#[tokio::test]
async fn calendar_creation_activity() -> Result<()> {
    let _ = env_logger::try_init();
    let (user, sync_state, _engine) =
        random_user_with_template("calendar_activities", TMPL).await?;
    sync_state.await_has_synced_history().await?;

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(30);
    Retry::spawn(retry_strategy, || async {
        if user.calendar_events().await?.len() != 1 {
            bail!("not all calendar_events found");
        }
        Ok(())
    })
    .await?;

    assert_eq!(user.calendar_events().await?.len(), 1);
    let activities = user.all_activities()?;

    let spaces = user.spaces().await?;
    assert_eq!(spaces.len(), 1);

    let main_space = spaces.first().expect("main space should be available");
    assert_eq!(main_space.calendar_events().await?.len(), 1);

    let activity = get_latest_activity(&user, main_space.room_id().to_string(), "creation").await?;
    assert_eq!(activity.type_str(), "creation");
    let object = activity.object().expect("we have an object");
    assert_eq!(object.type_str(), "event");
    assert_eq!(object.title().as_deref(), Some("Onboarding on Acter"));

    assert_latest_activity(&activities, activity.event_id_str()).await?;

    Ok(())
}
