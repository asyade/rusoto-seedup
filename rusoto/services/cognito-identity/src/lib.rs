
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

//! <fullname>Amazon Cognito</fullname> <p>Amazon Cognito is a web service that delivers scoped temporary credentials to mobile devices and other untrusted environments. Amazon Cognito uniquely identifies a device and supplies the user with a consistent identity over the lifetime of an application.</p> <p>Using Amazon Cognito, you can enable authentication with one or more third-party identity providers (Facebook, Google, or Login with Amazon), and you can also choose to support unauthenticated access from your app. Cognito delivers a unique identifier for each user and acts as an OpenID token provider trusted by AWS Security Token Service (STS) to access temporary, limited-privilege AWS credentials.</p> <p>To provide end-user credentials, first make an unsigned call to <a>GetId</a>. If the end user is authenticated with one of the supported identity providers, set the <code>Logins</code> map with the identity provider token. <code>GetId</code> returns a unique identifier for the user.</p> <p>Next, make an unsigned call to <a>GetCredentialsForIdentity</a>. This call expects the same <code>Logins</code> map as the <code>GetId</code> call, as well as the <code>IdentityID</code> originally returned by <code>GetId</code>. Assuming your identity pool has been configured via the <a>SetIdentityPoolRoles</a> operation, <code>GetCredentialsForIdentity</code> will return AWS credentials for your use. If your pool has not been configured with <code>SetIdentityPoolRoles</code>, or if you want to follow legacy flow, make an unsigned call to <a>GetOpenIdToken</a>, which returns the OpenID token necessary to call STS and retrieve AWS credentials. This call expects the same <code>Logins</code> map as the <code>GetId</code> call, as well as the <code>IdentityID</code> originally returned by <code>GetId</code>. The token returned by <code>GetOpenIdToken</code> can be passed to the STS operation <a href="http://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRoleWithWebIdentity.html">AssumeRoleWithWebIdentity</a> to retrieve AWS credentials.</p> <p>If you want to use Amazon Cognito in an Android, iOS, or Unity application, you will probably want to make API calls via the AWS Mobile SDK. To learn more, see the <a href="http://docs.aws.amazon.com/mobile/index.html">AWS Mobile SDK Developer Guide</a>.</p>
//!
//! If you're using the service, you're probably looking for [CognitoIdentityClient](struct.CognitoIdentityClient.html) and [CognitoIdentity](trait.CognitoIdentity.html).

extern crate hyper;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            
