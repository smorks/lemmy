use activitypub_federation::config::Data;
use actix_web::web::Json;
use lemmy_api_utils::context::LemmyContext;
use lemmy_db_schema::{
  source::private_message::{PrivateMessage, PrivateMessageUpdateForm},
  traits::Crud,
};
use lemmy_db_views_local_user::LocalUserView;
use lemmy_db_views_private_message::{
  api::{DeletePrivateMessageForRecipient, PrivateMessageResponse},
  PrivateMessageView,
};
use lemmy_utils::error::{LemmyErrorType, LemmyResult};

pub async fn delete_private_message_for_recipient(
  data: Json<DeletePrivateMessageForRecipient>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<PrivateMessageResponse>> {
  // Checking permissions
  let private_message_id = data.private_message_id;
  let orig_private_message = PrivateMessage::read(&mut context.pool(), private_message_id).await?;
  if local_user_view.person.id != orig_private_message.recipient_id {
    Err(LemmyErrorType::EditPrivateMessageNotAllowed)?
  }

  // Doing the update
  let private_message_id = data.private_message_id;
  let deleted = data.deleted;
  let _private_message = PrivateMessage::update(
    &mut context.pool(),
    private_message_id,
    &PrivateMessageUpdateForm {
      deleted_by_recipient: Some(deleted),
      ..Default::default()
    },
  )
  .await?;

  let view = PrivateMessageView::read(&mut context.pool(), private_message_id).await?;
  Ok(Json(PrivateMessageResponse {
    private_message_view: view,
  }))
}
