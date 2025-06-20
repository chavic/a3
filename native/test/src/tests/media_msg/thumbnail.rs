use acter::api::ActerModel;
use anyhow::{bail, Context, Result};
use core::time::Duration;
use futures::{pin_mut, stream::StreamExt, FutureExt};
use std::io::Write;
use tempfile::NamedTempFile;
use tokio::time::sleep;
use tokio_retry::{
    strategy::{jitter, FibonacciBackoff},
    Retry,
};

use crate::utils::{
    match_media_msg, random_user_with_random_convo, random_user_with_random_space,
    random_user_with_template,
};

#[tokio::test]
async fn room_msg_can_support_image_thumbnail() -> Result<()> {
    let _ = env_logger::try_init();

    let (mut user, room_id) = random_user_with_random_convo("room_msg_image_thumbnail").await?;
    let state_sync = user.start_sync();
    state_sync.await_has_synced_history().await?;

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(10);
    Retry::spawn(retry_strategy, || async {
        user.convo(room_id.to_string()).await
    })
    .await?;

    let convo = user.convo(room_id.to_string()).await?;
    let timeline = convo.timeline_stream();
    let stream = timeline.messages_stream();
    pin_mut!(stream);

    let bytes = include_bytes!("../fixtures/kingfisher.jpg");
    let mut tmp_jpg = NamedTempFile::new()?;
    tmp_jpg.as_file_mut().write_all(bytes)?;
    let jpg_name = tmp_jpg // it is randomly generated by system and not kingfisher.jpg
        .path()
        .file_name()
        .expect("it is not file")
        .to_string_lossy()
        .to_string();

    let bytes = include_bytes!("../fixtures/PNG_transparency_demonstration_1.png");
    let size = bytes.len() as u64;
    let mut tmp_png = NamedTempFile::new()?;
    tmp_png.as_file_mut().write_all(bytes)?;

    let mimetype = "image/jpeg";
    let thumb_mimetype = "image/png";
    let draft = user
        .image_draft(
            tmp_jpg.path().to_string_lossy().to_string(),
            mimetype.to_owned(),
        )
        .thumbnail_file_path(tmp_png.path().to_string_lossy().to_string())
        .thumbnail_info(None, None, Some(thumb_mimetype.to_owned()), Some(size));
    timeline.send_message(Box::new(draft)).await?;

    // image msg may reach via pushback action or reset action
    let mut i = 30;
    let mut found = None;
    while i > 0 {
        if let Some(diff) = stream.next().now_or_never().flatten() {
            match diff.action().as_str() {
                "PushBack" | "Set" => {
                    let value = diff
                        .value()
                        .expect("diff pushback action should have valid value");
                    if let Some(msg_content) = match_media_msg(&value, mimetype, &jpg_name) {
                        found = Some(msg_content);
                    }
                }
                "Reset" => {
                    let values = diff
                        .values()
                        .expect("diff reset action should have valid values");
                    for value in values.iter() {
                        if let Some(msg_content) = match_media_msg(value, mimetype, &jpg_name) {
                            found = Some(msg_content);
                            break;
                        }
                    }
                }
                _ => {}
            }
            // yay
            if found.is_some() {
                break;
            }
        }
        i -= 1;
        sleep(Duration::from_secs(1)).await;
    }
    let msg_content = found.context("Even after 30 seconds, image msg not received")?;
    let thumbnail_info = msg_content
        .thumbnail_info()
        .context("thumbnail info should exist")?;
    assert_eq!(
        thumbnail_info.mimetype().as_deref(),
        Some(thumb_mimetype),
        "we sent thumbnail in png format",
    );
    assert_eq!(
        thumbnail_info.size(),
        Some(size),
        "wrong file size in thumbnail",
    );

    Ok(())
}

#[tokio::test]
async fn room_msg_can_support_video_thumbnail() -> Result<()> {
    let _ = env_logger::try_init();

    let (mut user, room_id) = random_user_with_random_convo("room_msg_video_thumbnail").await?;
    let state_sync = user.start_sync();
    state_sync.await_has_synced_history().await?;

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(10);
    Retry::spawn(retry_strategy, || async {
        user.convo(room_id.to_string()).await
    })
    .await?;

    let convo = user.convo(room_id.to_string()).await?;
    let timeline = convo.timeline_stream();
    let stream = timeline.messages_stream();
    pin_mut!(stream);

    let bytes = include_bytes!("../fixtures/big_buck_bunny.mp4");
    let mut tmp_mp4 = NamedTempFile::new()?;
    tmp_mp4.as_file_mut().write_all(bytes)?;
    let mp4_name = tmp_mp4 // it is randomly generated by system and not big_buck_bunny.mp4
        .path()
        .file_name()
        .expect("it is not file")
        .to_string_lossy()
        .to_string();

    let bytes = include_bytes!("../fixtures/PNG_transparency_demonstration_1.png");
    let size = bytes.len() as u64;
    let mut tmp_png = NamedTempFile::new()?;
    tmp_png.as_file_mut().write_all(bytes)?;

    let mimetype = "video/mp4";
    let thumb_mimetype = "image/png";
    let draft = user
        .video_draft(
            tmp_mp4.path().to_string_lossy().to_string(),
            mimetype.to_owned(),
        )
        .thumbnail_file_path(tmp_png.path().to_string_lossy().to_string())
        .thumbnail_info(None, None, Some(thumb_mimetype.to_owned()), Some(size));
    timeline.send_message(Box::new(draft)).await?;

    // image msg may reach via pushback action or reset action
    let mut i = 30;
    let mut found = None;
    while i > 0 {
        if let Some(diff) = stream.next().now_or_never().flatten() {
            match diff.action().as_str() {
                "PushBack" | "Set" => {
                    let value = diff
                        .value()
                        .expect("diff pushback action should have valid value");
                    if let Some(msg_content) = match_media_msg(&value, mimetype, &mp4_name) {
                        found = Some(msg_content);
                    }
                }
                "Reset" => {
                    let values = diff
                        .values()
                        .expect("diff reset action should have valid values");
                    for value in values.iter() {
                        if let Some(msg_content) = match_media_msg(value, mimetype, &mp4_name) {
                            found = Some(msg_content);
                            break;
                        }
                    }
                }
                _ => {}
            }
            // yay
            if found.is_some() {
                break;
            }
        }
        i -= 1;
        sleep(Duration::from_secs(1)).await;
    }
    let msg_content = found.context("Even after 30 seconds, image msg not received")?;
    let thumbnail_info = msg_content
        .thumbnail_info()
        .context("thumbnail info should exist")?;
    assert_eq!(
        thumbnail_info.mimetype().as_deref(),
        Some(thumb_mimetype),
        "we sent thumbnail in png format",
    );
    assert_eq!(
        thumbnail_info.size(),
        Some(size),
        "wrong file size in thumbnail",
    );

    Ok(())
}

#[tokio::test]
async fn news_can_support_image_thumbnail() -> Result<()> {
    let _ = env_logger::try_init();
    let (mut user, room_id) = random_user_with_random_space("news_image_thumbnail").await?;
    let state_sync = user.start_sync();
    state_sync.await_has_synced_history().await?;

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(10);
    Retry::spawn(retry_strategy, || async {
        user.space(room_id.to_string()).await
    })
    .await?;

    let bytes = include_bytes!("../fixtures/kingfisher.jpg");
    let mut tmp_jpg = NamedTempFile::new()?;
    tmp_jpg.as_file_mut().write_all(bytes)?;

    let bytes = include_bytes!("../fixtures/PNG_transparency_demonstration_1.png");
    let size = bytes.len() as u64;
    let mut tmp_png = NamedTempFile::new()?;
    tmp_png.as_file_mut().write_all(bytes)?;

    let space = user.space(room_id.to_string()).await?;
    let mut draft = space.news_draft()?;
    let thumb_mimetype = "image/png";
    let image_draft = user
        .image_draft(
            tmp_jpg.path().to_string_lossy().to_string(),
            "image/jpeg".to_owned(),
        )
        .thumbnail_file_path(tmp_png.path().to_string_lossy().to_string())
        .thumbnail_info(None, None, Some(thumb_mimetype.to_owned()), Some(size));
    draft.add_slide(Box::new(image_draft.into())).await?;
    draft.send().await?;

    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(10);
    Retry::spawn(retry_strategy, || async {
        if space.latest_news_entries(1).await?.len() != 1 {
            bail!("news not found");
        }
        Ok(())
    })
    .await?;

    let slides = space.latest_news_entries(1).await?;
    let final_entry = slides.first().expect("Item is there");
    let image_slide = final_entry.get_slide(0).expect("we have a slide");
    assert_eq!(image_slide.type_str(), "image");

    let msg_content = image_slide.msg_content();
    assert!(
        msg_content.thumbnail_source().is_some(),
        "we sent thumbnail, but thumbnail source not available",
    );
    let thumbnail_info = msg_content
        .thumbnail_info()
        .context("we sent thumbnail, but thumbnail info not available")?;
    assert_eq!(
        thumbnail_info.mimetype().as_deref(),
        Some(thumb_mimetype),
        "we sent thumbnail in png format",
    );
    assert_eq!(
        thumbnail_info.size(),
        Some(size),
        "wrong file size in thumbnail",
    );

    Ok(())
}

#[tokio::test]
async fn news_can_support_video_thumbnail() -> Result<()> {
    let _ = env_logger::try_init();
    let (mut user, room_id) = random_user_with_random_space("news_video_thumbnail").await?;
    let state_sync = user.start_sync();
    state_sync.await_has_synced_history().await?;

    // wait for sync to catch up
    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(10);
    Retry::spawn(retry_strategy, || async {
        user.space(room_id.to_string()).await
    })
    .await?;

    let bytes = include_bytes!("../fixtures/big_buck_bunny.mp4");
    let mut tmp_mp4 = NamedTempFile::new()?;
    tmp_mp4.as_file_mut().write_all(bytes)?;

    let bytes = include_bytes!("../fixtures/PNG_transparency_demonstration_1.png");
    let size = bytes.len() as u64;
    let mut tmp_png = NamedTempFile::new()?;
    tmp_png.as_file_mut().write_all(bytes)?;

    let space = user.space(room_id.to_string()).await?;
    let mut draft = space.news_draft()?;
    let thumb_mimetype = "image/png";
    let video_draft = user
        .video_draft(
            tmp_mp4.path().to_string_lossy().to_string(),
            "video/mp4".to_owned(),
        )
        .thumbnail_file_path(tmp_png.path().to_string_lossy().to_string())
        .thumbnail_info(None, None, Some(thumb_mimetype.to_owned()), Some(size));
    draft.add_slide(Box::new(video_draft.into())).await?;
    draft.send().await?;

    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(10);
    Retry::spawn(retry_strategy, || async {
        if space.latest_news_entries(1).await?.len() != 1 {
            bail!("news not found");
        }
        Ok(())
    })
    .await?;

    let slides = space.latest_news_entries(1).await?;
    let final_entry = slides.first().expect("Item is there");
    let video_slide = final_entry.get_slide(0).expect("we have a slide");
    assert_eq!(video_slide.type_str(), "video");

    let msg_content = video_slide.msg_content();
    assert!(
        msg_content.thumbnail_source().is_some(),
        "we sent thumbnail, but thumbnail source not available",
    );
    let thumbnail_info = msg_content
        .thumbnail_info()
        .context("we sent thumbnail, but thumbnail info not available")?;
    assert_eq!(
        thumbnail_info.mimetype().as_deref(),
        Some(thumb_mimetype),
        "we sent thumbnail in png format",
    );
    assert_eq!(
        thumbnail_info.size(),
        Some(size),
        "wrong file size in thumbnail",
    );

    Ok(())
}

const TMPL: &str = r#"
version = "0.1"
name = "Smoketest Template"

[inputs]
main = { type = "user", is-default = true, required = true, description = "The starting user" }

[objects]
main_space = { type = "space", is-default = true, name = "{{ main.display_name }}’s attachment test space" }

[objects.acter-website-pin]
type = "pin"
title = "Acter Website"
url = "https://acter.global"

[objects.acter-source-pin]
type = "pin"
title = "Acter Source Code"
url = "https://github.com/acterglobal/a3"

[objects.example-data-pin]
type = "pin"
title = "Acter example pin"
content = { body = "example pin data" }
"#;

#[tokio::test]
async fn image_attachment_can_support_thumbnail() -> Result<()> {
    let _ = env_logger::try_init();
    let (user, sync_state, _engine) =
        random_user_with_template("image_attachment_thumbnail", TMPL).await?;
    sync_state.await_has_synced_history().await?;

    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(10);
    Retry::spawn(retry_strategy, || async {
        if user.pins().await?.len() != 3 {
            bail!("not all pins found");
        }
        Ok(())
    })
    .await?;

    let pin = user
        .pins()
        .await?
        .into_iter()
        .find(|p| !p.is_link())
        .expect("we’ve created one non-link pin");

    // START actual attachment on pin

    let attachments_manager = pin.attachments().await?;
    assert!(!attachments_manager.stats().has_attachments());

    // ---- let’s make a attachment

    let bytes = include_bytes!("../fixtures/kingfisher.jpg");
    let mut jpg_file = NamedTempFile::new()?;
    jpg_file.as_file_mut().write_all(bytes)?;

    let bytes = include_bytes!("../fixtures/PNG_transparency_demonstration_1.png");
    let size = bytes.len() as u64;
    let mut png_file = NamedTempFile::new()?;
    png_file.as_file_mut().write_all(bytes)?;

    let attachments_listener = attachments_manager.subscribe();
    let thumb_mimetype = "image/png";
    let base_draft = user
        .image_draft(
            jpg_file.path().to_string_lossy().to_string(),
            "image/jpeg".to_owned(),
        )
        .thumbnail_file_path(png_file.path().to_string_lossy().to_string())
        .thumbnail_info(None, None, Some(thumb_mimetype.to_owned()), Some(size));
    let attachment_id = attachments_manager
        .content_draft(Box::new(base_draft))
        .await?
        .send()
        .await?;

    let retry_strategy = FibonacciBackoff::from_millis(500).map(jitter).take(10);
    Retry::spawn(retry_strategy, || async {
        if attachments_listener.is_empty() {
            bail!("all still empty");
        }
        Ok(())
    })
    .await?;

    let attachments = attachments_manager.attachments().await?;
    assert_eq!(attachments.len(), 1);
    let attachment = attachments
        .first()
        .expect("first attachment should be available");
    assert_eq!(attachment.event_id(), attachment_id);
    assert_eq!(attachment.type_str(), "image");

    let msg_content = attachment
        .msg_content()
        .expect("msg content should be available");
    assert!(
        msg_content.thumbnail_source().is_some(),
        "we sent thumbnail, but thumbnail source not available",
    );
    let thumbnail_info = msg_content
        .thumbnail_info()
        .context("we sent thumbnail, but thumbnail info not available")?;
    assert_eq!(
        thumbnail_info.mimetype().as_deref(),
        Some(thumb_mimetype),
        "we sent thumbnail in png format",
    );
    assert_eq!(
        thumbnail_info.size(),
        Some(size),
        "wrong file size in thumbnail",
    );

    Ok(())
}

#[tokio::test]
async fn video_attachment_can_support_thumbnail() -> Result<()> {
    let _ = env_logger::try_init();
    let (user, sync_state, _engine) =
        random_user_with_template("image_attachment_thumbnail", TMPL).await?;
    sync_state.await_has_synced_history().await?;

    let retry_strategy = FibonacciBackoff::from_millis(100).map(jitter).take(10);
    Retry::spawn(retry_strategy, || async {
        if user.pins().await?.len() != 3 {
            bail!("not all pins found");
        }
        Ok(())
    })
    .await?;

    let pin = user
        .pins()
        .await?
        .into_iter()
        .find(|p| !p.is_link())
        .expect("we’ve created one non-link pin");

    // START actual attachment on pin

    let attachments_manager = pin.attachments().await?;
    assert!(!attachments_manager.stats().has_attachments());

    // ---- let’s make a attachment

    let bytes = include_bytes!("../fixtures/big_buck_bunny.mp4");
    let mut mp4_file = NamedTempFile::new()?;
    mp4_file.as_file_mut().write_all(bytes)?;

    let bytes = include_bytes!("../fixtures/PNG_transparency_demonstration_1.png");
    let size = bytes.len() as u64;
    let mut png_file = NamedTempFile::new()?;
    png_file.as_file_mut().write_all(bytes)?;

    let attachments_listener = attachments_manager.subscribe();
    let thumb_mimetype = "image/png";
    let base_draft = user
        .video_draft(
            mp4_file.path().to_string_lossy().to_string(),
            "video/mp4".to_owned(),
        )
        .thumbnail_file_path(png_file.path().to_string_lossy().to_string())
        .thumbnail_info(None, None, Some(thumb_mimetype.to_owned()), Some(size));
    let attachment_id = attachments_manager
        .content_draft(Box::new(base_draft))
        .await?
        .send()
        .await?;

    let retry_strategy = FibonacciBackoff::from_millis(500).map(jitter).take(10);
    Retry::spawn(retry_strategy, || async {
        if attachments_listener.is_empty() {
            bail!("all still empty");
        }
        Ok(())
    })
    .await?;

    let attachments = attachments_manager.attachments().await?;
    assert_eq!(attachments.len(), 1);
    let attachment = attachments
        .first()
        .expect("first attachment should be available");
    assert_eq!(attachment.event_id(), attachment_id);
    assert_eq!(attachment.type_str(), "video");

    let msg_content = attachment
        .msg_content()
        .expect("msg content should be available");
    assert!(
        msg_content.thumbnail_source().is_some(),
        "we sent thumbnail, but thumbnail source not available",
    );
    let thumbnail_info = msg_content
        .thumbnail_info()
        .context("we sent thumbnail, but thumbnail info not available")?;
    assert_eq!(
        thumbnail_info.mimetype().as_deref(),
        Some(thumb_mimetype),
        "we sent thumbnail in png format",
    );
    assert_eq!(
        thumbnail_info.size(),
        Some(size),
        "wrong file size in thumbnail",
    );

    Ok(())
}
