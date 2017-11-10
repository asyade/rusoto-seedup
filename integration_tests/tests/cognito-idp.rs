#![cfg(feature = "cognito-idp")]

extern crate rusoto_core;
extern crate rusoto_cognito_idp;

use rusoto_cognito_idp::{CognitoIdentityProvider, CognitoIdentityProviderClient, ListUserPoolsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_user_pools() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CognitoIdentityProviderClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListUserPoolsRequest{
    	max_results: 10,
    	..Default::default()
    };

    let result = client.list_user_pools(&request).unwrap();
	println!("{:#?}", result);
}