use serde::{Deserialize, Serialize};
use crate::{core::errors,types::{self,api, storage::enums}};

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwedbankPaymentsRequest {
    payment: Payment,
}

impl TryFrom<&types::PaymentsAuthorizeRouterData> for SwedbankPaymentsRequest  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(_item: &types::PaymentsAuthorizeRouterData) -> Result<Self,Self::Error> {
        todo!()
    }
}

//TODO: Fill the struct with respective fields
// Auth Struct
pub struct SwedbankAuthType {
    pub(super) api_key: String
}

impl TryFrom<&types::ConnectorAuthType> for SwedbankAuthType  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(_auth_type: &types::ConnectorAuthType) -> Result<Self, Self::Error> {
        todo!()
    }
}
// PaymentsResponse
//TODO: Append the remaining status flags
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SwedbankPaymentStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<SwedbankPaymentStatus> for enums::AttemptStatus {
    fn from(item: SwedbankPaymentStatus) -> Self {
        match item {
            SwedbankPaymentStatus::Succeeded => Self::Charged,
            SwedbankPaymentStatus::Failed => Self::Failure,
            SwedbankPaymentStatus::Processing => Self::Authorizing,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SwedbankPaymentsResponse {
    status: SwedbankPaymentStatus,
    id: String,
}

impl<F,T> TryFrom<types::ResponseRouterData<F, SwedbankPaymentsResponse, T, types::PaymentsResponseData>> for types::RouterData<F, T, types::PaymentsResponseData> {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(item: types::ResponseRouterData<F, SwedbankPaymentsResponse, T, types::PaymentsResponseData>) -> Result<Self,Self::Error> {
        Ok(Self {
            status: enums::AttemptStatus::from(item.response.status),
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::ConnectorTransactionId(item.response.id),
                redirection_data: None,
                redirect: false,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct SwedbankRefundRequest {}

impl<F> TryFrom<&types::RefundsRouterData<F>> for SwedbankRefundRequest {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: &types::RefundsRouterData<F>) -> Result<Self,Self::Error> {
       todo!()
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
            //TODO: Review mapping
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
}

impl TryFrom<types::RefundsResponseRouterData<api::Execute, RefundResponse>>
    for types::RefundsRouterData<api::Execute>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        _item: types::RefundsResponseRouterData<api::Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<types::RefundsResponseRouterData<api::RSync, RefundResponse>> for types::RefundsRouterData<api::RSync>
{
     type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: types::RefundsResponseRouterData<api::RSync, RefundResponse>) -> Result<Self,Self::Error> {
         todo!()
     }
 }

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct SwedbankErrorResponse {
    pub error_code: u16,
    pub detailed_error_code: String,
    pub detailed_error_description: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    operation: String,
    intent: String,
    currency: String,
    prices: Prices,
    description: String,
    user_agent: String,
    language: String,
    urls: Urls,
    payee_info: PayeeInfo,
    payer: Payer,
    cardholder: Cardholder,
    risk_indicator: RiskIndicator
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    #[serde(rename = "type")]
    type_of_response: String,
    amount: i64,
    vat_amount: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Urls {
    complete_url: String,
    cancel_url: String,
    callback_url:String,
    logo_url: String,
    terms_of_service_url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayeeInfo {
    payee_id: String,
    payee_reference: String,
    payee_name: String,
    product_category: String,
    order_reference: String,
    subsite: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Payer {
    payer_reference: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cardholder {
    first_name: String,
    last_name: String,
    email: String,
    msisdn: String,
    home_phone_number: String,
    work_phone_number: String,
    shipping_address: ShippingAddress,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingAddress {
    first_name: String,
    last_name: String,
    email: String,
    msisdn: String,
    street_address: String,
    co_address: String,
    city: String,
    zip_code: String,
    country_code: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskIndicator {
    delivery_email_address: String,
    delivery_time_frame_indicator: String,
    pre_order_date: String,
    pre_order_purchase_indicator: String,
    ship_indicator: String,
    gift_card_purchase: bool,
    re_order_purchase_indicator: String,
    pick_up_address: PickUpAddress
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PickUpAddress {
    name: String,
    street_address: String,
    co_address: String,
    city: String,
    zip_code: String,
    country_code: String,
}

