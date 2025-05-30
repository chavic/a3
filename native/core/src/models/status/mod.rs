use matrix_sdk::ruma::{
    events::{
        room::member::MembershipChange as MChange, AnyStateEvent, AnyTimelineEvent, StateEvent,
    },
    OwnedEventId, UserId,
};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

mod membership;
mod profile;
mod room_state;

use crate::{
    events::AnyActerEvent,
    referencing::{ExecuteReference, IndexKey},
};
pub use membership::MembershipContent;
pub use profile::{Change, ProfileContent};
pub use room_state::{
    PolicyRuleRoomContent, PolicyRuleServerContent, PolicyRuleUserContent, RoomAvatarContent,
    RoomCreateContent, RoomEncryptionContent, RoomGuestAccessContent, RoomHistoryVisibilityContent,
    RoomJoinRulesContent, RoomNameContent, RoomPinnedEventsContent, RoomPowerLevelsContent,
    RoomServerAclContent, RoomTombstoneContent, RoomTopicContent, SpaceChildContent,
    SpaceParentContent,
};

use super::{conversion::ParseError, ActerModel, Capability, EventMeta, Store};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ActerSupportedRoomStatusEvents {
    MembershipChange(MembershipContent),
    ProfileChange(ProfileContent),
    PolicyRuleRoom(PolicyRuleRoomContent),
    PolicyRuleServer(PolicyRuleServerContent),
    PolicyRuleUser(PolicyRuleUserContent),
    RoomAvatar(RoomAvatarContent),
    RoomCreate(RoomCreateContent),
    RoomEncryption(RoomEncryptionContent),
    RoomGuestAccess(RoomGuestAccessContent),
    RoomHistoryVisibility(RoomHistoryVisibilityContent),
    RoomJoinRules(RoomJoinRulesContent),
    RoomName(RoomNameContent),
    RoomPinnedEvents(RoomPinnedEventsContent),
    RoomPowerLevels(RoomPowerLevelsContent),
    RoomServerAcl(RoomServerAclContent),
    RoomTombstone(RoomTombstoneContent),
    RoomTopic(RoomTopicContent),
    SpaceChild(SpaceChildContent),
    SpaceParent(SpaceParentContent),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoomStatus {
    pub(crate) inner: ActerSupportedRoomStatusEvents,
    pub meta: EventMeta,
}

impl Deref for RoomStatus {
    type Target = ActerSupportedRoomStatusEvents;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFrom<AnyStateEvent> for RoomStatus {
    type Error = ParseError;

    fn try_from(event: AnyStateEvent) -> Result<RoomStatus, ParseError> {
        let meta = EventMeta {
            event_id: event.event_id().to_owned(),
            room_id: event.room_id().to_owned(),
            sender: event.sender().to_owned(),
            origin_server_ts: event.origin_server_ts(),
            redacted: None,
        };
        let make_err = |event| {
            ParseError::UnsupportedEvent(Box::new(AnyActerEvent::RegularTimelineEvent(
                AnyTimelineEvent::State(event),
            )))
        };
        match &event {
            AnyStateEvent::RoomMember(StateEvent::Original(inner)) => {
                let membership_change = inner.content.membership_change(
                    inner.prev_content().map(|c| c.details()),
                    &inner.sender,
                    &inner.state_key,
                );
                let inner_status = if let MChange::ProfileChanged {
                    displayname_change,
                    avatar_url_change,
                } = membership_change
                {
                    let content = ProfileContent::new(
                        inner.state_key.clone(),
                        displayname_change.map(|c| Change {
                            new_val: c.new.map(ToOwned::to_owned),
                            old_val: c.old.map(ToOwned::to_owned),
                        }),
                        avatar_url_change.map(|c| Change {
                            new_val: c.new.map(ToOwned::to_owned),
                            old_val: c.old.map(ToOwned::to_owned),
                        }),
                    );
                    ActerSupportedRoomStatusEvents::ProfileChange(content)
                } else if let Ok(content) =
                    MembershipContent::try_from((membership_change, inner.state_key.clone()))
                {
                    ActerSupportedRoomStatusEvents::MembershipChange(content)
                } else {
                    return Err(make_err(event));
                };
                Ok(RoomStatus {
                    inner: inner_status,
                    meta,
                })
            }
            AnyStateEvent::PolicyRuleRoom(StateEvent::Original(inner)) => {
                let content = PolicyRuleRoomContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::PolicyRuleRoom(content),
                    meta,
                })
            }
            AnyStateEvent::PolicyRuleServer(StateEvent::Original(inner)) => {
                let content = PolicyRuleServerContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::PolicyRuleServer(content),
                    meta,
                })
            }
            AnyStateEvent::PolicyRuleUser(StateEvent::Original(inner)) => {
                let content = PolicyRuleUserContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::PolicyRuleUser(content),
                    meta,
                })
            }
            AnyStateEvent::RoomAvatar(StateEvent::Original(inner)) => {
                let content = RoomAvatarContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomAvatar(content),
                    meta,
                })
            }
            AnyStateEvent::RoomCreate(StateEvent::Original(inner)) => {
                let content = RoomCreateContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomCreate(content),
                    meta,
                })
            }
            AnyStateEvent::RoomEncryption(StateEvent::Original(inner)) => {
                let content = RoomEncryptionContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomEncryption(content),
                    meta,
                })
            }
            AnyStateEvent::RoomGuestAccess(StateEvent::Original(inner)) => {
                let content = RoomGuestAccessContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomGuestAccess(content),
                    meta,
                })
            }
            AnyStateEvent::RoomHistoryVisibility(StateEvent::Original(inner)) => {
                let content = RoomHistoryVisibilityContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomHistoryVisibility(content),
                    meta,
                })
            }
            AnyStateEvent::RoomJoinRules(StateEvent::Original(inner)) => {
                let content = RoomJoinRulesContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomJoinRules(content),
                    meta,
                })
            }
            AnyStateEvent::RoomName(StateEvent::Original(inner)) => {
                let content = RoomNameContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomName(content),
                    meta,
                })
            }
            AnyStateEvent::RoomPinnedEvents(StateEvent::Original(inner)) => {
                let content = RoomPinnedEventsContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomPinnedEvents(content),
                    meta,
                })
            }
            AnyStateEvent::RoomPowerLevels(StateEvent::Original(inner)) => {
                let content = RoomPowerLevelsContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomPowerLevels(content),
                    meta,
                })
            }
            AnyStateEvent::RoomServerAcl(StateEvent::Original(inner)) => {
                let content = RoomServerAclContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomServerAcl(content),
                    meta,
                })
            }
            AnyStateEvent::RoomTombstone(StateEvent::Original(inner)) => {
                let content = RoomTombstoneContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomTombstone(content),
                    meta,
                })
            }
            AnyStateEvent::RoomTopic(StateEvent::Original(inner)) => {
                let content = RoomTopicContent::new(
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::RoomTopic(content),
                    meta,
                })
            }
            AnyStateEvent::SpaceChild(StateEvent::Original(inner)) => {
                let state_key = event.state_key().to_owned();
                let content = SpaceChildContent::new(
                    state_key,
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::SpaceChild(content),
                    meta,
                })
            }
            AnyStateEvent::SpaceParent(StateEvent::Original(inner)) => {
                let state_key = event.state_key().to_owned();
                let content = SpaceParentContent::new(
                    state_key,
                    inner.content.clone(),
                    inner.unsigned.prev_content.clone(),
                );
                Ok(RoomStatus {
                    inner: ActerSupportedRoomStatusEvents::SpaceParent(content),
                    meta,
                })
            }
            _ => Err(make_err(event)),
        }
    }
}

impl ActerModel for RoomStatus {
    fn indizes(&self, _user_id: &UserId) -> Vec<IndexKey> {
        vec![
            IndexKey::RoomHistory(self.meta.room_id.clone()),
            IndexKey::AllHistory,
        ]
    }

    fn event_meta(&self) -> &EventMeta {
        &self.meta
    }

    fn capabilities(&self) -> &[Capability] {
        &[]
    }

    fn belongs_to(&self) -> Option<Vec<OwnedEventId>> {
        // Do not trigger the parent to update, we have a manager
        None
    }

    async fn execute(self, store: &Store) -> crate::Result<Vec<ExecuteReference>> {
        store.save(self.into()).await
    }
}
