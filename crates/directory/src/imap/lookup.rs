use mail_send::Credentials;
use smtp_proto::{AUTH_CRAM_MD5, AUTH_LOGIN, AUTH_OAUTHBEARER, AUTH_PLAIN, AUTH_XOAUTH2};

use crate::{Directory, DirectoryError, Principal};

use super::{ImapDirectory, ImapError};

#[async_trait::async_trait]
impl Directory for ImapDirectory {
    async fn authenticate(
        &self,
        credentials: &Credentials<String>,
    ) -> crate::Result<Option<Principal>> {
        let mut client = self.pool.get().await?;
        let mechanism = match credentials {
            Credentials::Plain { .. }
                if (client.mechanisms & (AUTH_PLAIN | AUTH_LOGIN | AUTH_CRAM_MD5)) != 0 =>
            {
                if client.mechanisms & AUTH_CRAM_MD5 != 0 {
                    AUTH_CRAM_MD5
                } else if client.mechanisms & AUTH_PLAIN != 0 {
                    AUTH_PLAIN
                } else {
                    AUTH_LOGIN
                }
            }
            Credentials::OAuthBearer { .. } if client.mechanisms & AUTH_OAUTHBEARER != 0 => {
                AUTH_OAUTHBEARER
            }
            Credentials::XOauth2 { .. } if client.mechanisms & AUTH_XOAUTH2 != 0 => AUTH_XOAUTH2,
            _ => {
                tracing::warn!(
                    context = "remote",
                    event = "error",
                    protocol = "imap",
                    "IMAP server does not offer any supported auth mechanisms.",
                );
                return Ok(None);
            }
        };

        match client.authenticate(mechanism, credentials).await {
            Ok(_) => Ok(Some(Principal::default())),
            Err(err) => match &err {
                ImapError::AuthenticationFailed => Ok(None),
                _ => Err(err.into()),
            },
        }
    }

    async fn principal_by_name(&self, _name: &str) -> crate::Result<Option<Principal>> {
        Err(DirectoryError::unsupported("imap", "principal_by_name"))
    }

    async fn principal_by_id(&self, _id: u32) -> crate::Result<Option<Principal>> {
        Err(DirectoryError::unsupported("imap", "principal_by_id"))
    }

    async fn member_of(&self, _principal: &Principal) -> crate::Result<Vec<u32>> {
        Err(DirectoryError::unsupported("imap", "member_of"))
    }

    async fn emails_by_id(&self, _id: u32) -> crate::Result<Vec<String>> {
        Err(DirectoryError::unsupported("imap", "emails_by_id"))
    }

    async fn ids_by_email(&self, _address: &str) -> crate::Result<Vec<u32>> {
        Err(DirectoryError::unsupported("imap", "ids_by_email"))
    }

    async fn rcpt(&self, _address: &str) -> crate::Result<bool> {
        Err(DirectoryError::unsupported("imap", "rcpt"))
    }

    async fn vrfy(&self, _address: &str) -> crate::Result<Vec<String>> {
        Err(DirectoryError::unsupported("imap", "vrfy"))
    }

    async fn expn(&self, _address: &str) -> crate::Result<Vec<String>> {
        Err(DirectoryError::unsupported("imap", "expn"))
    }

    async fn query(&self, _query: &str, _params: &[&str]) -> crate::Result<bool> {
        Err(DirectoryError::unsupported("imap", "query"))
    }
}
