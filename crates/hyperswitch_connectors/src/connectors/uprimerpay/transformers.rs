use common_enums::enums;
use common_utils::{pii::Email, types::MinorUnit};
use error_stack::ResultExt;
use hyperswitch_domain_models::{
    payment_method_data::{Card, PaymentMethodData},
    router_data::{AccessToken, ConnectorAuthType},
    router_flow_types::access_token_auth::AccessTokenAuth,
    router_flow_types::refunds::{Execute, RSync},
    router_request_types::{AccessTokenRequestData, ResponseId},
    router_response_types::{PaymentsResponseData, RefundsResponseData, RedirectForm},
    types::{
        PaymentsAuthorizeRouterData, PaymentsSyncRouterData, RefreshTokenRouterData,
        RefundsRouterData,
    },
};
use hyperswitch_interfaces::errors;
use hyperswitch_masking::{ExposeInterface, Secret};
use serde::{Deserialize, Serialize};

use crate::{
    types::{
        PaymentsResponseRouterData, PaymentsSyncResponseRouterData, RefundsResponseRouterData,
        ResponseRouterData,
    },
    utils::{
        self, CardData, PaymentsAuthorizeRequestData, RouterData as RouterDataUtils,
    },
};

const UPRIMERPAY_SUCCESS_CODE: &str = "00000000";
const DEFAULT_CHALLENGE_WINDOW: &str = "05";

#[derive(Debug, Serialize)]
pub struct UprimerpayRouterData<T> {
    pub amount: MinorUnit,
    pub router_data: T,
}

impl<T> From<(MinorUnit, T)> for UprimerpayRouterData<T> {
    fn from((amount, router_data): (MinorUnit, T)) -> Self {
        Self {
            amount,
            router_data,
        }
    }
}

pub struct UprimerpayAuthType {
    pub app_id: Secret<String>,
    pub access_code: Secret<String>,
    pub secret_key: Secret<String>,
}

impl TryFrom<&ConnectorAuthType> for UprimerpayAuthType {
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(auth_type: &ConnectorAuthType) -> Result<Self, Self::Error> {
        match auth_type {
            ConnectorAuthType::SignatureKey {
                api_key,
                key1,
                api_secret,
            } => Ok(Self {
                app_id: api_key.to_owned(),
                access_code: key1.to_owned(),
                secret_key: api_secret.to_owned(),
            }),
            _ => Err(errors::ConnectorError::FailedToObtainAuthType.into()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayAccessTokenResponse {
    pub code: String,
    pub msg: Option<String>,
    pub data: Option<UprimerpayAccessTokenData>,
    pub success: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayAccessTokenData {
    pub token: Secret<String>,
    pub expire_in: Option<i64>,
}

impl TryFrom<ResponseRouterData<AccessTokenAuth, UprimerpayAccessTokenResponse, AccessTokenRequestData, AccessToken>>
    for RefreshTokenRouterData
{
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(
        item: ResponseRouterData<
            AccessTokenAuth,
            UprimerpayAccessTokenResponse,
            AccessTokenRequestData,
            AccessToken,
        >,
    ) -> Result<Self, Self::Error> {
        let response_data = item
            .response
            .data
            .ok_or(errors::ConnectorError::ResponseDeserializationFailed)?;

        Ok(Self {
            response: Ok(AccessToken {
                token: response_data.token,
                expires: response_data.expire_in.unwrap_or(3600),
            }),
            ..item.data
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayPaymentsRequest {
    amount: MinorUnit,
    app_id: Secret<String>,
    currency: enums::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    descriptor: Option<String>,
    merchant_order_id: String,
    request_id: String,
    cancel_url: String,
    success_url: String,
    failure_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_url: Option<String>,
    order_time: String,
    payment_method: UprimerpayPaymentMethod,
    billing: UprimerpayBilling,
    products: Vec<UprimerpayProduct>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<UprimerpayShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_data: Option<UprimerpayDeviceData>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayPaymentMethod {
    method_type: UprimerpayPaymentMethodType,
    card: UprimerpayCard,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UprimerpayPaymentMethodType {
    Card,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayCard {
    number: cards::CardNumber,
    cvv: Secret<String>,
    expiry_month: Secret<String>,
    expiry_year: Secret<String>,
    first_name: Secret<String>,
    last_name: Secret<String>,
    billing: UprimerpayBilling,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayBilling {
    first_name: Secret<String>,
    last_name: Secret<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<Secret<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<Email>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<enums::CountryAlpha2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Secret<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    street: Option<Secret<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_code: Option<Secret<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayProduct {
    code: String,
    name: String,
    quantity: u32,
    sku: String,
    unit_price: MinorUnit,
    total_amount: MinorUnit,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayShipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<Secret<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<Secret<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<Secret<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<enums::CountryAlpha2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Secret<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    street: Option<Secret<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    street2: Option<Secret<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_code: Option<Secret<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayDeviceData {
    accept_header: String,
    browser_java_enabled: String,
    browser_javascript_enabled: String,
    browser_user_agent: String,
    challenge_window: String,
    language: String,
    screen_color_depth: String,
    screen_height: String,
    screen_width: String,
    timezone: String,
}

impl TryFrom<(&UprimerpayRouterData<&PaymentsAuthorizeRouterData>, &Card)>
    for UprimerpayPaymentsRequest
{
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(
        value: (&UprimerpayRouterData<&PaymentsAuthorizeRouterData>, &Card),
    ) -> Result<Self, Self::Error> {
        let (item, card) = value;
        let auth = UprimerpayAuthType::try_from(&item.router_data.connector_auth_type)?;
        let router_data = item.router_data;
        let billing = get_billing(router_data)?;
        let return_url = router_data.request.get_router_return_url()?;
        let merchant_order_id = router_data
            .request
            .merchant_order_reference_id
            .clone()
            .unwrap_or_else(|| router_data.connector_request_reference_id.clone());
        let request_id = router_data.connector_request_reference_id.clone();
        let description = router_data
            .description
            .clone()
            .unwrap_or_else(|| "Payment".to_string());

        let payment_method = UprimerpayPaymentMethod {
            method_type: UprimerpayPaymentMethodType::Card,
            card: UprimerpayCard {
                number: card.card_number.clone(),
                cvv: card.card_cvc.clone(),
                expiry_month: card.get_card_expiry_month_2_digit()?,
                expiry_year: card.get_card_expiry_year_2_digit()?,
                first_name: billing.first_name.clone(),
                last_name: billing.last_name.clone(),
                billing: billing.clone(),
            },
        };

        Ok(Self {
            amount: item.amount,
            app_id: auth.app_id,
            currency: router_data.request.currency,
            descriptor: router_data
                .request
                .billing_descriptor
                .as_ref()
                .and_then(|descriptor| descriptor.name.clone())
                .map(|name| name.expose()),
            merchant_order_id,
            request_id: request_id.clone(),
            cancel_url: return_url.clone(),
            success_url: return_url.clone(),
            failure_url: return_url,
            notification_url: router_data.request.get_optional_webhook_url(),
            order_time: get_uprimerpay_timestamp()?,
            payment_method,
            billing,
            products: vec![UprimerpayProduct {
                code: request_id.clone(),
                name: description.clone(),
                quantity: 1,
                sku: request_id,
                unit_price: item.amount,
                total_amount: item.amount,
            }],
            shipping: get_shipping(router_data),
            device_data: get_device_data(&router_data.request.browser_info)?,
        })
    }
}

impl TryFrom<&UprimerpayRouterData<&PaymentsAuthorizeRouterData>> for UprimerpayPaymentsRequest {
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(item: &UprimerpayRouterData<&PaymentsAuthorizeRouterData>) -> Result<Self, Self::Error> {
        match item.router_data.request.payment_method_data.clone() {
            PaymentMethodData::Card(card) => Self::try_from((item, &card)),
            _ => Err(errors::ConnectorError::NotImplemented(
                utils::get_unimplemented_payment_method_error_message("Uprimerpay"),
            )
            .into()),
        }
    }
}

fn get_billing(
    item: &PaymentsAuthorizeRouterData,
) -> Result<UprimerpayBilling, error_stack::Report<errors::ConnectorError>> {
    Ok(UprimerpayBilling {
        first_name: item.get_billing_first_name()?,
        last_name: item.get_billing_last_name()?,
        date_of_birth: None,
        phone_number: item.get_optional_billing_phone_number(),
        email: item
            .get_optional_billing_email()
            .or_else(|| item.request.email.clone()),
        country_code: item.get_optional_billing_country(),
        state: item.get_optional_billing_state(),
        city: item.get_optional_billing_city(),
        street: item.get_optional_billing_line1(),
        post_code: item.get_optional_billing_zip(),
    })
}

fn get_shipping(item: &PaymentsAuthorizeRouterData) -> Option<UprimerpayShipping> {
    item.get_optional_shipping().map(|_| UprimerpayShipping {
        company: None,
        first_name: item.get_optional_shipping_first_name(),
        last_name: item.get_optional_shipping_last_name(),
        phone_number: item.get_optional_shipping_phone_number(),
        country_code: item.get_optional_shipping_country(),
        state: item.get_optional_shipping_state(),
        city: item.get_optional_shipping_city(),
        street: item.get_optional_shipping_line1(),
        street2: item.get_optional_shipping_line2(),
        post_code: item.get_optional_shipping_zip(),
    })
}

fn get_device_data(
    browser_info: &Option<hyperswitch_domain_models::router_request_types::BrowserInformation>,
) -> Result<Option<UprimerpayDeviceData>, error_stack::Report<errors::ConnectorError>> {
    browser_info
        .clone()
        .map(|browser_info| {
            Ok(UprimerpayDeviceData {
                accept_header: browser_info
                    .accept_header
                    .unwrap_or_else(|| "*/*".to_string()),
                browser_java_enabled: browser_info
                    .java_enabled
                    .unwrap_or(false)
                    .to_string(),
                browser_javascript_enabled: browser_info
                    .java_script_enabled
                    .unwrap_or(true)
                    .to_string(),
                browser_user_agent: browser_info
                    .user_agent
                    .unwrap_or_else(|| "Hyperswitch".to_string()),
                challenge_window: DEFAULT_CHALLENGE_WINDOW.to_string(),
                language: browser_info.language.unwrap_or_else(|| "en-US".to_string()),
                screen_color_depth: browser_info.color_depth.unwrap_or(24).to_string(),
                screen_height: browser_info.screen_height.unwrap_or(1080).to_string(),
                screen_width: browser_info.screen_width.unwrap_or(1920).to_string(),
                timezone: browser_info.time_zone.unwrap_or(0).to_string(),
            })
        })
        .transpose()
}

pub fn get_uprimerpay_timestamp() -> Result<String, error_stack::Report<errors::ConnectorError>> {
    time::OffsetDateTime::now_utc()
        .format(&time::macros::format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second]+0000"
        ))
        .change_context(errors::ConnectorError::RequestEncodingFailed)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayPaymentsResponse {
    pub code: String,
    pub msg: Option<String>,
    pub data: Option<UprimerpayPaymentData>,
    pub success: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayPaymentData {
    pub request_id: Option<String>,
    pub merchant_order_id: Option<String>,
    pub id: String,
    pub status: UprimerpayPaymentStatus,
    pub error_code: Option<String>,
    #[serde(alias = "errorMsg")]
    pub error_message: Option<String>,
    pub next_action: Option<UprimerpayNextAction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UprimerpayPaymentStatus {
    Succeed,
    RequestCustomerAction,
    Processing,
    Pending,
    RetryPending,
    Failed,
    Failure,
    Declined,
    Cancelled,
    Canceled,
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UprimerpayWebhookTransactionType {
    Sale,
    Refund,
    Authorize,
    Capture,
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayWebhookNotification {
    pub request_id: String,
    pub app_id: String,
    pub merchant_order_id: String,
    pub amount: MinorUnit,
    pub currency: enums::Currency,
    pub captured_amount: Option<MinorUnit>,
    pub id: String,
    pub transaction_type: UprimerpayWebhookTransactionType,
    pub status: UprimerpayPaymentStatus,
    pub payment_method: String,
    pub payment_acceptance: Option<String>,
    pub merchant_memo: Option<String>,
    pub error_code: Option<String>,
    #[serde(alias = "errorMsg")]
    pub error_message: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub raw_code: Option<String>,
    pub raw_refusal_description: Option<String>,
    pub auth_code: Option<String>,
    pub short_card_no: Option<String>,
    pub country_code: Option<String>,
    pub eci: Option<String>,
    #[serde(rename = "threeDS")]
    pub three_ds: Option<String>,
    pub subscription_request_id: Option<String>,
    pub subscription_id: Option<String>,
    pub period_num: Option<i64>,
    pub cancellation_reason: Option<String>,
    pub original_id: Option<String>,
    pub refund_reason: Option<String>,
    pub retry_code: Option<String>,
    pub retry_msg: Option<String>,
}

impl UprimerpayWebhookNotification {
    pub fn is_refund_event(&self) -> bool {
        matches!(self.transaction_type, UprimerpayWebhookTransactionType::Refund)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayNextAction {
    pub action_type: Option<UprimerpayNextActionType>,
    pub url: Option<url::Url>,
    pub method: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UprimerpayNextActionType {
    Redirect,
    #[serde(other)]
    Unknown,
}

impl From<UprimerpayPaymentStatus> for enums::AttemptStatus {
    fn from(status: UprimerpayPaymentStatus) -> Self {
        match status {
            UprimerpayPaymentStatus::Succeed => Self::Charged,
            UprimerpayPaymentStatus::RequestCustomerAction => Self::AuthenticationPending,
            UprimerpayPaymentStatus::Processing
            | UprimerpayPaymentStatus::Pending
            | UprimerpayPaymentStatus::RetryPending => Self::Authorizing,
            UprimerpayPaymentStatus::Failed
            | UprimerpayPaymentStatus::Failure
            | UprimerpayPaymentStatus::Declined
            | UprimerpayPaymentStatus::Cancelled
            | UprimerpayPaymentStatus::Canceled => Self::Failure,
            UprimerpayPaymentStatus::Unknown => Self::Pending,
        }
    }
}

fn get_payment_response_data(
    response: &UprimerpayPaymentsResponse,
) -> Result<&UprimerpayPaymentData, error_stack::Report<errors::ConnectorError>> {
    response
        .data
        .as_ref()
        .ok_or(errors::ConnectorError::ResponseDeserializationFailed.into())
}

fn get_redirection_data(
    payment_data: &UprimerpayPaymentData,
) -> Option<RedirectForm> {
    payment_data.next_action.as_ref().and_then(|next_action| {
        next_action.url.clone().map(|url| {
            let method = next_action
                .method
                .as_deref()
                .filter(|method| method.eq_ignore_ascii_case("POST"))
                .map(|_| common_utils::request::Method::Post)
                .unwrap_or(common_utils::request::Method::Get);
            RedirectForm::from((
                url,
                method,
            ))
        })
    })
}

impl TryFrom<PaymentsResponseRouterData<UprimerpayPaymentsResponse>>
    for PaymentsAuthorizeRouterData
{
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(
        item: PaymentsResponseRouterData<UprimerpayPaymentsResponse>,
    ) -> Result<Self, Self::Error> {
        let payment_data = get_payment_response_data(&item.response)?;
        let status = enums::AttemptStatus::from(payment_data.status.clone());
        Ok(Self {
            response: Ok(PaymentsResponseData::TransactionResponse {
                resource_id: ResponseId::ConnectorTransactionId(payment_data.id.clone()),
                redirection_data: Box::new(get_redirection_data(payment_data)),
                mandate_reference: Box::new(None),
                connector_metadata: None,
                network_txn_id: None,
                network_txn_link_id: None,
                connector_response_reference_id: payment_data
                    .merchant_order_id
                    .clone()
                    .or_else(|| payment_data.request_id.clone()),
                incremental_authorization_allowed: None,
                authentication_data: None,
                charges: None,
            }),
            status,
            ..item.data
        })
    }
}

impl TryFrom<PaymentsSyncResponseRouterData<UprimerpayPaymentsResponse>> for PaymentsSyncRouterData {
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(
        item: PaymentsSyncResponseRouterData<UprimerpayPaymentsResponse>,
    ) -> Result<Self, Self::Error> {
        match item.data.request.sync_type {
            hyperswitch_domain_models::router_request_types::SyncRequestType::SinglePaymentSync => {
                let payment_data = get_payment_response_data(&item.response)?;
                let status = enums::AttemptStatus::from(payment_data.status.clone());
                Ok(Self {
                    response: Ok(PaymentsResponseData::TransactionResponse {
                        resource_id: ResponseId::ConnectorTransactionId(payment_data.id.clone()),
                        redirection_data: Box::new(get_redirection_data(payment_data)),
                        mandate_reference: Box::new(None),
                        connector_metadata: None,
                        network_txn_id: None,
                        network_txn_link_id: None,
                        connector_response_reference_id: payment_data
                            .merchant_order_id
                            .clone()
                            .or_else(|| payment_data.request_id.clone()),
                        incremental_authorization_allowed: None,
                        authentication_data: None,
                        charges: None,
                    }),
                    status,
                    ..item.data
                })
            }
            hyperswitch_domain_models::router_request_types::SyncRequestType::MultipleCaptureSync(_) => {
                Err(errors::ConnectorError::NotImplemented(
                    "manual multiple capture sync".to_string(),
                )
                .into())
            }
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayRefundRequest {
    request_id: String,
    app_id: Secret<String>,
    merchant_order_id: String,
    amount: MinorUnit,
    currency: enums::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_reason: Option<String>,
    refund_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_url: Option<String>,
}

impl<F> TryFrom<&UprimerpayRouterData<&RefundsRouterData<F>>> for UprimerpayRefundRequest {
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(item: &UprimerpayRouterData<&RefundsRouterData<F>>) -> Result<Self, Self::Error> {
        let auth = UprimerpayAuthType::try_from(&item.router_data.connector_auth_type)?;
        Ok(Self {
            request_id: item.router_data.request.refund_id.clone(),
            app_id: auth.app_id,
            merchant_order_id: item.router_data.request.refund_id.clone(),
            amount: item.amount,
            currency: item.router_data.request.currency,
            refund_reason: item.router_data.request.reason.clone(),
            refund_time: get_uprimerpay_timestamp()?,
            notification_url: item.router_data.request.webhook_url.clone(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayRefundResponse {
    pub code: String,
    pub msg: Option<String>,
    pub data: Option<UprimerpayRefundData>,
    pub success: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayRefundData {
    pub request_id: Option<String>,
    pub merchant_order_id: Option<String>,
    pub id: String,
    pub status: Option<UprimerpayPaymentStatus>,
    pub error_code: Option<String>,
    #[serde(alias = "errorMsg")]
    pub error_message: Option<String>,
}

fn get_refund_status(status: Option<UprimerpayPaymentStatus>) -> enums::RefundStatus {
    match status {
        Some(UprimerpayPaymentStatus::Succeed) => enums::RefundStatus::Success,
        Some(
            UprimerpayPaymentStatus::Failed
            | UprimerpayPaymentStatus::Failure
            | UprimerpayPaymentStatus::Declined
            | UprimerpayPaymentStatus::Cancelled
            | UprimerpayPaymentStatus::Canceled,
        ) => enums::RefundStatus::Failure,
        Some(UprimerpayPaymentStatus::RequestCustomerAction)
        | Some(UprimerpayPaymentStatus::Processing)
        | Some(UprimerpayPaymentStatus::Pending)
        | Some(UprimerpayPaymentStatus::RetryPending)
        | Some(UprimerpayPaymentStatus::Unknown)
        | None => enums::RefundStatus::Pending,
    }
}

fn get_refund_response_data(
    response: &UprimerpayRefundResponse,
) -> Result<&UprimerpayRefundData, error_stack::Report<errors::ConnectorError>> {
    response
        .data
        .as_ref()
        .ok_or(errors::ConnectorError::ResponseDeserializationFailed.into())
}

impl TryFrom<RefundsResponseRouterData<Execute, UprimerpayRefundResponse>>
    for RefundsRouterData<Execute>
{
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(
        item: RefundsResponseRouterData<Execute, UprimerpayRefundResponse>,
    ) -> Result<Self, Self::Error> {
        let refund_data = get_refund_response_data(&item.response)?;
        Ok(Self {
            response: Ok(RefundsResponseData {
                connector_refund_id: refund_data.id.clone(),
                refund_status: get_refund_status(refund_data.status.clone()),
            }),
            ..item.data
        })
    }
}

impl TryFrom<RefundsResponseRouterData<RSync, UprimerpayRefundResponse>>
    for RefundsRouterData<RSync>
{
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(
        item: RefundsResponseRouterData<RSync, UprimerpayRefundResponse>,
    ) -> Result<Self, Self::Error> {
        let refund_data = get_refund_response_data(&item.response)?;
        Ok(Self {
            response: Ok(RefundsResponseData {
                connector_refund_id: refund_data.id.clone(),
                refund_status: get_refund_status(refund_data.status.clone()),
            }),
            ..item.data
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UprimerpayErrorResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    #[serde(alias = "errorMsg")]
    pub message: Option<String>,
    pub success: Option<bool>,
    pub data: Option<serde_json::Value>,
}

impl UprimerpayErrorResponse {
    pub fn error_code(&self) -> String {
        self.code
            .clone()
            .unwrap_or_else(|| UPRIMERPAY_SUCCESS_CODE.to_string())
    }

    pub fn error_message(&self) -> String {
        self.msg
            .clone()
            .or_else(|| self.message.clone())
            .unwrap_or_else(|| "UprimerPay request failed".to_string())
    }
}
