use crate::{activities_new::comment::like_or_dislike_comment, inbox::new_inbox_routing::Activity};
use activitystreams::activity::kind::DislikeType;
use lemmy_apub_lib::{PublicUrl, ReceiveActivity};
use lemmy_utils::LemmyError;
use lemmy_websocket::LemmyContext;
use url::Url;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DislikeComment {
  actor: Url,
  to: PublicUrl,
  object: Url,
  cc: Vec<Url>,
  #[serde(rename = "type")]
  kind: DislikeType,
}

#[async_trait::async_trait(?Send)]
impl ReceiveActivity for Activity<DislikeComment> {
  async fn receive(
    &self,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    like_or_dislike_comment(
      -1,
      &self.inner.actor,
      &self.inner.object,
      context,
      request_counter,
    )
    .await
  }
}