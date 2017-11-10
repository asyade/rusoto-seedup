extern crate rusoto_mock;

use ::*;

use rusoto_core::{Region, SignedRequest};
use self::rusoto_mock::*;

#[test]
fn initiate_multipart_upload_happy_path() {
    let body = MockResponseReader::read_response("test_resources/custom", "s3_initiate_multipart_upload.xml");
    let mock = MockRequestDispatcher::with_status(200).with_body(&body);

    let client = S3Client::new(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.create_multipart_upload(&CreateMultipartUploadRequest {
        bucket: "example-bucket".to_owned(),
        key: "example-object".to_owned(),
        ..Default::default()
    });

    match result {
        Err(_) => panic!("Couldn't parse initiate_multipart_upload"),
        Ok(result) => {
            assert_eq!(sstr("example-bucket"), result.bucket);
            assert_eq!(sstr("example-object"), result.key);
            assert_eq!(sstr("VXBsb2FkIElEIGZvciA2aWWpbmcncyBteS1tb3ZpZS5tMnRzIHVwbG9hZA"),
                        result.upload_id);
        }
    }
}

#[test]
fn complete_multipart_upload_happy_path() {
    let body = MockResponseReader::read_response("test_resources/custom", "s3_complete_multipart_upload.xml");
    let mock = MockRequestDispatcher::with_status(200).with_body(&body);
    
    let client = S3Client::new(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.complete_multipart_upload(&CompleteMultipartUploadRequest {
        bucket: "example-bucket".to_owned(),
        key: "example-object".to_owned(),
        upload_id: "VXBsb2FkIElEIGZvciA2aWWpbmcncyBteS1tb3ZpZS5tMnRzIHVwbG9hZA".to_owned(),
        ..Default::default()
    });

    match result {
        Err(_) => panic!("Couldn't parse s3_complete_multipart_upload"),
        Ok(result) => {
            assert_eq!(result.bucket, sstr("testbucket2"));
            assert_eq!(result.key, sstr("foo.zip"));
            assert_eq!(result.e_tag, sstr("\"525a81fcbc4181997bd96e4096fa7304-1\""));
        }
    }
}

#[test]
fn list_multipart_upload_happy_path() {
    let body = MockResponseReader::read_response("test_resources/custom", "s3_list_multipart_uploads.xml");
    let mock = MockRequestDispatcher::with_status(200).with_body(&body);
    
    let client = S3Client::new(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.list_multipart_uploads(&ListMultipartUploadsRequest {
        bucket: "example-bucket".to_owned(),
        ..Default::default()
    });

    match result {
        Err(_) => panic!("Couldn't parse s3_list_multipart_uploads.xml"),
        Ok(result) => {
            assert_eq!(result.bucket, sstr("rusoto1440826511"));
            assert!(result.uploads.is_some());

            let an_upload = &result.uploads.unwrap()[0];
            assert_eq!(an_upload.upload_id,
                        sstr("eUeGzA6xR2jAH7KUhTSwrrNVfu8XPIYdoWpa7meOiceoGQLQhtKfPg_APCnuVRsyWd7bx8SS5jNssgdtTU5tTziGOz.j1URgseoqpdHqnyZRikJHTLd6iXF.GjKBEhky"));
            assert_eq!(an_upload.key, sstr("join.me.zip"));

            let test_initiator = Initiator {
                id: sstr("arn:aws:iam::347452556412:user/matthew"),
                display_name: sstr("matthew"),
            };

            assert_eq!(an_upload.initiator.as_ref().unwrap().id, test_initiator.id);
            assert_eq!(an_upload.initiator.as_ref().unwrap().display_name,
                        test_initiator.display_name);

            assert_eq!(an_upload.initiated, sstr("2015-09-01T19:22:56.000Z"));

            let test_owner = Owner {
                id: sstr("b84c6b0c308085829b6562b586f6664fc00faab6cfd441e90ad418ea916eed83"),
                display_name: sstr("matthew"),
            };

            assert_eq!(an_upload.owner.as_ref().unwrap().id, test_owner.id);
            assert_eq!(an_upload.owner.as_ref().unwrap().display_name,
                        test_owner.display_name);

            assert_eq!(an_upload.storage_class, sstr("STANDARD"));
        }
    }
}

#[test]
fn list_multipart_upload_parts_happy_path() {
    let mock = MockRequestDispatcher::with_status(200)
        .with_body(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <ListPartsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
            <Bucket>rusoto1440826511</Bucket>
            <Key>testfile.zip</Key>
            <UploadId>PeePB_uORK5f2AURP_SWcQ4NO1P1oqnGNNNFK3nhFfzMeksdvG7x7nFfH1qk7a3HSossNYB7t8QhcN1Fg6ax7AXbwvAKIZ9DilB4tUcpM7qyUEgkszN4iDmMvSaImGFK</UploadId>
            <Initiator>
                <ID>arn:aws:iam::347452556412:user/matthew</ID>
                <DisplayName>matthew</DisplayName>
            </Initiator>
            <Owner>
                <ID>b84c6b0c308085829b6562b586f6664fc00faab6cfd441e90ad418ea916eed83</ID>
                <DisplayName>matthew</DisplayName>
            </Owner>
            <StorageClass>STANDARD</StorageClass>
            <PartNumberMarker>0</PartNumberMarker>
            <NextPartNumberMarker>2</NextPartNumberMarker>
            <MaxParts>1000</MaxParts>
            <IsTruncated>false</IsTruncated>
            <Part>
                <PartNumber>1</PartNumber>
                <LastModified>2015-09-08T21:02:04.000Z</LastModified>
                <ETag>&quot;ddcaa99616d7cd06d0a5abfef6ccebbb&quot;</ETag>
                <Size>5242880</Size>
            </Part>
            <Part>
                <PartNumber>2</PartNumber>
                <LastModified>2015-09-08T21:02:09.000Z</LastModified>
                <ETag>&quot;c865f7d241e2c9e3d3b5fee6955c616e&quot;</ETag>
                <Size>5242880</Size>
            </Part>
        </ListPartsResult>"#)
        .with_request_checker(|request: &SignedRequest| {
            assert_eq!(request.method, "GET");
            assert_eq!(request.path, "/rusoto1440826511/testfile.zip");
            assert_eq!(request.payload, None);
        });

    let mut req = ListPartsRequest::default();
    req.bucket = "rusoto1440826511".to_owned();
    req.key = "testfile.zip".to_owned();

    let client = S3Client::new(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.list_parts(&req).unwrap();
    assert_eq!(result.bucket, sstr("rusoto1440826511"));
    assert_eq!(result.upload_id,
                sstr("PeePB_uORK5f2AURP_SWcQ4NO1P1oqnGNNNFK3nhFfzMeksdvG7x7nFfH1qk7a3HSossNYB7t8QhcN1Fg6ax7AXbwvAKIZ9DilB4tUcpM7qyUEgkszN4iDmMvSaImGFK"));
    assert_eq!(result.key, sstr("testfile.zip"));

    let test_initiator = Initiator {
        id: sstr("arn:aws:iam::347452556412:user/matthew"),
        display_name: sstr("matthew"),
    };

    assert_eq!(result.initiator.as_ref().unwrap().id, test_initiator.id);
    assert_eq!(result.initiator.as_ref().unwrap().display_name,
                test_initiator.display_name);

    let test_owner = Owner {
        id: sstr("b84c6b0c308085829b6562b586f6664fc00faab6cfd441e90ad418ea916eed83"),
        display_name: sstr("matthew"),
    };

    assert_eq!(result.owner.as_ref().unwrap().id, test_owner.id);
    assert_eq!(result.owner.as_ref().unwrap().display_name,
                test_owner.display_name);

    assert_eq!(result.storage_class, sstr("STANDARD"));

    assert!(result.parts.is_some());

    let parts = result.parts.unwrap();
    assert_eq!(parts.len(), 2);

    assert_eq!(parts[0].part_number, Some(1));
    assert_eq!(parts[0].e_tag, sstr("\"ddcaa99616d7cd06d0a5abfef6ccebbb\""));
    assert_eq!(parts[0].size, Some(5242880));
    assert_eq!(parts[0].last_modified, sstr("2015-09-08T21:02:04.000Z"));
}

#[test]
fn list_multipart_uploads_no_uploads() {
    let mock = MockRequestDispatcher::with_status(200).with_body(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <ListMultipartUploadsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
            <Bucket>rusoto1440826568</Bucket>
            <KeyMarker></KeyMarker>
            <UploadIdMarker></UploadIdMarker>
            <NextKeyMarker></NextKeyMarker>
            <NextUploadIdMarker></NextUploadIdMarker>
            <MaxUploads>1000</MaxUploads>
            <IsTruncated>false</IsTruncated>
        </ListMultipartUploadsResult>
    "#);

    let mut req = ListMultipartUploadsRequest::default();
    req.bucket = "test-bucket".to_owned();

    let client = S3Client::new(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.list_multipart_uploads(&req).unwrap();

    assert_eq!(result.bucket, sstr("rusoto1440826568"));
    assert!(result.uploads.is_none());
}


#[test]
// sample response from the S3 documentation
// tests the model generation and deserialization end-to-end
fn should_parse_sample_list_buckets_response() {
    let mock = MockRequestDispatcher::with_status(200)
        .with_body(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <ListAllMyBucketsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01">
            <Owner>
            <ID>bcaf1ffd86f461ca5fb16fd081034f</ID>
            <DisplayName>webfile</DisplayName>
            </Owner>
            <Buckets>
            <Bucket>
                    <Name>quotes</Name>
                    <CreationDate>2006-02-03T16:45:09.000Z</CreationDate>
            </Bucket>
            <Bucket>
                    <Name>samples</Name>
                    <CreationDate>2006-02-03T16:41:58.000Z</CreationDate>
            </Bucket>
            </Buckets>
        </ListAllMyBucketsResult>
        "#)
        .with_request_checker(|request: &SignedRequest| {
            assert_eq!(request.method, "GET");
            assert_eq!(request.path, "/");
            assert_eq!(request.payload, None);
        });

    let client = S3Client::new(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.list_buckets().unwrap();

    let owner = result.owner.unwrap();
    assert_eq!(owner.display_name, Some("webfile".to_string()));
    assert_eq!(owner.id, Some("bcaf1ffd86f461ca5fb16fd081034f".to_string()));

    let buckets = result.buckets.unwrap();
    assert_eq!(buckets.len(), 2);

    let bucket1 = buckets.get(0).unwrap();
    assert_eq!(bucket1.name, Some("quotes".to_string()));
    assert_eq!(bucket1.creation_date,
                Some("2006-02-03T16:45:09.000Z".to_string()));
}

#[test]
fn should_parse_headers() {
    let mock = MockRequestDispatcher::with_status(200)
        .with_body("")
        .with_header("x-amz-expiration", "foo")
        .with_header("x-amz-restore", "bar");

    let client = S3Client::new(mock, MockCredentialsProvider, Region::UsEast1);
    let request = HeadObjectRequest::default();
    let result = client.head_object(&request).unwrap();

    assert_eq!(result.expiration, Some("foo".to_string()));
    assert_eq!(result.restore, Some("bar".to_string()));
}

#[test]
fn should_serialize_complicated_request() {
    let request = GetObjectRequest {
        bucket: "bucket".to_string(),
        if_match: sstr("if_match"),
        if_modified_since: sstr("if_modified_since"),
        if_none_match: sstr("if_none_match"),
        if_unmodified_since: sstr("if_unmodified_since"),
        key: "key".to_string(),
        part_number: Some(1),
        range: sstr("range"),
        request_payer: sstr("request_payer"),
        response_cache_control: sstr("response_cache_control"),
        response_content_disposition: sstr("response_content_disposition"),
        response_content_encoding: sstr("response_content_encoding"),
        response_content_language: sstr("response_content_language"),
        response_content_type: sstr("response_content_type"),
        response_expires: sstr("response_expires"),
        sse_customer_algorithm: sstr("sse_customer_algorithm"),
        sse_customer_key: sstr("sse_customer_key"),
        sse_customer_key_md5: sstr("sse_customer_key_md5"),
        version_id: sstr("version_id"),
    };

    let mock = MockRequestDispatcher::with_status(200)
        .with_body("")
        .with_request_checker(|request: &SignedRequest| {
            assert_eq!(request.method, "GET");
            assert_eq!(request.path, "/bucket/key");
            assert_eq!(*request.params.get("response-content-type").unwrap(),
                        sstr("response_content_type"));
            assert!(request.headers.get("range").unwrap().contains(&Vec::from("range")));
            assert_eq!(request.payload, None);
        });

    let client = S3Client::new(mock, MockCredentialsProvider, Region::UsEast1);
    let _ = client.get_object(&request).unwrap();
}

#[test]
fn should_parse_location_constraint() {
    let body = MockResponseReader::read_response("test_resources/generated/valid", "s3-get-bucket-location.xml");
    let mock = MockRequestDispatcher::with_status(200).with_body(&body);
    
    let client = S3Client::new(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.get_bucket_location(&GetBucketLocationRequest {
        bucket: "example-bucket".to_owned()
    });

    match result {
        Err(_) => panic!("Couldn't parse get_bucket_location"),
        Ok(result) => {
            assert_eq!(sstr("EU"), result.location_constraint);
        }
    }
}

/// returns Some(String)
fn sstr(value: &'static str) -> Option<String> {
    Some(value.to_string())
}
