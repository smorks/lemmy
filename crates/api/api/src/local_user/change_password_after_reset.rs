use actix_web::web::{Data, Json};
use lemmy_api_utils::{context::LemmyContext, utils::password_length_check};
use lemmy_db_schema::source::{
  local_user::LocalUser,
  login_token::LoginToken,
  password_reset_request::PasswordResetRequest,
};
use lemmy_db_views_site::api::{PasswordChangeAfterReset, SuccessResponse};
use lemmy_utils::error::{LemmyErrorType, LemmyResult};

pub async fn change_password_after_reset(
  data: Json<PasswordChangeAfterReset>,
  context: Data<LemmyContext>,
) -> LemmyResult<Json<SuccessResponse>> {
  // Fetch the user_id from the token
  let token = data.token.clone();
  let local_user_id = PasswordResetRequest::read_and_delete(&mut context.pool(), &token)
    .await?
    .local_user_id;

  password_length_check(&data.password)?;

  // Make sure passwords match
  if data.password != data.password_verify {
    Err(LemmyErrorType::PasswordsDoNotMatch)?
  }

  // Update the user with the new password
  let password = data.password.clone();
  LocalUser::update_password(&mut context.pool(), local_user_id, &password).await?;

  LoginToken::invalidate_all(&mut context.pool(), local_user_id).await?;

  Ok(Json(SuccessResponse::default()))
}
