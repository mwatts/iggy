use crate::binary::binary_client::BinaryClient;
use crate::binary::{fail_if_not_authenticated, mapper, ClientState};
use crate::bytes_serializable::BytesSerializable;
use crate::client::PersonalAccessTokenClient;
use crate::command::*;
use crate::error::IggyError;
use crate::models::identity_info::IdentityInfo;
use crate::models::personal_access_token::{PersonalAccessTokenInfo, RawPersonalAccessToken};
use crate::personal_access_tokens::create_personal_access_token::CreatePersonalAccessToken;
use crate::personal_access_tokens::delete_personal_access_token::DeletePersonalAccessToken;
use crate::personal_access_tokens::get_personal_access_tokens::GetPersonalAccessTokens;
use crate::personal_access_tokens::login_with_personal_access_token::LoginWithPersonalAccessToken;
use crate::utils::personal_access_token_expiry::PersonalAccessTokenExpiry;

#[async_trait::async_trait]
impl<B: BinaryClient> PersonalAccessTokenClient for B {
    async fn get_personal_access_tokens(&self) -> Result<Vec<PersonalAccessTokenInfo>, IggyError> {
        fail_if_not_authenticated(self).await?;
        let response = self
            .send_with_response(
                GET_PERSONAL_ACCESS_TOKENS_CODE,
                GetPersonalAccessTokens {}.as_bytes(),
            )
            .await?;
        mapper::map_personal_access_tokens(response)
    }

    async fn create_personal_access_token(
        &self,
        name: &str,
        expiry: PersonalAccessTokenExpiry,
    ) -> Result<RawPersonalAccessToken, IggyError> {
        fail_if_not_authenticated(self).await?;
        let response = self
            .send_with_response(
                CREATE_PERSONAL_ACCESS_TOKEN_CODE,
                CreatePersonalAccessToken {
                    name: name.to_string(),
                    expiry: expiry.into(),
                }
                .as_bytes(),
            )
            .await?;
        mapper::map_raw_pat(response)
    }

    async fn delete_personal_access_token(&self, name: &str) -> Result<(), IggyError> {
        fail_if_not_authenticated(self).await?;
        self.send_with_response(
            DELETE_PERSONAL_ACCESS_TOKEN_CODE,
            DeletePersonalAccessToken {
                name: name.to_string(),
            }
            .as_bytes(),
        )
        .await?;
        Ok(())
    }

    async fn login_with_personal_access_token(
        &self,
        token: &str,
    ) -> Result<IdentityInfo, IggyError> {
        let response = self
            .send_with_response(
                LOGIN_WITH_PERSONAL_ACCESS_TOKEN_CODE,
                LoginWithPersonalAccessToken {
                    token: token.to_string(),
                }
                .as_bytes(),
            )
            .await?;
        self.set_state(ClientState::Authenticated).await;
        mapper::map_identity_info(response)
    }
}
