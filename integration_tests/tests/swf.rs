#![cfg(feature = "swf")]

extern crate rusoto_core;
extern crate rusoto_swf;

use rusoto_swf::{Swf, SwfClient, ListDomainsInput};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_domains() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SwfClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let mut request = ListDomainsInput::default();
    request.maximum_page_size = Some(10);
    request.registration_status = "REGISTERED".to_string();

    client.list_domains(&request).unwrap();
}
