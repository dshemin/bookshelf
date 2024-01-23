use garde::Validate;
use serde::Serialize;
use uuid::Uuid;

use crate::domain_type;

/// The internal user.
///
/// Holds all information about internal user.
#[derive(Serialize)]
pub struct InternalUser {
    pub(crate) id: ID,
    pub(crate) login: Login,
    pub(crate) password: Password,
    pub(crate) role: Role,
}

impl InternalUser {
    pub fn new(login: Login, password: Password, role: Role) -> Self {
        Self {
            id: ID::new_v4(),
            login,
            password,
            role,
        }
    }
}

/// The external user.
///
/// Holds all information about external user.
#[derive(Serialize)]
pub struct ExternalUser {
    pub(crate) id: ID,
    pub(crate) external_id: ExternalID,
    pub(crate) role: Role,
}

impl ExternalUser {
    pub fn new(external_id: ExternalID, role: Role) -> Self {
        Self {
            id: ID::new_v4(),
            external_id,
            role,
        }
    }
}

/// User unique identifier.
pub type ID = Uuid;

domain_type!(Login, LoginResult, String, email);
domain_type!(
    Password,
    PasswordResult,
    String,
    ascii,
    length(min = 8),
    custom(is_strong_password)
);
domain_type!(ExternalID, ExternalIDResult, String, length(min = 1));

fn is_strong_password(value: &str, _: &()) -> garde::Result {
    if value.find(char::is_alphabetic).is_none() {
        return Err(garde::Error::new("didn't contains a letter"));
    }

    if value.find(char::is_numeric).is_none() {
        return Err(garde::Error::new("didn't contains a digit"));
    }

    if value.find(|x: char| x.is_ascii_punctuation()).is_none() {
        return Err(garde::Error::new("didn't contains a special character"));
    }

    Ok(())
}

/// User's role.
#[derive(Serialize)]
pub enum Role {
    Admin,
    User,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod login {
        use super::*;

        mod new {
            use super::*;

            #[test]
            fn valid() {
                let result = Login::new("user@example.com");

                assert!(result.is_ok());
                assert_eq!(result.unwrap(), Login("user@example.com".to_owned()));
            }

            #[test]
            fn invalid_empty() {
                let result = Login::new("");

                assert!(result.is_err());
                let err = result.err().unwrap();
                assert_eq!("not a valid email: value is empty", err.to_string());
            }

            #[test]
            fn invalid_not_a_email() {
                let result = Login::new("foo");

                assert!(result.is_err());
                let err = result.err().unwrap();
                assert_eq!("not a valid email: value is missing `@`", err.to_string());
            }
        }
    }

    mod password {
        use super::*;

        mod new {
            use super::*;

            #[test]
            fn valid() {
                let result = Password::new("Pa$$w0rd");

                assert!(result.is_ok());
                assert_eq!(result.unwrap(), Password("Pa$$w0rd".to_owned()));
            }

            #[test]
            fn invalid_non_ascii() {
                let result = Password::new("Pa$$w0rdƱ");

                assert!(result.is_err());
                let err = result.err().unwrap();
                assert_eq!("not ascii", err.to_string());
            }

            #[test]
            fn invalid_less_than_required() {
                let result = Password::new("p1$");

                assert!(result.is_err());
                let err = result.err().unwrap();
                assert_eq!("length is lower than 8", err.to_string());
            }

            #[test]
            fn invalid_did_not_has_a_letter() {
                let result = Password::new("$145");

                assert!(result.is_err());
                let err = result.err().unwrap();
                assert_eq!("didn't contains a letter", err.to_string());
            }

            #[test]
            fn invalid_did_not_has_a_digit() {
                let result = Password::new("pa$$word");

                assert!(result.is_err());
                let err = result.err().unwrap();
                assert_eq!("didn't contains a digit", err.to_string());
            }

            #[test]
            fn invalid_did_not_has_a_special_character() {
                let result = Password::new("passw0rd");

                assert!(result.is_err());
                let err = result.err().unwrap();
                assert_eq!("didn't contains a special character", err.to_string());
            }
        }
    }

    mod external_id {
        use super::*;

        mod new {
            use super::*;

            #[test]
            fn valid() {
                let result = ExternalID::new("1");

                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ExternalID("1".to_owned()));
            }

            #[test]
            fn invalid_less_than_required() {
                let result = ExternalID::new("");

                assert!(result.is_err());
                let err = result.err().unwrap();
                assert_eq!("length is lower than 1", err.to_string());
            }
        }
    }
}
