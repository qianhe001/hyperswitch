use hyperswitch_masking::Secret;
use router::types::{self, domain, storage::enums};
use test_utils::connector_auth;

use crate::utils::{self, ConnectorActions};

#[derive(Clone, Copy)]
struct UprimerpayTest;
impl ConnectorActions for UprimerpayTest {}
impl utils::Connector for UprimerpayTest {
    fn get_data(&self) -> types::api::ConnectorData {
        use router::connector::Uprimerpay;
        utils::construct_connector_data_old(
            Box::new(Uprimerpay::new()),
            types::Connector::Uprimerpay,
            types::api::GetToken::Connector,
            None,
        )
    }

    fn get_auth_token(&self) -> types::ConnectorAuthType {
        utils::to_connector_auth_type(
            connector_auth::ConnectorAuthentication::new()
                .uprimerpay
                .expect("Missing connector authentication configuration")
                .into(),
        )
    }

    fn get_name(&self) -> String {
        "uprimerpay".to_string()
    }
}

static CONNECTOR: UprimerpayTest = UprimerpayTest {};

fn payment_method_details() -> Option<types::PaymentsAuthorizeData> {
    Some(types::PaymentsAuthorizeData {
        capture_method: Some(enums::CaptureMethod::Automatic),
        router_return_url: Some("https://example.com/return".to_string()),
        webhook_url: Some("https://example.com/webhook".to_string()),
        email: Some("john.doe@example.com".to_string().try_into().unwrap()),
        payment_method_data: domain::PaymentMethodData::Card(utils::CCardType::default().0),
        ..utils::PaymentAuthorizeType::default().0
    })
}

fn payment_info() -> Option<utils::PaymentInfo> {
    Some(utils::PaymentInfo {
        address: Some(domain::PaymentAddress::new(
            None,
            None,
            Some(hyperswitch_domain_models::address::Address {
                address: Some(hyperswitch_domain_models::address::AddressDetails {
                    first_name: Some(Secret::new("John".to_string())),
                    last_name: Some(Secret::new("Doe".to_string())),
                    line1: Some(Secret::new("123 Main Street".to_string())),
                    city: Some("San Francisco".to_string()),
                    state: Some(Secret::new("CA".to_string())),
                    zip: Some(Secret::new("94105".to_string())),
                    country: Some(enums::CountryAlpha2::US),
                    ..Default::default()
                }),
                phone: Some(hyperswitch_domain_models::address::PhoneDetails {
                    number: Some(Secret::new("4155550100".to_string())),
                    country_code: Some("+1".to_string()),
                }),
                email: Some("john.doe@example.com".to_string().try_into().unwrap()),
            }),
            None,
        )),
        auth_type: Some(enums::AuthenticationType::ThreeDs),
        ..Default::default()
    })
}

#[actix_web::test]
#[ignore]
async fn should_make_payment() {
    let response = CONNECTOR
        .make_payment(payment_method_details(), payment_info())
        .await
        .expect("Authorize payment response");
    assert!(
        matches!(
            response.status,
            enums::AttemptStatus::Charged
                | enums::AttemptStatus::AuthenticationPending
                | enums::AttemptStatus::Authorizing
        ),
        "Unexpected payment status: {:?}",
        response.status
    );
}

#[actix_web::test]
#[ignore]
async fn should_make_payment_and_refund() {
    let response = CONNECTOR
        .make_payment_and_refund(payment_method_details(), None, payment_info())
        .await
        .expect("Refund response");
    assert!(
        matches!(
            response.response.unwrap().refund_status,
            enums::RefundStatus::Success | enums::RefundStatus::Pending
        ),
        "Unexpected refund status"
    );
}
