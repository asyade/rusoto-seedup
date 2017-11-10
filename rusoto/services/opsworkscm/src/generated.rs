
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
#[doc="<p>Stores account attributes. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AccountAttribute {
    #[doc="<p> The maximum allowed value. </p>"]
    #[serde(rename="Maximum")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum: Option<i64>,
    #[doc="<p> The attribute name. The following are supported attribute names. </p> <ul> <li> <p> <i>ServerLimit:</i> The number of current servers/maximum number of servers allowed. By default, you can have a maximum of 10 servers. </p> </li> <li> <p> <i>ManualBackupLimit:</i> The number of current manual backups/maximum number of backups allowed. By default, you can have a maximum of 50 manual backups saved. </p> </li> </ul>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p> The current usage, such as the current number of servers that are associated with the account. </p>"]
    #[serde(rename="Used")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub used: Option<i64>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct AssociateNodeRequest {
    #[doc="<p>Engine attributes used for associating the node. </p> <p class=\"title\"> <b>Attributes accepted in a AssociateNode request:</b> </p> <ul> <li> <p> <code>CHEF_ORGANIZATION</code>: The Chef organization with which the node is associated. By default only one organization named <code>default</code> can exist. </p> </li> <li> <p> <code>CHEF_NODE_PUBLIC_KEY</code>: A PEM-formatted public key. This key is required for the <code>chef-client</code> agent to access the Chef API. </p> </li> </ul>"]
    #[serde(rename="EngineAttributes")]
    pub engine_attributes: Vec<EngineAttribute>,
    #[doc="<p>The name of the Chef client node. </p>"]
    #[serde(rename="NodeName")]
    pub node_name: String,
    #[doc="<p>The name of the server with which to associate the node. </p>"]
    #[serde(rename="ServerName")]
    pub server_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AssociateNodeResponse {
    #[doc="<p>Contains a token which can be passed to the <code>DescribeNodeAssociationStatus</code> API call to get the status of the association request. </p>"]
    #[serde(rename="NodeAssociationStatusToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_association_status_token: Option<String>,
}

#[doc="<p>Describes a single backup. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Backup {
    #[doc="<p>The ARN of the backup. </p>"]
    #[serde(rename="BackupArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub backup_arn: Option<String>,
    #[doc="<p> The generated ID of the backup. Example: <code>myServerName-yyyyMMddHHmmssSSS</code> </p>"]
    #[serde(rename="BackupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub backup_id: Option<String>,
    #[doc="<p> The backup type. Valid values are <code>automated</code> or <code>manual</code>. </p>"]
    #[serde(rename="BackupType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub backup_type: Option<String>,
    #[doc="<p> The time stamp when the backup was created in the database. Example: <code>2016-07-29T13:38:47.520Z</code> </p>"]
    #[serde(rename="CreatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<f64>,
    #[doc="<p> A user-provided description for a manual backup. This field is empty for automated backups. </p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p> The engine type that is obtained from the server when the backup is created. </p>"]
    #[serde(rename="Engine")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine: Option<String>,
    #[doc="<p> The engine model that is obtained from the server when the backup is created. </p>"]
    #[serde(rename="EngineModel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_model: Option<String>,
    #[doc="<p> The engine version that is obtained from the server when the backup is created. </p>"]
    #[serde(rename="EngineVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_version: Option<String>,
    #[doc="<p> The EC2 instance profile ARN that is obtained from the server when the backup is created. Because this value is stored, you are not required to provide the InstanceProfileArn again if you restore a backup. </p>"]
    #[serde(rename="InstanceProfileArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_profile_arn: Option<String>,
    #[doc="<p> The instance type that is obtained from the server when the backup is created. </p>"]
    #[serde(rename="InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type: Option<String>,
    #[doc="<p> The key pair that is obtained from the server when the backup is created. </p>"]
    #[serde(rename="KeyPair")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_pair: Option<String>,
    #[doc="<p> The preferred backup period that is obtained from the server when the backup is created. </p>"]
    #[serde(rename="PreferredBackupWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[doc="<p> The preferred maintenance period that is obtained from the server when the backup is created. </p>"]
    #[serde(rename="PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p> The Amazon S3 URL of the backup's log file. </p>"]
    #[serde(rename="S3LogUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_log_url: Option<String>,
    #[doc="<p> The security group IDs that are obtained from the server when the backup is created. </p>"]
    #[serde(rename="SecurityGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[doc="<p> The name of the server from which the backup was made. </p>"]
    #[serde(rename="ServerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_name: Option<String>,
    #[doc="<p> The service role ARN that is obtained from the server when the backup is created. </p>"]
    #[serde(rename="ServiceRoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_role_arn: Option<String>,
    #[doc="<p>The status of a backup while in progress. </p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p> An informational message about backup status. </p>"]
    #[serde(rename="StatusDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_description: Option<String>,
    #[doc="<p> The subnet IDs that are obtained from the server when the backup is created. </p>"]
    #[serde(rename="SubnetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[doc="<p> The version of AWS OpsWorks for Chef Automate-specific tools that is obtained from the server when the backup is created. </p>"]
    #[serde(rename="ToolsVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tools_version: Option<String>,
    #[doc="<p> The IAM user ARN of the requester for manual backups. This field is empty for automated backups. </p>"]
    #[serde(rename="UserArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateBackupRequest {
    #[doc="<p> A user-defined description of the backup. </p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The name of the server that you want to back up. </p>"]
    #[serde(rename="ServerName")]
    pub server_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateBackupResponse {
    #[doc="<p>Backup created by request.</p>"]
    #[serde(rename="Backup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub backup: Option<Backup>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateServerRequest {
    #[doc="<p> Associate a public IP address with a server that you are launching. Valid values are <code>true</code> or <code>false</code>. The default value is <code>true</code>. </p>"]
    #[serde(rename="AssociatePublicIpAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    #[doc="<p> If you specify this field, AWS OpsWorks for Chef Automate creates the server by using the backup represented by BackupId. </p>"]
    #[serde(rename="BackupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub backup_id: Option<String>,
    #[doc="<p> The number of automated backups that you want to keep. Whenever a new backup is created, AWS OpsWorks for Chef Automate deletes the oldest backups if this number is exceeded. The default value is <code>1</code>. </p>"]
    #[serde(rename="BackupRetentionCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub backup_retention_count: Option<i64>,
    #[doc="<p> Enable or disable scheduled backups. Valid values are <code>true</code> or <code>false</code>. The default value is <code>true</code>. </p>"]
    #[serde(rename="DisableAutomatedBackup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disable_automated_backup: Option<bool>,
    #[doc="<p> The configuration management engine to use. Valid values include <code>Chef</code>. </p>"]
    #[serde(rename="Engine")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine: Option<String>,
    #[doc="<p>Optional engine attributes on a specified server. </p> <p class=\"title\"> <b>Attributes accepted in a createServer request:</b> </p> <ul> <li> <p> <code>CHEF_PIVOTAL_KEY</code>: A base64-encoded RSA private key that is not stored by AWS OpsWorks for Chef. This private key is required to access the Chef API. When no CHEF_PIVOTAL_KEY is set, one is generated and returned in the response. </p> </li> <li> <p> <code>CHEF_DELIVERY_ADMIN_PASSWORD</code>: The password for the administrative user in the Chef Automate GUI. The password length is a minimum of eight characters, and a maximum of 32. The password can contain letters, numbers, and special characters (!/@#$%^&amp;+=_). The password must contain at least one lower case letter, one upper case letter, one number, and one special character. When no CHEF_DELIVERY_ADMIN_PASSWORD is set, one is generated and returned in the response.</p> </li> </ul>"]
    #[serde(rename="EngineAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_attributes: Option<Vec<EngineAttribute>>,
    #[doc="<p> The engine model, or option. Valid values include <code>Single</code>. </p>"]
    #[serde(rename="EngineModel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_model: Option<String>,
    #[doc="<p> The major release version of the engine that you want to use. Values depend on the engine that you choose. </p>"]
    #[serde(rename="EngineVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_version: Option<String>,
    #[doc="<p> The ARN of the instance profile that your Amazon EC2 instances use. Although the AWS OpsWorks console typically creates the instance profile for you, if you are using API commands instead, run the service-role-creation.yaml AWS CloudFormation template, located at https://s3.amazonaws.com/opsworks-cm-us-east-1-prod-default-assets/misc/opsworks-cm-roles.yaml. This template creates a CloudFormation stack that includes the instance profile you need. </p>"]
    #[serde(rename="InstanceProfileArn")]
    pub instance_profile_arn: String,
    #[doc="<p> The Amazon EC2 instance type to use. Valid values must be specified in the following format: <code>^([cm][34]|t2).*</code> For example, <code>m4.large</code>. Valid values are <code>t2.medium</code>, <code>m4.large</code>, or <code>m4.2xlarge</code>. </p>"]
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[doc="<p> The Amazon EC2 key pair to set for the instance. This parameter is optional; if desired, you may specify this parameter to connect to your instances by using SSH. </p>"]
    #[serde(rename="KeyPair")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_pair: Option<String>,
    #[doc="<p> The start time for a one-hour period during which AWS OpsWorks for Chef Automate backs up application-level data on your server if automated backups are enabled. Valid values must be specified in one of the following formats: </p> <ul> <li> <p> <code>HH:MM</code> for daily backups</p> </li> <li> <p> <code>DDD:HH:MM</code> for weekly backups</p> </li> </ul> <p>The specified time is in coordinated universal time (UTC). The default value is a random, daily start time.</p> <p> <b>Example:</b> <code>08:00</code>, which represents a daily start time of 08:00 UTC.</p> <p> <b>Example:</b> <code>Mon:08:00</code>, which represents a start time of every Monday at 08:00 UTC. (8:00 a.m.)</p>"]
    #[serde(rename="PreferredBackupWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[doc="<p> The start time for a one-hour period each week during which AWS OpsWorks for Chef Automate performs maintenance on the instance. Valid values must be specified in the following format: <code>DDD:HH:MM</code>. The specified time is in coordinated universal time (UTC). The default value is a random one-hour period on Tuesday, Wednesday, or Friday. See <code>TimeWindowDefinition</code> for more information. </p> <p> <b>Example:</b> <code>Mon:08:00</code>, which represents a start time of every Monday at 08:00 UTC. (8:00 a.m.) </p>"]
    #[serde(rename="PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p> A list of security group IDs to attach to the Amazon EC2 instance. If you add this parameter, the specified security groups must be within the VPC that is specified by <code>SubnetIds</code>. </p> <p> If you do not specify this parameter, AWS OpsWorks for Chef Automate creates one new security group that uses TCP ports 22 and 443, open to 0.0.0.0/0 (everyone). </p>"]
    #[serde(rename="SecurityGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[doc="<p> The name of the server. The server name must be unique within your AWS account, within each region. Server names must start with a letter; then letters, numbers, or hyphens (-) are allowed, up to a maximum of 40 characters. </p>"]
    #[serde(rename="ServerName")]
    pub server_name: String,
    #[doc="<p> The service role that the AWS OpsWorks for Chef Automate service backend uses to work with your account. Although the AWS OpsWorks management console typically creates the service role for you, if you are using the AWS CLI or API commands, run the service-role-creation.yaml AWS CloudFormation template, located at https://s3.amazonaws.com/opsworks-stuff/latest/service-role-creation.yaml. This template creates a CloudFormation stack that includes the service role that you need. </p>"]
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: String,
    #[doc="<p> The IDs of subnets in which to launch the server EC2 instance. </p> <p> Amazon EC2-Classic customers: This field is required. All servers must run within a VPC. The VPC must have \"Auto Assign Public IP\" enabled. </p> <p> EC2-VPC customers: This field is optional. If you do not specify subnet IDs, your EC2 instances are created in a default subnet that is selected by Amazon EC2. If you specify subnet IDs, the VPC must have \"Auto Assign Public IP\" enabled. </p> <p>For more information about supported Amazon EC2 platforms, see <a href=\"http://docs.aws.amazon.com/https:/docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-supported-platforms.html\">Supported Platforms</a>.</p>"]
    #[serde(rename="SubnetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateServerResponse {
    #[doc="<p>The server that is created by the request. </p>"]
    #[serde(rename="Server")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server: Option<Server>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteBackupRequest {
    #[doc="<p>The ID of the backup to delete. Run the DescribeBackups command to get a list of backup IDs. Backup IDs are in the format <code>ServerName-yyyyMMddHHmmssSSS</code>. </p>"]
    #[serde(rename="BackupId")]
    pub backup_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteBackupResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteServerRequest {
    #[doc="<p>The ID of the server to delete.</p>"]
    #[serde(rename="ServerName")]
    pub server_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteServerResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeAccountAttributesRequest;

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeAccountAttributesResponse {
    #[doc="<p> The attributes that are currently set for the account. </p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<AccountAttribute>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeBackupsRequest {
    #[doc="<p>Describes a single backup. </p>"]
    #[serde(rename="BackupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub backup_id: Option<String>,
    #[doc="<p>To receive a paginated response, use this parameter to specify the maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results. </p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>NextToken is a string that is returned in some command responses. It indicates that not all entries have been returned, and that you must run at least one more request to get remaining items. To get remaining results, call <code>DescribeBackups</code> again, and assign the token from the previous results as the value of the <code>nextToken</code> parameter. If there are no more results, the response object's <code>nextToken</code> parameter value is <code>null</code>. Setting a <code>nextToken</code> value that was not returned in your previous results causes an <code>InvalidNextTokenException</code> to occur.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Returns backups for the server with the specified ServerName. </p>"]
    #[serde(rename="ServerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_name: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeBackupsResponse {
    #[doc="<p>Contains the response to a <code>DescribeBackups</code> request. </p>"]
    #[serde(rename="Backups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub backups: Option<Vec<Backup>>,
    #[doc="<p>NextToken is a string that is returned in some command responses. It indicates that not all entries have been returned, and that you must run at least one more request to get remaining items. To get remaining results, call <code>DescribeBackups</code> again, and assign the token from the previous results as the value of the <code>nextToken</code> parameter. If there are no more results, the response object's <code>nextToken</code> parameter value is <code>null</code>. Setting a <code>nextToken</code> value that was not returned in your previous results causes an <code>InvalidNextTokenException</code> to occur. </p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEventsRequest {
    #[doc="<p>To receive a paginated response, use this parameter to specify the maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results. </p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>NextToken is a string that is returned in some command responses. It indicates that not all entries have been returned, and that you must run at least one more request to get remaining items. To get remaining results, call <code>DescribeEvents</code> again, and assign the token from the previous results as the value of the <code>nextToken</code> parameter. If there are no more results, the response object's <code>nextToken</code> parameter value is <code>null</code>. Setting a <code>nextToken</code> value that was not returned in your previous results causes an <code>InvalidNextTokenException</code> to occur. </p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The name of the server for which you want to view events.</p>"]
    #[serde(rename="ServerName")]
    pub server_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEventsResponse {
    #[doc="<p>NextToken is a string that is returned in some command responses. It indicates that not all entries have been returned, and that you must run at least one more request to get remaining items. To get remaining results, call <code>DescribeEvents</code> again, and assign the token from the previous results as the value of the <code>nextToken</code> parameter. If there are no more results, the response object's <code>nextToken</code> parameter value is <code>null</code>. Setting a <code>nextToken</code> value that was not returned in your previous results causes an <code>InvalidNextTokenException</code> to occur. </p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Contains the response to a <code>DescribeEvents</code> request. </p>"]
    #[serde(rename="ServerEvents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_events: Option<Vec<ServerEvent>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeNodeAssociationStatusRequest {
    #[serde(rename="NodeAssociationStatusToken")]
    pub node_association_status_token: String,
    #[doc="<p>The name of the server from which to disassociate the node. </p>"]
    #[serde(rename="ServerName")]
    pub server_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeNodeAssociationStatusResponse {
    #[doc="<p>The status of the association or disassociation request. </p> <p class=\"title\"> <b>Possible values:</b> </p> <ul> <li> <p> <code>SUCCESS</code>: The association or disassociation succeeded. </p> </li> <li> <p> <code>FAILED</code>: The association or disassociation failed. </p> </li> <li> <p> <code>IN_PROGRESS</code>: The association or disassociation is still in progress. </p> </li> </ul>"]
    #[serde(rename="NodeAssociationStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_association_status: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeServersRequest {
    #[doc="<p>To receive a paginated response, use this parameter to specify the maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results. </p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>NextToken is a string that is returned in some command responses. It indicates that not all entries have been returned, and that you must run at least one more request to get remaining items. To get remaining results, call <code>DescribeServers</code> again, and assign the token from the previous results as the value of the <code>nextToken</code> parameter. If there are no more results, the response object's <code>nextToken</code> parameter value is <code>null</code>. Setting a <code>nextToken</code> value that was not returned in your previous results causes an <code>InvalidNextTokenException</code> to occur. </p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Describes the server with the specified ServerName.</p>"]
    #[serde(rename="ServerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_name: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeServersResponse {
    #[doc="<p>NextToken is a string that is returned in some command responses. It indicates that not all entries have been returned, and that you must run at least one more request to get remaining items. To get remaining results, call <code>DescribeServers</code> again, and assign the token from the previous results as the value of the <code>nextToken</code> parameter. If there are no more results, the response object's <code>nextToken</code> parameter value is <code>null</code>. Setting a <code>nextToken</code> value that was not returned in your previous results causes an <code>InvalidNextTokenException</code> to occur. </p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Contains the response to a <code>DescribeServers</code> request. </p>"]
    #[serde(rename="Servers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub servers: Option<Vec<Server>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DisassociateNodeRequest {
    #[doc="<p>Engine attributes used for disassociating the node. </p> <p class=\"title\"> <b>Attributes accepted in a DisassociateNode request:</b> </p> <ul> <li> <p> <code>CHEF_ORGANIZATION</code>: The Chef organization with which the node was associated. By default only one organization named <code>default</code> can exist. </p> </li> </ul>"]
    #[serde(rename="EngineAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_attributes: Option<Vec<EngineAttribute>>,
    #[doc="<p>The name of the Chef client node. </p>"]
    #[serde(rename="NodeName")]
    pub node_name: String,
    #[doc="<p>The name of the server from which to disassociate the node. </p>"]
    #[serde(rename="ServerName")]
    pub server_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DisassociateNodeResponse {
    #[doc="<p>Contains a token which can be passed to the <code>DescribeNodeAssociationStatus</code> API call to get the status of the disassociation request. </p>"]
    #[serde(rename="NodeAssociationStatusToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_association_status_token: Option<String>,
}

#[doc="<p>A name and value pair that is specific to the engine of the server. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EngineAttribute {
    #[doc="<p>The name of the engine attribute. </p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The value of the engine attribute. </p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RestoreServerRequest {
    #[doc="<p> The ID of the backup that you want to use to restore a server. </p>"]
    #[serde(rename="BackupId")]
    pub backup_id: String,
    #[doc="<p> The type of the instance to create. Valid values must be specified in the following format: <code>^([cm][34]|t2).*</code> For example, <code>m4.large</code>. Valid values are <code>t2.medium</code>, <code>m4.large</code>, and <code>m4.2xlarge</code>. If you do not specify this parameter, RestoreServer uses the instance type from the specified backup. </p>"]
    #[serde(rename="InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type: Option<String>,
    #[doc="<p> The name of the key pair to set on the new EC2 instance. This can be helpful if the administrator no longer has the SSH key. </p>"]
    #[serde(rename="KeyPair")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_pair: Option<String>,
    #[doc="<p> The name of the server that you want to restore. </p>"]
    #[serde(rename="ServerName")]
    pub server_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RestoreServerResponse;

#[doc="<p>Describes a configuration management server. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Server {
    #[doc="<p>Associate a public IP address with a server that you are launching. </p>"]
    #[serde(rename="AssociatePublicIpAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    #[doc="<p>The number of automated backups to keep. </p>"]
    #[serde(rename="BackupRetentionCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub backup_retention_count: Option<i64>,
    #[doc="<p>The ARN of the CloudFormation stack that was used to create the server. </p>"]
    #[serde(rename="CloudFormationStackArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_formation_stack_arn: Option<String>,
    #[doc="<p>Time stamp of server creation. Example <code>2016-07-29T13:38:47.520Z</code> </p>"]
    #[serde(rename="CreatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<f64>,
    #[doc="<p>Disables automated backups. The number of stored backups is dependent on the value of PreferredBackupCount. </p>"]
    #[serde(rename="DisableAutomatedBackup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disable_automated_backup: Option<bool>,
    #[doc="<p> A DNS name that can be used to access the engine. Example: <code>myserver-asdfghjkl.us-east-1.opsworks.io</code> </p>"]
    #[serde(rename="Endpoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint: Option<String>,
    #[doc="<p>The engine type of the server. The valid value in this release is <code>Chef</code>. </p>"]
    #[serde(rename="Engine")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine: Option<String>,
    #[doc="<p>The response of a createServer() request returns the master credential to access the server in EngineAttributes. These credentials are not stored by AWS OpsWorks for Chef Automate; they are returned only as part of the result of createServer(). </p> <p class=\"title\"> <b>Attributes returned in a createServer response:</b> </p> <ul> <li> <p> <code>CHEF_PIVOTAL_KEY</code>: A base64-encoded RSA private key that is generated by AWS OpsWorks for Chef Automate. This private key is required to access the Chef API.</p> </li> <li> <p> <code>CHEF_STARTER_KIT</code>: A base64-encoded ZIP file. The ZIP file contains a Chef starter kit, which includes a README, a configuration file, and the required RSA private key. Save this file, unzip it, and then change to the directory where you've unzipped the file contents. From this directory, you can run Knife commands.</p> </li> </ul>"]
    #[serde(rename="EngineAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_attributes: Option<Vec<EngineAttribute>>,
    #[doc="<p>The engine model of the server. The valid value in this release is <code>Single</code>. </p>"]
    #[serde(rename="EngineModel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_model: Option<String>,
    #[doc="<p>The engine version of the server. Because Chef is the engine available in this release, the valid value for EngineVersion is <code>12</code>. </p>"]
    #[serde(rename="EngineVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_version: Option<String>,
    #[doc="<p>The instance profile ARN of the server. </p>"]
    #[serde(rename="InstanceProfileArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_profile_arn: Option<String>,
    #[doc="<p> The instance type for the server, as specified in the CloudFormation stack. This might not be the same instance type that is shown in the EC2 console. </p>"]
    #[serde(rename="InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type: Option<String>,
    #[doc="<p>The key pair associated with the server. </p>"]
    #[serde(rename="KeyPair")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_pair: Option<String>,
    #[doc="<p>The status of the most recent server maintenance run. Shows <code>SUCCESS</code> or <code>FAILED</code>. </p>"]
    #[serde(rename="MaintenanceStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maintenance_status: Option<String>,
    #[doc="<p>The preferred backup period specified for the server. </p>"]
    #[serde(rename="PreferredBackupWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[doc="<p>The preferred maintenance period specified for the server. </p>"]
    #[serde(rename="PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p> The security group IDs for the server, as specified in the CloudFormation stack. These might not be the same security groups that are shown in the EC2 console. </p>"]
    #[serde(rename="SecurityGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[doc="<p>The ARN of the server. </p>"]
    #[serde(rename="ServerArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_arn: Option<String>,
    #[doc="<p>The name of the server. </p>"]
    #[serde(rename="ServerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_name: Option<String>,
    #[doc="<p>The service role ARN used to create the server. </p>"]
    #[serde(rename="ServiceRoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_role_arn: Option<String>,
    #[doc="<p> The server's status. This field displays the states of actions in progress, such as creating, running, or backing up the server, as well as the server's health state. </p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p> Depending on the server status, this field has either a human-readable message (such as a create or backup error), or an escaped block of JSON (used for health check results). </p>"]
    #[serde(rename="StatusReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_reason: Option<String>,
    #[doc="<p> The subnet IDs specified in a CreateServer request. </p>"]
    #[serde(rename="SubnetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[doc="<p>An event that is related to the server, such as the start of maintenance or backup. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ServerEvent {
    #[doc="<p>The time when the event occurred. </p>"]
    #[serde(rename="CreatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<f64>,
    #[doc="<p>The Amazon S3 URL of the event's log file.</p>"]
    #[serde(rename="LogUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_url: Option<String>,
    #[doc="<p>A human-readable informational or status message.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[doc="<p>The name of the server on or for which the event occurred. </p>"]
    #[serde(rename="ServerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct StartMaintenanceRequest {
    #[doc="<p>The name of the server on which to run maintenance. </p>"]
    #[serde(rename="ServerName")]
    pub server_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartMaintenanceResponse {
    #[doc="<p>Contains the response to a <code>StartMaintenance</code> request. </p>"]
    #[serde(rename="Server")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server: Option<Server>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateServerEngineAttributesRequest {
    #[doc="<p>The name of the engine attribute to update. </p>"]
    #[serde(rename="AttributeName")]
    pub attribute_name: String,
    #[doc="<p>The value to set for the attribute. </p>"]
    #[serde(rename="AttributeValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_value: Option<String>,
    #[doc="<p>The name of the server to update. </p>"]
    #[serde(rename="ServerName")]
    pub server_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateServerEngineAttributesResponse {
    #[doc="<p>Contains the response to an <code>UpdateServerEngineAttributes</code> request. </p>"]
    #[serde(rename="Server")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server: Option<Server>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateServerRequest {
    #[doc="<p>Sets the number of automated backups that you want to keep. </p>"]
    #[serde(rename="BackupRetentionCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub backup_retention_count: Option<i64>,
    #[doc="<p>Setting DisableAutomatedBackup to <code>true</code> disables automated or scheduled backups. Automated backups are enabled by default. </p>"]
    #[serde(rename="DisableAutomatedBackup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disable_automated_backup: Option<bool>,
    #[serde(rename="PreferredBackupWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename="PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p>The name of the server to update. </p>"]
    #[serde(rename="ServerName")]
    pub server_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateServerResponse {
    #[doc="<p>Contains the response to a <code>UpdateServer</code> request. </p>"]
    #[serde(rename="Server")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server: Option<Server>,
}

/// Errors returned by AssociateNode
#[derive(Debug, PartialEq)]
pub enum AssociateNodeError {
    ///<p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AssociateNodeError {
    pub fn from_body(body: &str) -> AssociateNodeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidStateException" => {
                        AssociateNodeError::InvalidState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AssociateNodeError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssociateNodeError::Validation(error_message.to_string())
                    }
                    _ => AssociateNodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateNodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateNodeError {
    fn from(err: serde_json::error::Error) -> AssociateNodeError {
        AssociateNodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateNodeError {
    fn from(err: CredentialsError) -> AssociateNodeError {
        AssociateNodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateNodeError {
    fn from(err: HttpDispatchError) -> AssociateNodeError {
        AssociateNodeError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateNodeError {
    fn from(err: io::Error) -> AssociateNodeError {
        AssociateNodeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateNodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateNodeError {
    fn description(&self) -> &str {
        match *self {
            AssociateNodeError::InvalidState(ref cause) => cause,
            AssociateNodeError::ResourceNotFound(ref cause) => cause,
            AssociateNodeError::Validation(ref cause) => cause,
            AssociateNodeError::Credentials(ref err) => err.description(),
            AssociateNodeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AssociateNodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateBackup
#[derive(Debug, PartialEq)]
pub enum CreateBackupError {
    ///<p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    ///<p>The limit of servers or backups has been reached. </p>
    LimitExceeded(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateBackupError {
    pub fn from_body(body: &str) -> CreateBackupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidStateException" => {
                        CreateBackupError::InvalidState(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateBackupError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateBackupError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateBackupError::Validation(error_message.to_string())
                    }
                    _ => CreateBackupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateBackupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateBackupError {
    fn from(err: serde_json::error::Error) -> CreateBackupError {
        CreateBackupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBackupError {
    fn from(err: CredentialsError) -> CreateBackupError {
        CreateBackupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBackupError {
    fn from(err: HttpDispatchError) -> CreateBackupError {
        CreateBackupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBackupError {
    fn from(err: io::Error) -> CreateBackupError {
        CreateBackupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBackupError {
    fn description(&self) -> &str {
        match *self {
            CreateBackupError::InvalidState(ref cause) => cause,
            CreateBackupError::LimitExceeded(ref cause) => cause,
            CreateBackupError::ResourceNotFound(ref cause) => cause,
            CreateBackupError::Validation(ref cause) => cause,
            CreateBackupError::Credentials(ref err) => err.description(),
            CreateBackupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBackupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateServer
#[derive(Debug, PartialEq)]
pub enum CreateServerError {
    ///<p>The limit of servers or backups has been reached. </p>
    LimitExceeded(String),
    ///<p>The requested resource cannot be created because it already exists. </p>
    ResourceAlreadyExists(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateServerError {
    pub fn from_body(body: &str) -> CreateServerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "LimitExceededException" => {
                        CreateServerError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateServerError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateServerError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateServerError::Validation(error_message.to_string())
                    }
                    _ => CreateServerError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateServerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateServerError {
    fn from(err: serde_json::error::Error) -> CreateServerError {
        CreateServerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateServerError {
    fn from(err: CredentialsError) -> CreateServerError {
        CreateServerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateServerError {
    fn from(err: HttpDispatchError) -> CreateServerError {
        CreateServerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateServerError {
    fn from(err: io::Error) -> CreateServerError {
        CreateServerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateServerError {
    fn description(&self) -> &str {
        match *self {
            CreateServerError::LimitExceeded(ref cause) => cause,
            CreateServerError::ResourceAlreadyExists(ref cause) => cause,
            CreateServerError::ResourceNotFound(ref cause) => cause,
            CreateServerError::Validation(ref cause) => cause,
            CreateServerError::Credentials(ref err) => err.description(),
            CreateServerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateServerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBackup
#[derive(Debug, PartialEq)]
pub enum DeleteBackupError {
    ///<p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBackupError {
    pub fn from_body(body: &str) -> DeleteBackupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidStateException" => {
                        DeleteBackupError::InvalidState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteBackupError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteBackupError::Validation(error_message.to_string())
                    }
                    _ => DeleteBackupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBackupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteBackupError {
    fn from(err: serde_json::error::Error) -> DeleteBackupError {
        DeleteBackupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBackupError {
    fn from(err: CredentialsError) -> DeleteBackupError {
        DeleteBackupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBackupError {
    fn from(err: HttpDispatchError) -> DeleteBackupError {
        DeleteBackupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBackupError {
    fn from(err: io::Error) -> DeleteBackupError {
        DeleteBackupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBackupError {
    fn description(&self) -> &str {
        match *self {
            DeleteBackupError::InvalidState(ref cause) => cause,
            DeleteBackupError::ResourceNotFound(ref cause) => cause,
            DeleteBackupError::Validation(ref cause) => cause,
            DeleteBackupError::Credentials(ref err) => err.description(),
            DeleteBackupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBackupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteServer
#[derive(Debug, PartialEq)]
pub enum DeleteServerError {
    ///<p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteServerError {
    pub fn from_body(body: &str) -> DeleteServerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidStateException" => {
                        DeleteServerError::InvalidState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteServerError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteServerError::Validation(error_message.to_string())
                    }
                    _ => DeleteServerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteServerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteServerError {
    fn from(err: serde_json::error::Error) -> DeleteServerError {
        DeleteServerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteServerError {
    fn from(err: CredentialsError) -> DeleteServerError {
        DeleteServerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteServerError {
    fn from(err: HttpDispatchError) -> DeleteServerError {
        DeleteServerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteServerError {
    fn from(err: io::Error) -> DeleteServerError {
        DeleteServerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteServerError {
    fn description(&self) -> &str {
        match *self {
            DeleteServerError::InvalidState(ref cause) => cause,
            DeleteServerError::ResourceNotFound(ref cause) => cause,
            DeleteServerError::Validation(ref cause) => cause,
            DeleteServerError::Credentials(ref err) => err.description(),
            DeleteServerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteServerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAccountAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeAccountAttributesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeAccountAttributesError {
    pub fn from_body(body: &str) -> DescribeAccountAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DescribeAccountAttributesError::Validation(error_message.to_string())
                    }
                    _ => DescribeAccountAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAccountAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAccountAttributesError {
    fn from(err: serde_json::error::Error) -> DescribeAccountAttributesError {
        DescribeAccountAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAccountAttributesError {
    fn from(err: CredentialsError) -> DescribeAccountAttributesError {
        DescribeAccountAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAccountAttributesError {
    fn from(err: HttpDispatchError) -> DescribeAccountAttributesError {
        DescribeAccountAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAccountAttributesError {
    fn from(err: io::Error) -> DescribeAccountAttributesError {
        DescribeAccountAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAccountAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAccountAttributesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAccountAttributesError::Validation(ref cause) => cause,
            DescribeAccountAttributesError::Credentials(ref err) => err.description(),
            DescribeAccountAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAccountAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBackups
#[derive(Debug, PartialEq)]
pub enum DescribeBackupsError {
    ///<p>This occurs when the provided nextToken is not valid. </p>
    InvalidNextToken(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeBackupsError {
    pub fn from_body(body: &str) -> DescribeBackupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        DescribeBackupsError::InvalidNextToken(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeBackupsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeBackupsError::Validation(error_message.to_string())
                    }
                    _ => DescribeBackupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeBackupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeBackupsError {
    fn from(err: serde_json::error::Error) -> DescribeBackupsError {
        DescribeBackupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBackupsError {
    fn from(err: CredentialsError) -> DescribeBackupsError {
        DescribeBackupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBackupsError {
    fn from(err: HttpDispatchError) -> DescribeBackupsError {
        DescribeBackupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBackupsError {
    fn from(err: io::Error) -> DescribeBackupsError {
        DescribeBackupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeBackupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBackupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeBackupsError::InvalidNextToken(ref cause) => cause,
            DescribeBackupsError::ResourceNotFound(ref cause) => cause,
            DescribeBackupsError::Validation(ref cause) => cause,
            DescribeBackupsError::Credentials(ref err) => err.description(),
            DescribeBackupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeBackupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {
    ///<p>This occurs when the provided nextToken is not valid. </p>
    InvalidNextToken(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEventsError {
    pub fn from_body(body: &str) -> DescribeEventsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        DescribeEventsError::InvalidNextToken(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeEventsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEventsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventsError {
    fn from(err: serde_json::error::Error) -> DescribeEventsError {
        DescribeEventsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEventsError {
    fn from(err: CredentialsError) -> DescribeEventsError {
        DescribeEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventsError {
    fn from(err: HttpDispatchError) -> DescribeEventsError {
        DescribeEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEventsError {
    fn from(err: io::Error) -> DescribeEventsError {
        DescribeEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventsError::InvalidNextToken(ref cause) => cause,
            DescribeEventsError::ResourceNotFound(ref cause) => cause,
            DescribeEventsError::Validation(ref cause) => cause,
            DescribeEventsError::Credentials(ref err) => err.description(),
            DescribeEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeNodeAssociationStatus
#[derive(Debug, PartialEq)]
pub enum DescribeNodeAssociationStatusError {
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeNodeAssociationStatusError {
    pub fn from_body(body: &str) -> DescribeNodeAssociationStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => DescribeNodeAssociationStatusError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DescribeNodeAssociationStatusError::Validation(error_message.to_string())
                    }
                    _ => DescribeNodeAssociationStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeNodeAssociationStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeNodeAssociationStatusError {
    fn from(err: serde_json::error::Error) -> DescribeNodeAssociationStatusError {
        DescribeNodeAssociationStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeNodeAssociationStatusError {
    fn from(err: CredentialsError) -> DescribeNodeAssociationStatusError {
        DescribeNodeAssociationStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeNodeAssociationStatusError {
    fn from(err: HttpDispatchError) -> DescribeNodeAssociationStatusError {
        DescribeNodeAssociationStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeNodeAssociationStatusError {
    fn from(err: io::Error) -> DescribeNodeAssociationStatusError {
        DescribeNodeAssociationStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeNodeAssociationStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeNodeAssociationStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeNodeAssociationStatusError::ResourceNotFound(ref cause) => cause,
            DescribeNodeAssociationStatusError::Validation(ref cause) => cause,
            DescribeNodeAssociationStatusError::Credentials(ref err) => err.description(),
            DescribeNodeAssociationStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeNodeAssociationStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeServers
#[derive(Debug, PartialEq)]
pub enum DescribeServersError {
    ///<p>This occurs when the provided nextToken is not valid. </p>
    InvalidNextToken(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeServersError {
    pub fn from_body(body: &str) -> DescribeServersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        DescribeServersError::InvalidNextToken(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeServersError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeServersError::Validation(error_message.to_string())
                    }
                    _ => DescribeServersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeServersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeServersError {
    fn from(err: serde_json::error::Error) -> DescribeServersError {
        DescribeServersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeServersError {
    fn from(err: CredentialsError) -> DescribeServersError {
        DescribeServersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeServersError {
    fn from(err: HttpDispatchError) -> DescribeServersError {
        DescribeServersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeServersError {
    fn from(err: io::Error) -> DescribeServersError {
        DescribeServersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeServersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeServersError {
    fn description(&self) -> &str {
        match *self {
            DescribeServersError::InvalidNextToken(ref cause) => cause,
            DescribeServersError::ResourceNotFound(ref cause) => cause,
            DescribeServersError::Validation(ref cause) => cause,
            DescribeServersError::Credentials(ref err) => err.description(),
            DescribeServersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeServersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateNode
#[derive(Debug, PartialEq)]
pub enum DisassociateNodeError {
    ///<p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisassociateNodeError {
    pub fn from_body(body: &str) -> DisassociateNodeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidStateException" => {
                        DisassociateNodeError::InvalidState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DisassociateNodeError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisassociateNodeError::Validation(error_message.to_string())
                    }
                    _ => DisassociateNodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateNodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateNodeError {
    fn from(err: serde_json::error::Error) -> DisassociateNodeError {
        DisassociateNodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateNodeError {
    fn from(err: CredentialsError) -> DisassociateNodeError {
        DisassociateNodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateNodeError {
    fn from(err: HttpDispatchError) -> DisassociateNodeError {
        DisassociateNodeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateNodeError {
    fn from(err: io::Error) -> DisassociateNodeError {
        DisassociateNodeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateNodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateNodeError {
    fn description(&self) -> &str {
        match *self {
            DisassociateNodeError::InvalidState(ref cause) => cause,
            DisassociateNodeError::ResourceNotFound(ref cause) => cause,
            DisassociateNodeError::Validation(ref cause) => cause,
            DisassociateNodeError::Credentials(ref err) => err.description(),
            DisassociateNodeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisassociateNodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreServer
#[derive(Debug, PartialEq)]
pub enum RestoreServerError {
    ///<p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RestoreServerError {
    pub fn from_body(body: &str) -> RestoreServerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidStateException" => {
                        RestoreServerError::InvalidState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RestoreServerError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RestoreServerError::Validation(error_message.to_string())
                    }
                    _ => RestoreServerError::Unknown(String::from(body)),
                }
            }
            Err(_) => RestoreServerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RestoreServerError {
    fn from(err: serde_json::error::Error) -> RestoreServerError {
        RestoreServerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RestoreServerError {
    fn from(err: CredentialsError) -> RestoreServerError {
        RestoreServerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreServerError {
    fn from(err: HttpDispatchError) -> RestoreServerError {
        RestoreServerError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestoreServerError {
    fn from(err: io::Error) -> RestoreServerError {
        RestoreServerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestoreServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreServerError {
    fn description(&self) -> &str {
        match *self {
            RestoreServerError::InvalidState(ref cause) => cause,
            RestoreServerError::ResourceNotFound(ref cause) => cause,
            RestoreServerError::Validation(ref cause) => cause,
            RestoreServerError::Credentials(ref err) => err.description(),
            RestoreServerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RestoreServerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartMaintenance
#[derive(Debug, PartialEq)]
pub enum StartMaintenanceError {
    ///<p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartMaintenanceError {
    pub fn from_body(body: &str) -> StartMaintenanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidStateException" => {
                        StartMaintenanceError::InvalidState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StartMaintenanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartMaintenanceError::Validation(error_message.to_string())
                    }
                    _ => StartMaintenanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartMaintenanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartMaintenanceError {
    fn from(err: serde_json::error::Error) -> StartMaintenanceError {
        StartMaintenanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartMaintenanceError {
    fn from(err: CredentialsError) -> StartMaintenanceError {
        StartMaintenanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartMaintenanceError {
    fn from(err: HttpDispatchError) -> StartMaintenanceError {
        StartMaintenanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartMaintenanceError {
    fn from(err: io::Error) -> StartMaintenanceError {
        StartMaintenanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartMaintenanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartMaintenanceError {
    fn description(&self) -> &str {
        match *self {
            StartMaintenanceError::InvalidState(ref cause) => cause,
            StartMaintenanceError::ResourceNotFound(ref cause) => cause,
            StartMaintenanceError::Validation(ref cause) => cause,
            StartMaintenanceError::Credentials(ref err) => err.description(),
            StartMaintenanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartMaintenanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateServer
#[derive(Debug, PartialEq)]
pub enum UpdateServerError {
    ///<p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateServerError {
    pub fn from_body(body: &str) -> UpdateServerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidStateException" => {
                        UpdateServerError::InvalidState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateServerError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateServerError::Validation(error_message.to_string())
                    }
                    _ => UpdateServerError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateServerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateServerError {
    fn from(err: serde_json::error::Error) -> UpdateServerError {
        UpdateServerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateServerError {
    fn from(err: CredentialsError) -> UpdateServerError {
        UpdateServerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateServerError {
    fn from(err: HttpDispatchError) -> UpdateServerError {
        UpdateServerError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateServerError {
    fn from(err: io::Error) -> UpdateServerError {
        UpdateServerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateServerError {
    fn description(&self) -> &str {
        match *self {
            UpdateServerError::InvalidState(ref cause) => cause,
            UpdateServerError::ResourceNotFound(ref cause) => cause,
            UpdateServerError::Validation(ref cause) => cause,
            UpdateServerError::Credentials(ref err) => err.description(),
            UpdateServerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateServerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateServerEngineAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateServerEngineAttributesError {
    ///<p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    ///<p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateServerEngineAttributesError {
    pub fn from_body(body: &str) -> UpdateServerEngineAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidStateException" => {
                        UpdateServerEngineAttributesError::InvalidState(String::from(error_message))
                    }
                    "ResourceNotFoundException" => UpdateServerEngineAttributesError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        UpdateServerEngineAttributesError::Validation(error_message.to_string())
                    }
                    _ => UpdateServerEngineAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateServerEngineAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateServerEngineAttributesError {
    fn from(err: serde_json::error::Error) -> UpdateServerEngineAttributesError {
        UpdateServerEngineAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateServerEngineAttributesError {
    fn from(err: CredentialsError) -> UpdateServerEngineAttributesError {
        UpdateServerEngineAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateServerEngineAttributesError {
    fn from(err: HttpDispatchError) -> UpdateServerEngineAttributesError {
        UpdateServerEngineAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateServerEngineAttributesError {
    fn from(err: io::Error) -> UpdateServerEngineAttributesError {
        UpdateServerEngineAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateServerEngineAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateServerEngineAttributesError {
    fn description(&self) -> &str {
        match *self {
            UpdateServerEngineAttributesError::InvalidState(ref cause) => cause,
            UpdateServerEngineAttributesError::ResourceNotFound(ref cause) => cause,
            UpdateServerEngineAttributesError::Validation(ref cause) => cause,
            UpdateServerEngineAttributesError::Credentials(ref err) => err.description(),
            UpdateServerEngineAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateServerEngineAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the OpsWorksCM API. OpsWorksCM clients implement this trait.
pub trait OpsWorksCM {
    #[doc="<p> Associates a new node with the Chef server. This command is an alternative to <code>knife bootstrap</code>. For more information about how to disassociate a node, see <a>DisassociateNode</a>.</p> <p> A node can can only be associated with servers that are in a <code>HEALTHY</code> state. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. The AssociateNode API call can be integrated into Auto Scaling configurations, AWS Cloudformation templates, or the user data of a server's instance. </p> <p> Example: <code>aws opsworks-cm associate-node --server-name <i>MyServer</i> --node-name <i>MyManagedNode</i> --engine-attributes \"Name=<i>MyOrganization</i>,Value=default\" \"Name=<i>Chef_node_public_key</i>,Value=<i>Public_key_contents</i>\"</code> </p>"]
    fn associate_node(&self,
                      input: &AssociateNodeRequest)
                      -> Result<AssociateNodeResponse, AssociateNodeError>;


    #[doc="<p> Creates an application-level backup of a server. While the server is in the <code>BACKING_UP</code> state, the server cannot be changed, and no additional backup can be created. </p> <p> Backups can be created for servers in <code>RUNNING</code>, <code>HEALTHY</code>, and <code>UNHEALTHY</code> states. By default, you can create a maximum of 50 manual backups. </p> <p> This operation is asynchronous. </p> <p> A <code>LimitExceededException</code> is thrown when the maximum number of manual backups is reached. An <code>InvalidStateException</code> is thrown when the server is not in any of the following states: RUNNING, HEALTHY, or UNHEALTHY. A <code>ResourceNotFoundException</code> is thrown when the server is not found. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p>"]
    fn create_backup(&self,
                     input: &CreateBackupRequest)
                     -> Result<CreateBackupResponse, CreateBackupError>;


    #[doc="<p> Creates and immedately starts a new server. The server is ready to use when it is in the <code>HEALTHY</code> state. By default, you can create a maximum of 10 servers. </p> <p> This operation is asynchronous. </p> <p> A <code>LimitExceededException</code> is thrown when you have created the maximum number of servers (10). A <code>ResourceAlreadyExistsException</code> is thrown when a server with the same name already exists in the account. A <code>ResourceNotFoundException</code> is thrown when you specify a backup ID that is not valid or is for a backup that does not exist. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p> <p> If you do not specify a security group by adding the <code>SecurityGroupIds</code> parameter, AWS OpsWorks creates a new security group. The default security group opens the Chef server to the world on TCP port 443. If a KeyName is present, AWS OpsWorks enables SSH access. SSH is also open to the world on TCP port 22. </p> <p>By default, the Chef Server is accessible from any IP address. We recommend that you update your security group rules to allow access from known IP addresses and address ranges only. To edit security group rules, open Security Groups in the navigation pane of the EC2 management console. </p>"]
    fn create_server(&self,
                     input: &CreateServerRequest)
                     -> Result<CreateServerResponse, CreateServerError>;


    #[doc="<p> Deletes a backup. You can delete both manual and automated backups. This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when a backup deletion is already in progress. A <code>ResourceNotFoundException</code> is thrown when the backup does not exist. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p>"]
    fn delete_backup(&self,
                     input: &DeleteBackupRequest)
                     -> Result<DeleteBackupResponse, DeleteBackupError>;


    #[doc="<p> Deletes the server and the underlying AWS CloudFormation stack (including the server's EC2 instance). When you run this command, the server state is updated to <code>DELETING</code>. After the server is deleted, it is no longer returned by <code>DescribeServer</code> requests. If the AWS CloudFormation stack cannot be deleted, the server cannot be deleted. </p> <p> This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when a server deletion is already in progress. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p> <p> </p>"]
    fn delete_server(&self,
                     input: &DeleteServerRequest)
                     -> Result<DeleteServerResponse, DeleteServerError>;


    #[doc="<p> Describes your account attributes, and creates requests to increase limits before they are reached or exceeded. </p> <p> This operation is synchronous. </p>"]
    fn describe_account_attributes
        (&self)
         -> Result<DescribeAccountAttributesResponse, DescribeAccountAttributesError>;


    #[doc="<p> Describes backups. The results are ordered by time, with newest backups first. If you do not specify a BackupId or ServerName, the command returns all backups. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the backup does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn describe_backups(&self,
                        input: &DescribeBackupsRequest)
                        -> Result<DescribeBackupsResponse, DescribeBackupsError>;


    #[doc="<p> Describes events for a specified server. Results are ordered by time, with newest events first. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn describe_events(&self,
                       input: &DescribeEventsRequest)
                       -> Result<DescribeEventsResponse, DescribeEventsError>;


    #[doc="<p> Returns the current status of an existing association or disassociation request. </p> <p> A <code>ResourceNotFoundException</code> is thrown when no recent association or disassociation request with the specified token is found, or when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn describe_node_association_status
        (&self,
         input: &DescribeNodeAssociationStatusRequest)
         -> Result<DescribeNodeAssociationStatusResponse, DescribeNodeAssociationStatusError>;


    #[doc="<p> Lists all configuration management servers that are identified with your account. Only the stored results from Amazon DynamoDB are returned. AWS OpsWorks for Chef Automate does not query other services. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn describe_servers(&self,
                        input: &DescribeServersRequest)
                        -> Result<DescribeServersResponse, DescribeServersError>;


    #[doc="<p> Disassociates a node from a Chef server, and removes the node from the Chef server's managed nodes. After a node is disassociated, the node key pair is no longer valid for accessing the Chef API. For more information about how to associate a node, see <a>AssociateNode</a>. </p> <p>A node can can only be disassociated from a server that is in a <code>HEALTHY</code> state. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn disassociate_node(&self,
                         input: &DisassociateNodeRequest)
                         -> Result<DisassociateNodeResponse, DisassociateNodeError>;


    #[doc="<p> Restores a backup to a server that is in a <code>CONNECTION_LOST</code>, <code>HEALTHY</code>, <code>RUNNING</code>, <code>UNHEALTHY</code>, or <code>TERMINATED</code> state. When you run RestoreServer, the server's EC2 instance is deleted, and a new EC2 instance is configured. RestoreServer maintains the existing server endpoint, so configuration management of the server's client devices (nodes) should continue to work. </p> <p> This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when the server is not in a valid state. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn restore_server(&self,
                      input: &RestoreServerRequest)
                      -> Result<RestoreServerResponse, RestoreServerError>;


    #[doc="<p> Manually starts server maintenance. This command can be useful if an earlier maintenance attempt failed, and the underlying cause of maintenance failure has been resolved. The server is in an <code>UNDER_MAINTENANCE</code> state while maintenance is in progress. </p> <p> Maintenance can only be started on servers in <code>HEALTHY</code> and <code>UNHEALTHY</code> states. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn start_maintenance(&self,
                         input: &StartMaintenanceRequest)
                         -> Result<StartMaintenanceResponse, StartMaintenanceError>;


    #[doc="<p> Updates settings for a server. </p> <p> This operation is synchronous. </p>"]
    fn update_server(&self,
                     input: &UpdateServerRequest)
                     -> Result<UpdateServerResponse, UpdateServerError>;


    #[doc="<p> Updates engine-specific attributes on a specified server. The server enters the <code>MODIFYING</code> state when this operation is in progress. Only one update can occur at a time. You can use this command to reset the Chef server's private key (<code>CHEF_PIVOTAL_KEY</code>). </p> <p> This operation is asynchronous. </p> <p> This operation can only be called for servers in <code>HEALTHY</code> or <code>UNHEALTHY</code> states. Otherwise, an <code>InvalidStateException</code> is raised. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn update_server_engine_attributes
        (&self,
         input: &UpdateServerEngineAttributesRequest)
         -> Result<UpdateServerEngineAttributesResponse, UpdateServerEngineAttributesError>;
}
/// A client for the OpsWorksCM API.
pub struct OpsWorksCMClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> OpsWorksCMClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        OpsWorksCMClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> OpsWorksCM for OpsWorksCMClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p> Associates a new node with the Chef server. This command is an alternative to <code>knife bootstrap</code>. For more information about how to disassociate a node, see <a>DisassociateNode</a>.</p> <p> A node can can only be associated with servers that are in a <code>HEALTHY</code> state. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. The AssociateNode API call can be integrated into Auto Scaling configurations, AWS Cloudformation templates, or the user data of a server's instance. </p> <p> Example: <code>aws opsworks-cm associate-node --server-name <i>MyServer</i> --node-name <i>MyManagedNode</i> --engine-attributes \"Name=<i>MyOrganization</i>,Value=default\" \"Name=<i>Chef_node_public_key</i>,Value=<i>Public_key_contents</i>\"</code> </p>"]
    fn associate_node(&self,
                      input: &AssociateNodeRequest)
                      -> Result<AssociateNodeResponse, AssociateNodeError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.AssociateNode");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AssociateNodeResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AssociateNodeError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Creates an application-level backup of a server. While the server is in the <code>BACKING_UP</code> state, the server cannot be changed, and no additional backup can be created. </p> <p> Backups can be created for servers in <code>RUNNING</code>, <code>HEALTHY</code>, and <code>UNHEALTHY</code> states. By default, you can create a maximum of 50 manual backups. </p> <p> This operation is asynchronous. </p> <p> A <code>LimitExceededException</code> is thrown when the maximum number of manual backups is reached. An <code>InvalidStateException</code> is thrown when the server is not in any of the following states: RUNNING, HEALTHY, or UNHEALTHY. A <code>ResourceNotFoundException</code> is thrown when the server is not found. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p>"]
    fn create_backup(&self,
                     input: &CreateBackupRequest)
                     -> Result<CreateBackupResponse, CreateBackupError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.CreateBackup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateBackupResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateBackupError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Creates and immedately starts a new server. The server is ready to use when it is in the <code>HEALTHY</code> state. By default, you can create a maximum of 10 servers. </p> <p> This operation is asynchronous. </p> <p> A <code>LimitExceededException</code> is thrown when you have created the maximum number of servers (10). A <code>ResourceAlreadyExistsException</code> is thrown when a server with the same name already exists in the account. A <code>ResourceNotFoundException</code> is thrown when you specify a backup ID that is not valid or is for a backup that does not exist. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p> <p> If you do not specify a security group by adding the <code>SecurityGroupIds</code> parameter, AWS OpsWorks creates a new security group. The default security group opens the Chef server to the world on TCP port 443. If a KeyName is present, AWS OpsWorks enables SSH access. SSH is also open to the world on TCP port 22. </p> <p>By default, the Chef Server is accessible from any IP address. We recommend that you update your security group rules to allow access from known IP addresses and address ranges only. To edit security group rules, open Security Groups in the navigation pane of the EC2 management console. </p>"]
    fn create_server(&self,
                     input: &CreateServerRequest)
                     -> Result<CreateServerResponse, CreateServerError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.CreateServer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateServerResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateServerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Deletes a backup. You can delete both manual and automated backups. This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when a backup deletion is already in progress. A <code>ResourceNotFoundException</code> is thrown when the backup does not exist. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p>"]
    fn delete_backup(&self,
                     input: &DeleteBackupRequest)
                     -> Result<DeleteBackupResponse, DeleteBackupError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DeleteBackup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteBackupResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBackupError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Deletes the server and the underlying AWS CloudFormation stack (including the server's EC2 instance). When you run this command, the server state is updated to <code>DELETING</code>. After the server is deleted, it is no longer returned by <code>DescribeServer</code> requests. If the AWS CloudFormation stack cannot be deleted, the server cannot be deleted. </p> <p> This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when a server deletion is already in progress. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p> <p> </p>"]
    fn delete_server(&self,
                     input: &DeleteServerRequest)
                     -> Result<DeleteServerResponse, DeleteServerError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DeleteServer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteServerResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteServerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Describes your account attributes, and creates requests to increase limits before they are reached or exceeded. </p> <p> This operation is synchronous. </p>"]
    fn describe_account_attributes
        (&self)
         -> Result<DescribeAccountAttributesResponse, DescribeAccountAttributesError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OpsWorksCM_V2016_11_01.DescribeAccountAttributes");
        request.set_payload(Some(b"{}".to_vec()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeAccountAttributesResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeAccountAttributesError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="<p> Describes backups. The results are ordered by time, with newest backups first. If you do not specify a BackupId or ServerName, the command returns all backups. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the backup does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn describe_backups(&self,
                        input: &DescribeBackupsRequest)
                        -> Result<DescribeBackupsResponse, DescribeBackupsError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DescribeBackups");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeBackupsResponse>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeBackupsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Describes events for a specified server. Results are ordered by time, with newest events first. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn describe_events(&self,
                       input: &DescribeEventsRequest)
                       -> Result<DescribeEventsResponse, DescribeEventsError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DescribeEvents");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeEventsResponse>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeEventsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Returns the current status of an existing association or disassociation request. </p> <p> A <code>ResourceNotFoundException</code> is thrown when no recent association or disassociation request with the specified token is found, or when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn describe_node_association_status
        (&self,
         input: &DescribeNodeAssociationStatusRequest)
         -> Result<DescribeNodeAssociationStatusResponse, DescribeNodeAssociationStatusError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OpsWorksCM_V2016_11_01.DescribeNodeAssociationStatus");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeNodeAssociationStatusResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeNodeAssociationStatusError::from_body(String::from_utf8_lossy(&body)
                                                                      .as_ref()))
            }
        }
    }


    #[doc="<p> Lists all configuration management servers that are identified with your account. Only the stored results from Amazon DynamoDB are returned. AWS OpsWorks for Chef Automate does not query other services. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn describe_servers(&self,
                        input: &DescribeServersRequest)
                        -> Result<DescribeServersResponse, DescribeServersError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DescribeServers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeServersResponse>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeServersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Disassociates a node from a Chef server, and removes the node from the Chef server's managed nodes. After a node is disassociated, the node key pair is no longer valid for accessing the Chef API. For more information about how to associate a node, see <a>AssociateNode</a>. </p> <p>A node can can only be disassociated from a server that is in a <code>HEALTHY</code> state. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn disassociate_node(&self,
                         input: &DisassociateNodeRequest)
                         -> Result<DisassociateNodeResponse, DisassociateNodeError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DisassociateNode");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DisassociateNodeResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DisassociateNodeError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Restores a backup to a server that is in a <code>CONNECTION_LOST</code>, <code>HEALTHY</code>, <code>RUNNING</code>, <code>UNHEALTHY</code>, or <code>TERMINATED</code> state. When you run RestoreServer, the server's EC2 instance is deleted, and a new EC2 instance is configured. RestoreServer maintains the existing server endpoint, so configuration management of the server's client devices (nodes) should continue to work. </p> <p> This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when the server is not in a valid state. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn restore_server(&self,
                      input: &RestoreServerRequest)
                      -> Result<RestoreServerResponse, RestoreServerError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.RestoreServer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RestoreServerResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RestoreServerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Manually starts server maintenance. This command can be useful if an earlier maintenance attempt failed, and the underlying cause of maintenance failure has been resolved. The server is in an <code>UNDER_MAINTENANCE</code> state while maintenance is in progress. </p> <p> Maintenance can only be started on servers in <code>HEALTHY</code> and <code>UNHEALTHY</code> states. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn start_maintenance(&self,
                         input: &StartMaintenanceRequest)
                         -> Result<StartMaintenanceResponse, StartMaintenanceError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.StartMaintenance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartMaintenanceResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartMaintenanceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Updates settings for a server. </p> <p> This operation is synchronous. </p>"]
    fn update_server(&self,
                     input: &UpdateServerRequest)
                     -> Result<UpdateServerResponse, UpdateServerError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.UpdateServer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateServerResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateServerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Updates engine-specific attributes on a specified server. The server enters the <code>MODIFYING</code> state when this operation is in progress. Only one update can occur at a time. You can use this command to reset the Chef server's private key (<code>CHEF_PIVOTAL_KEY</code>). </p> <p> This operation is asynchronous. </p> <p> This operation can only be called for servers in <code>HEALTHY</code> or <code>UNHEALTHY</code> states. Otherwise, an <code>InvalidStateException</code> is raised. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>"]
    fn update_server_engine_attributes
        (&self,
         input: &UpdateServerEngineAttributesRequest)
         -> Result<UpdateServerEngineAttributesResponse, UpdateServerEngineAttributesError> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OpsWorksCM_V2016_11_01.UpdateServerEngineAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateServerEngineAttributesResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateServerEngineAttributesError::from_body(String::from_utf8_lossy(&body)
                                                                     .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
