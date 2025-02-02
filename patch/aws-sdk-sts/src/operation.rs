// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssumeRole`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`assume_role`](crate::client::Client::assume_role).
///
/// See [`crate::client::fluent_builders::AssumeRole`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssumeRole {
    _private: (),
}
impl AssumeRole {
    /// Creates a new builder-style object to manufacture [`AssumeRoleInput`](crate::input::AssumeRoleInput).
    pub fn builder() -> crate::input::assume_role_input::Builder {
        crate::input::assume_role_input::Builder::default()
    }
    /// Creates a new `AssumeRole` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssumeRole {
    type Output =
        std::result::Result<crate::output::AssumeRoleOutput, crate::error::AssumeRoleError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_assume_role_error(response)
        } else {
            crate::operation_deser::parse_assume_role_response(response)
        }
    }
}

/// Operation shape for `AssumeRoleWithSAML`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`assume_role_with_saml`](crate::client::Client::assume_role_with_saml).
///
/// See [`crate::client::fluent_builders::AssumeRoleWithSAML`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssumeRoleWithSAML {
    _private: (),
}
impl AssumeRoleWithSAML {
    /// Creates a new builder-style object to manufacture [`AssumeRoleWithSamlInput`](crate::input::AssumeRoleWithSamlInput).
    pub fn builder() -> crate::input::assume_role_with_saml_input::Builder {
        crate::input::assume_role_with_saml_input::Builder::default()
    }
    /// Creates a new `AssumeRoleWithSAML` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssumeRoleWithSAML {
    type Output = std::result::Result<
        crate::output::AssumeRoleWithSamlOutput,
        crate::error::AssumeRoleWithSAMLError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_assume_role_with_saml_error(response)
        } else {
            crate::operation_deser::parse_assume_role_with_saml_response(response)
        }
    }
}

/// Operation shape for `AssumeRoleWithWebIdentity`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`assume_role_with_web_identity`](crate::client::Client::assume_role_with_web_identity).
///
/// See [`crate::client::fluent_builders::AssumeRoleWithWebIdentity`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssumeRoleWithWebIdentity {
    _private: (),
}
impl AssumeRoleWithWebIdentity {
    /// Creates a new builder-style object to manufacture [`AssumeRoleWithWebIdentityInput`](crate::input::AssumeRoleWithWebIdentityInput).
    pub fn builder() -> crate::input::assume_role_with_web_identity_input::Builder {
        crate::input::assume_role_with_web_identity_input::Builder::default()
    }
    /// Creates a new `AssumeRoleWithWebIdentity` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssumeRoleWithWebIdentity {
    type Output = std::result::Result<
        crate::output::AssumeRoleWithWebIdentityOutput,
        crate::error::AssumeRoleWithWebIdentityError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_assume_role_with_web_identity_error(response)
        } else {
            crate::operation_deser::parse_assume_role_with_web_identity_response(response)
        }
    }
}

/// Operation shape for `DecodeAuthorizationMessage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`decode_authorization_message`](crate::client::Client::decode_authorization_message).
///
/// See [`crate::client::fluent_builders::DecodeAuthorizationMessage`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DecodeAuthorizationMessage {
    _private: (),
}
impl DecodeAuthorizationMessage {
    /// Creates a new builder-style object to manufacture [`DecodeAuthorizationMessageInput`](crate::input::DecodeAuthorizationMessageInput).
    pub fn builder() -> crate::input::decode_authorization_message_input::Builder {
        crate::input::decode_authorization_message_input::Builder::default()
    }
    /// Creates a new `DecodeAuthorizationMessage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DecodeAuthorizationMessage {
    type Output = std::result::Result<
        crate::output::DecodeAuthorizationMessageOutput,
        crate::error::DecodeAuthorizationMessageError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_decode_authorization_message_error(response)
        } else {
            crate::operation_deser::parse_decode_authorization_message_response(response)
        }
    }
}

/// Operation shape for `GetAccessKeyInfo`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_access_key_info`](crate::client::Client::get_access_key_info).
///
/// See [`crate::client::fluent_builders::GetAccessKeyInfo`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAccessKeyInfo {
    _private: (),
}
impl GetAccessKeyInfo {
    /// Creates a new builder-style object to manufacture [`GetAccessKeyInfoInput`](crate::input::GetAccessKeyInfoInput).
    pub fn builder() -> crate::input::get_access_key_info_input::Builder {
        crate::input::get_access_key_info_input::Builder::default()
    }
    /// Creates a new `GetAccessKeyInfo` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAccessKeyInfo {
    type Output = std::result::Result<
        crate::output::GetAccessKeyInfoOutput,
        crate::error::GetAccessKeyInfoError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_access_key_info_error(response)
        } else {
            crate::operation_deser::parse_get_access_key_info_response(response)
        }
    }
}

/// Operation shape for `GetCallerIdentity`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_caller_identity`](crate::client::Client::get_caller_identity).
///
/// See [`crate::client::fluent_builders::GetCallerIdentity`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetCallerIdentity {
    _private: (),
}
impl GetCallerIdentity {
    /// Creates a new builder-style object to manufacture [`GetCallerIdentityInput`](crate::input::GetCallerIdentityInput).
    pub fn builder() -> crate::input::get_caller_identity_input::Builder {
        crate::input::get_caller_identity_input::Builder::default()
    }
    /// Creates a new `GetCallerIdentity` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCallerIdentity {
    type Output = std::result::Result<
        crate::output::GetCallerIdentityOutput,
        crate::error::GetCallerIdentityError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_caller_identity_error(response)
        } else {
            crate::operation_deser::parse_get_caller_identity_response(response)
        }
    }
}

/// Operation shape for `GetFederationToken`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_federation_token`](crate::client::Client::get_federation_token).
///
/// See [`crate::client::fluent_builders::GetFederationToken`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetFederationToken {
    _private: (),
}
impl GetFederationToken {
    /// Creates a new builder-style object to manufacture [`GetFederationTokenInput`](crate::input::GetFederationTokenInput).
    pub fn builder() -> crate::input::get_federation_token_input::Builder {
        crate::input::get_federation_token_input::Builder::default()
    }
    /// Creates a new `GetFederationToken` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetFederationToken {
    type Output = std::result::Result<
        crate::output::GetFederationTokenOutput,
        crate::error::GetFederationTokenError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_federation_token_error(response)
        } else {
            crate::operation_deser::parse_get_federation_token_response(response)
        }
    }
}

/// Operation shape for `GetSessionToken`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_session_token`](crate::client::Client::get_session_token).
///
/// See [`crate::client::fluent_builders::GetSessionToken`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetSessionToken {
    _private: (),
}
impl GetSessionToken {
    /// Creates a new builder-style object to manufacture [`GetSessionTokenInput`](crate::input::GetSessionTokenInput).
    pub fn builder() -> crate::input::get_session_token_input::Builder {
        crate::input::get_session_token_input::Builder::default()
    }
    /// Creates a new `GetSessionToken` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSessionToken {
    type Output = std::result::Result<
        crate::output::GetSessionTokenOutput,
        crate::error::GetSessionTokenError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_session_token_error(response)
        } else {
            crate::operation_deser::parse_get_session_token_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
