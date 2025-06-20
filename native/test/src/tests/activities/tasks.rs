use anyhow::{bail, Result};
use tokio_retry::{
    strategy::{jitter, FibonacciBackoff},
    Retry,
};

use crate::{
    tests::activities::{all_activities_observer, assert_triggered_with_latest_activity},
    utils::random_user_with_template,
};

const TMPL: &str = r#"
version = "0.1"
name = "Smoketest Template"

[inputs]
main = { type = "user", is-default = true, required = true, description = "The starting user" }

[objects]
main_space = { type = "space", is-default = true, name = "{{ main.display_name }}’s tasks test space" }

[objects.list]
type = "task-list"
name = "Onboarding on Acter"

[objects.task-1]
type = "task"
title = "Check the weather"
"m.relates_to" = { event_id = "{{ list.id }}" }

"#;

#[tokio::test]
async fn task_creation_activity() -> Result<()> {
    let _ = env_logger::try_init();
    let (user, sync_state, _engine) = random_user_with_template("tasks_activities", TMPL).await?;
    sync_state.await_has_synced_history().await?;

    let mut act_obs = all_activities_observer(&user).await?;

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(30);
    let task_lists = Retry::spawn(retry_strategy, || async {
        let task_lists = user.task_lists().await?;
        if task_lists.len() != 1 {
            bail!("not all task_lists found");
        }
        Ok(task_lists)
    })
    .await?;

    assert_eq!(task_lists.len(), 1);

    let task_list = task_lists
        .first()
        .expect("first tasklist should be available");

    let tasks = task_list.tasks().await?;
    assert_eq!(tasks.len(), 1);

    let task = tasks.first().expect("first task should be available");

    let activity = user.activity(task_list.event_id_str()).await?;
    assert_eq!(activity.type_str(), "creation");
    let object = activity.object().expect("we have an object");
    assert_eq!(object.type_str(), "task-list");
    assert_eq!(object.title().as_deref(), Some("Onboarding on Acter"));

    let activity = user.activity(task.event_id_str()).await?;
    assert_eq!(activity.type_str(), "taskAdd");
    assert_eq!(activity.title().as_deref(), Some("Check the weather"));
    // on task add the "object" is our list this happened on
    let object = activity.object().expect("we have an object");
    assert_eq!(object.type_str(), "task-list");
    assert_eq!(object.title().as_deref(), Some("Onboarding on Acter"));
    assert_triggered_with_latest_activity(&mut act_obs, activity.event_id_str()).await?;
    Ok(())
}
