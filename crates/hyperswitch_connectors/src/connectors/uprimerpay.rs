pub mod transformers;

use std::sync::LazyLock;

use api_models::{
    payments::PaymentIdType,
    webhooks::{IncomingWebhookEvent, ObjectReferenceId, RefundIdType},
};
use common_enums::enums;
use common_utils::{
    crypto::{self, GenerateDigest},
    errors::CustomResult,
    ext_traits::{ByteSliceExt, BytesExt, ValueExt},
    request::{Method, Request, RequestBuilder, RequestContent},
    types::{AmountConvertor, MinorUnit, MinorUnitForConnector},
};
use error_stack::ResultExt;
use hyperswitch_domain_models::{
    api::ApplicationResponse,
    router_data::{AccessToken, ConnectorAuthType, ErrorResponse, RouterData},
    router_flow_types::{
        access_token_auth::AccessTokenAuth,
        payments::{Authorize, Capture, PSync, PaymentMethodToken, Session, SetupMandate, Void},
        refunds::{Execute, RSync},
    },
    router_request_types::{
        AccessTokenRequestData, PaymentMethodTokenizationData, PaymentsAuthorizeData,
        PaymentsCancelData, PaymentsCaptureData, PaymentsSessionData, PaymentsSyncData,
        RefundsData, SetupMandateRequestData,
    },
    router_response_types::{
        ConnectorInfo, PaymentMethodDetails, PaymentsResponseData, RefundsResponseData,
        SupportedPaymentMethods, SupportedPaymentMethodsExt,
    },
    types::{
        PaymentsAuthorizeRouterData, PaymentsSyncRouterData, RefreshTokenRouterData,
        RefundSyncRouterData, RefundsRouterData,
    },
};
use hyperswitch_interfaces::{
    api::{
        self, ConnectorCommon, ConnectorCommonExt, ConnectorIntegration, ConnectorSpecifications,
        ConnectorValidation,
    },
    configs::Connectors,
    consts::{NO_ERROR_CODE, NO_ERROR_MESSAGE},
    errors,
    events::connector_api_logs::ConnectorEvent,
    types::{self, Response},
    webhooks::{
        IncomingWebhook, IncomingWebhookFlowError, IncomingWebhookRequestDetails, WebhookContext,
    },
};
use hyperswitch_masking::{ExposeInterface, Mask, PeekInterface};
use transformers as uprimerpay;

use crate::{
    constants::headers,
    types::ResponseRouterData,
    utils::{self, convert_amount, RefundsRequestData},
};

const X_ACCESS_CODE: &str = "X-AccessCode";
const X_SECRET_KEY: &str = "X-SecretKey";
const X_SIGNATURE: &str = "X-Signature";

#[derive(Clone)]
pub struct Uprimerpay {
    amount_converter: &'static (dyn AmountConvertor<Output = MinorUnit> + Sync),
}

impl Uprimerpay {
    pub fn new() -> &'static Self {
        &Self {
            amount_converter: &MinorUnitForConnector,
        }
    }

    fn build_uprimerpay_headers<Flow, Request, Response>(
        &self,
        req: &RouterData<Flow, Request, Response>,
        include_bearer_token: bool,
    ) -> CustomResult<Vec<(String, hyperswitch_masking::Maskable<String>)>, errors::ConnectorError>
    {
        let auth = uprimerpay::UprimerpayAuthType::try_from(&req.connector_auth_type)?;
        let mut headers = vec![
            (
                headers::CONTENT_TYPE.to_string(),
                self.common_get_content_type().to_string().into(),
            ),
            (X_ACCESS_CODE.to_string(), auth.access_code.expose().into_masked()),
            (X_SECRET_KEY.to_string(), auth.secret_key.expose().into_masked()),
        ];

        if include_bearer_token {
            let access_token = req
                .access_token
                .clone()
                .ok_or(errors::ConnectorError::FailedToObtainAuthType)?;
            headers.push((
                headers::AUTHORIZATION.to_string(),
                format!("Bearer {}", access_token.token.peek()).into_masked(),
            ));
        }

        Ok(headers)
    }

    fn build_signed_request<Flow, Req, Res>(
        &self,
        req: &RouterData<Flow, Req, Res>,
        url: String,
        body: RequestContent,
    ) -> CustomResult<Option<Request>, errors::ConnectorError> {
        let mut headers = self.build_uprimerpay_headers(req, true)?;
        let signature = self.get_signature(&req.connector_auth_type, &body)?;
        headers.push((X_SIGNATURE.to_string(), signature.into_masked()));

        Ok(Some(
            RequestBuilder::new()
                .method(Method::Post)
                .url(&url)
                .attach_default_headers()
                .headers(headers)
                .set_body(body)
                .build(),
        ))
    }

    fn get_signature(
        &self,
        auth_type: &ConnectorAuthType,
        body: &RequestContent,
    ) -> CustomResult<String, errors::ConnectorError> {
        let auth = uprimerpay::UprimerpayAuthType::try_from(auth_type)?;
        let signature_payload = format!("{}{}", body.get_inner_value().peek(), auth.secret_key.peek());
        let digest = crypto::Md5
            .generate_digest(signature_payload.as_bytes())
            .change_context(errors::ConnectorError::RequestEncodingFailed)?;
        Ok(hex::encode(digest))
    }
}

impl api::Payment for Uprimerpay {}
impl api::PaymentSession for Uprimerpay {}
impl api::ConnectorAccessToken for Uprimerpay {}
impl api::PaymentAuthorize for Uprimerpay {}
impl api::PaymentSync for Uprimerpay {}
impl api::PaymentCapture for Uprimerpay {}
impl api::PaymentVoid for Uprimerpay {}
impl api::MandateSetup for Uprimerpay {}
impl api::Refund for Uprimerpay {}
impl api::RefundExecute for Uprimerpay {}
impl api::RefundSync for Uprimerpay {}
impl api::PaymentToken for Uprimerpay {}

impl ConnectorIntegration<PaymentMethodToken, PaymentMethodTokenizationData, PaymentsResponseData>
    for Uprimerpay
{
}

impl ConnectorIntegration<Capture, PaymentsCaptureData, PaymentsResponseData> for Uprimerpay {}

impl ConnectorIntegration<Void, PaymentsCancelData, PaymentsResponseData> for Uprimerpay {}

impl ConnectorIntegration<SetupMandate, SetupMandateRequestData, PaymentsResponseData>
    for Uprimerpay
{
}

impl<Flow, Request, Response> ConnectorCommonExt<Flow, Request, Response> for Uprimerpay
where
    Self: ConnectorIntegration<Flow, Request, Response>,
{
    fn build_headers(
        &self,
        req: &RouterData<Flow, Request, Response>,
        _connectors: &Connectors,
    ) -> CustomResult<Vec<(String, hyperswitch_masking::Maskable<String>)>, errors::ConnectorError>
    {
        self.build_uprimerpay_headers(req, true)
    }
}

impl ConnectorCommon for Uprimerpay {
    fn id(&self) -> &'static str {
        "uprimerpay"
    }

    fn get_currency_unit(&self) -> api::CurrencyUnit {
        api::CurrencyUnit::Minor
    }

    fn common_get_content_type(&self) -> &'static str {
        "application/json"
    }

    fn base_url<'a>(&self, connectors: &'a Connectors) -> &'a str {
        connectors.uprimerpay.base_url.as_ref()
    }

    fn get_auth_header(
        &self,
        auth_type: &ConnectorAuthType,
    ) -> CustomResult<Vec<(String, hyperswitch_masking::Maskable<String>)>, errors::ConnectorError>
    {
        let auth = uprimerpay::UprimerpayAuthType::try_from(auth_type)?;
        Ok(vec![
            (X_ACCESS_CODE.to_string(), auth.access_code.expose().into_masked()),
            (X_SECRET_KEY.to_string(), auth.secret_key.expose().into_masked()),
        ])
    }

    fn build_error_response(
        &self,
        res: Response,
        event_builder: Option<&mut ConnectorEvent>,
    ) -> CustomResult<ErrorResponse, errors::ConnectorError> {
        let response: Result<uprimerpay::UprimerpayErrorResponse, _> =
            res.response.parse_struct("UprimerpayErrorResponse");

        match response {
            Ok(response) => {
                event_builder.map(|i| i.set_error_response_body(&response));
                router_env::logger::info!(connector_response=?response);
                Ok(ErrorResponse {
                    status_code: res.status_code,
                    code: response.error_code(),
                    message: response.error_message(),
                    reason: response.msg.or(response.message),
                    attempt_status: None,
                    connector_transaction_id: None,
                    connector_response_reference_id: None,
                    network_advice_code: None,
                    network_decline_code: None,
                    network_error_message: None,
                    connector_metadata: None,
                })
            }
            Err(error_msg) => {
                event_builder.map(|event| {
                    event.set_error(serde_json::json!({
                        "error": res.response.escape_ascii().to_string(),
                        "status_code": res.status_code
                    }))
                });
                router_env::logger::error!(deserialization_error =? error_msg);
                Ok(ErrorResponse {
                    status_code: res.status_code,
                    code: NO_ERROR_CODE.to_string(),
                    message: NO_ERROR_MESSAGE.to_string(),
                    reason: Some(res.response.escape_ascii().to_string()),
                    attempt_status: None,
                    connector_transaction_id: None,
                    connector_response_reference_id: None,
                    network_advice_code: None,
                    network_decline_code: None,
                    network_error_message: None,
                    connector_metadata: None,
                })
            }
        }
    }
}

impl ConnectorValidation for Uprimerpay {
    fn validate_connector_against_payment_request(
        &self,
        capture_method: Option<enums::CaptureMethod>,
        _payment_method: enums::PaymentMethod,
        _pmt: Option<enums::PaymentMethodType>,
    ) -> CustomResult<(), errors::ConnectorError> {
        match capture_method.unwrap_or_default() {
            enums::CaptureMethod::Automatic | enums::CaptureMethod::SequentialAutomatic => Ok(()),
            capture_method => Err(utils::construct_not_supported_error_report(
                capture_method,
                self.id(),
            )),
        }
    }

    fn validate_psync_reference_id(
        &self,
        _data: &PaymentsSyncData,
        _is_three_ds: bool,
        _status: enums::AttemptStatus,
        _connector_meta_data: Option<common_utils::pii::SecretSerdeValue>,
    ) -> CustomResult<(), errors::ConnectorError> {
        Ok(())
    }

    fn is_webhook_source_verification_mandatory(&self) -> bool {
        true
    }
}

impl ConnectorIntegration<Session, PaymentsSessionData, PaymentsResponseData> for Uprimerpay {}

impl ConnectorIntegration<AccessTokenAuth, AccessTokenRequestData, AccessToken> for Uprimerpay {
    fn get_headers(
        &self,
        req: &RefreshTokenRouterData,
        _connectors: &Connectors,
    ) -> CustomResult<Vec<(String, hyperswitch_masking::Maskable<String>)>, errors::ConnectorError>
    {
        self.build_uprimerpay_headers(req, false)
    }

    fn get_url(
        &self,
        _req: &RefreshTokenRouterData,
        connectors: &Connectors,
    ) -> CustomResult<String, errors::ConnectorError> {
        Ok(format!("{}authorize", self.base_url(connectors)))
    }

    fn get_http_method(&self) -> Method {
        Method::Get
    }

    fn build_request(
        &self,
        req: &RefreshTokenRouterData,
        connectors: &Connectors,
    ) -> CustomResult<Option<Request>, errors::ConnectorError> {
        Ok(Some(
            RequestBuilder::new()
                .method(types::RefreshTokenType::get_http_method(self))
                .url(&types::RefreshTokenType::get_url(self, req, connectors)?)
                .attach_default_headers()
                .headers(types::RefreshTokenType::get_headers(self, req, connectors)?)
                .build(),
        ))
    }

    fn handle_response(
        &self,
        data: &RefreshTokenRouterData,
        event_builder: Option<&mut ConnectorEvent>,
        res: Response,
    ) -> CustomResult<RefreshTokenRouterData, errors::ConnectorError> {
        let response: uprimerpay::UprimerpayAccessTokenResponse = res
            .response
            .parse_struct("UprimerpayAccessTokenResponse")
            .change_context(errors::ConnectorError::ResponseDeserializationFailed)?;
        event_builder.map(|i| i.set_response_body(&response));
        router_env::logger::info!(connector_response=?response);
        RouterData::try_from(ResponseRouterData {
            response,
            data: data.clone(),
            http_code: res.status_code,
        })
    }

    fn get_error_response(
        &self,
        res: Response,
        event_builder: Option<&mut ConnectorEvent>,
    ) -> CustomResult<ErrorResponse, errors::ConnectorError> {
        self.build_error_response(res, event_builder)
    }
}

impl ConnectorIntegration<Authorize, PaymentsAuthorizeData, PaymentsResponseData> for Uprimerpay {
    fn get_headers(
        &self,
        req: &PaymentsAuthorizeRouterData,
        connectors: &Connectors,
    ) -> CustomResult<Vec<(String, hyperswitch_masking::Maskable<String>)>, errors::ConnectorError>
    {
        self.build_headers(req, connectors)
    }

    fn get_content_type(&self) -> &'static str {
        self.common_get_content_type()
    }

    fn get_http_method(&self) -> Method {
        Method::Post
    }

    fn get_url(
        &self,
        _req: &PaymentsAuthorizeRouterData,
        connectors: &Connectors,
    ) -> CustomResult<String, errors::ConnectorError> {
        Ok(format!("{}api/acquire/payment/create", self.base_url(connectors)))
    }

    fn get_request_body(
        &self,
        req: &PaymentsAuthorizeRouterData,
        _connectors: &Connectors,
    ) -> CustomResult<RequestContent, errors::ConnectorError> {
        let amount = convert_amount(
            self.amount_converter,
            req.request.minor_amount,
            req.request.currency,
        )?;
        let connector_router_data = uprimerpay::UprimerpayRouterData::from((amount, req));
        let connector_req = uprimerpay::UprimerpayPaymentsRequest::try_from(&connector_router_data)?;
        Ok(RequestContent::Json(Box::new(connector_req)))
    }

    fn build_request(
        &self,
        req: &PaymentsAuthorizeRouterData,
        connectors: &Connectors,
    ) -> CustomResult<Option<Request>, errors::ConnectorError> {
        let body = types::PaymentsAuthorizeType::get_request_body(self, req, connectors)?;
        self.build_signed_request(
            req,
            types::PaymentsAuthorizeType::get_url(self, req, connectors)?,
            body,
        )
    }

    fn handle_response(
        &self,
        data: &PaymentsAuthorizeRouterData,
        event_builder: Option<&mut ConnectorEvent>,
        res: Response,
    ) -> CustomResult<PaymentsAuthorizeRouterData, errors::ConnectorError> {
        let response: uprimerpay::UprimerpayPaymentsResponse = res
            .response
            .parse_struct("UprimerpayPaymentsResponse")
            .change_context(errors::ConnectorError::ResponseDeserializationFailed)?;
        event_builder.map(|i| i.set_response_body(&response));
        router_env::logger::info!(connector_response=?response);
        RouterData::try_from(ResponseRouterData {
            response,
            data: data.clone(),
            http_code: res.status_code,
        })
    }

    fn get_error_response(
        &self,
        res: Response,
        event_builder: Option<&mut ConnectorEvent>,
    ) -> CustomResult<ErrorResponse, errors::ConnectorError> {
        self.build_error_response(res, event_builder)
    }
}

impl ConnectorIntegration<PSync, PaymentsSyncData, PaymentsResponseData> for Uprimerpay {
    fn get_headers(
        &self,
        req: &PaymentsSyncRouterData,
        connectors: &Connectors,
    ) -> CustomResult<Vec<(String, hyperswitch_masking::Maskable<String>)>, errors::ConnectorError>
    {
        self.build_headers(req, connectors)
    }

    fn get_content_type(&self) -> &'static str {
        self.common_get_content_type()
    }

    fn get_url(
        &self,
        req: &PaymentsSyncRouterData,
        connectors: &Connectors,
    ) -> CustomResult<String, errors::ConnectorError> {
        let connector_payment_id = req
            .request
            .connector_transaction_id
            .get_connector_transaction_id()
            .change_context(errors::ConnectorError::MissingConnectorTransactionID)?;

        Ok(format!(
            "{}api/acquire/payment/{connector_payment_id}/get",
            self.base_url(connectors)
        ))
    }

    fn build_request(
        &self,
        req: &PaymentsSyncRouterData,
        connectors: &Connectors,
    ) -> CustomResult<Option<Request>, errors::ConnectorError> {
        Ok(Some(
            RequestBuilder::new()
                .method(Method::Get)
                .url(&types::PaymentsSyncType::get_url(self, req, connectors)?)
                .attach_default_headers()
                .headers(types::PaymentsSyncType::get_headers(self, req, connectors)?)
                .build(),
        ))
    }

    fn handle_response(
        &self,
        data: &PaymentsSyncRouterData,
        event_builder: Option<&mut ConnectorEvent>,
        res: Response,
    ) -> CustomResult<PaymentsSyncRouterData, errors::ConnectorError> {
        let response: uprimerpay::UprimerpayPaymentsResponse = res
            .response
            .parse_struct("UprimerpayPaymentsSyncResponse")
            .change_context(errors::ConnectorError::ResponseDeserializationFailed)?;
        event_builder.map(|i| i.set_response_body(&response));
        router_env::logger::info!(connector_response=?response);
        RouterData::try_from(ResponseRouterData {
            response,
            data: data.clone(),
            http_code: res.status_code,
        })
    }

    fn get_error_response(
        &self,
        res: Response,
        event_builder: Option<&mut ConnectorEvent>,
    ) -> CustomResult<ErrorResponse, errors::ConnectorError> {
        self.build_error_response(res, event_builder)
    }
}

impl ConnectorIntegration<Execute, RefundsData, RefundsResponseData> for Uprimerpay {
    fn get_headers(
        &self,
        req: &RefundsRouterData<Execute>,
        connectors: &Connectors,
    ) -> CustomResult<Vec<(String, hyperswitch_masking::Maskable<String>)>, errors::ConnectorError>
    {
        self.build_headers(req, connectors)
    }

    fn get_content_type(&self) -> &'static str {
        self.common_get_content_type()
    }

    fn get_http_method(&self) -> Method {
        Method::Post
    }

    fn get_url(
        &self,
        req: &RefundsRouterData<Execute>,
        connectors: &Connectors,
    ) -> CustomResult<String, errors::ConnectorError> {
        Ok(format!(
            "{}api/acquire/payment/{}/refund",
            self.base_url(connectors),
            req.request.connector_transaction_id
        ))
    }

    fn get_request_body(
        &self,
        req: &RefundsRouterData<Execute>,
        _connectors: &Connectors,
    ) -> CustomResult<RequestContent, errors::ConnectorError> {
        let amount = convert_amount(
            self.amount_converter,
            req.request.minor_refund_amount,
            req.request.currency,
        )?;
        let connector_router_data = uprimerpay::UprimerpayRouterData::from((amount, req));
        let connector_req = uprimerpay::UprimerpayRefundRequest::try_from(&connector_router_data)?;
        Ok(RequestContent::Json(Box::new(connector_req)))
    }

    fn build_request(
        &self,
        req: &RefundsRouterData<Execute>,
        connectors: &Connectors,
    ) -> CustomResult<Option<Request>, errors::ConnectorError> {
        let body = types::RefundExecuteType::get_request_body(self, req, connectors)?;
        self.build_signed_request(
            req,
            types::RefundExecuteType::get_url(self, req, connectors)?,
            body,
        )
    }

    fn handle_response(
        &self,
        data: &RefundsRouterData<Execute>,
        event_builder: Option<&mut ConnectorEvent>,
        res: Response,
    ) -> CustomResult<RefundsRouterData<Execute>, errors::ConnectorError> {
        let response: uprimerpay::UprimerpayRefundResponse = res
            .response
            .parse_struct("UprimerpayRefundResponse")
            .change_context(errors::ConnectorError::ResponseDeserializationFailed)?;
        event_builder.map(|i| i.set_response_body(&response));
        router_env::logger::info!(connector_response=?response);
        RouterData::try_from(ResponseRouterData {
            response,
            data: data.clone(),
            http_code: res.status_code,
        })
    }

    fn get_error_response(
        &self,
        res: Response,
        event_builder: Option<&mut ConnectorEvent>,
    ) -> CustomResult<ErrorResponse, errors::ConnectorError> {
        self.build_error_response(res, event_builder)
    }
}

impl ConnectorIntegration<RSync, RefundsData, RefundsResponseData> for Uprimerpay {
    fn get_headers(
        &self,
        req: &RefundSyncRouterData,
        connectors: &Connectors,
    ) -> CustomResult<Vec<(String, hyperswitch_masking::Maskable<String>)>, errors::ConnectorError>
    {
        self.build_headers(req, connectors)
    }

    fn get_content_type(&self) -> &'static str {
        self.common_get_content_type()
    }

    fn get_url(
        &self,
        req: &RefundSyncRouterData,
        connectors: &Connectors,
    ) -> CustomResult<String, errors::ConnectorError> {
        let refund_id = req.request.get_connector_refund_id()?;
        Ok(format!(
            "{}api/acquire/payment/{refund_id}/get",
            self.base_url(connectors)
        ))
    }

    fn build_request(
        &self,
        req: &RefundSyncRouterData,
        connectors: &Connectors,
    ) -> CustomResult<Option<Request>, errors::ConnectorError> {
        Ok(Some(
            RequestBuilder::new()
                .method(Method::Get)
                .url(&types::RefundSyncType::get_url(self, req, connectors)?)
                .attach_default_headers()
                .headers(types::RefundSyncType::get_headers(self, req, connectors)?)
                .build(),
        ))
    }

    fn handle_response(
        &self,
        data: &RefundSyncRouterData,
        event_builder: Option<&mut ConnectorEvent>,
        res: Response,
    ) -> CustomResult<RefundSyncRouterData, errors::ConnectorError> {
        let response: uprimerpay::UprimerpayRefundResponse = res
            .response
            .parse_struct("UprimerpayRefundSyncResponse")
            .change_context(errors::ConnectorError::ResponseDeserializationFailed)?;
        event_builder.map(|i| i.set_response_body(&response));
        router_env::logger::info!(connector_response=?response);
        RouterData::try_from(ResponseRouterData {
            response,
            data: data.clone(),
            http_code: res.status_code,
        })
    }

    fn get_error_response(
        &self,
        res: Response,
        event_builder: Option<&mut ConnectorEvent>,
    ) -> CustomResult<ErrorResponse, errors::ConnectorError> {
        self.build_error_response(res, event_builder)
    }
}

#[async_trait::async_trait]
impl IncomingWebhook for Uprimerpay {
    async fn verify_webhook_source(
        &self,
        request: &IncomingWebhookRequestDetails<'_>,
        _merchant_id: &common_utils::id_type::MerchantId,
        _connector_webhook_details: Option<common_utils::pii::SecretSerdeValue>,
        connector_account_details: crypto::Encryptable<
            hyperswitch_masking::Secret<serde_json::Value>,
        >,
        _connector_name: &str,
    ) -> CustomResult<bool, errors::ConnectorError> {
        let connector_account_details = connector_account_details
            .parse_value::<ConnectorAuthType>("ConnectorAuthType")
            .change_context(errors::ConnectorError::WebhookSourceVerificationFailed)?;
        let auth = uprimerpay::UprimerpayAuthType::try_from(&connector_account_details)?;
        let signature = utils::get_header_key_value(X_SIGNATURE, request.headers)
            .change_context(errors::ConnectorError::WebhookSignatureNotFound)?;
        let mut signature_payload = request.body.to_vec();
        signature_payload.extend_from_slice(auth.secret_key.peek().as_bytes());
        let digest = crypto::Md5
            .generate_digest(&signature_payload)
            .change_context(errors::ConnectorError::WebhookSourceVerificationFailed)?;

        Ok(hex::encode(digest).eq_ignore_ascii_case(signature))
    }

    fn get_webhook_object_reference_id(
        &self,
        request: &IncomingWebhookRequestDetails<'_>,
    ) -> CustomResult<ObjectReferenceId, errors::ConnectorError> {
        let notification: uprimerpay::UprimerpayWebhookNotification = request
            .body
            .parse_struct("UprimerpayWebhookNotification")
            .change_context(errors::ConnectorError::WebhookReferenceIdNotFound)?;

        if notification.is_refund_event() {
            Ok(ObjectReferenceId::RefundId(RefundIdType::RefundId(
                notification.merchant_order_id,
            )))
        } else {
            Ok(ObjectReferenceId::PaymentId(
                PaymentIdType::ConnectorTransactionId(
                    notification.original_id.unwrap_or(notification.id),
                ),
            ))
        }
    }

    fn get_webhook_event_type(
        &self,
        request: &IncomingWebhookRequestDetails<'_>,
        _context: Option<&WebhookContext>,
    ) -> CustomResult<IncomingWebhookEvent, errors::ConnectorError> {
        let notification: uprimerpay::UprimerpayWebhookNotification = request
            .body
            .parse_struct("UprimerpayWebhookNotification")
            .change_context(errors::ConnectorError::WebhookEventTypeNotFound)?;

        let event = match notification.transaction_type {
            uprimerpay::UprimerpayWebhookTransactionType::Refund => match notification.status {
                uprimerpay::UprimerpayPaymentStatus::Succeed => IncomingWebhookEvent::RefundSuccess,
                uprimerpay::UprimerpayPaymentStatus::Failed
                | uprimerpay::UprimerpayPaymentStatus::Failure
                | uprimerpay::UprimerpayPaymentStatus::Declined
                | uprimerpay::UprimerpayPaymentStatus::Cancelled
                | uprimerpay::UprimerpayPaymentStatus::Canceled => {
                    IncomingWebhookEvent::RefundFailure
                }
                uprimerpay::UprimerpayPaymentStatus::Pending
                | uprimerpay::UprimerpayPaymentStatus::Processing
                | uprimerpay::UprimerpayPaymentStatus::RetryPending
                | uprimerpay::UprimerpayPaymentStatus::RequestCustomerAction
                | uprimerpay::UprimerpayPaymentStatus::Unknown => {
                    IncomingWebhookEvent::EventNotSupported
                }
            },
            uprimerpay::UprimerpayWebhookTransactionType::Authorize => match notification.status {
                uprimerpay::UprimerpayPaymentStatus::Succeed => {
                    IncomingWebhookEvent::PaymentIntentAuthorizationSuccess
                }
                uprimerpay::UprimerpayPaymentStatus::Failed
                | uprimerpay::UprimerpayPaymentStatus::Failure
                | uprimerpay::UprimerpayPaymentStatus::Declined
                | uprimerpay::UprimerpayPaymentStatus::Cancelled
                | uprimerpay::UprimerpayPaymentStatus::Canceled => {
                    IncomingWebhookEvent::PaymentIntentAuthorizationFailure
                }
                uprimerpay::UprimerpayPaymentStatus::Pending
                | uprimerpay::UprimerpayPaymentStatus::Processing
                | uprimerpay::UprimerpayPaymentStatus::RetryPending
                | uprimerpay::UprimerpayPaymentStatus::RequestCustomerAction => {
                    IncomingWebhookEvent::PaymentIntentProcessing
                }
                uprimerpay::UprimerpayPaymentStatus::Unknown => {
                    IncomingWebhookEvent::EventNotSupported
                }
            },
            uprimerpay::UprimerpayWebhookTransactionType::Capture => match notification.status {
                uprimerpay::UprimerpayPaymentStatus::Succeed => {
                    IncomingWebhookEvent::PaymentIntentCaptureSuccess
                }
                uprimerpay::UprimerpayPaymentStatus::Failed
                | uprimerpay::UprimerpayPaymentStatus::Failure
                | uprimerpay::UprimerpayPaymentStatus::Declined
                | uprimerpay::UprimerpayPaymentStatus::Cancelled
                | uprimerpay::UprimerpayPaymentStatus::Canceled => {
                    IncomingWebhookEvent::PaymentIntentCaptureFailure
                }
                uprimerpay::UprimerpayPaymentStatus::Pending
                | uprimerpay::UprimerpayPaymentStatus::Processing
                | uprimerpay::UprimerpayPaymentStatus::RetryPending
                | uprimerpay::UprimerpayPaymentStatus::RequestCustomerAction => {
                    IncomingWebhookEvent::PaymentIntentProcessing
                }
                uprimerpay::UprimerpayPaymentStatus::Unknown => {
                    IncomingWebhookEvent::EventNotSupported
                }
            },
            uprimerpay::UprimerpayWebhookTransactionType::Sale => match notification.status {
                uprimerpay::UprimerpayPaymentStatus::Succeed => {
                    IncomingWebhookEvent::PaymentIntentSuccess
                }
                uprimerpay::UprimerpayPaymentStatus::Failed
                | uprimerpay::UprimerpayPaymentStatus::Failure
                | uprimerpay::UprimerpayPaymentStatus::Declined
                | uprimerpay::UprimerpayPaymentStatus::Cancelled
                | uprimerpay::UprimerpayPaymentStatus::Canceled => {
                    IncomingWebhookEvent::PaymentIntentFailure
                }
                uprimerpay::UprimerpayPaymentStatus::Pending
                | uprimerpay::UprimerpayPaymentStatus::Processing
                | uprimerpay::UprimerpayPaymentStatus::RetryPending
                | uprimerpay::UprimerpayPaymentStatus::RequestCustomerAction => {
                    IncomingWebhookEvent::PaymentIntentProcessing
                }
                uprimerpay::UprimerpayPaymentStatus::Unknown => {
                    IncomingWebhookEvent::EventNotSupported
                }
            },
            uprimerpay::UprimerpayWebhookTransactionType::Unknown => {
                IncomingWebhookEvent::EventNotSupported
            }
        };

        Ok(event)
    }

    fn get_webhook_resource_object(
        &self,
        request: &IncomingWebhookRequestDetails<'_>,
    ) -> CustomResult<Box<dyn hyperswitch_masking::ErasedMaskSerialize>, errors::ConnectorError>
    {
        let notification: uprimerpay::UprimerpayWebhookNotification = request
            .body
            .parse_struct("UprimerpayWebhookNotification")
            .change_context(errors::ConnectorError::WebhookResourceObjectNotFound)?;

        Ok(Box::new(notification))
    }

    fn get_webhook_api_response(
        &self,
        _request: &IncomingWebhookRequestDetails<'_>,
        _error_kind: Option<IncomingWebhookFlowError>,
        _connector_authentication_type: Option<
            crypto::Encryptable<hyperswitch_masking::Secret<serde_json::Value>>,
        >,
    ) -> CustomResult<ApplicationResponse<serde_json::Value>, errors::ConnectorError> {
        Ok(ApplicationResponse::TextPlain("SUCCESS".to_string()))
    }
}

static UPRIMERPAY_SUPPORTED_PAYMENT_METHODS: LazyLock<SupportedPaymentMethods> =
    LazyLock::new(|| {
        let supported_capture_methods = vec![
            enums::CaptureMethod::Automatic,
            enums::CaptureMethod::SequentialAutomatic,
        ];
        let supported_card_network = vec![
            common_enums::CardNetwork::Mastercard,
            common_enums::CardNetwork::Visa,
        ];

        let mut supported_payment_methods = SupportedPaymentMethods::new();

        for payment_method_type in [
            enums::PaymentMethodType::Credit,
            enums::PaymentMethodType::Debit,
        ] {
            supported_payment_methods.add(
                enums::PaymentMethod::Card,
                payment_method_type,
                PaymentMethodDetails {
                    mandates: enums::FeatureStatus::NotSupported,
                    refunds: enums::FeatureStatus::Supported,
                    supported_capture_methods: supported_capture_methods.clone(),
                    specific_features: Some(
                        api_models::feature_matrix::PaymentMethodSpecificFeatures::Card(
                            api_models::feature_matrix::CardSpecificFeatures {
                                three_ds: common_enums::FeatureStatus::Supported,
                                no_three_ds: common_enums::FeatureStatus::Supported,
                                supported_card_networks: supported_card_network.clone(),
                            },
                        ),
                    ),
                },
            );
        }

        for payment_method_type in [
            enums::PaymentMethodType::AliPay,
            enums::PaymentMethodType::AliPayHk,
            enums::PaymentMethodType::WeChatPay,
        ] {
            supported_payment_methods.add(
                enums::PaymentMethod::Wallet,
                payment_method_type,
                PaymentMethodDetails {
                    mandates: enums::FeatureStatus::NotSupported,
                    refunds: enums::FeatureStatus::Supported,
                    supported_capture_methods: supported_capture_methods.clone(),
                    specific_features: None,
                },
            );
        }

        supported_payment_methods
    });

static UPRIMERPAY_CONNECTOR_INFO: ConnectorInfo = ConnectorInfo {
    display_name: "UprimerPay",
    description: "UprimerPay provides acquiring, card, and wallet payment processing APIs.",
    connector_type: enums::HyperswitchConnectorCategory::PaymentGateway,
    integration_status: enums::ConnectorIntegrationStatus::Sandbox,
};

static UPRIMERPAY_SUPPORTED_WEBHOOK_FLOWS: [enums::EventClass; 2] =
    [enums::EventClass::Payments, enums::EventClass::Refunds];

impl ConnectorSpecifications for Uprimerpay {
    fn get_connector_about(&self) -> Option<&'static ConnectorInfo> {
        Some(&UPRIMERPAY_CONNECTOR_INFO)
    }

    fn get_supported_payment_methods(&self) -> Option<&'static SupportedPaymentMethods> {
        Some(&*UPRIMERPAY_SUPPORTED_PAYMENT_METHODS)
    }

    fn get_supported_webhook_flows(&self) -> Option<&'static [enums::EventClass]> {
        Some(&UPRIMERPAY_SUPPORTED_WEBHOOK_FLOWS)
    }
}
