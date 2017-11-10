
// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="<p>An entitlement represents capacity in a product owned by the customer. For example, a customer might own some number of users or seats in an SaaS application or some amount of data capacity in a multi-tenant database.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Entitlement {
    #[doc="<p>The customer identifier is a handle to each unique customer in an application. Customer identifiers are obtained through the ResolveCustomer operation in AWS Marketplace Metering Service.</p>"]
    #[serde(rename="CustomerIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_identifier: Option<String>,
    #[doc="<p>The dimension for which the given entitlement applies. Dimensions represent categories of capacity in a product and are specified when the product is listed in AWS Marketplace.</p>"]
    #[serde(rename="Dimension")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dimension: Option<String>,
    #[doc="<p>The expiration date represents the minimum date through which this entitlement is expected to remain valid. For contractual products listed on AWS Marketplace, the expiration date is the date at which the customer will renew or cancel their contract. Customers who are opting to renew their contract will still have entitlements with an expiration date.</p>"]
    #[serde(rename="ExpirationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expiration_date: Option<f64>,
    #[doc="<p>The product code for which the given entitlement applies. Product codes are provided by AWS Marketplace when the product listing is created.</p>"]
    #[serde(rename="ProductCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_code: Option<String>,
    #[doc="<p>The EntitlementValue represents the amount of capacity that the customer is entitled to for the product.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<EntitlementValue>,
}

#[doc="<p>The EntitlementValue represents the amount of capacity that the customer is entitled to for the product.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EntitlementValue {
    #[doc="<p>The BooleanValue field will be populated with a boolean value when the entitlement is a boolean type. Otherwise, the field will not be set.</p>"]
    #[serde(rename="BooleanValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub boolean_value: Option<bool>,
    #[doc="<p>The DoubleValue field will be populated with a double value when the entitlement is a double type. Otherwise, the field will not be set.</p>"]
    #[serde(rename="DoubleValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub double_value: Option<f64>,
    #[doc="<p>The IntegerValue field will be populated with an integer value when the entitlement is an integer type. Otherwise, the field will not be set.</p>"]
    #[serde(rename="IntegerValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub integer_value: Option<i64>,
    #[doc="<p>The StringValue field will be populated with a string value when the entitlement is a string type. Otherwise, the field will not be set.</p>"]
    #[serde(rename="StringValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub string_value: Option<String>,
}

#[doc="<p>The GetEntitlementsRequest contains parameters for the GetEntitlements operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetEntitlementsRequest {
    #[doc="<p>Filter is used to return entitlements for a specific customer or for a specific dimension. Filters are described as keys mapped to a lists of values. Filtered requests are <i>unioned</i> for each value in the value list, and then <i>intersected</i> for each filter key.</p>"]
    #[serde(rename="Filter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[doc="<p>The maximum number of items to retrieve from the GetEntitlements operation. For pagination, use the NextToken field in subsequent calls to GetEntitlements.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>For paginated calls to GetEntitlements, pass the NextToken from the previous GetEntitlementsResult.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Product code is used to uniquely identify a product in AWS Marketplace. The product code will be provided by AWS Marketplace when the product listing is created.</p>"]
    #[serde(rename="ProductCode")]
    pub product_code: String,
}

#[doc="<p>The GetEntitlementsRequest contains results from the GetEntitlements operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetEntitlementsResult {
    #[doc="<p>The set of entitlements found through the GetEntitlements operation. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>"]
    #[serde(rename="Entitlements")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,
    #[doc="<p>For paginated results, use NextToken in subsequent calls to GetEntitlements. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

/// Errors returned by GetEntitlements
#[derive(Debug, PartialEq)]
pub enum GetEntitlementsError {
    ///<p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    ///<p>One or more parameters in your request was invalid.</p>
    InvalidParameter(String),
    ///<p>The calls to the GetEntitlements API are throttled.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetEntitlementsError {
    pub fn from_body(body: &str) -> GetEntitlementsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceErrorException" => {
                        GetEntitlementsError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetEntitlementsError::InvalidParameter(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetEntitlementsError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetEntitlementsError::Validation(error_message.to_string())
                    }
                    _ => GetEntitlementsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetEntitlementsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetEntitlementsError {
    fn from(err: serde_json::error::Error) -> GetEntitlementsError {
        GetEntitlementsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetEntitlementsError {
    fn from(err: CredentialsError) -> GetEntitlementsError {
        GetEntitlementsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEntitlementsError {
    fn from(err: HttpDispatchError) -> GetEntitlementsError {
        GetEntitlementsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEntitlementsError {
    fn from(err: io::Error) -> GetEntitlementsError {
        GetEntitlementsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEntitlementsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEntitlementsError {
    fn description(&self) -> &str {
        match *self {
            GetEntitlementsError::InternalServiceError(ref cause) => cause,
            GetEntitlementsError::InvalidParameter(ref cause) => cause,
            GetEntitlementsError::Throttling(ref cause) => cause,
            GetEntitlementsError::Validation(ref cause) => cause,
            GetEntitlementsError::Credentials(ref err) => err.description(),
            GetEntitlementsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetEntitlementsError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Marketplace Entitlement Service API. AWS Marketplace Entitlement Service clients implement this trait.
pub trait MarketplaceEntitlement {
    #[doc="<p>GetEntitlements retrieves entitlement values for a given product. The results can be filtered based on customer identifier or product dimensions.</p>"]
    fn get_entitlements(&self,
                        input: &GetEntitlementsRequest)
                        -> Result<GetEntitlementsResult, GetEntitlementsError>;
}
/// A client for the AWS Marketplace Entitlement Service API.
pub struct MarketplaceEntitlementClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> MarketplaceEntitlementClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        MarketplaceEntitlementClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> MarketplaceEntitlement for MarketplaceEntitlementClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>GetEntitlements retrieves entitlement values for a given product. The results can be filtered based on customer identifier or product dimensions.</p>"]
    fn get_entitlements(&self,
                        input: &GetEntitlementsRequest)
                        -> Result<GetEntitlementsResult, GetEntitlementsError> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("entitlement.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPEntitlementService.GetEntitlements");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetEntitlementsResult>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetEntitlementsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
