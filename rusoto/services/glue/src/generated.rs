
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
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Action {
    #[serde(rename="Arguments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename="JobName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchCreatePartitionRequest {
    #[doc="<p>The ID of the catalog in which the partion is to be created. Currently, this should be the AWS account ID.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the metadata database in which the partition is to be created.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>A list of <code>PartitionInput</code> structures that define the partitions to be created.</p>"]
    #[serde(rename="PartitionInputList")]
    pub partition_input_list: Vec<PartitionInput>,
    #[doc="<p>The name of the metadata table in which the partition is to be created.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchCreatePartitionResponse {
    #[doc="<p>Errors encountered when trying to create the requested partitions.</p>"]
    #[serde(rename="Errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<PartitionError>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchDeleteConnectionRequest {
    #[doc="<p>The ID of the Data Catalog in which the connections reside. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>A list of names of the connections to delete.</p>"]
    #[serde(rename="ConnectionNameList")]
    pub connection_name_list: Vec<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchDeleteConnectionResponse {
    #[doc="<p>A map of the names of connections that were not successfully deleted to error details.</p>"]
    #[serde(rename="Errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<::std::collections::HashMap<String, ErrorDetail>>,
    #[doc="<p>A list of names of the connection definitions that were successfully deleted.</p>"]
    #[serde(rename="Succeeded")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub succeeded: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchDeletePartitionRequest {
    #[doc="<p>The ID of the Data Catalog where the partition to be deleted resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database in which the table in question resides.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>A list of <code>PartitionInput</code> structures that define the partitions to be deleted.</p>"]
    #[serde(rename="PartitionsToDelete")]
    pub partitions_to_delete: Vec<PartitionValueList>,
    #[doc="<p>The name of the table where the partitions to be deleted is located.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchDeletePartitionResponse {
    #[doc="<p>Errors encountered when trying to delete the requested partitions.</p>"]
    #[serde(rename="Errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<PartitionError>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchDeleteTableRequest {
    #[doc="<p>The ID of the Data Catalog where the table resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database where the tables to delete reside.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>A list of the table to delete.</p>"]
    #[serde(rename="TablesToDelete")]
    pub tables_to_delete: Vec<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchDeleteTableResponse {
    #[doc="<p>A list of errors encountered in attempting to delete the specified tables.</p>"]
    #[serde(rename="Errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<TableError>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchGetPartitionRequest {
    #[doc="<p>The ID of the Data Catalog where the partitions in question reside. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database where the partitions reside.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>A list of partition values identifying the partitions to retrieve.</p>"]
    #[serde(rename="PartitionsToGet")]
    pub partitions_to_get: Vec<PartitionValueList>,
    #[doc="<p>The name of the partitions' table.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchGetPartitionResponse {
    #[doc="<p>A list of the requested partitions.</p>"]
    #[serde(rename="Partitions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partitions: Option<Vec<Partition>>,
    #[doc="<p>A list of the partition values in the request for which partions were not returned.</p>"]
    #[serde(rename="UnprocessedKeys")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unprocessed_keys: Option<Vec<PartitionValueList>>,
}

#[doc="<p>Specifies a table definition in the Data Catalog.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CatalogEntry {
    #[doc="<p>The database in which the table metadata resides.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>The name of the table in question.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[doc="<p>A structure containing migration status information.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CatalogImportStatus {
    #[doc="<p>True if the migration has completed, or False otherwise.</p>"]
    #[serde(rename="ImportCompleted")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub import_completed: Option<bool>,
    #[doc="<p>The time that the migration was started.</p>"]
    #[serde(rename="ImportTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub import_time: Option<f64>,
    #[doc="<p>The name of the person who initiated the migration.</p>"]
    #[serde(rename="ImportedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub imported_by: Option<String>,
}

#[doc="<p>Classifiers are written in Python and triggered during a Crawl Task. You can write your own Classifiers to best categorize your data sources and specify the appropriate schemas to use for them. A Classifier first checks whether a given file is in a format it can handle, and then, if so, creates a schema in the form of a <code>StructType</code> object that matches that data format.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Classifier {
    #[doc="<p>A GrokClassifier object.</p>"]
    #[serde(rename="GrokClassifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub grok_classifier: Option<GrokClassifier>,
}

#[doc="<p>Represents a directional edge in a directed acyclic graph (DAG).</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CodeGenEdge {
    #[doc="<p>The ID of the node at which the edge starts.</p>"]
    #[serde(rename="Source")]
    pub source: String,
    #[doc="<p>The ID of the node at which the edge ends.</p>"]
    #[serde(rename="Target")]
    pub target: String,
    #[doc="<p>The target of the edge.</p>"]
    #[serde(rename="TargetParameter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_parameter: Option<String>,
}

#[doc="<p>Represents a node in a directed acyclic graph (DAG)</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CodeGenNode {
    #[doc="<p>Properties of the node, in the form of name-value pairs.</p>"]
    #[serde(rename="Args")]
    pub args: Vec<CodeGenNodeArg>,
    #[doc="<p>A node identifier that is unique within the node's graph.</p>"]
    #[serde(rename="Id")]
    pub id: String,
    #[doc="<p>The line number of the node.</p>"]
    #[serde(rename="LineNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub line_number: Option<i64>,
    #[doc="<p>The type of node this is.</p>"]
    #[serde(rename="NodeType")]
    pub node_type: String,
}

#[doc="<p>An argument or property of a node.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CodeGenNodeArg {
    #[doc="<p>The name of the argument or property.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>True if the value is used as a parameter.</p>"]
    #[serde(rename="Param")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub param: Option<bool>,
    #[doc="<p>The value of the argument or property.</p>"]
    #[serde(rename="Value")]
    pub value: String,
}

#[doc="<p>A column in a <code>Table</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Column {
    #[doc="<p>Free-form text comment.</p>"]
    #[serde(rename="Comment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment: Option<String>,
    #[doc="<p>The name of the <code>Column</code>.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The datatype of data in the <code>Column</code>.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Condition {
    #[serde(rename="JobName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename="LogicalOperator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub logical_operator: Option<String>,
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
}

#[doc="<p>Defines a connection to a data source.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Connection {
    #[doc="<p>A list of key-value pairs used as parameters for this connection.</p>"]
    #[serde(rename="ConnectionProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_properties: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The type of the connection.</p>"]
    #[serde(rename="ConnectionType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_type: Option<String>,
    #[doc="<p>The time this connection definition was created.</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>Description of the connection.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The user, group or role that last updated this connection definition.</p>"]
    #[serde(rename="LastUpdatedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated_by: Option<String>,
    #[doc="<p>The last time this connection definition was updated.</p>"]
    #[serde(rename="LastUpdatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[doc="<p>A list of criteria that can be used in selecting this connection.</p>"]
    #[serde(rename="MatchCriteria")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
    #[doc="<p>The name of the connection definition.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>A map of physical connection requirements, such as VPC and SecurityGroup, needed for making this connection successfully.</p>"]
    #[serde(rename="PhysicalConnectionRequirements")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub physical_connection_requirements: Option<PhysicalConnectionRequirements>,
}

#[doc="<p>A structure used to specify a connection to create or update.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ConnectionInput {
    #[doc="<p>A list of key-value pairs used as parameters for this connection.</p>"]
    #[serde(rename="ConnectionProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_properties: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The type of the connection.</p>"]
    #[serde(rename="ConnectionType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_type: Option<String>,
    #[doc="<p>Description of the connection.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>A list of criteria that can be used in selecting this connection.</p>"]
    #[serde(rename="MatchCriteria")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
    #[doc="<p>The name of the connection.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>A map of physical connection requirements, such as VPC and SecurityGroup, needed for making this connection successfully.</p>"]
    #[serde(rename="PhysicalConnectionRequirements")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub physical_connection_requirements: Option<PhysicalConnectionRequirements>,
}

#[doc="<p>Specifies the connections used by a job.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ConnectionsList {
    #[doc="<p>A list of connections used by the job.</p>"]
    #[serde(rename="Connections")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connections: Option<Vec<String>>,
}

#[doc="<p>Specifies a crawler program that examines a data source and uses classifiers to try to its schema. If successful, the crawler records metatdata concerning the data source in the Data Catalog.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Crawler {
    #[doc="<p>A list of custom <code>Classifier</code>s associated with this Crawler.</p>"]
    #[serde(rename="Classifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    #[doc="<p>If this Crawler is running, contains the total time elapsed since the last crawl began.</p>"]
    #[serde(rename="CrawlElapsedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub crawl_elapsed_time: Option<i64>,
    #[doc="<p>The time when the Crawler was created.</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>The <code>Database</code> where this Crawler's output should be stored.</p>"]
    #[serde(rename="DatabaseName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub database_name: Option<String>,
    #[doc="<p>A description of this Crawler and where it should be used.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The status of the last crawl, and potentially error information if an error occurred.</p>"]
    #[serde(rename="LastCrawl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_crawl: Option<LastCrawlInfo>,
    #[doc="<p>The time the Crawler was last updated.</p>"]
    #[serde(rename="LastUpdated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated: Option<f64>,
    #[doc="<p>The <code>Crawler</code> name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The ARN of an IAM role used to access customer resources such as data in S3.</p>"]
    #[serde(rename="Role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,
    #[doc="<p>A <code>Schedule</code> object that specifies the schedule on which this Crawler is to be run.</p>"]
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule: Option<Schedule>,
    #[doc="<p>Sets policy for the crawler's update and delete behavior.</p>"]
    #[serde(rename="SchemaChangePolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    #[doc="<p>Indicates whether this Crawler is running, or whether a run is pending.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[doc="<p>The table prefix used for catalog tables created.</p>"]
    #[serde(rename="TablePrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_prefix: Option<String>,
    #[doc="<p>A collection of targets to crawl.</p>"]
    #[serde(rename="Targets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub targets: Option<CrawlerTargets>,
    #[doc="<p>The version of the Crawler.</p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="<p>Metrics for a specified crawler.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CrawlerMetrics {
    #[doc="<p>The name of the crawler.</p>"]
    #[serde(rename="CrawlerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub crawler_name: Option<String>,
    #[doc="<p>The duration of the crawler's most recent run, in seconds.</p>"]
    #[serde(rename="LastRuntimeSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_runtime_seconds: Option<f64>,
    #[doc="<p>The median duration of this crawler's runs, in seconds.</p>"]
    #[serde(rename="MedianRuntimeSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub median_runtime_seconds: Option<f64>,
    #[doc="<p>True if the crawler is estimating its </p>"]
    #[serde(rename="StillEstimating")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub still_estimating: Option<bool>,
    #[doc="<p>A list of the tables created by this crawler.</p>"]
    #[serde(rename="TablesCreated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tables_created: Option<i64>,
    #[doc="<p>A list of the tables deleted by this crawler.</p>"]
    #[serde(rename="TablesDeleted")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tables_deleted: Option<i64>,
    #[doc="<p>A list of the tables created by this crawler.</p>"]
    #[serde(rename="TablesUpdated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tables_updated: Option<i64>,
    #[doc="<p>The estimated time left to complete a running crawl.</p>"]
    #[serde(rename="TimeLeftSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub time_left_seconds: Option<f64>,
}

#[doc="<p>Specifies crawler targets.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CrawlerTargets {
    #[doc="<p>Specifies JDBC targets.</p>"]
    #[serde(rename="JdbcTargets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jdbc_targets: Option<Vec<JdbcTarget>>,
    #[doc="<p>Specifies targets in AWS S3.</p>"]
    #[serde(rename="S3Targets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_targets: Option<Vec<S3Target>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateClassifierRequest {
    #[doc="<p>A grok classifier to create.</p>"]
    #[serde(rename="GrokClassifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub grok_classifier: Option<CreateGrokClassifierRequest>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateClassifierResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateConnectionRequest {
    #[doc="<p>The ID of the Data Catalog in which to create the connection. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>A <code>ConnectionInput</code> object defining the connection to create.</p>"]
    #[serde(rename="ConnectionInput")]
    pub connection_input: ConnectionInput,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateConnectionResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateCrawlerRequest {
    #[doc="<p>A list of custom <code>Classifier</code> names that the user has registered. By default, all AWS classifiers are included in a crawl, but these custom classifiers always override the default classifiers for a given classification.</p>"]
    #[serde(rename="Classifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    #[doc="<p>The Glue <code>Database</code> where results will be stored, such as: <code>arn:aws:daylight:us-east-1::database/sometable/*</code>.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>A description of the new <code>Crawler</code>.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Name of the new <code>Crawler</code>.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The AWS ARN of the IAM role used by the new <code>Crawler</code> to access customer resources.</p>"]
    #[serde(rename="Role")]
    pub role: String,
    #[doc="<p>A cron expression that can be used as a Cloudwatch event (see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html\">CloudWatch Schedule Expression Syntax</a>. For example, to run every day at 12:15 UTC, specify: <code>cron(15 12 * * ? *)</code>.</p>"]
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule: Option<String>,
    #[doc="<p>Policy for the crawler's update and deletion behavior.</p>"]
    #[serde(rename="SchemaChangePolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    #[doc="<p>The table prefix used for catalog tables created.</p>"]
    #[serde(rename="TablePrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_prefix: Option<String>,
    #[doc="<p>A list of collection of targets to crawl.</p>"]
    #[serde(rename="Targets")]
    pub targets: CrawlerTargets,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateCrawlerResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateDatabaseRequest {
    #[doc="<p>The ID of the Data Catalog in which to create the database. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>A <code>DatabaseInput</code> object defining the metadata database to create in the catalog.</p>"]
    #[serde(rename="DatabaseInput")]
    pub database_input: DatabaseInput,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateDatabaseResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateDevEndpointRequest {
    #[doc="<p>The name to be assigned to the new DevEndpoint.</p>"]
    #[serde(rename="EndpointName")]
    pub endpoint_name: String,
    #[doc="<p>Path to one or more Java Jars in an S3 bucket that should be loaded in your DevEndpoint.</p>"]
    #[serde(rename="ExtraJarsS3Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    #[doc="<p>Path to one or more Python libraries in an S3 bucket that should be loaded in your DevEndpoint.</p>"]
    #[serde(rename="ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    #[doc="<p>The number of nodes to use.</p>"]
    #[serde(rename="NumberOfNodes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number_of_nodes: Option<i64>,
    #[doc="<p>The public key to use for authentication.</p>"]
    #[serde(rename="PublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_key: Option<String>,
    #[doc="<p>The IAM role for the DevEndpoint.</p>"]
    #[serde(rename="RoleArn")]
    pub role_arn: String,
    #[doc="<p>Security group IDs for the security groups to be used by the new DevEndpoint.</p>"]
    #[serde(rename="SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[doc="<p>The subnet ID for the new DevEndpoint to use.</p>"]
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateDevEndpointResponse {
    #[doc="<p>The AWS availability zone where this DevEndpoint is located.</p>"]
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc="<p>The point in time at which this DevEndpoint was created.</p>"]
    #[serde(rename="CreatedTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[doc="<p>The name assigned to the new DevEndpoint.</p>"]
    #[serde(rename="EndpointName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_name: Option<String>,
    #[doc="<p>Path to one or more Java Jars in an S3 bucket that will be loaded in your DevEndpoint.</p>"]
    #[serde(rename="ExtraJarsS3Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    #[doc="<p>Path to one or more Python libraries in an S3 bucket that will be loaded in your DevEndpoint.</p>"]
    #[serde(rename="ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    #[doc="<p>The reason for a current failure in this DevEndpoint.</p>"]
    #[serde(rename="FailureReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failure_reason: Option<String>,
    #[doc="<p>The number of nodes in this DevEndpoint.</p>"]
    #[serde(rename="NumberOfNodes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number_of_nodes: Option<i64>,
    #[doc="<p>The AWS ARN of the role assigned to the new DevEndpoint.</p>"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="<p>The security groups assigned to the new DevEndpoint.</p>"]
    #[serde(rename="SecurityGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[doc="<p>The current status of the new DevEndpoint.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The subnet ID assigned to the new DevEndpoint.</p>"]
    #[serde(rename="SubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc="<p>The ID of the VPC used by this DevEndpoint.</p>"]
    #[serde(rename="VpcId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_id: Option<String>,
    #[doc="<p>The address of the YARN endpoint used by this DevEndpoint.</p>"]
    #[serde(rename="YarnEndpointAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub yarn_endpoint_address: Option<String>,
}

#[doc="<p>Specifies a Grok classifier for CreateClassifier to create.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateGrokClassifierRequest {
    #[doc="<p>The type of result that the classifier matches, such as Twitter Json, Omniture logs, Cloudwatch logs, and so forth.</p>"]
    #[serde(rename="Classification")]
    pub classification: String,
    #[doc="<p>Custom grok patterns used by this classifier.</p>"]
    #[serde(rename="CustomPatterns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_patterns: Option<String>,
    #[doc="<p>The grok pattern used by this classifier.</p>"]
    #[serde(rename="GrokPattern")]
    pub grok_pattern: String,
    #[doc="<p>The name of the new Classifier.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateJobRequest {
    #[doc="<p>The number of capacity units allocated to this job.</p>"]
    #[serde(rename="AllocatedCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allocated_capacity: Option<i64>,
    #[doc="<p>The JobCommand that executes this job.</p>"]
    #[serde(rename="Command")]
    pub command: JobCommand,
    #[doc="<p>The connections used for this job.</p>"]
    #[serde(rename="Connections")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connections: Option<ConnectionsList>,
    #[doc="<p>The default parameters for this job.</p>"]
    #[serde(rename="DefaultArguments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_arguments: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Description of the job.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>An ExecutionProperty specifying the maximum number of concurrent runs allowed for this job.</p>"]
    #[serde(rename="ExecutionProperty")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    #[doc="<p>Location of the logs for this job.</p>"]
    #[serde(rename="LogUri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_uri: Option<String>,
    #[doc="<p>The maximum number of times to retry this job if it fails.</p>"]
    #[serde(rename="MaxRetries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_retries: Option<i64>,
    #[doc="<p>The name you assign to this job.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The role associated with this job.</p>"]
    #[serde(rename="Role")]
    pub role: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateJobResponse {
    #[doc="<p>The unique name of the new job that has been created.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreatePartitionRequest {
    #[doc="<p>The ID of the catalog in which the partion is to be created. Currently, this should be the AWS account ID.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the metadata database in which the partition is to be created.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>A <code>PartitionInput</code> structure defining the partition to be created.</p>"]
    #[serde(rename="PartitionInput")]
    pub partition_input: PartitionInput,
    #[doc="<p>The name of the metadata table in which the partition is to be created.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreatePartitionResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateScriptRequest {
    #[doc="<p>A list of the edges in the DAG.</p>"]
    #[serde(rename="DagEdges")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dag_edges: Option<Vec<CodeGenEdge>>,
    #[doc="<p>A list of the nodes in the DAG.</p>"]
    #[serde(rename="DagNodes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dag_nodes: Option<Vec<CodeGenNode>>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateScriptResponse {
    #[doc="<p>The Python script generated from the DAG.</p>"]
    #[serde(rename="PythonScript")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub python_script: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateTableRequest {
    #[doc="<p>The ID of the Data Catalog in which to create the <code>Table</code>. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The catalog database in which to create the new table.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>The <code>TableInput</code> object that defines the metadata table to create in the catalog.</p>"]
    #[serde(rename="TableInput")]
    pub table_input: TableInput,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateTableResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateTriggerRequest {
    #[doc="<p>The actions initiated by this trigger when it fires.</p>"]
    #[serde(rename="Actions")]
    pub actions: Vec<Action>,
    #[doc="<p>A description of the new trigger.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The name to assign to the new trigger.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>A predicate to specify when the new trigger should fire.</p>"]
    #[serde(rename="Predicate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub predicate: Option<Predicate>,
    #[doc="<p>A cron schedule expression for the new trigger.</p>"]
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule: Option<String>,
    #[doc="<p>The type of the new trigger.</p>"]
    #[serde(rename="Type")]
    pub type_: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateTriggerResponse {
    #[doc="<p>The name assigned to the new trigger.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateUserDefinedFunctionRequest {
    #[doc="<p>The ID of the Data Catalog in which to create the function. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database in which to create the function.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>A <code>FunctionInput</code> object that defines the function to create in the Data Catalog.</p>"]
    #[serde(rename="FunctionInput")]
    pub function_input: UserDefinedFunctionInput,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateUserDefinedFunctionResponse;

#[doc="<p>The <code>Database</code> object represents a logical grouping of tables that may reside in a Hive metastore or an RDBMS.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Database {
    #[doc="<p>The time at which the metadata database was created in the catalog.</p>"]
    #[serde(rename="CreateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create_time: Option<f64>,
    #[doc="<p>Description of the database.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The location of the database (for example, an HDFS path).</p>"]
    #[serde(rename="LocationUri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location_uri: Option<String>,
    #[doc="<p>Name of the database.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>A list of key-value pairs that define parameters and properties of the database.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

#[doc="<p>The structure used to create or updata a database.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DatabaseInput {
    #[doc="<p>Description of the database</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The location of the database (for example, an HDFS path).</p>"]
    #[serde(rename="LocationUri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location_uri: Option<String>,
    #[doc="<p>Name of the database.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>A list of key-value pairs that define parameters and properties of the database.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteClassifierRequest {
    #[doc="<p>Name of the <code>Classifier</code> to remove.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteClassifierResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteConnectionRequest {
    #[doc="<p>The ID of the Data Catalog in which the connection resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the connection to delete.</p>"]
    #[serde(rename="ConnectionName")]
    pub connection_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteConnectionResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteCrawlerRequest {
    #[doc="<p>Name of the <code>Crawler</code> to remove.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteCrawlerResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteDatabaseRequest {
    #[doc="<p>The ID of the Data Catalog in which the database resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the Database to delete.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteDatabaseResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteDevEndpointRequest {
    #[doc="<p>The name of the DevEndpoint.</p>"]
    #[serde(rename="EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteDevEndpointResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteJobRequest {
    #[doc="<p>The name of the job to delete.</p>"]
    #[serde(rename="JobName")]
    pub job_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteJobResponse {
    #[doc="<p>The name of the job that was deleted.</p>"]
    #[serde(rename="JobName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeletePartitionRequest {
    #[doc="<p>The ID of the Data Catalog where the partition to be deleted resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database in which the table in question resides.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>The values that define the partition.</p>"]
    #[serde(rename="PartitionValues")]
    pub partition_values: Vec<String>,
    #[doc="<p>The name of the table where the partition to be deleted is located.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeletePartitionResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteTableRequest {
    #[doc="<p>The ID of the Data Catalog where the table resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database in which the table resides.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>The name of the table to be deleted.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteTableResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteTriggerRequest {
    #[doc="<p>The name of the trigger to delete.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteTriggerResponse {
    #[doc="<p>The name of the trigger that was deleted.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteUserDefinedFunctionRequest {
    #[doc="<p>The ID of the Data Catalog where the function to be deleted is located. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database where the function is located.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>The name of the function definition to be deleted.</p>"]
    #[serde(rename="FunctionName")]
    pub function_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteUserDefinedFunctionResponse;

#[doc="<p>A development endpoint where a developer can remotely debug ETL scripts.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DevEndpoint {
    #[doc="<p>The AWS availability zone where this DevEndpoint is located.</p>"]
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc="<p>The point in time at which this DevEndpoint was created.</p>"]
    #[serde(rename="CreatedTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[doc="<p>The name of the DevEndpoint.</p>"]
    #[serde(rename="EndpointName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_name: Option<String>,
    #[doc="<p>Path to one or more Java Jars in an S3 bucket that should be loaded in your DevEndpoint.</p>"]
    #[serde(rename="ExtraJarsS3Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    #[doc="<p>Path to one or more Python libraries in an S3 bucket that should be loaded in your DevEndpoint.</p>"]
    #[serde(rename="ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    #[doc="<p>The reason for a current failure in this DevEndpoint.</p>"]
    #[serde(rename="FailureReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failure_reason: Option<String>,
    #[doc="<p>The point in time at which this DevEndpoint was last modified.</p>"]
    #[serde(rename="LastModifiedTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_timestamp: Option<f64>,
    #[doc="<p>The status of the last update.</p>"]
    #[serde(rename="LastUpdateStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_update_status: Option<String>,
    #[doc="<p>The number of nodes used by this DevEndpoint.</p>"]
    #[serde(rename="NumberOfNodes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number_of_nodes: Option<i64>,
    #[doc="<p>The public address used by this DevEndpoint.</p>"]
    #[serde(rename="PublicAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_address: Option<String>,
    #[doc="<p>The public key to be used by this DevEndpoint for authentication.</p>"]
    #[serde(rename="PublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_key: Option<String>,
    #[doc="<p>The AWS ARN of the IAM role used in this DevEndpoint.</p>"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="<p>A list of security group identifiers used in this DevEndpoint.</p>"]
    #[serde(rename="SecurityGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[doc="<p>The current status of this DevEndpoint.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The subnet ID for this DevEndpoint.</p>"]
    #[serde(rename="SubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc="<p>The ID of the virtual private cloud (VPC) used by this DevEndpoint.</p>"]
    #[serde(rename="VpcId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_id: Option<String>,
    #[doc="<p>The YARN endpoint address used by this DevEndpoint.</p>"]
    #[serde(rename="YarnEndpointAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub yarn_endpoint_address: Option<String>,
}

#[doc="<p>Custom libraries to be loaded into a DevEndpoint.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DevEndpointCustomLibraries {
    #[doc="<p>Path to one or more Java Jars in an S3 bucket that should be loaded in your DevEndpoint.</p>"]
    #[serde(rename="ExtraJarsS3Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    #[doc="<p>Path to one or more Python libraries in an S3 bucket that should be loaded in your DevEndpoint.</p>"]
    #[serde(rename="ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
}

#[doc="<p>Contains details about an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ErrorDetail {
    #[doc="<p>The code associated with this error.</p>"]
    #[serde(rename="ErrorCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_code: Option<String>,
    #[doc="<p>A message describing the error.</p>"]
    #[serde(rename="ErrorMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<String>,
}

#[doc="<p>An execution property of a job.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ExecutionProperty {
    #[doc="<p>The maximum number of concurrent runs allowed for a job.</p>"]
    #[serde(rename="MaxConcurrentRuns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_concurrent_runs: Option<i64>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCatalogImportStatusRequest {
    #[doc="<p>The ID of the catalog to migrate. Currently, this should be the AWS account ID.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCatalogImportStatusResponse {
    #[doc="<p>The status of the specified catalog migration.</p>"]
    #[serde(rename="ImportStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub import_status: Option<CatalogImportStatus>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetClassifierRequest {
    #[doc="<p>Name of the <code>Classifier</code> to retrieve.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetClassifierResponse {
    #[doc="<p>The requested <code>Classifier</code>.</p>"]
    #[serde(rename="Classifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub classifier: Option<Classifier>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetClassifiersRequest {
    #[doc="<p>Size of the list to return (optional).</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>An optional continuation token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetClassifiersResponse {
    #[doc="<p>The requested list of <code>Classifier</code> objects.</p>"]
    #[serde(rename="Classifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub classifiers: Option<Vec<Classifier>>,
    #[doc="<p>A continuation token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetConnectionRequest {
    #[doc="<p>The ID of the Data Catalog in which the connection resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the connection definition to retrieve.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetConnectionResponse {
    #[doc="<p>The requested connection definition.</p>"]
    #[serde(rename="Connection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection: Option<Connection>,
}

#[doc="<p>Filters the connection definitions returned by the <code>GetConnections</code> API.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetConnectionsFilter {
    #[doc="<p>The type of connections to return.</p>"]
    #[serde(rename="ConnectionType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_type: Option<String>,
    #[doc="<p>A criteria string that must match the criteria recorded in the connection definition for that connection definition to be returned.</p>"]
    #[serde(rename="MatchCriteria")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetConnectionsRequest {
    #[doc="<p>The ID of the Data Catalog in which the connections reside. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>A filter that controls which connections will be returned.</p>"]
    #[serde(rename="Filter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter: Option<GetConnectionsFilter>,
    #[doc="<p>The maximum number of connections to return in one response.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, if this is a continuation call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetConnectionsResponse {
    #[doc="<p>A list of requested connection definitions.</p>"]
    #[serde(rename="ConnectionList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_list: Option<Vec<Connection>>,
    #[doc="<p>A continuation token, if the list of connections returned does not include the last of the filtered connections.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCrawlerMetricsRequest {
    #[doc="<p>A list of the names of crawlers about which to retrieve metrics.</p>"]
    #[serde(rename="CrawlerNameList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub crawler_name_list: Option<Vec<String>>,
    #[doc="<p>The maximum size of a list to return.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, if this is a continuation call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCrawlerMetricsResponse {
    #[doc="<p>A list of metrics for the specified crawler.</p>"]
    #[serde(rename="CrawlerMetricsList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub crawler_metrics_list: Option<Vec<CrawlerMetrics>>,
    #[doc="<p>A continuation token, if the returned list does not contain the last metric available.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCrawlerRequest {
    #[doc="<p>Name of the <code>Crawler</code> to retrieve metadata for.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCrawlerResponse {
    #[doc="<p>The metadata for the specified <code>Crawler</code>.</p>"]
    #[serde(rename="Crawler")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub crawler: Option<Crawler>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCrawlersRequest {
    #[doc="<p>The number of Crawlers to return on each call.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, if this is a continuation request.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCrawlersResponse {
    #[doc="<p>A list of <code>Crawler</code> metadata.</p>"]
    #[serde(rename="Crawlers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub crawlers: Option<Vec<Crawler>>,
    #[doc="<p>A continuation token, if the returned list has not reached the end of those defined in this customer account.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetDatabaseRequest {
    #[doc="<p>The ID of the Data Catalog in which the database resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the database to retrieve.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetDatabaseResponse {
    #[doc="<p>The definition of the specified database in the catalog.</p>"]
    #[serde(rename="Database")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub database: Option<Database>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetDatabasesRequest {
    #[doc="<p>The ID of the Data Catalog from which to retrieve <code>Databases</code>. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The maximum number of databases to return in one response.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, if this is a continuation call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetDatabasesResponse {
    #[doc="<p>A list of <code>Database</code> objects from the specified catalog.</p>"]
    #[serde(rename="DatabaseList")]
    pub database_list: Vec<Database>,
    #[doc="<p>A continuation token for paginating the returned list of tokens, returned if the current segment of the list is not the last.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetDataflowGraphRequest {
    #[doc="<p>The Python script to transform.</p>"]
    #[serde(rename="PythonScript")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub python_script: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetDataflowGraphResponse {
    #[doc="<p>A list of the edges in the resulting DAG.</p>"]
    #[serde(rename="DagEdges")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dag_edges: Option<Vec<CodeGenEdge>>,
    #[doc="<p>A list of the nodes in the resulting DAG.</p>"]
    #[serde(rename="DagNodes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dag_nodes: Option<Vec<CodeGenNode>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetDevEndpointRequest {
    #[doc="<p>Name of the DevEndpoint for which to retrieve information.</p>"]
    #[serde(rename="EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetDevEndpointResponse {
    #[doc="<p>A DevEndpoint definition.</p>"]
    #[serde(rename="DevEndpoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dev_endpoint: Option<DevEndpoint>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetDevEndpointsRequest {
    #[doc="<p>The maximum size of information to return.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, if this is a continuation call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetDevEndpointsResponse {
    #[doc="<p>A list of DevEndpoint definitions.</p>"]
    #[serde(rename="DevEndpoints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dev_endpoints: Option<Vec<DevEndpoint>>,
    #[doc="<p>A continuation token, if not all DevEndpoint definitions have yet been returned.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetJobRequest {
    #[doc="<p>The name of the job to retrieve.</p>"]
    #[serde(rename="JobName")]
    pub job_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetJobResponse {
    #[doc="<p>The requested job definition.</p>"]
    #[serde(rename="Job")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetJobRunRequest {
    #[doc="<p>Name of the job being run.</p>"]
    #[serde(rename="JobName")]
    pub job_name: String,
    #[doc="<p>A list of the predecessor runs to return as well.</p>"]
    #[serde(rename="PredecessorsIncluded")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub predecessors_included: Option<bool>,
    #[doc="<p>The ID of the job run.</p>"]
    #[serde(rename="RunId")]
    pub run_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetJobRunResponse {
    #[doc="<p>The requested job-run metadata.</p>"]
    #[serde(rename="JobRun")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_run: Option<JobRun>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetJobRunsRequest {
    #[doc="<p>The name of the job for which to retrieve all job runs.</p>"]
    #[serde(rename="JobName")]
    pub job_name: String,
    #[doc="<p>The maximum size of the response.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, if this is a continuation call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetJobRunsResponse {
    #[doc="<p>A list of job-run metatdata objects.</p>"]
    #[serde(rename="JobRuns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_runs: Option<Vec<JobRun>>,
    #[doc="<p>A continuation token, if not all reequested job runs have been returned.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetJobsRequest {
    #[doc="<p>The maximum size of the response.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, if this is a continuation call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetJobsResponse {
    #[doc="<p>A list of jobs.</p>"]
    #[serde(rename="Jobs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    #[doc="<p>A continuation token, if not all jobs have yet been returned.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetMappingRequest {
    #[doc="<p>Parameters for the mapping.</p>"]
    #[serde(rename="Location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<Location>,
    #[doc="<p>A list of target tables.</p>"]
    #[serde(rename="Sinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sinks: Option<Vec<CatalogEntry>>,
    #[doc="<p>Specifies the source table.</p>"]
    #[serde(rename="Source")]
    pub source: CatalogEntry,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetMappingResponse {
    #[doc="<p>A list of mappings to the specified targets.</p>"]
    #[serde(rename="Mapping")]
    pub mapping: Vec<MappingEntry>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetPartitionRequest {
    #[doc="<p>The ID of the Data Catalog where the partition in question resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database where the partition resides.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>The values that define the partition.</p>"]
    #[serde(rename="PartitionValues")]
    pub partition_values: Vec<String>,
    #[doc="<p>The name of the partition's table.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetPartitionResponse {
    #[doc="<p>The requested information, in the form of a <code>Partition</code> object.</p>"]
    #[serde(rename="Partition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition: Option<Partition>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetPartitionsRequest {
    #[doc="<p>The ID of the Data Catalog where the partitions in question reside. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database where the partitions reside.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>An expression filtering the partitions to be returned.</p>"]
    #[serde(rename="Expression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expression: Option<String>,
    #[doc="<p>The maximum number of partitions to return in a single response.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, if this is not the first call to retrieve these partitions.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The segment of the table's partitions to scan in this request.</p>"]
    #[serde(rename="Segment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment: Option<Segment>,
    #[doc="<p>The name of the partitions' table.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetPartitionsResponse {
    #[doc="<p>A continuation token, if the returned list of partitions does not does not include the last one.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of requested partitions.</p>"]
    #[serde(rename="Partitions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partitions: Option<Vec<Partition>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetPlanRequest {
    #[doc="<p>Parameters for the mapping.</p>"]
    #[serde(rename="Location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<Location>,
    #[doc="<p>The list of mappings from a source table to target tables.</p>"]
    #[serde(rename="Mapping")]
    pub mapping: Vec<MappingEntry>,
    #[doc="<p>The target tables.</p>"]
    #[serde(rename="Sinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sinks: Option<Vec<CatalogEntry>>,
    #[doc="<p>The source table.</p>"]
    #[serde(rename="Source")]
    pub source: CatalogEntry,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetPlanResponse {
    #[doc="<p>A python script to perform the mapping.</p>"]
    #[serde(rename="PythonScript")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub python_script: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetTableRequest {
    #[doc="<p>The ID of the Data Catalog where the table resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the database in the catalog in which the table resides.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>The name of the table for which to retrieve the definition.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetTableResponse {
    #[doc="<p>The <code>Table</code> object that defines the specified table.</p>"]
    #[serde(rename="Table")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table: Option<Table>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetTableVersionsRequest {
    #[doc="<p>The ID of the Data Catalog where the tables reside. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The database in the catalog in which the table resides.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>The maximum number of table versions to return in one response.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, if this is not the first call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The name of the table.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetTableVersionsResponse {
    #[doc="<p>A continuation token, if the list of available versions does not include the last one.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of strings identifying available versions of the specified table.</p>"]
    #[serde(rename="TableVersions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_versions: Option<Vec<TableVersion>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetTablesRequest {
    #[doc="<p>The ID of the Data Catalog where the tables reside. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The database in the catalog whose tables to list.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>A regular expression pattern. If present, only those tables whose names match the pattern are returned.</p>"]
    #[serde(rename="Expression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expression: Option<String>,
    #[doc="<p>The maximum number of tables to return in a single response.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, included if this is a continuation call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetTablesResponse {
    #[doc="<p>A continuation token, present if the current list segment is not the last.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of the requested <code>Table</code> objects.</p>"]
    #[serde(rename="TableList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_list: Option<Vec<Table>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetTriggerRequest {
    #[doc="<p>The name of the trigger to retrieve.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetTriggerResponse {
    #[doc="<p>The requested trigger definition.</p>"]
    #[serde(rename="Trigger")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trigger: Option<Trigger>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetTriggersRequest {
    #[doc="<p>The name of the job for which to retrieve triggers.</p>"]
    #[serde(rename="DependentJobName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dependent_job_name: Option<String>,
    #[doc="<p>The maximum size of the response.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, if this is a continuation call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetTriggersResponse {
    #[doc="<p>A continuation token, if not all the requested triggers have yet been returned.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of triggers for the specified job.</p>"]
    #[serde(rename="Triggers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub triggers: Option<Vec<Trigger>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetUserDefinedFunctionRequest {
    #[doc="<p>The ID of the Data Catalog where the function to be retrieved is located. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database where the function is located.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>The name of the function.</p>"]
    #[serde(rename="FunctionName")]
    pub function_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetUserDefinedFunctionResponse {
    #[doc="<p>The requested function definition.</p>"]
    #[serde(rename="UserDefinedFunction")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_defined_function: Option<UserDefinedFunction>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetUserDefinedFunctionsRequest {
    #[doc="<p>The ID of the Data Catalog where the functions to be retrieved are located. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database where the functions are located.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>The maximum number of functions to return in one response.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A continuation token, if this is a continuation call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>An optional function-name pattern string that filters the function definitions returned.</p>"]
    #[serde(rename="Pattern")]
    pub pattern: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetUserDefinedFunctionsResponse {
    #[doc="<p>A continuation token, if the list of functions returned does not include the last requested function.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of requested function definitions.</p>"]
    #[serde(rename="UserDefinedFunctions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_defined_functions: Option<Vec<UserDefinedFunction>>,
}

#[doc="<p>A classifier that uses <code>grok</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GrokClassifier {
    #[doc="<p>The data form that the classifier matches, such as Twitter, JSON, Omniture Logs, and so forth.</p>"]
    #[serde(rename="Classification")]
    pub classification: String,
    #[doc="<p>The time this classifier was registered.</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>Custom grok patterns used by this classifier.</p>"]
    #[serde(rename="CustomPatterns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_patterns: Option<String>,
    #[doc="<p>The grok pattern used by this classifier.</p>"]
    #[serde(rename="GrokPattern")]
    pub grok_pattern: String,
    #[doc="<p>The time this classifier was last updated.</p>"]
    #[serde(rename="LastUpdated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated: Option<f64>,
    #[doc="<p>The name of the classifier.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The version of this classifier.</p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ImportCatalogToGlueRequest {
    #[doc="<p>The ID of the catalog to import. Currently, this should be the AWS account ID.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ImportCatalogToGlueResponse;

#[doc="<p>Specifies a JDBC target for a crawl.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct JdbcTarget {
    #[doc="<p>The name of the connection to use for the JDBC target.</p>"]
    #[serde(rename="ConnectionName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_name: Option<String>,
    #[doc="<p>A list of items to exclude from the crawl.</p>"]
    #[serde(rename="Exclusions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    #[doc="<p>The path of the JDBC target.</p>"]
    #[serde(rename="Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
}

#[doc="<p>Specifies a job in the Data Catalog.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Job {
    #[doc="<p>The number of capacity units allocated to this job.</p>"]
    #[serde(rename="AllocatedCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allocated_capacity: Option<i64>,
    #[doc="<p>The JobCommand that executes this job.</p>"]
    #[serde(rename="Command")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub command: Option<JobCommand>,
    #[doc="<p>The connections used for this job.</p>"]
    #[serde(rename="Connections")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connections: Option<ConnectionsList>,
    #[doc="<p>The time and date that this job specification was created.</p>"]
    #[serde(rename="CreatedOn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_on: Option<f64>,
    #[doc="<p>The default parameters for this job.</p>"]
    #[serde(rename="DefaultArguments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_arguments: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Description of this job.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>An ExecutionProperty specifying the maximum number of concurrent runs allowed for this job.</p>"]
    #[serde(rename="ExecutionProperty")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    #[doc="<p>The last point in time when this job specification was modified.</p>"]
    #[serde(rename="LastModifiedOn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[doc="<p>Location of the logs for this job.</p>"]
    #[serde(rename="LogUri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_uri: Option<String>,
    #[doc="<p>The maximum number of times to retry this job if it fails.</p>"]
    #[serde(rename="MaxRetries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_retries: Option<i64>,
    #[doc="<p>The name you assign to this job.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The role associated with this job.</p>"]
    #[serde(rename="Role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,
}

#[doc="<p>Defines a point which a job can resume processing.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct JobBookmarkEntry {
    #[doc="<p>The attempt ID number.</p>"]
    #[serde(rename="Attempt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attempt: Option<i64>,
    #[doc="<p>The bookmark itself.</p>"]
    #[serde(rename="JobBookmark")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_bookmark: Option<String>,
    #[doc="<p>Name of the job in question.</p>"]
    #[serde(rename="JobName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_name: Option<String>,
    #[doc="<p>The run ID number.</p>"]
    #[serde(rename="Run")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub run: Option<i64>,
    #[doc="<p>Version of the job.</p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="<p>Specifies code that executes a job.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct JobCommand {
    #[doc="<p>The name of this job command.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Specifies the location of a script that executes a job.</p>"]
    #[serde(rename="ScriptLocation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub script_location: Option<String>,
}

#[doc="<p>Contains information about a job run.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct JobRun {
    #[doc="<p>The amount of infrastructure capacity allocated to this job run.</p>"]
    #[serde(rename="AllocatedCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allocated_capacity: Option<i64>,
    #[doc="<p>The job arguments associated with this run.</p>"]
    #[serde(rename="Arguments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The number or the attempt to run this job.</p>"]
    #[serde(rename="Attempt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attempt: Option<i64>,
    #[doc="<p>The date and time this job run completed.</p>"]
    #[serde(rename="CompletedOn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_on: Option<f64>,
    #[doc="<p>An error message associated with this job run.</p>"]
    #[serde(rename="ErrorMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<String>,
    #[doc="<p>The ID of this job run.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>The name of the job being run.</p>"]
    #[serde(rename="JobName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_name: Option<String>,
    #[doc="<p>The current state of the job run.</p>"]
    #[serde(rename="JobRunState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_run_state: Option<String>,
    #[doc="<p>The last time this job run was modified.</p>"]
    #[serde(rename="LastModifiedOn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[doc="<p>A list of predecessors to this job run.</p>"]
    #[serde(rename="PredecessorRuns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub predecessor_runs: Option<Vec<Predecessor>>,
    #[doc="<p>The ID of the previous run of this job.</p>"]
    #[serde(rename="PreviousRunId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous_run_id: Option<String>,
    #[doc="<p>The date and time at which this job run was started.</p>"]
    #[serde(rename="StartedOn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_on: Option<f64>,
    #[doc="<p>The name of the trigger for this job run.</p>"]
    #[serde(rename="TriggerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trigger_name: Option<String>,
}

#[doc="<p>Specifies information used to update an existing job.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct JobUpdate {
    #[doc="<p>The number of capacity units allocated to this job.</p>"]
    #[serde(rename="AllocatedCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allocated_capacity: Option<i64>,
    #[doc="<p>The JobCommand that executes this job.</p>"]
    #[serde(rename="Command")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub command: Option<JobCommand>,
    #[doc="<p>The connections used for this job.</p>"]
    #[serde(rename="Connections")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connections: Option<ConnectionsList>,
    #[doc="<p>The default parameters for this job.</p>"]
    #[serde(rename="DefaultArguments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_arguments: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Description of the job.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>An ExecutionProperty specifying the maximum number of concurrent runs allowed for this job.</p>"]
    #[serde(rename="ExecutionProperty")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    #[doc="<p>Location of the logs for this job.</p>"]
    #[serde(rename="LogUri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_uri: Option<String>,
    #[doc="<p>The maximum number of times to retry this job if it fails.</p>"]
    #[serde(rename="MaxRetries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_retries: Option<i64>,
    #[doc="<p>The role associated with this job.</p>"]
    #[serde(rename="Role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,
}

#[doc="<p>Status and error information about the most recent crawl.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct LastCrawlInfo {
    #[doc="<p>Error information about the last crawl, if an error occurred.</p>"]
    #[serde(rename="ErrorMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<String>,
    #[doc="<p>The log group for the last crawl.</p>"]
    #[serde(rename="LogGroup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_group: Option<String>,
    #[doc="<p>The log stream for the last crawl.</p>"]
    #[serde(rename="LogStream")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_stream: Option<String>,
    #[doc="<p>The prefix for a message about this crawl.</p>"]
    #[serde(rename="MessagePrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_prefix: Option<String>,
    #[doc="<p>The time at which the crawl started.</p>"]
    #[serde(rename="StartTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_time: Option<f64>,
    #[doc="<p>Status of the last crawl.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[doc="<p>The location of resources.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct Location {
    #[doc="<p>A JDBC location.</p>"]
    #[serde(rename="Jdbc")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jdbc: Option<Vec<CodeGenNodeArg>>,
    #[doc="<p>An AWS S3 location.</p>"]
    #[serde(rename="S3")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3: Option<Vec<CodeGenNodeArg>>,
}

#[doc="<p>Defines a mapping.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct MappingEntry {
    #[doc="<p>The source path.</p>"]
    #[serde(rename="SourcePath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_path: Option<String>,
    #[doc="<p>The name of the source table.</p>"]
    #[serde(rename="SourceTable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_table: Option<String>,
    #[doc="<p>The source type.</p>"]
    #[serde(rename="SourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_type: Option<String>,
    #[doc="<p>The target path.</p>"]
    #[serde(rename="TargetPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_path: Option<String>,
    #[doc="<p>The target table.</p>"]
    #[serde(rename="TargetTable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_table: Option<String>,
    #[doc="<p>The target type.</p>"]
    #[serde(rename="TargetType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_type: Option<String>,
}

#[doc="<p>Specifies the sort order of a sorted column.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Order {
    #[doc="<p>The name of the column.</p>"]
    #[serde(rename="Column")]
    pub column: String,
    #[doc="<p>Indicates that the column is sorted in ascending order (<code>== 1</code>), or in descending order (<code>==0</code>).</p>"]
    #[serde(rename="SortOrder")]
    pub sort_order: i64,
}

#[doc="<p>Represents a slice of table data.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Partition {
    #[doc="<p>The time at which the partition was created.</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>The name of the catalog database where the table in question is located.</p>"]
    #[serde(rename="DatabaseName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub database_name: Option<String>,
    #[doc="<p>The last time at which the partition was accessed.</p>"]
    #[serde(rename="LastAccessTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_access_time: Option<f64>,
    #[doc="<p>The last time at which column statistics were computed for this partition.</p>"]
    #[serde(rename="LastAnalyzedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    #[doc="<p>Partition parameters, in the form of a list of key-value pairs.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Provides information about the physical location where the partition is stored.</p>"]
    #[serde(rename="StorageDescriptor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    #[doc="<p>The name of the table in question.</p>"]
    #[serde(rename="TableName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_name: Option<String>,
    #[doc="<p>The values of the partition.</p>"]
    #[serde(rename="Values")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[doc="<p>Contains information about a partition error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PartitionError {
    #[doc="<p>Details about the partition error.</p>"]
    #[serde(rename="ErrorDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[doc="<p>The values that define the partition.</p>"]
    #[serde(rename="PartitionValues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition_values: Option<Vec<String>>,
}

#[doc="<p>The structure used to create and update a partion.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PartitionInput {
    #[doc="<p>The last time at which the partition was accessed.</p>"]
    #[serde(rename="LastAccessTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_access_time: Option<f64>,
    #[doc="<p>The last time at which column statistics were computed for this partition.</p>"]
    #[serde(rename="LastAnalyzedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    #[doc="<p>Partition parameters, in the form of a list of key-value pairs.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Provides information about the physical location where the partition is stored.</p>"]
    #[serde(rename="StorageDescriptor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    #[doc="<p>The values of the partition.</p>"]
    #[serde(rename="Values")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PartitionValueList {
    #[serde(rename="Values")]
    pub values: Vec<String>,
}

#[doc="<p>Specifies the physical requirements for a connection.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PhysicalConnectionRequirements {
    #[doc="<p>The connection's availability zone.</p>"]
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc="<p>The security group ID list used by the connection.</p>"]
    #[serde(rename="SecurityGroupIdList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_group_id_list: Option<Vec<String>>,
    #[doc="<p>The subnet ID used by the connection.</p>"]
    #[serde(rename="SubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_id: Option<String>,
}

#[doc="<p>A job run that preceded this one.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Predecessor {
    #[doc="<p>The name of the predecessor job.</p>"]
    #[serde(rename="JobName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_name: Option<String>,
    #[doc="<p>The job-run ID of the precessor job run.</p>"]
    #[serde(rename="RunId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub run_id: Option<String>,
}

#[doc="<p>Defines the predicate of the trigger, which determines when it fires.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Predicate {
    #[doc="<p>A list of the conditions that determine when the trigger will fire.</p>"]
    #[serde(rename="Conditions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[doc="<p>Currently \"OR\" is not supported.</p>"]
    #[serde(rename="Logical")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub logical: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ResetJobBookmarkRequest {
    #[doc="<p>The name of the job in question.</p>"]
    #[serde(rename="JobName")]
    pub job_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ResetJobBookmarkResponse {
    #[doc="<p>The reset bookmark entry.</p>"]
    #[serde(rename="JobBookmarkEntry")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_bookmark_entry: Option<JobBookmarkEntry>,
}

#[doc="<p>URIs for function resources.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ResourceUri {
    #[doc="<p>The type of the resource.</p>"]
    #[serde(rename="ResourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_type: Option<String>,
    #[doc="<p>The URI for accessing the resource.</p>"]
    #[serde(rename="Uri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uri: Option<String>,
}

#[doc="<p>Specifies a crawler target in AWS S3.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct S3Target {
    #[doc="<p>A list of S3 objects to exclude from the crawl.</p>"]
    #[serde(rename="Exclusions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    #[doc="<p>The path to the S3 target.</p>"]
    #[serde(rename="Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
}

#[doc="<p>A scheduling object using a <code>cron</code> statement to schedule an event.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Schedule {
    #[doc="<p>A <code>cron</code> expression that can be used as a Cloudwatch event to schedule something (see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html\">CloudWatch Schedule Expression Syntax</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>"]
    #[serde(rename="ScheduleExpression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule_expression: Option<String>,
    #[doc="<p>The state of the schedule.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
}

#[doc="<p>Crawler policy for update and deletion behavior.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SchemaChangePolicy {
    #[doc="<p>The deletion behavior.</p>"]
    #[serde(rename="DeleteBehavior")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_behavior: Option<String>,
    #[doc="<p>The update behavior.</p>"]
    #[serde(rename="UpdateBehavior")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub update_behavior: Option<String>,
}

#[doc="<p>Defines a non-overlapping region of a table's partitions, allowing multiple requests to be executed in parallel.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct Segment {
    #[doc="<p>The zero-based index number of the this segment. For example, if the total number of segments is 4, SegmentNumber values will range from zero through three.</p>"]
    #[serde(rename="SegmentNumber")]
    pub segment_number: i64,
    #[doc="<p>The total numer of segments.</p>"]
    #[serde(rename="TotalSegments")]
    pub total_segments: i64,
}

#[doc="<p>Information about a serialization/deserialization program (SerDe) which serves as an extractor and loader.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SerDeInfo {
    #[doc="<p>Name of the SerDe.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>A list of initialization parameters for the SerDe, in key-value form.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Usually the class that implements the SerDe. An example is: <code>org.apache.hadoop.hive.serde2.columnar.ColumnarSerDe</code>.</p>"]
    #[serde(rename="SerializationLibrary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub serialization_library: Option<String>,
}

#[doc="<p>Specifies skewed values in a table. Skewed are ones that occur with very high frequency.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SkewedInfo {
    #[doc="<p>A list of names of columns that contain skewed values.</p>"]
    #[serde(rename="SkewedColumnNames")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub skewed_column_names: Option<Vec<String>>,
    #[doc="<p>A mapping of skewed values to the columns that contain them.</p>"]
    #[serde(rename="SkewedColumnValueLocationMaps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub skewed_column_value_location_maps: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>A list of values that appear so frequently as to be considered skewed.</p>"]
    #[serde(rename="SkewedColumnValues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub skewed_column_values: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct StartCrawlerRequest {
    #[doc="<p>Name of the <code>Crawler</code> to start.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartCrawlerResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct StartCrawlerScheduleRequest {
    #[doc="<p>Name of the crawler to schedule.</p>"]
    #[serde(rename="CrawlerName")]
    pub crawler_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartCrawlerScheduleResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct StartJobRunRequest {
    #[doc="<p>The infrastructure capacity to allocate to this job.</p>"]
    #[serde(rename="AllocatedCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allocated_capacity: Option<i64>,
    #[doc="<p>Specific arguments for this job run.</p>"]
    #[serde(rename="Arguments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The name of the job to start.</p>"]
    #[serde(rename="JobName")]
    pub job_name: String,
    #[doc="<p>The ID of the job run to start.</p>"]
    #[serde(rename="JobRunId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartJobRunResponse {
    #[doc="<p>The ID assigned to this job run.</p>"]
    #[serde(rename="JobRunId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct StartTriggerRequest {
    #[doc="<p>The name of the trigger to start.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartTriggerResponse {
    #[doc="<p>The name of the trigger that was started.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct StopCrawlerRequest {
    #[doc="<p>Name of the <code>Crawler</code> to stop.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StopCrawlerResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct StopCrawlerScheduleRequest {
    #[doc="<p>Name of the crawler whose schedule state to set.</p>"]
    #[serde(rename="CrawlerName")]
    pub crawler_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StopCrawlerScheduleResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct StopTriggerRequest {
    #[doc="<p>The name of the trigger to stop.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StopTriggerResponse {
    #[doc="<p>The name of the trigger that was stopped.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[doc="<p>Describes the physical storage of table data.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct StorageDescriptor {
    #[doc="<p>A list of reducer grouping columns, clustering columns, and bucketing columns in the table.</p>"]
    #[serde(rename="BucketColumns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bucket_columns: Option<Vec<String>>,
    #[doc="<p>A list of the <code>Columns</code> in the table.</p>"]
    #[serde(rename="Columns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub columns: Option<Vec<Column>>,
    #[doc="<p>True if the data in the table is compressed, or False if not.</p>"]
    #[serde(rename="Compressed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compressed: Option<bool>,
    #[doc="<p>The input format: <code>SequenceFileInputFormat</code> (binary), or <code>TextInputFormat</code>, or a custom format.</p>"]
    #[serde(rename="InputFormat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_format: Option<String>,
    #[doc="<p>The physical location of the table. By default this takes the form of the warehouse location, followed by the database location in the warehouse, followed by the table name.</p>"]
    #[serde(rename="Location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
    #[doc="<p>Must be specified if the table contains any dimension columns.</p>"]
    #[serde(rename="NumberOfBuckets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number_of_buckets: Option<i64>,
    #[doc="<p>The output format: <code>SequenceFileOutputFormat</code> (binary), or <code>IgnoreKeyTextOutputFormat</code>, or a custom format.</p>"]
    #[serde(rename="OutputFormat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_format: Option<String>,
    #[doc="<p>User-supplied properties in key-value form.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Serialization/deserialization (SerDe) information.</p>"]
    #[serde(rename="SerdeInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub serde_info: Option<SerDeInfo>,
    #[doc="<p>Information about values that appear very frequently in a column (skewed values).</p>"]
    #[serde(rename="SkewedInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub skewed_info: Option<SkewedInfo>,
    #[doc="<p>A list specifying the sort order of each bucket in the table.</p>"]
    #[serde(rename="SortColumns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_columns: Option<Vec<Order>>,
    #[doc="<p>True if the table data is stored in subdirectories, or False if not.</p>"]
    #[serde(rename="StoredAsSubDirectories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stored_as_sub_directories: Option<bool>,
}

#[doc="<p>Represents a collection of related data organized in columns and rows.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Table {
    #[doc="<p>Time when the table definition was created in the Data Catalog.</p>"]
    #[serde(rename="CreateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create_time: Option<f64>,
    #[doc="<p>Person or entity who created the table.</p>"]
    #[serde(rename="CreatedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_by: Option<String>,
    #[doc="<p>Name of the metadata database where the table metadata resides.</p>"]
    #[serde(rename="DatabaseName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub database_name: Option<String>,
    #[doc="<p>Description of the table.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Last time the table was accessed. This is usually taken from HDFS, and may not be reliable.</p>"]
    #[serde(rename="LastAccessTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_access_time: Option<f64>,
    #[doc="<p>Last time column statistics were computed for this table.</p>"]
    #[serde(rename="LastAnalyzedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    #[doc="<p>Name of the table.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>Owner of the table.</p>"]
    #[serde(rename="Owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<String>,
    #[doc="<p>Properties associated with this table, as a list of key-value pairs.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>A list of columns by which the table is partitioned. Only primitive types are supported as partition keys.</p>"]
    #[serde(rename="PartitionKeys")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition_keys: Option<Vec<Column>>,
    #[doc="<p>Retention time for this table.</p>"]
    #[serde(rename="Retention")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retention: Option<i64>,
    #[doc="<p>A storage descriptor containing information about the physical storage of this table.</p>"]
    #[serde(rename="StorageDescriptor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    #[doc="<p>The type of this table (<code>EXTERNAL_TABLE</code>, <code>VIRTUAL_VIEW</code>, etc.).</p>"]
    #[serde(rename="TableType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_type: Option<String>,
    #[doc="<p>Last time the table was updated.</p>"]
    #[serde(rename="UpdateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub update_time: Option<f64>,
    #[doc="<p>If the table is a view, the expanded text of the view; otherwise <code>null</code>.</p>"]
    #[serde(rename="ViewExpandedText")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub view_expanded_text: Option<String>,
    #[doc="<p>If the table is a view, the original text of the view; otherwise <code>null</code>.</p>"]
    #[serde(rename="ViewOriginalText")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub view_original_text: Option<String>,
}

#[doc="<p>An error record for table operations.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TableError {
    #[doc="<p>Detail about the error.</p>"]
    #[serde(rename="ErrorDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[doc="<p>Name of the table.</p>"]
    #[serde(rename="TableName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_name: Option<String>,
}

#[doc="<p>Structure used to create or update the table.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TableInput {
    #[doc="<p>Description of the table.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Last time the table was accessed.</p>"]
    #[serde(rename="LastAccessTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_access_time: Option<f64>,
    #[doc="<p>Last time column statistics were computed for this table.</p>"]
    #[serde(rename="LastAnalyzedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    #[doc="<p>Name of the table.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>Owner of the table.</p>"]
    #[serde(rename="Owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<String>,
    #[doc="<p>Properties associated with this table, as a list of key-value pairs.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>A list of columns by which the table is partitioned. Only primitive types are supported as partition keys.</p>"]
    #[serde(rename="PartitionKeys")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition_keys: Option<Vec<Column>>,
    #[doc="<p>Retention time for this table.</p>"]
    #[serde(rename="Retention")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retention: Option<i64>,
    #[doc="<p>A storage descriptor containing information about the physical storage of this table.</p>"]
    #[serde(rename="StorageDescriptor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    #[doc="<p>The type of this table (<code>EXTERNAL_TABLE</code>, <code>VIRTUAL_VIEW</code>, etc.).</p>"]
    #[serde(rename="TableType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_type: Option<String>,
    #[doc="<p>If the table is a view, the expanded text of the view; otherwise <code>null</code>.</p>"]
    #[serde(rename="ViewExpandedText")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub view_expanded_text: Option<String>,
    #[doc="<p>If the table is a view, the original text of the view; otherwise <code>null</code>.</p>"]
    #[serde(rename="ViewOriginalText")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub view_original_text: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct TableVersion {
    #[serde(rename="Table")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table: Option<Table>,
    #[serde(rename="VersionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version_id: Option<String>,
}

#[doc="<p>Information about a specific trigger.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Trigger {
    #[doc="<p>The actions initiated by this trigger.</p>"]
    #[serde(rename="Actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[doc="<p>A description of this trigger.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The trigger ID.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>Name of the trigger.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The predicate of this trigger.</p>"]
    #[serde(rename="Predicate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub predicate: Option<Predicate>,
    #[doc="<p>A cron schedule expression.</p>"]
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule: Option<String>,
    #[doc="<p>The current state of the trigger.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[doc="<p>The type of trigger that this is.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[doc="<p>A structure used to provide information used to updata a trigger.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TriggerUpdate {
    #[doc="<p>The actions initiated by this trigger.</p>"]
    #[serde(rename="Actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[doc="<p>A description of this trigger.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The name of the trigger.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The predicate of this trigger, which defines when it will fire.</p>"]
    #[serde(rename="Predicate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub predicate: Option<Predicate>,
    #[doc="<p>A cron expression specifying the schedule.</p>"]
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateClassifierRequest {
    #[doc="<p>A <code>GrokClassifier</code> object with updated fields.</p>"]
    #[serde(rename="GrokClassifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub grok_classifier: Option<UpdateGrokClassifierRequest>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateClassifierResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateConnectionRequest {
    #[doc="<p>The ID of the Data Catalog in which the connection resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>A <code>ConnectionInput</code> object that redefines the connection in question.</p>"]
    #[serde(rename="ConnectionInput")]
    pub connection_input: ConnectionInput,
    #[doc="<p>The name of the connection definition to update.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateConnectionResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateCrawlerRequest {
    #[doc="<p>A list of custom <code>Classifier</code> names that the user has registered. By default, all AWS classifiers are included in a crawl, but these custom classifiers always override the default classifiers for a given classification.</p>"]
    #[serde(rename="Classifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    #[doc="<p>The Glue <code>Database</code> where results will be stored, such as: <code>arn:aws:daylight:us-east-1::database/sometable/*</code>.</p>"]
    #[serde(rename="DatabaseName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub database_name: Option<String>,
    #[doc="<p>A description of the new <code>Crawler</code>.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Name of the new <code>Crawler</code>.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The AWS ARN of the IAM role used by the new <code>Crawler</code> to access customer resources.</p>"]
    #[serde(rename="Role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,
    #[doc="<p>A cron expression that can be used as a Cloudwatch event (see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html\">CloudWatch Schedule Expression Syntax</a>. For example, to run every day at 12:15 UTC, specify: <code>cron(15 12 * * ? *)</code>.</p>"]
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule: Option<String>,
    #[doc="<p>Policy for the crawler's update and deletion behavior.</p>"]
    #[serde(rename="SchemaChangePolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    #[doc="<p>The table prefix used for catalog tables created.</p>"]
    #[serde(rename="TablePrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_prefix: Option<String>,
    #[doc="<p>A list of collection of targets to crawl.</p>"]
    #[serde(rename="Targets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub targets: Option<CrawlerTargets>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateCrawlerResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateCrawlerScheduleRequest {
    #[doc="<p>Name of the crawler whose schedule to update.</p>"]
    #[serde(rename="CrawlerName")]
    pub crawler_name: String,
    #[doc="<p>Cron expression of the updated schedule.</p>"]
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateCrawlerScheduleResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateDatabaseRequest {
    #[doc="<p>The ID of the Data Catalog in which the metadata database resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>A <code>DatabaseInput</code> object specifying the new definition of the metadata database in the catalog.</p>"]
    #[serde(rename="DatabaseInput")]
    pub database_input: DatabaseInput,
    #[doc="<p>The name of the metadata database to update in the catalog.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateDatabaseResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateDevEndpointRequest {
    #[doc="<p>Custom Python or Java custom libraries to be loaded in the DevEndpoint.</p>"]
    #[serde(rename="CustomLibraries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_libraries: Option<DevEndpointCustomLibraries>,
    #[doc="<p>The name of the DevEndpoint to be updated.</p>"]
    #[serde(rename="EndpointName")]
    pub endpoint_name: String,
    #[doc="<p>The public key for the DevEndpoint to use.</p>"]
    #[serde(rename="PublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_key: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateDevEndpointResponse;

#[doc="<p>Specifies a Grok classifier to update when passed to UpdateClassifier.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateGrokClassifierRequest {
    #[doc="<p>The type of result that the classifier matches, such as Twitter Json, Omniture logs, Cloudwatch logs, and so forth.</p>"]
    #[serde(rename="Classification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub classification: Option<String>,
    #[doc="<p>Custom grok patterns used by this classifier.</p>"]
    #[serde(rename="CustomPatterns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_patterns: Option<String>,
    #[doc="<p>The grok pattern used by this classifier.</p>"]
    #[serde(rename="GrokPattern")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub grok_pattern: Option<String>,
    #[doc="<p>The name of the <code>GrokClassifier</code>.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateJobRequest {
    #[doc="<p>Name of the job definition to update.</p>"]
    #[serde(rename="JobName")]
    pub job_name: String,
    #[doc="<p>Specifies the values with which to update the job.</p>"]
    #[serde(rename="JobUpdate")]
    pub job_update: JobUpdate,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateJobResponse {
    #[doc="<p>Returns the name of the updated job.</p>"]
    #[serde(rename="JobName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdatePartitionRequest {
    #[doc="<p>The ID of the Data Catalog where the partition to be updated resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database in which the table in question resides.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>The new partition object to which to update the partition.</p>"]
    #[serde(rename="PartitionInput")]
    pub partition_input: PartitionInput,
    #[doc="<p>A list of the values defining the partition.</p>"]
    #[serde(rename="PartitionValueList")]
    pub partition_value_list: Vec<String>,
    #[doc="<p>The name of the table where the partition to be updated is located.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdatePartitionResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateTableRequest {
    #[doc="<p>The ID of the Data Catalog where the table resides. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database in which the table resides.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>An updated <code>TableInput</code> object to define the metadata table in the catalog.</p>"]
    #[serde(rename="TableInput")]
    pub table_input: TableInput,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateTableResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateTriggerRequest {
    #[doc="<p>The name of the trigger to update.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The new values with which to update the trigger.</p>"]
    #[serde(rename="TriggerUpdate")]
    pub trigger_update: TriggerUpdate,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateTriggerResponse {
    #[doc="<p>The resulting trigger definition.</p>"]
    #[serde(rename="Trigger")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trigger: Option<Trigger>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateUserDefinedFunctionRequest {
    #[doc="<p>The ID of the Data Catalog where the function to be updated is located. If none is supplied, the AWS account ID is used by default.</p>"]
    #[serde(rename="CatalogId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalog_id: Option<String>,
    #[doc="<p>The name of the catalog database where the function to be updated is located.</p>"]
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[doc="<p>A <code>FunctionInput</code> object that re-defines the function in the Data Catalog.</p>"]
    #[serde(rename="FunctionInput")]
    pub function_input: UserDefinedFunctionInput,
    #[doc="<p>The name of the function.</p>"]
    #[serde(rename="FunctionName")]
    pub function_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateUserDefinedFunctionResponse;

#[doc="<p>Represents the equivalent of a Hive user-defined function (<code>UDF</code>) definition.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UserDefinedFunction {
    #[doc="<p>The Java class that contains the function code.</p>"]
    #[serde(rename="ClassName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_name: Option<String>,
    #[doc="<p>The time at which the function was created.</p>"]
    #[serde(rename="CreateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create_time: Option<f64>,
    #[doc="<p>The name of the function.</p>"]
    #[serde(rename="FunctionName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub function_name: Option<String>,
    #[doc="<p>The owner of the function.</p>"]
    #[serde(rename="OwnerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_name: Option<String>,
    #[doc="<p>The owner type.</p>"]
    #[serde(rename="OwnerType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_type: Option<String>,
    #[doc="<p>The resource URIs for the function.</p>"]
    #[serde(rename="ResourceUris")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_uris: Option<Vec<ResourceUri>>,
}

#[doc="<p>A structure used to create or updata a user-defined function.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UserDefinedFunctionInput {
    #[doc="<p>The Java class that contains the function code.</p>"]
    #[serde(rename="ClassName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_name: Option<String>,
    #[doc="<p>The name of the function.</p>"]
    #[serde(rename="FunctionName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub function_name: Option<String>,
    #[doc="<p>The owner of the function.</p>"]
    #[serde(rename="OwnerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_name: Option<String>,
    #[doc="<p>The owner type.</p>"]
    #[serde(rename="OwnerType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_type: Option<String>,
    #[doc="<p>The resource URIs for the function.</p>"]
    #[serde(rename="ResourceUris")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_uris: Option<Vec<ResourceUri>>,
}

/// Errors returned by BatchCreatePartition
#[derive(Debug, PartialEq)]
pub enum BatchCreatePartitionError {
    ///<p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl BatchCreatePartitionError {
    pub fn from_body(body: &str) -> BatchCreatePartitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        BatchCreatePartitionError::AlreadyExists(String::from(error_message))
                    }
                    "EntityNotFoundException" => {
                        BatchCreatePartitionError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        BatchCreatePartitionError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        BatchCreatePartitionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        BatchCreatePartitionError::OperationTimeout(String::from(error_message))
                    }
                    "ResourceNumberLimitExceededException" => BatchCreatePartitionError::ResourceNumberLimitExceeded(String::from(error_message)),
                    "ValidationException" => {
                        BatchCreatePartitionError::Validation(error_message.to_string())
                    }
                    _ => BatchCreatePartitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchCreatePartitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchCreatePartitionError {
    fn from(err: serde_json::error::Error) -> BatchCreatePartitionError {
        BatchCreatePartitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchCreatePartitionError {
    fn from(err: CredentialsError) -> BatchCreatePartitionError {
        BatchCreatePartitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchCreatePartitionError {
    fn from(err: HttpDispatchError) -> BatchCreatePartitionError {
        BatchCreatePartitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchCreatePartitionError {
    fn from(err: io::Error) -> BatchCreatePartitionError {
        BatchCreatePartitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchCreatePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchCreatePartitionError {
    fn description(&self) -> &str {
        match *self {
            BatchCreatePartitionError::AlreadyExists(ref cause) => cause,
            BatchCreatePartitionError::EntityNotFound(ref cause) => cause,
            BatchCreatePartitionError::InternalService(ref cause) => cause,
            BatchCreatePartitionError::InvalidInput(ref cause) => cause,
            BatchCreatePartitionError::OperationTimeout(ref cause) => cause,
            BatchCreatePartitionError::ResourceNumberLimitExceeded(ref cause) => cause,
            BatchCreatePartitionError::Validation(ref cause) => cause,
            BatchCreatePartitionError::Credentials(ref err) => err.description(),
            BatchCreatePartitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchCreatePartitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeleteConnection
#[derive(Debug, PartialEq)]
pub enum BatchDeleteConnectionError {
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl BatchDeleteConnectionError {
    pub fn from_body(body: &str) -> BatchDeleteConnectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => {
                        BatchDeleteConnectionError::InternalService(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        BatchDeleteConnectionError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchDeleteConnectionError::Validation(error_message.to_string())
                    }
                    _ => BatchDeleteConnectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchDeleteConnectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchDeleteConnectionError {
    fn from(err: serde_json::error::Error) -> BatchDeleteConnectionError {
        BatchDeleteConnectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchDeleteConnectionError {
    fn from(err: CredentialsError) -> BatchDeleteConnectionError {
        BatchDeleteConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchDeleteConnectionError {
    fn from(err: HttpDispatchError) -> BatchDeleteConnectionError {
        BatchDeleteConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchDeleteConnectionError {
    fn from(err: io::Error) -> BatchDeleteConnectionError {
        BatchDeleteConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchDeleteConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeleteConnectionError {
    fn description(&self) -> &str {
        match *self {
            BatchDeleteConnectionError::InternalService(ref cause) => cause,
            BatchDeleteConnectionError::OperationTimeout(ref cause) => cause,
            BatchDeleteConnectionError::Validation(ref cause) => cause,
            BatchDeleteConnectionError::Credentials(ref err) => err.description(),
            BatchDeleteConnectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchDeleteConnectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeletePartition
#[derive(Debug, PartialEq)]
pub enum BatchDeletePartitionError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl BatchDeletePartitionError {
    pub fn from_body(body: &str) -> BatchDeletePartitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        BatchDeletePartitionError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        BatchDeletePartitionError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        BatchDeletePartitionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        BatchDeletePartitionError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchDeletePartitionError::Validation(error_message.to_string())
                    }
                    _ => BatchDeletePartitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchDeletePartitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchDeletePartitionError {
    fn from(err: serde_json::error::Error) -> BatchDeletePartitionError {
        BatchDeletePartitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchDeletePartitionError {
    fn from(err: CredentialsError) -> BatchDeletePartitionError {
        BatchDeletePartitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchDeletePartitionError {
    fn from(err: HttpDispatchError) -> BatchDeletePartitionError {
        BatchDeletePartitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchDeletePartitionError {
    fn from(err: io::Error) -> BatchDeletePartitionError {
        BatchDeletePartitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchDeletePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeletePartitionError {
    fn description(&self) -> &str {
        match *self {
            BatchDeletePartitionError::EntityNotFound(ref cause) => cause,
            BatchDeletePartitionError::InternalService(ref cause) => cause,
            BatchDeletePartitionError::InvalidInput(ref cause) => cause,
            BatchDeletePartitionError::OperationTimeout(ref cause) => cause,
            BatchDeletePartitionError::Validation(ref cause) => cause,
            BatchDeletePartitionError::Credentials(ref err) => err.description(),
            BatchDeletePartitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchDeletePartitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeleteTable
#[derive(Debug, PartialEq)]
pub enum BatchDeleteTableError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl BatchDeleteTableError {
    pub fn from_body(body: &str) -> BatchDeleteTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        BatchDeleteTableError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        BatchDeleteTableError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        BatchDeleteTableError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        BatchDeleteTableError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchDeleteTableError::Validation(error_message.to_string())
                    }
                    _ => BatchDeleteTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchDeleteTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchDeleteTableError {
    fn from(err: serde_json::error::Error) -> BatchDeleteTableError {
        BatchDeleteTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchDeleteTableError {
    fn from(err: CredentialsError) -> BatchDeleteTableError {
        BatchDeleteTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchDeleteTableError {
    fn from(err: HttpDispatchError) -> BatchDeleteTableError {
        BatchDeleteTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchDeleteTableError {
    fn from(err: io::Error) -> BatchDeleteTableError {
        BatchDeleteTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchDeleteTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeleteTableError {
    fn description(&self) -> &str {
        match *self {
            BatchDeleteTableError::EntityNotFound(ref cause) => cause,
            BatchDeleteTableError::InternalService(ref cause) => cause,
            BatchDeleteTableError::InvalidInput(ref cause) => cause,
            BatchDeleteTableError::OperationTimeout(ref cause) => cause,
            BatchDeleteTableError::Validation(ref cause) => cause,
            BatchDeleteTableError::Credentials(ref err) => err.description(),
            BatchDeleteTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchDeleteTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetPartition
#[derive(Debug, PartialEq)]
pub enum BatchGetPartitionError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl BatchGetPartitionError {
    pub fn from_body(body: &str) -> BatchGetPartitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        BatchGetPartitionError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        BatchGetPartitionError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        BatchGetPartitionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        BatchGetPartitionError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchGetPartitionError::Validation(error_message.to_string())
                    }
                    _ => BatchGetPartitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetPartitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetPartitionError {
    fn from(err: serde_json::error::Error) -> BatchGetPartitionError {
        BatchGetPartitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetPartitionError {
    fn from(err: CredentialsError) -> BatchGetPartitionError {
        BatchGetPartitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetPartitionError {
    fn from(err: HttpDispatchError) -> BatchGetPartitionError {
        BatchGetPartitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetPartitionError {
    fn from(err: io::Error) -> BatchGetPartitionError {
        BatchGetPartitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetPartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetPartitionError {
    fn description(&self) -> &str {
        match *self {
            BatchGetPartitionError::EntityNotFound(ref cause) => cause,
            BatchGetPartitionError::InternalService(ref cause) => cause,
            BatchGetPartitionError::InvalidInput(ref cause) => cause,
            BatchGetPartitionError::OperationTimeout(ref cause) => cause,
            BatchGetPartitionError::Validation(ref cause) => cause,
            BatchGetPartitionError::Credentials(ref err) => err.description(),
            BatchGetPartitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchGetPartitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateClassifier
#[derive(Debug, PartialEq)]
pub enum CreateClassifierError {
    ///<p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateClassifierError {
    pub fn from_body(body: &str) -> CreateClassifierError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateClassifierError::AlreadyExists(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateClassifierError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        CreateClassifierError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateClassifierError::Validation(error_message.to_string())
                    }
                    _ => CreateClassifierError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateClassifierError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateClassifierError {
    fn from(err: serde_json::error::Error) -> CreateClassifierError {
        CreateClassifierError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateClassifierError {
    fn from(err: CredentialsError) -> CreateClassifierError {
        CreateClassifierError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateClassifierError {
    fn from(err: HttpDispatchError) -> CreateClassifierError {
        CreateClassifierError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateClassifierError {
    fn from(err: io::Error) -> CreateClassifierError {
        CreateClassifierError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClassifierError {
    fn description(&self) -> &str {
        match *self {
            CreateClassifierError::AlreadyExists(ref cause) => cause,
            CreateClassifierError::InvalidInput(ref cause) => cause,
            CreateClassifierError::OperationTimeout(ref cause) => cause,
            CreateClassifierError::Validation(ref cause) => cause,
            CreateClassifierError::Credentials(ref err) => err.description(),
            CreateClassifierError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateClassifierError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConnection
#[derive(Debug, PartialEq)]
pub enum CreateConnectionError {
    ///<p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateConnectionError {
    pub fn from_body(body: &str) -> CreateConnectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateConnectionError::AlreadyExists(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateConnectionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        CreateConnectionError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateConnectionError::Validation(error_message.to_string())
                    }
                    _ => CreateConnectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateConnectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateConnectionError {
    fn from(err: serde_json::error::Error) -> CreateConnectionError {
        CreateConnectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateConnectionError {
    fn from(err: CredentialsError) -> CreateConnectionError {
        CreateConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateConnectionError {
    fn from(err: HttpDispatchError) -> CreateConnectionError {
        CreateConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateConnectionError {
    fn from(err: io::Error) -> CreateConnectionError {
        CreateConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConnectionError {
    fn description(&self) -> &str {
        match *self {
            CreateConnectionError::AlreadyExists(ref cause) => cause,
            CreateConnectionError::InvalidInput(ref cause) => cause,
            CreateConnectionError::OperationTimeout(ref cause) => cause,
            CreateConnectionError::Validation(ref cause) => cause,
            CreateConnectionError::Credentials(ref err) => err.description(),
            CreateConnectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateConnectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCrawler
#[derive(Debug, PartialEq)]
pub enum CreateCrawlerError {
    ///<p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateCrawlerError {
    pub fn from_body(body: &str) -> CreateCrawlerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateCrawlerError::AlreadyExists(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateCrawlerError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        CreateCrawlerError::OperationTimeout(String::from(error_message))
                    }
                    "ResourceNumberLimitExceededException" => {
                        CreateCrawlerError::ResourceNumberLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateCrawlerError::Validation(error_message.to_string())
                    }
                    _ => CreateCrawlerError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCrawlerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCrawlerError {
    fn from(err: serde_json::error::Error) -> CreateCrawlerError {
        CreateCrawlerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCrawlerError {
    fn from(err: CredentialsError) -> CreateCrawlerError {
        CreateCrawlerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCrawlerError {
    fn from(err: HttpDispatchError) -> CreateCrawlerError {
        CreateCrawlerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCrawlerError {
    fn from(err: io::Error) -> CreateCrawlerError {
        CreateCrawlerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCrawlerError {
    fn description(&self) -> &str {
        match *self {
            CreateCrawlerError::AlreadyExists(ref cause) => cause,
            CreateCrawlerError::InvalidInput(ref cause) => cause,
            CreateCrawlerError::OperationTimeout(ref cause) => cause,
            CreateCrawlerError::ResourceNumberLimitExceeded(ref cause) => cause,
            CreateCrawlerError::Validation(ref cause) => cause,
            CreateCrawlerError::Credentials(ref err) => err.description(),
            CreateCrawlerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateCrawlerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDatabase
#[derive(Debug, PartialEq)]
pub enum CreateDatabaseError {
    ///<p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateDatabaseError {
    pub fn from_body(body: &str) -> CreateDatabaseError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateDatabaseError::AlreadyExists(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreateDatabaseError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateDatabaseError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        CreateDatabaseError::OperationTimeout(String::from(error_message))
                    }
                    "ResourceNumberLimitExceededException" => CreateDatabaseError::ResourceNumberLimitExceeded(String::from(error_message)),
                    "ValidationException" => {
                        CreateDatabaseError::Validation(error_message.to_string())
                    }
                    _ => CreateDatabaseError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDatabaseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDatabaseError {
    fn from(err: serde_json::error::Error) -> CreateDatabaseError {
        CreateDatabaseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDatabaseError {
    fn from(err: CredentialsError) -> CreateDatabaseError {
        CreateDatabaseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDatabaseError {
    fn from(err: HttpDispatchError) -> CreateDatabaseError {
        CreateDatabaseError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDatabaseError {
    fn from(err: io::Error) -> CreateDatabaseError {
        CreateDatabaseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDatabaseError {
    fn description(&self) -> &str {
        match *self {
            CreateDatabaseError::AlreadyExists(ref cause) => cause,
            CreateDatabaseError::InternalService(ref cause) => cause,
            CreateDatabaseError::InvalidInput(ref cause) => cause,
            CreateDatabaseError::OperationTimeout(ref cause) => cause,
            CreateDatabaseError::ResourceNumberLimitExceeded(ref cause) => cause,
            CreateDatabaseError::Validation(ref cause) => cause,
            CreateDatabaseError::Credentials(ref err) => err.description(),
            CreateDatabaseError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDatabaseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDevEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateDevEndpointError {
    ///<p>Access to a resource was denied.</p>
    AccessDenied(String),
    ///<p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    ///<p>The same unique identifier was associated with two different records.</p>
    IdempotentParameterMismatch(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateDevEndpointError {
    pub fn from_body(body: &str) -> CreateDevEndpointError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        CreateDevEndpointError::AccessDenied(String::from(error_message))
                    }
                    "AlreadyExistsException" => {
                        CreateDevEndpointError::AlreadyExists(String::from(error_message))
                    }
                    "IdempotentParameterMismatchException" => CreateDevEndpointError::IdempotentParameterMismatch(String::from(error_message)),
                    "InternalServiceException" => {
                        CreateDevEndpointError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateDevEndpointError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        CreateDevEndpointError::OperationTimeout(String::from(error_message))
                    }
                    "ResourceNumberLimitExceededException" => CreateDevEndpointError::ResourceNumberLimitExceeded(String::from(error_message)),
                    "ValidationException" => {
                        CreateDevEndpointError::Validation(error_message.to_string())
                    }
                    _ => CreateDevEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDevEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDevEndpointError {
    fn from(err: serde_json::error::Error) -> CreateDevEndpointError {
        CreateDevEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDevEndpointError {
    fn from(err: CredentialsError) -> CreateDevEndpointError {
        CreateDevEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDevEndpointError {
    fn from(err: HttpDispatchError) -> CreateDevEndpointError {
        CreateDevEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDevEndpointError {
    fn from(err: io::Error) -> CreateDevEndpointError {
        CreateDevEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            CreateDevEndpointError::AccessDenied(ref cause) => cause,
            CreateDevEndpointError::AlreadyExists(ref cause) => cause,
            CreateDevEndpointError::IdempotentParameterMismatch(ref cause) => cause,
            CreateDevEndpointError::InternalService(ref cause) => cause,
            CreateDevEndpointError::InvalidInput(ref cause) => cause,
            CreateDevEndpointError::OperationTimeout(ref cause) => cause,
            CreateDevEndpointError::ResourceNumberLimitExceeded(ref cause) => cause,
            CreateDevEndpointError::Validation(ref cause) => cause,
            CreateDevEndpointError::Credentials(ref err) => err.description(),
            CreateDevEndpointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDevEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    ///<p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    ///<p>The same unique identifier was associated with two different records.</p>
    IdempotentParameterMismatch(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateJobError {
    pub fn from_body(body: &str) -> CreateJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateJobError::AlreadyExists(String::from(error_message))
                    }
                    "IdempotentParameterMismatchException" => {
                        CreateJobError::IdempotentParameterMismatch(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreateJobError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateJobError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        CreateJobError::OperationTimeout(String::from(error_message))
                    }
                    "ResourceNumberLimitExceededException" => {
                        CreateJobError::ResourceNumberLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => CreateJobError::Validation(error_message.to_string()),
                    _ => CreateJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateJobError {
    fn from(err: serde_json::error::Error) -> CreateJobError {
        CreateJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateJobError {
    fn from(err: CredentialsError) -> CreateJobError {
        CreateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateJobError {
    fn from(err: HttpDispatchError) -> CreateJobError {
        CreateJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateJobError {
    fn from(err: io::Error) -> CreateJobError {
        CreateJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateJobError {
    fn description(&self) -> &str {
        match *self {
            CreateJobError::AlreadyExists(ref cause) => cause,
            CreateJobError::IdempotentParameterMismatch(ref cause) => cause,
            CreateJobError::InternalService(ref cause) => cause,
            CreateJobError::InvalidInput(ref cause) => cause,
            CreateJobError::OperationTimeout(ref cause) => cause,
            CreateJobError::ResourceNumberLimitExceeded(ref cause) => cause,
            CreateJobError::Validation(ref cause) => cause,
            CreateJobError::Credentials(ref err) => err.description(),
            CreateJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePartition
#[derive(Debug, PartialEq)]
pub enum CreatePartitionError {
    ///<p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreatePartitionError {
    pub fn from_body(body: &str) -> CreatePartitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreatePartitionError::AlreadyExists(String::from(error_message))
                    }
                    "EntityNotFoundException" => {
                        CreatePartitionError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreatePartitionError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreatePartitionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        CreatePartitionError::OperationTimeout(String::from(error_message))
                    }
                    "ResourceNumberLimitExceededException" => CreatePartitionError::ResourceNumberLimitExceeded(String::from(error_message)),
                    "ValidationException" => {
                        CreatePartitionError::Validation(error_message.to_string())
                    }
                    _ => CreatePartitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePartitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePartitionError {
    fn from(err: serde_json::error::Error) -> CreatePartitionError {
        CreatePartitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePartitionError {
    fn from(err: CredentialsError) -> CreatePartitionError {
        CreatePartitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePartitionError {
    fn from(err: HttpDispatchError) -> CreatePartitionError {
        CreatePartitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePartitionError {
    fn from(err: io::Error) -> CreatePartitionError {
        CreatePartitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePartitionError {
    fn description(&self) -> &str {
        match *self {
            CreatePartitionError::AlreadyExists(ref cause) => cause,
            CreatePartitionError::EntityNotFound(ref cause) => cause,
            CreatePartitionError::InternalService(ref cause) => cause,
            CreatePartitionError::InvalidInput(ref cause) => cause,
            CreatePartitionError::OperationTimeout(ref cause) => cause,
            CreatePartitionError::ResourceNumberLimitExceeded(ref cause) => cause,
            CreatePartitionError::Validation(ref cause) => cause,
            CreatePartitionError::Credentials(ref err) => err.description(),
            CreatePartitionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreatePartitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateScript
#[derive(Debug, PartialEq)]
pub enum CreateScriptError {
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateScriptError {
    pub fn from_body(body: &str) -> CreateScriptError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => {
                        CreateScriptError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateScriptError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        CreateScriptError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateScriptError::Validation(error_message.to_string())
                    }
                    _ => CreateScriptError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateScriptError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateScriptError {
    fn from(err: serde_json::error::Error) -> CreateScriptError {
        CreateScriptError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateScriptError {
    fn from(err: CredentialsError) -> CreateScriptError {
        CreateScriptError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateScriptError {
    fn from(err: HttpDispatchError) -> CreateScriptError {
        CreateScriptError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateScriptError {
    fn from(err: io::Error) -> CreateScriptError {
        CreateScriptError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateScriptError {
    fn description(&self) -> &str {
        match *self {
            CreateScriptError::InternalService(ref cause) => cause,
            CreateScriptError::InvalidInput(ref cause) => cause,
            CreateScriptError::OperationTimeout(ref cause) => cause,
            CreateScriptError::Validation(ref cause) => cause,
            CreateScriptError::Credentials(ref err) => err.description(),
            CreateScriptError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateScriptError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTable
#[derive(Debug, PartialEq)]
pub enum CreateTableError {
    ///<p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateTableError {
    pub fn from_body(body: &str) -> CreateTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateTableError::AlreadyExists(String::from(error_message))
                    }
                    "EntityNotFoundException" => {
                        CreateTableError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreateTableError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateTableError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        CreateTableError::OperationTimeout(String::from(error_message))
                    }
                    "ResourceNumberLimitExceededException" => {
                        CreateTableError::ResourceNumberLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateTableError::Validation(error_message.to_string())
                    }
                    _ => CreateTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTableError {
    fn from(err: serde_json::error::Error) -> CreateTableError {
        CreateTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTableError {
    fn from(err: CredentialsError) -> CreateTableError {
        CreateTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTableError {
    fn from(err: HttpDispatchError) -> CreateTableError {
        CreateTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTableError {
    fn from(err: io::Error) -> CreateTableError {
        CreateTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTableError {
    fn description(&self) -> &str {
        match *self {
            CreateTableError::AlreadyExists(ref cause) => cause,
            CreateTableError::EntityNotFound(ref cause) => cause,
            CreateTableError::InternalService(ref cause) => cause,
            CreateTableError::InvalidInput(ref cause) => cause,
            CreateTableError::OperationTimeout(ref cause) => cause,
            CreateTableError::ResourceNumberLimitExceeded(ref cause) => cause,
            CreateTableError::Validation(ref cause) => cause,
            CreateTableError::Credentials(ref err) => err.description(),
            CreateTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTrigger
#[derive(Debug, PartialEq)]
pub enum CreateTriggerError {
    ///<p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateTriggerError {
    pub fn from_body(body: &str) -> CreateTriggerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateTriggerError::AlreadyExists(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreateTriggerError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateTriggerError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        CreateTriggerError::OperationTimeout(String::from(error_message))
                    }
                    "ResourceNumberLimitExceededException" => {
                        CreateTriggerError::ResourceNumberLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateTriggerError::Validation(error_message.to_string())
                    }
                    _ => CreateTriggerError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTriggerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTriggerError {
    fn from(err: serde_json::error::Error) -> CreateTriggerError {
        CreateTriggerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTriggerError {
    fn from(err: CredentialsError) -> CreateTriggerError {
        CreateTriggerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTriggerError {
    fn from(err: HttpDispatchError) -> CreateTriggerError {
        CreateTriggerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTriggerError {
    fn from(err: io::Error) -> CreateTriggerError {
        CreateTriggerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTriggerError {
    fn description(&self) -> &str {
        match *self {
            CreateTriggerError::AlreadyExists(ref cause) => cause,
            CreateTriggerError::InternalService(ref cause) => cause,
            CreateTriggerError::InvalidInput(ref cause) => cause,
            CreateTriggerError::OperationTimeout(ref cause) => cause,
            CreateTriggerError::ResourceNumberLimitExceeded(ref cause) => cause,
            CreateTriggerError::Validation(ref cause) => cause,
            CreateTriggerError::Credentials(ref err) => err.description(),
            CreateTriggerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTriggerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum CreateUserDefinedFunctionError {
    ///<p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateUserDefinedFunctionError {
    pub fn from_body(body: &str) -> CreateUserDefinedFunctionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateUserDefinedFunctionError::AlreadyExists(String::from(error_message))
                    }
                    "EntityNotFoundException" => {
                        CreateUserDefinedFunctionError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreateUserDefinedFunctionError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateUserDefinedFunctionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => CreateUserDefinedFunctionError::OperationTimeout(String::from(error_message)),
                    "ValidationException" => {
                        CreateUserDefinedFunctionError::Validation(error_message.to_string())
                    }
                    _ => CreateUserDefinedFunctionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUserDefinedFunctionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUserDefinedFunctionError {
    fn from(err: serde_json::error::Error) -> CreateUserDefinedFunctionError {
        CreateUserDefinedFunctionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUserDefinedFunctionError {
    fn from(err: CredentialsError) -> CreateUserDefinedFunctionError {
        CreateUserDefinedFunctionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUserDefinedFunctionError {
    fn from(err: HttpDispatchError) -> CreateUserDefinedFunctionError {
        CreateUserDefinedFunctionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateUserDefinedFunctionError {
    fn from(err: io::Error) -> CreateUserDefinedFunctionError {
        CreateUserDefinedFunctionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            CreateUserDefinedFunctionError::AlreadyExists(ref cause) => cause,
            CreateUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            CreateUserDefinedFunctionError::InternalService(ref cause) => cause,
            CreateUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            CreateUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
            CreateUserDefinedFunctionError::Validation(ref cause) => cause,
            CreateUserDefinedFunctionError::Credentials(ref err) => err.description(),
            CreateUserDefinedFunctionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateUserDefinedFunctionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteClassifier
#[derive(Debug, PartialEq)]
pub enum DeleteClassifierError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteClassifierError {
    pub fn from_body(body: &str) -> DeleteClassifierError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        DeleteClassifierError::EntityNotFound(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        DeleteClassifierError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteClassifierError::Validation(error_message.to_string())
                    }
                    _ => DeleteClassifierError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteClassifierError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteClassifierError {
    fn from(err: serde_json::error::Error) -> DeleteClassifierError {
        DeleteClassifierError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteClassifierError {
    fn from(err: CredentialsError) -> DeleteClassifierError {
        DeleteClassifierError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteClassifierError {
    fn from(err: HttpDispatchError) -> DeleteClassifierError {
        DeleteClassifierError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteClassifierError {
    fn from(err: io::Error) -> DeleteClassifierError {
        DeleteClassifierError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClassifierError {
    fn description(&self) -> &str {
        match *self {
            DeleteClassifierError::EntityNotFound(ref cause) => cause,
            DeleteClassifierError::OperationTimeout(ref cause) => cause,
            DeleteClassifierError::Validation(ref cause) => cause,
            DeleteClassifierError::Credentials(ref err) => err.description(),
            DeleteClassifierError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteClassifierError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteConnectionError {
    pub fn from_body(body: &str) -> DeleteConnectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        DeleteConnectionError::EntityNotFound(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        DeleteConnectionError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteConnectionError::Validation(error_message.to_string())
                    }
                    _ => DeleteConnectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteConnectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteConnectionError {
    fn from(err: serde_json::error::Error) -> DeleteConnectionError {
        DeleteConnectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteConnectionError {
    fn from(err: CredentialsError) -> DeleteConnectionError {
        DeleteConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConnectionError {
    fn from(err: HttpDispatchError) -> DeleteConnectionError {
        DeleteConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteConnectionError {
    fn from(err: io::Error) -> DeleteConnectionError {
        DeleteConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConnectionError {
    fn description(&self) -> &str {
        match *self {
            DeleteConnectionError::EntityNotFound(ref cause) => cause,
            DeleteConnectionError::OperationTimeout(ref cause) => cause,
            DeleteConnectionError::Validation(ref cause) => cause,
            DeleteConnectionError::Credentials(ref err) => err.description(),
            DeleteConnectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteConnectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCrawler
#[derive(Debug, PartialEq)]
pub enum DeleteCrawlerError {
    ///<p>The operation cannot be performed because the crawler is already running.</p>
    CrawlerRunning(String),
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteCrawlerError {
    pub fn from_body(body: &str) -> DeleteCrawlerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CrawlerRunningException" => {
                        DeleteCrawlerError::CrawlerRunning(String::from(error_message))
                    }
                    "EntityNotFoundException" => {
                        DeleteCrawlerError::EntityNotFound(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        DeleteCrawlerError::OperationTimeout(String::from(error_message))
                    }
                    "SchedulerTransitioningException" => {
                        DeleteCrawlerError::SchedulerTransitioning(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteCrawlerError::Validation(error_message.to_string())
                    }
                    _ => DeleteCrawlerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCrawlerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCrawlerError {
    fn from(err: serde_json::error::Error) -> DeleteCrawlerError {
        DeleteCrawlerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCrawlerError {
    fn from(err: CredentialsError) -> DeleteCrawlerError {
        DeleteCrawlerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCrawlerError {
    fn from(err: HttpDispatchError) -> DeleteCrawlerError {
        DeleteCrawlerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCrawlerError {
    fn from(err: io::Error) -> DeleteCrawlerError {
        DeleteCrawlerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCrawlerError {
    fn description(&self) -> &str {
        match *self {
            DeleteCrawlerError::CrawlerRunning(ref cause) => cause,
            DeleteCrawlerError::EntityNotFound(ref cause) => cause,
            DeleteCrawlerError::OperationTimeout(ref cause) => cause,
            DeleteCrawlerError::SchedulerTransitioning(ref cause) => cause,
            DeleteCrawlerError::Validation(ref cause) => cause,
            DeleteCrawlerError::Credentials(ref err) => err.description(),
            DeleteCrawlerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteCrawlerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDatabase
#[derive(Debug, PartialEq)]
pub enum DeleteDatabaseError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteDatabaseError {
    pub fn from_body(body: &str) -> DeleteDatabaseError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        DeleteDatabaseError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeleteDatabaseError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteDatabaseError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        DeleteDatabaseError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDatabaseError::Validation(error_message.to_string())
                    }
                    _ => DeleteDatabaseError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDatabaseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDatabaseError {
    fn from(err: serde_json::error::Error) -> DeleteDatabaseError {
        DeleteDatabaseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDatabaseError {
    fn from(err: CredentialsError) -> DeleteDatabaseError {
        DeleteDatabaseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDatabaseError {
    fn from(err: HttpDispatchError) -> DeleteDatabaseError {
        DeleteDatabaseError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDatabaseError {
    fn from(err: io::Error) -> DeleteDatabaseError {
        DeleteDatabaseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDatabaseError {
    fn description(&self) -> &str {
        match *self {
            DeleteDatabaseError::EntityNotFound(ref cause) => cause,
            DeleteDatabaseError::InternalService(ref cause) => cause,
            DeleteDatabaseError::InvalidInput(ref cause) => cause,
            DeleteDatabaseError::OperationTimeout(ref cause) => cause,
            DeleteDatabaseError::Validation(ref cause) => cause,
            DeleteDatabaseError::Credentials(ref err) => err.description(),
            DeleteDatabaseError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDatabaseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDevEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteDevEndpointError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteDevEndpointError {
    pub fn from_body(body: &str) -> DeleteDevEndpointError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        DeleteDevEndpointError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeleteDevEndpointError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteDevEndpointError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        DeleteDevEndpointError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDevEndpointError::Validation(error_message.to_string())
                    }
                    _ => DeleteDevEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDevEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDevEndpointError {
    fn from(err: serde_json::error::Error) -> DeleteDevEndpointError {
        DeleteDevEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDevEndpointError {
    fn from(err: CredentialsError) -> DeleteDevEndpointError {
        DeleteDevEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDevEndpointError {
    fn from(err: HttpDispatchError) -> DeleteDevEndpointError {
        DeleteDevEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDevEndpointError {
    fn from(err: io::Error) -> DeleteDevEndpointError {
        DeleteDevEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteDevEndpointError::EntityNotFound(ref cause) => cause,
            DeleteDevEndpointError::InternalService(ref cause) => cause,
            DeleteDevEndpointError::InvalidInput(ref cause) => cause,
            DeleteDevEndpointError::OperationTimeout(ref cause) => cause,
            DeleteDevEndpointError::Validation(ref cause) => cause,
            DeleteDevEndpointError::Credentials(ref err) => err.description(),
            DeleteDevEndpointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDevEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteJob
#[derive(Debug, PartialEq)]
pub enum DeleteJobError {
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteJobError {
    pub fn from_body(body: &str) -> DeleteJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => {
                        DeleteJobError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteJobError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        DeleteJobError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => DeleteJobError::Validation(error_message.to_string()),
                    _ => DeleteJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteJobError {
    fn from(err: serde_json::error::Error) -> DeleteJobError {
        DeleteJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteJobError {
    fn from(err: CredentialsError) -> DeleteJobError {
        DeleteJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteJobError {
    fn from(err: HttpDispatchError) -> DeleteJobError {
        DeleteJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteJobError {
    fn from(err: io::Error) -> DeleteJobError {
        DeleteJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteJobError {
    fn description(&self) -> &str {
        match *self {
            DeleteJobError::InternalService(ref cause) => cause,
            DeleteJobError::InvalidInput(ref cause) => cause,
            DeleteJobError::OperationTimeout(ref cause) => cause,
            DeleteJobError::Validation(ref cause) => cause,
            DeleteJobError::Credentials(ref err) => err.description(),
            DeleteJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePartition
#[derive(Debug, PartialEq)]
pub enum DeletePartitionError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeletePartitionError {
    pub fn from_body(body: &str) -> DeletePartitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        DeletePartitionError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeletePartitionError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeletePartitionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        DeletePartitionError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeletePartitionError::Validation(error_message.to_string())
                    }
                    _ => DeletePartitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeletePartitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeletePartitionError {
    fn from(err: serde_json::error::Error) -> DeletePartitionError {
        DeletePartitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePartitionError {
    fn from(err: CredentialsError) -> DeletePartitionError {
        DeletePartitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePartitionError {
    fn from(err: HttpDispatchError) -> DeletePartitionError {
        DeletePartitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePartitionError {
    fn from(err: io::Error) -> DeletePartitionError {
        DeletePartitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePartitionError {
    fn description(&self) -> &str {
        match *self {
            DeletePartitionError::EntityNotFound(ref cause) => cause,
            DeletePartitionError::InternalService(ref cause) => cause,
            DeletePartitionError::InvalidInput(ref cause) => cause,
            DeletePartitionError::OperationTimeout(ref cause) => cause,
            DeletePartitionError::Validation(ref cause) => cause,
            DeletePartitionError::Credentials(ref err) => err.description(),
            DeletePartitionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeletePartitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTable
#[derive(Debug, PartialEq)]
pub enum DeleteTableError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteTableError {
    pub fn from_body(body: &str) -> DeleteTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        DeleteTableError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeleteTableError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteTableError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        DeleteTableError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteTableError::Validation(error_message.to_string())
                    }
                    _ => DeleteTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTableError {
    fn from(err: serde_json::error::Error) -> DeleteTableError {
        DeleteTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTableError {
    fn from(err: CredentialsError) -> DeleteTableError {
        DeleteTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTableError {
    fn from(err: HttpDispatchError) -> DeleteTableError {
        DeleteTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTableError {
    fn from(err: io::Error) -> DeleteTableError {
        DeleteTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTableError {
    fn description(&self) -> &str {
        match *self {
            DeleteTableError::EntityNotFound(ref cause) => cause,
            DeleteTableError::InternalService(ref cause) => cause,
            DeleteTableError::InvalidInput(ref cause) => cause,
            DeleteTableError::OperationTimeout(ref cause) => cause,
            DeleteTableError::Validation(ref cause) => cause,
            DeleteTableError::Credentials(ref err) => err.description(),
            DeleteTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTrigger
#[derive(Debug, PartialEq)]
pub enum DeleteTriggerError {
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteTriggerError {
    pub fn from_body(body: &str) -> DeleteTriggerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => {
                        DeleteTriggerError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteTriggerError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        DeleteTriggerError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteTriggerError::Validation(error_message.to_string())
                    }
                    _ => DeleteTriggerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTriggerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTriggerError {
    fn from(err: serde_json::error::Error) -> DeleteTriggerError {
        DeleteTriggerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTriggerError {
    fn from(err: CredentialsError) -> DeleteTriggerError {
        DeleteTriggerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTriggerError {
    fn from(err: HttpDispatchError) -> DeleteTriggerError {
        DeleteTriggerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTriggerError {
    fn from(err: io::Error) -> DeleteTriggerError {
        DeleteTriggerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTriggerError {
    fn description(&self) -> &str {
        match *self {
            DeleteTriggerError::InternalService(ref cause) => cause,
            DeleteTriggerError::InvalidInput(ref cause) => cause,
            DeleteTriggerError::OperationTimeout(ref cause) => cause,
            DeleteTriggerError::Validation(ref cause) => cause,
            DeleteTriggerError::Credentials(ref err) => err.description(),
            DeleteTriggerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTriggerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum DeleteUserDefinedFunctionError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteUserDefinedFunctionError {
    pub fn from_body(body: &str) -> DeleteUserDefinedFunctionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        DeleteUserDefinedFunctionError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeleteUserDefinedFunctionError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteUserDefinedFunctionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => DeleteUserDefinedFunctionError::OperationTimeout(String::from(error_message)),
                    "ValidationException" => {
                        DeleteUserDefinedFunctionError::Validation(error_message.to_string())
                    }
                    _ => DeleteUserDefinedFunctionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUserDefinedFunctionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUserDefinedFunctionError {
    fn from(err: serde_json::error::Error) -> DeleteUserDefinedFunctionError {
        DeleteUserDefinedFunctionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserDefinedFunctionError {
    fn from(err: CredentialsError) -> DeleteUserDefinedFunctionError {
        DeleteUserDefinedFunctionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserDefinedFunctionError {
    fn from(err: HttpDispatchError) -> DeleteUserDefinedFunctionError {
        DeleteUserDefinedFunctionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteUserDefinedFunctionError {
    fn from(err: io::Error) -> DeleteUserDefinedFunctionError {
        DeleteUserDefinedFunctionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            DeleteUserDefinedFunctionError::InternalService(ref cause) => cause,
            DeleteUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            DeleteUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
            DeleteUserDefinedFunctionError::Validation(ref cause) => cause,
            DeleteUserDefinedFunctionError::Credentials(ref err) => err.description(),
            DeleteUserDefinedFunctionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteUserDefinedFunctionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCatalogImportStatus
#[derive(Debug, PartialEq)]
pub enum GetCatalogImportStatusError {
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetCatalogImportStatusError {
    pub fn from_body(body: &str) -> GetCatalogImportStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => {
                        GetCatalogImportStatusError::InternalService(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetCatalogImportStatusError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCatalogImportStatusError::Validation(error_message.to_string())
                    }
                    _ => GetCatalogImportStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCatalogImportStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCatalogImportStatusError {
    fn from(err: serde_json::error::Error) -> GetCatalogImportStatusError {
        GetCatalogImportStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCatalogImportStatusError {
    fn from(err: CredentialsError) -> GetCatalogImportStatusError {
        GetCatalogImportStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCatalogImportStatusError {
    fn from(err: HttpDispatchError) -> GetCatalogImportStatusError {
        GetCatalogImportStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCatalogImportStatusError {
    fn from(err: io::Error) -> GetCatalogImportStatusError {
        GetCatalogImportStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCatalogImportStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCatalogImportStatusError {
    fn description(&self) -> &str {
        match *self {
            GetCatalogImportStatusError::InternalService(ref cause) => cause,
            GetCatalogImportStatusError::OperationTimeout(ref cause) => cause,
            GetCatalogImportStatusError::Validation(ref cause) => cause,
            GetCatalogImportStatusError::Credentials(ref err) => err.description(),
            GetCatalogImportStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCatalogImportStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetClassifier
#[derive(Debug, PartialEq)]
pub enum GetClassifierError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetClassifierError {
    pub fn from_body(body: &str) -> GetClassifierError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetClassifierError::EntityNotFound(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetClassifierError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetClassifierError::Validation(error_message.to_string())
                    }
                    _ => GetClassifierError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetClassifierError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetClassifierError {
    fn from(err: serde_json::error::Error) -> GetClassifierError {
        GetClassifierError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetClassifierError {
    fn from(err: CredentialsError) -> GetClassifierError {
        GetClassifierError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetClassifierError {
    fn from(err: HttpDispatchError) -> GetClassifierError {
        GetClassifierError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetClassifierError {
    fn from(err: io::Error) -> GetClassifierError {
        GetClassifierError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetClassifierError {
    fn description(&self) -> &str {
        match *self {
            GetClassifierError::EntityNotFound(ref cause) => cause,
            GetClassifierError::OperationTimeout(ref cause) => cause,
            GetClassifierError::Validation(ref cause) => cause,
            GetClassifierError::Credentials(ref err) => err.description(),
            GetClassifierError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetClassifierError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetClassifiers
#[derive(Debug, PartialEq)]
pub enum GetClassifiersError {
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetClassifiersError {
    pub fn from_body(body: &str) -> GetClassifiersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "OperationTimeoutException" => {
                        GetClassifiersError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetClassifiersError::Validation(error_message.to_string())
                    }
                    _ => GetClassifiersError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetClassifiersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetClassifiersError {
    fn from(err: serde_json::error::Error) -> GetClassifiersError {
        GetClassifiersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetClassifiersError {
    fn from(err: CredentialsError) -> GetClassifiersError {
        GetClassifiersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetClassifiersError {
    fn from(err: HttpDispatchError) -> GetClassifiersError {
        GetClassifiersError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetClassifiersError {
    fn from(err: io::Error) -> GetClassifiersError {
        GetClassifiersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetClassifiersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetClassifiersError {
    fn description(&self) -> &str {
        match *self {
            GetClassifiersError::OperationTimeout(ref cause) => cause,
            GetClassifiersError::Validation(ref cause) => cause,
            GetClassifiersError::Credentials(ref err) => err.description(),
            GetClassifiersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetClassifiersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnection
#[derive(Debug, PartialEq)]
pub enum GetConnectionError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetConnectionError {
    pub fn from_body(body: &str) -> GetConnectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetConnectionError::EntityNotFound(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetConnectionError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetConnectionError::Validation(error_message.to_string())
                    }
                    _ => GetConnectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetConnectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetConnectionError {
    fn from(err: serde_json::error::Error) -> GetConnectionError {
        GetConnectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetConnectionError {
    fn from(err: CredentialsError) -> GetConnectionError {
        GetConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetConnectionError {
    fn from(err: HttpDispatchError) -> GetConnectionError {
        GetConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetConnectionError {
    fn from(err: io::Error) -> GetConnectionError {
        GetConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectionError {
    fn description(&self) -> &str {
        match *self {
            GetConnectionError::EntityNotFound(ref cause) => cause,
            GetConnectionError::OperationTimeout(ref cause) => cause,
            GetConnectionError::Validation(ref cause) => cause,
            GetConnectionError::Credentials(ref err) => err.description(),
            GetConnectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetConnectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnections
#[derive(Debug, PartialEq)]
pub enum GetConnectionsError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetConnectionsError {
    pub fn from_body(body: &str) -> GetConnectionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetConnectionsError::EntityNotFound(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetConnectionsError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetConnectionsError::Validation(error_message.to_string())
                    }
                    _ => GetConnectionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetConnectionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetConnectionsError {
    fn from(err: serde_json::error::Error) -> GetConnectionsError {
        GetConnectionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetConnectionsError {
    fn from(err: CredentialsError) -> GetConnectionsError {
        GetConnectionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetConnectionsError {
    fn from(err: HttpDispatchError) -> GetConnectionsError {
        GetConnectionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetConnectionsError {
    fn from(err: io::Error) -> GetConnectionsError {
        GetConnectionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetConnectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectionsError {
    fn description(&self) -> &str {
        match *self {
            GetConnectionsError::EntityNotFound(ref cause) => cause,
            GetConnectionsError::OperationTimeout(ref cause) => cause,
            GetConnectionsError::Validation(ref cause) => cause,
            GetConnectionsError::Credentials(ref err) => err.description(),
            GetConnectionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetConnectionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCrawler
#[derive(Debug, PartialEq)]
pub enum GetCrawlerError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetCrawlerError {
    pub fn from_body(body: &str) -> GetCrawlerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetCrawlerError::EntityNotFound(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetCrawlerError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => GetCrawlerError::Validation(error_message.to_string()),
                    _ => GetCrawlerError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCrawlerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCrawlerError {
    fn from(err: serde_json::error::Error) -> GetCrawlerError {
        GetCrawlerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCrawlerError {
    fn from(err: CredentialsError) -> GetCrawlerError {
        GetCrawlerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCrawlerError {
    fn from(err: HttpDispatchError) -> GetCrawlerError {
        GetCrawlerError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCrawlerError {
    fn from(err: io::Error) -> GetCrawlerError {
        GetCrawlerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCrawlerError {
    fn description(&self) -> &str {
        match *self {
            GetCrawlerError::EntityNotFound(ref cause) => cause,
            GetCrawlerError::OperationTimeout(ref cause) => cause,
            GetCrawlerError::Validation(ref cause) => cause,
            GetCrawlerError::Credentials(ref err) => err.description(),
            GetCrawlerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCrawlerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCrawlerMetrics
#[derive(Debug, PartialEq)]
pub enum GetCrawlerMetricsError {
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetCrawlerMetricsError {
    pub fn from_body(body: &str) -> GetCrawlerMetricsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "OperationTimeoutException" => {
                        GetCrawlerMetricsError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCrawlerMetricsError::Validation(error_message.to_string())
                    }
                    _ => GetCrawlerMetricsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCrawlerMetricsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCrawlerMetricsError {
    fn from(err: serde_json::error::Error) -> GetCrawlerMetricsError {
        GetCrawlerMetricsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCrawlerMetricsError {
    fn from(err: CredentialsError) -> GetCrawlerMetricsError {
        GetCrawlerMetricsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCrawlerMetricsError {
    fn from(err: HttpDispatchError) -> GetCrawlerMetricsError {
        GetCrawlerMetricsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCrawlerMetricsError {
    fn from(err: io::Error) -> GetCrawlerMetricsError {
        GetCrawlerMetricsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCrawlerMetricsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCrawlerMetricsError {
    fn description(&self) -> &str {
        match *self {
            GetCrawlerMetricsError::OperationTimeout(ref cause) => cause,
            GetCrawlerMetricsError::Validation(ref cause) => cause,
            GetCrawlerMetricsError::Credentials(ref err) => err.description(),
            GetCrawlerMetricsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCrawlerMetricsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCrawlers
#[derive(Debug, PartialEq)]
pub enum GetCrawlersError {
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetCrawlersError {
    pub fn from_body(body: &str) -> GetCrawlersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "OperationTimeoutException" => {
                        GetCrawlersError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCrawlersError::Validation(error_message.to_string())
                    }
                    _ => GetCrawlersError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCrawlersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCrawlersError {
    fn from(err: serde_json::error::Error) -> GetCrawlersError {
        GetCrawlersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCrawlersError {
    fn from(err: CredentialsError) -> GetCrawlersError {
        GetCrawlersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCrawlersError {
    fn from(err: HttpDispatchError) -> GetCrawlersError {
        GetCrawlersError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCrawlersError {
    fn from(err: io::Error) -> GetCrawlersError {
        GetCrawlersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCrawlersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCrawlersError {
    fn description(&self) -> &str {
        match *self {
            GetCrawlersError::OperationTimeout(ref cause) => cause,
            GetCrawlersError::Validation(ref cause) => cause,
            GetCrawlersError::Credentials(ref err) => err.description(),
            GetCrawlersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCrawlersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDatabase
#[derive(Debug, PartialEq)]
pub enum GetDatabaseError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetDatabaseError {
    pub fn from_body(body: &str) -> GetDatabaseError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetDatabaseError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetDatabaseError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDatabaseError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetDatabaseError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDatabaseError::Validation(error_message.to_string())
                    }
                    _ => GetDatabaseError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDatabaseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDatabaseError {
    fn from(err: serde_json::error::Error) -> GetDatabaseError {
        GetDatabaseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDatabaseError {
    fn from(err: CredentialsError) -> GetDatabaseError {
        GetDatabaseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDatabaseError {
    fn from(err: HttpDispatchError) -> GetDatabaseError {
        GetDatabaseError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDatabaseError {
    fn from(err: io::Error) -> GetDatabaseError {
        GetDatabaseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDatabaseError {
    fn description(&self) -> &str {
        match *self {
            GetDatabaseError::EntityNotFound(ref cause) => cause,
            GetDatabaseError::InternalService(ref cause) => cause,
            GetDatabaseError::InvalidInput(ref cause) => cause,
            GetDatabaseError::OperationTimeout(ref cause) => cause,
            GetDatabaseError::Validation(ref cause) => cause,
            GetDatabaseError::Credentials(ref err) => err.description(),
            GetDatabaseError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDatabaseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDatabases
#[derive(Debug, PartialEq)]
pub enum GetDatabasesError {
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetDatabasesError {
    pub fn from_body(body: &str) -> GetDatabasesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => {
                        GetDatabasesError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDatabasesError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetDatabasesError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDatabasesError::Validation(error_message.to_string())
                    }
                    _ => GetDatabasesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDatabasesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDatabasesError {
    fn from(err: serde_json::error::Error) -> GetDatabasesError {
        GetDatabasesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDatabasesError {
    fn from(err: CredentialsError) -> GetDatabasesError {
        GetDatabasesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDatabasesError {
    fn from(err: HttpDispatchError) -> GetDatabasesError {
        GetDatabasesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDatabasesError {
    fn from(err: io::Error) -> GetDatabasesError {
        GetDatabasesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDatabasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDatabasesError {
    fn description(&self) -> &str {
        match *self {
            GetDatabasesError::InternalService(ref cause) => cause,
            GetDatabasesError::InvalidInput(ref cause) => cause,
            GetDatabasesError::OperationTimeout(ref cause) => cause,
            GetDatabasesError::Validation(ref cause) => cause,
            GetDatabasesError::Credentials(ref err) => err.description(),
            GetDatabasesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDatabasesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDataflowGraph
#[derive(Debug, PartialEq)]
pub enum GetDataflowGraphError {
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetDataflowGraphError {
    pub fn from_body(body: &str) -> GetDataflowGraphError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => {
                        GetDataflowGraphError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDataflowGraphError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetDataflowGraphError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDataflowGraphError::Validation(error_message.to_string())
                    }
                    _ => GetDataflowGraphError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDataflowGraphError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDataflowGraphError {
    fn from(err: serde_json::error::Error) -> GetDataflowGraphError {
        GetDataflowGraphError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDataflowGraphError {
    fn from(err: CredentialsError) -> GetDataflowGraphError {
        GetDataflowGraphError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDataflowGraphError {
    fn from(err: HttpDispatchError) -> GetDataflowGraphError {
        GetDataflowGraphError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDataflowGraphError {
    fn from(err: io::Error) -> GetDataflowGraphError {
        GetDataflowGraphError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDataflowGraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDataflowGraphError {
    fn description(&self) -> &str {
        match *self {
            GetDataflowGraphError::InternalService(ref cause) => cause,
            GetDataflowGraphError::InvalidInput(ref cause) => cause,
            GetDataflowGraphError::OperationTimeout(ref cause) => cause,
            GetDataflowGraphError::Validation(ref cause) => cause,
            GetDataflowGraphError::Credentials(ref err) => err.description(),
            GetDataflowGraphError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDataflowGraphError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevEndpoint
#[derive(Debug, PartialEq)]
pub enum GetDevEndpointError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetDevEndpointError {
    pub fn from_body(body: &str) -> GetDevEndpointError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetDevEndpointError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetDevEndpointError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDevEndpointError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetDevEndpointError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDevEndpointError::Validation(error_message.to_string())
                    }
                    _ => GetDevEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDevEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDevEndpointError {
    fn from(err: serde_json::error::Error) -> GetDevEndpointError {
        GetDevEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDevEndpointError {
    fn from(err: CredentialsError) -> GetDevEndpointError {
        GetDevEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDevEndpointError {
    fn from(err: HttpDispatchError) -> GetDevEndpointError {
        GetDevEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDevEndpointError {
    fn from(err: io::Error) -> GetDevEndpointError {
        GetDevEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            GetDevEndpointError::EntityNotFound(ref cause) => cause,
            GetDevEndpointError::InternalService(ref cause) => cause,
            GetDevEndpointError::InvalidInput(ref cause) => cause,
            GetDevEndpointError::OperationTimeout(ref cause) => cause,
            GetDevEndpointError::Validation(ref cause) => cause,
            GetDevEndpointError::Credentials(ref err) => err.description(),
            GetDevEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDevEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevEndpoints
#[derive(Debug, PartialEq)]
pub enum GetDevEndpointsError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetDevEndpointsError {
    pub fn from_body(body: &str) -> GetDevEndpointsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetDevEndpointsError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetDevEndpointsError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDevEndpointsError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetDevEndpointsError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDevEndpointsError::Validation(error_message.to_string())
                    }
                    _ => GetDevEndpointsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDevEndpointsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDevEndpointsError {
    fn from(err: serde_json::error::Error) -> GetDevEndpointsError {
        GetDevEndpointsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDevEndpointsError {
    fn from(err: CredentialsError) -> GetDevEndpointsError {
        GetDevEndpointsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDevEndpointsError {
    fn from(err: HttpDispatchError) -> GetDevEndpointsError {
        GetDevEndpointsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDevEndpointsError {
    fn from(err: io::Error) -> GetDevEndpointsError {
        GetDevEndpointsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDevEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDevEndpointsError {
    fn description(&self) -> &str {
        match *self {
            GetDevEndpointsError::EntityNotFound(ref cause) => cause,
            GetDevEndpointsError::InternalService(ref cause) => cause,
            GetDevEndpointsError::InvalidInput(ref cause) => cause,
            GetDevEndpointsError::OperationTimeout(ref cause) => cause,
            GetDevEndpointsError::Validation(ref cause) => cause,
            GetDevEndpointsError::Credentials(ref err) => err.description(),
            GetDevEndpointsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDevEndpointsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJob
#[derive(Debug, PartialEq)]
pub enum GetJobError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetJobError {
    pub fn from_body(body: &str) -> GetJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetJobError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetJobError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetJobError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetJobError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => GetJobError::Validation(error_message.to_string()),
                    _ => GetJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetJobError {
    fn from(err: serde_json::error::Error) -> GetJobError {
        GetJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobError {
    fn from(err: CredentialsError) -> GetJobError {
        GetJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobError {
    fn from(err: HttpDispatchError) -> GetJobError {
        GetJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobError {
    fn from(err: io::Error) -> GetJobError {
        GetJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobError {
    fn description(&self) -> &str {
        match *self {
            GetJobError::EntityNotFound(ref cause) => cause,
            GetJobError::InternalService(ref cause) => cause,
            GetJobError::InvalidInput(ref cause) => cause,
            GetJobError::OperationTimeout(ref cause) => cause,
            GetJobError::Validation(ref cause) => cause,
            GetJobError::Credentials(ref err) => err.description(),
            GetJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobRun
#[derive(Debug, PartialEq)]
pub enum GetJobRunError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetJobRunError {
    pub fn from_body(body: &str) -> GetJobRunError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetJobRunError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetJobRunError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetJobRunError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetJobRunError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => GetJobRunError::Validation(error_message.to_string()),
                    _ => GetJobRunError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetJobRunError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetJobRunError {
    fn from(err: serde_json::error::Error) -> GetJobRunError {
        GetJobRunError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobRunError {
    fn from(err: CredentialsError) -> GetJobRunError {
        GetJobRunError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobRunError {
    fn from(err: HttpDispatchError) -> GetJobRunError {
        GetJobRunError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobRunError {
    fn from(err: io::Error) -> GetJobRunError {
        GetJobRunError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobRunError {
    fn description(&self) -> &str {
        match *self {
            GetJobRunError::EntityNotFound(ref cause) => cause,
            GetJobRunError::InternalService(ref cause) => cause,
            GetJobRunError::InvalidInput(ref cause) => cause,
            GetJobRunError::OperationTimeout(ref cause) => cause,
            GetJobRunError::Validation(ref cause) => cause,
            GetJobRunError::Credentials(ref err) => err.description(),
            GetJobRunError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobRunError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobRuns
#[derive(Debug, PartialEq)]
pub enum GetJobRunsError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetJobRunsError {
    pub fn from_body(body: &str) -> GetJobRunsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetJobRunsError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetJobRunsError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetJobRunsError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetJobRunsError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => GetJobRunsError::Validation(error_message.to_string()),
                    _ => GetJobRunsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetJobRunsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetJobRunsError {
    fn from(err: serde_json::error::Error) -> GetJobRunsError {
        GetJobRunsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobRunsError {
    fn from(err: CredentialsError) -> GetJobRunsError {
        GetJobRunsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobRunsError {
    fn from(err: HttpDispatchError) -> GetJobRunsError {
        GetJobRunsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobRunsError {
    fn from(err: io::Error) -> GetJobRunsError {
        GetJobRunsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobRunsError {
    fn description(&self) -> &str {
        match *self {
            GetJobRunsError::EntityNotFound(ref cause) => cause,
            GetJobRunsError::InternalService(ref cause) => cause,
            GetJobRunsError::InvalidInput(ref cause) => cause,
            GetJobRunsError::OperationTimeout(ref cause) => cause,
            GetJobRunsError::Validation(ref cause) => cause,
            GetJobRunsError::Credentials(ref err) => err.description(),
            GetJobRunsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobRunsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobs
#[derive(Debug, PartialEq)]
pub enum GetJobsError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetJobsError {
    pub fn from_body(body: &str) -> GetJobsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetJobsError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetJobsError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetJobsError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetJobsError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => GetJobsError::Validation(error_message.to_string()),
                    _ => GetJobsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetJobsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetJobsError {
    fn from(err: serde_json::error::Error) -> GetJobsError {
        GetJobsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobsError {
    fn from(err: CredentialsError) -> GetJobsError {
        GetJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobsError {
    fn from(err: HttpDispatchError) -> GetJobsError {
        GetJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobsError {
    fn from(err: io::Error) -> GetJobsError {
        GetJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobsError {
    fn description(&self) -> &str {
        match *self {
            GetJobsError::EntityNotFound(ref cause) => cause,
            GetJobsError::InternalService(ref cause) => cause,
            GetJobsError::InvalidInput(ref cause) => cause,
            GetJobsError::OperationTimeout(ref cause) => cause,
            GetJobsError::Validation(ref cause) => cause,
            GetJobsError::Credentials(ref err) => err.description(),
            GetJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMapping
#[derive(Debug, PartialEq)]
pub enum GetMappingError {
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetMappingError {
    pub fn from_body(body: &str) -> GetMappingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => {
                        GetMappingError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetMappingError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetMappingError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => GetMappingError::Validation(error_message.to_string()),
                    _ => GetMappingError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetMappingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetMappingError {
    fn from(err: serde_json::error::Error) -> GetMappingError {
        GetMappingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMappingError {
    fn from(err: CredentialsError) -> GetMappingError {
        GetMappingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMappingError {
    fn from(err: HttpDispatchError) -> GetMappingError {
        GetMappingError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMappingError {
    fn from(err: io::Error) -> GetMappingError {
        GetMappingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMappingError {
    fn description(&self) -> &str {
        match *self {
            GetMappingError::InternalService(ref cause) => cause,
            GetMappingError::InvalidInput(ref cause) => cause,
            GetMappingError::OperationTimeout(ref cause) => cause,
            GetMappingError::Validation(ref cause) => cause,
            GetMappingError::Credentials(ref err) => err.description(),
            GetMappingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetMappingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPartition
#[derive(Debug, PartialEq)]
pub enum GetPartitionError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetPartitionError {
    pub fn from_body(body: &str) -> GetPartitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetPartitionError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetPartitionError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetPartitionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetPartitionError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetPartitionError::Validation(error_message.to_string())
                    }
                    _ => GetPartitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPartitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPartitionError {
    fn from(err: serde_json::error::Error) -> GetPartitionError {
        GetPartitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPartitionError {
    fn from(err: CredentialsError) -> GetPartitionError {
        GetPartitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPartitionError {
    fn from(err: HttpDispatchError) -> GetPartitionError {
        GetPartitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPartitionError {
    fn from(err: io::Error) -> GetPartitionError {
        GetPartitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPartitionError {
    fn description(&self) -> &str {
        match *self {
            GetPartitionError::EntityNotFound(ref cause) => cause,
            GetPartitionError::InternalService(ref cause) => cause,
            GetPartitionError::InvalidInput(ref cause) => cause,
            GetPartitionError::OperationTimeout(ref cause) => cause,
            GetPartitionError::Validation(ref cause) => cause,
            GetPartitionError::Credentials(ref err) => err.description(),
            GetPartitionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPartitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPartitions
#[derive(Debug, PartialEq)]
pub enum GetPartitionsError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetPartitionsError {
    pub fn from_body(body: &str) -> GetPartitionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetPartitionsError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetPartitionsError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetPartitionsError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetPartitionsError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetPartitionsError::Validation(error_message.to_string())
                    }
                    _ => GetPartitionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPartitionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPartitionsError {
    fn from(err: serde_json::error::Error) -> GetPartitionsError {
        GetPartitionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPartitionsError {
    fn from(err: CredentialsError) -> GetPartitionsError {
        GetPartitionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPartitionsError {
    fn from(err: HttpDispatchError) -> GetPartitionsError {
        GetPartitionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPartitionsError {
    fn from(err: io::Error) -> GetPartitionsError {
        GetPartitionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPartitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPartitionsError {
    fn description(&self) -> &str {
        match *self {
            GetPartitionsError::EntityNotFound(ref cause) => cause,
            GetPartitionsError::InternalService(ref cause) => cause,
            GetPartitionsError::InvalidInput(ref cause) => cause,
            GetPartitionsError::OperationTimeout(ref cause) => cause,
            GetPartitionsError::Validation(ref cause) => cause,
            GetPartitionsError::Credentials(ref err) => err.description(),
            GetPartitionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPartitionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPlan
#[derive(Debug, PartialEq)]
pub enum GetPlanError {
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetPlanError {
    pub fn from_body(body: &str) -> GetPlanError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => {
                        GetPlanError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetPlanError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetPlanError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => GetPlanError::Validation(error_message.to_string()),
                    _ => GetPlanError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPlanError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPlanError {
    fn from(err: serde_json::error::Error) -> GetPlanError {
        GetPlanError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPlanError {
    fn from(err: CredentialsError) -> GetPlanError {
        GetPlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPlanError {
    fn from(err: HttpDispatchError) -> GetPlanError {
        GetPlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPlanError {
    fn from(err: io::Error) -> GetPlanError {
        GetPlanError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPlanError {
    fn description(&self) -> &str {
        match *self {
            GetPlanError::InternalService(ref cause) => cause,
            GetPlanError::InvalidInput(ref cause) => cause,
            GetPlanError::OperationTimeout(ref cause) => cause,
            GetPlanError::Validation(ref cause) => cause,
            GetPlanError::Credentials(ref err) => err.description(),
            GetPlanError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPlanError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTable
#[derive(Debug, PartialEq)]
pub enum GetTableError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetTableError {
    pub fn from_body(body: &str) -> GetTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetTableError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetTableError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetTableError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetTableError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => GetTableError::Validation(error_message.to_string()),
                    _ => GetTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTableError {
    fn from(err: serde_json::error::Error) -> GetTableError {
        GetTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTableError {
    fn from(err: CredentialsError) -> GetTableError {
        GetTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTableError {
    fn from(err: HttpDispatchError) -> GetTableError {
        GetTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTableError {
    fn from(err: io::Error) -> GetTableError {
        GetTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTableError {
    fn description(&self) -> &str {
        match *self {
            GetTableError::EntityNotFound(ref cause) => cause,
            GetTableError::InternalService(ref cause) => cause,
            GetTableError::InvalidInput(ref cause) => cause,
            GetTableError::OperationTimeout(ref cause) => cause,
            GetTableError::Validation(ref cause) => cause,
            GetTableError::Credentials(ref err) => err.description(),
            GetTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTableVersions
#[derive(Debug, PartialEq)]
pub enum GetTableVersionsError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetTableVersionsError {
    pub fn from_body(body: &str) -> GetTableVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetTableVersionsError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetTableVersionsError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetTableVersionsError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetTableVersionsError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetTableVersionsError::Validation(error_message.to_string())
                    }
                    _ => GetTableVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTableVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTableVersionsError {
    fn from(err: serde_json::error::Error) -> GetTableVersionsError {
        GetTableVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTableVersionsError {
    fn from(err: CredentialsError) -> GetTableVersionsError {
        GetTableVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTableVersionsError {
    fn from(err: HttpDispatchError) -> GetTableVersionsError {
        GetTableVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTableVersionsError {
    fn from(err: io::Error) -> GetTableVersionsError {
        GetTableVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTableVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTableVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetTableVersionsError::EntityNotFound(ref cause) => cause,
            GetTableVersionsError::InternalService(ref cause) => cause,
            GetTableVersionsError::InvalidInput(ref cause) => cause,
            GetTableVersionsError::OperationTimeout(ref cause) => cause,
            GetTableVersionsError::Validation(ref cause) => cause,
            GetTableVersionsError::Credentials(ref err) => err.description(),
            GetTableVersionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTableVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTables
#[derive(Debug, PartialEq)]
pub enum GetTablesError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetTablesError {
    pub fn from_body(body: &str) -> GetTablesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetTablesError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetTablesError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetTablesError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetTablesError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => GetTablesError::Validation(error_message.to_string()),
                    _ => GetTablesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTablesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTablesError {
    fn from(err: serde_json::error::Error) -> GetTablesError {
        GetTablesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTablesError {
    fn from(err: CredentialsError) -> GetTablesError {
        GetTablesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTablesError {
    fn from(err: HttpDispatchError) -> GetTablesError {
        GetTablesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTablesError {
    fn from(err: io::Error) -> GetTablesError {
        GetTablesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTablesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTablesError {
    fn description(&self) -> &str {
        match *self {
            GetTablesError::EntityNotFound(ref cause) => cause,
            GetTablesError::InternalService(ref cause) => cause,
            GetTablesError::InvalidInput(ref cause) => cause,
            GetTablesError::OperationTimeout(ref cause) => cause,
            GetTablesError::Validation(ref cause) => cause,
            GetTablesError::Credentials(ref err) => err.description(),
            GetTablesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTablesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTrigger
#[derive(Debug, PartialEq)]
pub enum GetTriggerError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetTriggerError {
    pub fn from_body(body: &str) -> GetTriggerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetTriggerError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetTriggerError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetTriggerError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetTriggerError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => GetTriggerError::Validation(error_message.to_string()),
                    _ => GetTriggerError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTriggerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTriggerError {
    fn from(err: serde_json::error::Error) -> GetTriggerError {
        GetTriggerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTriggerError {
    fn from(err: CredentialsError) -> GetTriggerError {
        GetTriggerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTriggerError {
    fn from(err: HttpDispatchError) -> GetTriggerError {
        GetTriggerError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTriggerError {
    fn from(err: io::Error) -> GetTriggerError {
        GetTriggerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTriggerError {
    fn description(&self) -> &str {
        match *self {
            GetTriggerError::EntityNotFound(ref cause) => cause,
            GetTriggerError::InternalService(ref cause) => cause,
            GetTriggerError::InvalidInput(ref cause) => cause,
            GetTriggerError::OperationTimeout(ref cause) => cause,
            GetTriggerError::Validation(ref cause) => cause,
            GetTriggerError::Credentials(ref err) => err.description(),
            GetTriggerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTriggerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTriggers
#[derive(Debug, PartialEq)]
pub enum GetTriggersError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetTriggersError {
    pub fn from_body(body: &str) -> GetTriggersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetTriggersError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetTriggersError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetTriggersError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetTriggersError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetTriggersError::Validation(error_message.to_string())
                    }
                    _ => GetTriggersError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTriggersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTriggersError {
    fn from(err: serde_json::error::Error) -> GetTriggersError {
        GetTriggersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTriggersError {
    fn from(err: CredentialsError) -> GetTriggersError {
        GetTriggersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTriggersError {
    fn from(err: HttpDispatchError) -> GetTriggersError {
        GetTriggersError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTriggersError {
    fn from(err: io::Error) -> GetTriggersError {
        GetTriggersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTriggersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTriggersError {
    fn description(&self) -> &str {
        match *self {
            GetTriggersError::EntityNotFound(ref cause) => cause,
            GetTriggersError::InternalService(ref cause) => cause,
            GetTriggersError::InvalidInput(ref cause) => cause,
            GetTriggersError::OperationTimeout(ref cause) => cause,
            GetTriggersError::Validation(ref cause) => cause,
            GetTriggersError::Credentials(ref err) => err.description(),
            GetTriggersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTriggersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum GetUserDefinedFunctionError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetUserDefinedFunctionError {
    pub fn from_body(body: &str) -> GetUserDefinedFunctionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetUserDefinedFunctionError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetUserDefinedFunctionError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetUserDefinedFunctionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetUserDefinedFunctionError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetUserDefinedFunctionError::Validation(error_message.to_string())
                    }
                    _ => GetUserDefinedFunctionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetUserDefinedFunctionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetUserDefinedFunctionError {
    fn from(err: serde_json::error::Error) -> GetUserDefinedFunctionError {
        GetUserDefinedFunctionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUserDefinedFunctionError {
    fn from(err: CredentialsError) -> GetUserDefinedFunctionError {
        GetUserDefinedFunctionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUserDefinedFunctionError {
    fn from(err: HttpDispatchError) -> GetUserDefinedFunctionError {
        GetUserDefinedFunctionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetUserDefinedFunctionError {
    fn from(err: io::Error) -> GetUserDefinedFunctionError {
        GetUserDefinedFunctionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            GetUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            GetUserDefinedFunctionError::InternalService(ref cause) => cause,
            GetUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            GetUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
            GetUserDefinedFunctionError::Validation(ref cause) => cause,
            GetUserDefinedFunctionError::Credentials(ref err) => err.description(),
            GetUserDefinedFunctionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetUserDefinedFunctionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUserDefinedFunctions
#[derive(Debug, PartialEq)]
pub enum GetUserDefinedFunctionsError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetUserDefinedFunctionsError {
    pub fn from_body(body: &str) -> GetUserDefinedFunctionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        GetUserDefinedFunctionsError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetUserDefinedFunctionsError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetUserDefinedFunctionsError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        GetUserDefinedFunctionsError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetUserDefinedFunctionsError::Validation(error_message.to_string())
                    }
                    _ => GetUserDefinedFunctionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetUserDefinedFunctionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetUserDefinedFunctionsError {
    fn from(err: serde_json::error::Error) -> GetUserDefinedFunctionsError {
        GetUserDefinedFunctionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUserDefinedFunctionsError {
    fn from(err: CredentialsError) -> GetUserDefinedFunctionsError {
        GetUserDefinedFunctionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUserDefinedFunctionsError {
    fn from(err: HttpDispatchError) -> GetUserDefinedFunctionsError {
        GetUserDefinedFunctionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetUserDefinedFunctionsError {
    fn from(err: io::Error) -> GetUserDefinedFunctionsError {
        GetUserDefinedFunctionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetUserDefinedFunctionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserDefinedFunctionsError {
    fn description(&self) -> &str {
        match *self {
            GetUserDefinedFunctionsError::EntityNotFound(ref cause) => cause,
            GetUserDefinedFunctionsError::InternalService(ref cause) => cause,
            GetUserDefinedFunctionsError::InvalidInput(ref cause) => cause,
            GetUserDefinedFunctionsError::OperationTimeout(ref cause) => cause,
            GetUserDefinedFunctionsError::Validation(ref cause) => cause,
            GetUserDefinedFunctionsError::Credentials(ref err) => err.description(),
            GetUserDefinedFunctionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetUserDefinedFunctionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportCatalogToGlue
#[derive(Debug, PartialEq)]
pub enum ImportCatalogToGlueError {
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ImportCatalogToGlueError {
    pub fn from_body(body: &str) -> ImportCatalogToGlueError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => {
                        ImportCatalogToGlueError::InternalService(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        ImportCatalogToGlueError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        ImportCatalogToGlueError::Validation(error_message.to_string())
                    }
                    _ => ImportCatalogToGlueError::Unknown(String::from(body)),
                }
            }
            Err(_) => ImportCatalogToGlueError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ImportCatalogToGlueError {
    fn from(err: serde_json::error::Error) -> ImportCatalogToGlueError {
        ImportCatalogToGlueError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ImportCatalogToGlueError {
    fn from(err: CredentialsError) -> ImportCatalogToGlueError {
        ImportCatalogToGlueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ImportCatalogToGlueError {
    fn from(err: HttpDispatchError) -> ImportCatalogToGlueError {
        ImportCatalogToGlueError::HttpDispatch(err)
    }
}
impl From<io::Error> for ImportCatalogToGlueError {
    fn from(err: io::Error) -> ImportCatalogToGlueError {
        ImportCatalogToGlueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ImportCatalogToGlueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportCatalogToGlueError {
    fn description(&self) -> &str {
        match *self {
            ImportCatalogToGlueError::InternalService(ref cause) => cause,
            ImportCatalogToGlueError::OperationTimeout(ref cause) => cause,
            ImportCatalogToGlueError::Validation(ref cause) => cause,
            ImportCatalogToGlueError::Credentials(ref err) => err.description(),
            ImportCatalogToGlueError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ImportCatalogToGlueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ResetJobBookmark
#[derive(Debug, PartialEq)]
pub enum ResetJobBookmarkError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ResetJobBookmarkError {
    pub fn from_body(body: &str) -> ResetJobBookmarkError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        ResetJobBookmarkError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ResetJobBookmarkError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ResetJobBookmarkError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        ResetJobBookmarkError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        ResetJobBookmarkError::Validation(error_message.to_string())
                    }
                    _ => ResetJobBookmarkError::Unknown(String::from(body)),
                }
            }
            Err(_) => ResetJobBookmarkError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ResetJobBookmarkError {
    fn from(err: serde_json::error::Error) -> ResetJobBookmarkError {
        ResetJobBookmarkError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ResetJobBookmarkError {
    fn from(err: CredentialsError) -> ResetJobBookmarkError {
        ResetJobBookmarkError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResetJobBookmarkError {
    fn from(err: HttpDispatchError) -> ResetJobBookmarkError {
        ResetJobBookmarkError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResetJobBookmarkError {
    fn from(err: io::Error) -> ResetJobBookmarkError {
        ResetJobBookmarkError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResetJobBookmarkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetJobBookmarkError {
    fn description(&self) -> &str {
        match *self {
            ResetJobBookmarkError::EntityNotFound(ref cause) => cause,
            ResetJobBookmarkError::InternalService(ref cause) => cause,
            ResetJobBookmarkError::InvalidInput(ref cause) => cause,
            ResetJobBookmarkError::OperationTimeout(ref cause) => cause,
            ResetJobBookmarkError::Validation(ref cause) => cause,
            ResetJobBookmarkError::Credentials(ref err) => err.description(),
            ResetJobBookmarkError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ResetJobBookmarkError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartCrawler
#[derive(Debug, PartialEq)]
pub enum StartCrawlerError {
    ///<p>The operation cannot be performed because the crawler is already running.</p>
    CrawlerRunning(String),
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartCrawlerError {
    pub fn from_body(body: &str) -> StartCrawlerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CrawlerRunningException" => {
                        StartCrawlerError::CrawlerRunning(String::from(error_message))
                    }
                    "EntityNotFoundException" => {
                        StartCrawlerError::EntityNotFound(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        StartCrawlerError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartCrawlerError::Validation(error_message.to_string())
                    }
                    _ => StartCrawlerError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartCrawlerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartCrawlerError {
    fn from(err: serde_json::error::Error) -> StartCrawlerError {
        StartCrawlerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartCrawlerError {
    fn from(err: CredentialsError) -> StartCrawlerError {
        StartCrawlerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartCrawlerError {
    fn from(err: HttpDispatchError) -> StartCrawlerError {
        StartCrawlerError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartCrawlerError {
    fn from(err: io::Error) -> StartCrawlerError {
        StartCrawlerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartCrawlerError {
    fn description(&self) -> &str {
        match *self {
            StartCrawlerError::CrawlerRunning(ref cause) => cause,
            StartCrawlerError::EntityNotFound(ref cause) => cause,
            StartCrawlerError::OperationTimeout(ref cause) => cause,
            StartCrawlerError::Validation(ref cause) => cause,
            StartCrawlerError::Credentials(ref err) => err.description(),
            StartCrawlerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartCrawlerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartCrawlerSchedule
#[derive(Debug, PartialEq)]
pub enum StartCrawlerScheduleError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>There is no applicable schedule.</p>
    NoSchedule(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>The specified scheduler is already running.</p>
    SchedulerRunning(String),
    ///<p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartCrawlerScheduleError {
    pub fn from_body(body: &str) -> StartCrawlerScheduleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        StartCrawlerScheduleError::EntityNotFound(String::from(error_message))
                    }
                    "NoScheduleException" => {
                        StartCrawlerScheduleError::NoSchedule(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        StartCrawlerScheduleError::OperationTimeout(String::from(error_message))
                    }
                    "SchedulerRunningException" => {
                        StartCrawlerScheduleError::SchedulerRunning(String::from(error_message))
                    }
                    "SchedulerTransitioningException" => StartCrawlerScheduleError::SchedulerTransitioning(String::from(error_message)),
                    "ValidationException" => {
                        StartCrawlerScheduleError::Validation(error_message.to_string())
                    }
                    _ => StartCrawlerScheduleError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartCrawlerScheduleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartCrawlerScheduleError {
    fn from(err: serde_json::error::Error) -> StartCrawlerScheduleError {
        StartCrawlerScheduleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartCrawlerScheduleError {
    fn from(err: CredentialsError) -> StartCrawlerScheduleError {
        StartCrawlerScheduleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartCrawlerScheduleError {
    fn from(err: HttpDispatchError) -> StartCrawlerScheduleError {
        StartCrawlerScheduleError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartCrawlerScheduleError {
    fn from(err: io::Error) -> StartCrawlerScheduleError {
        StartCrawlerScheduleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartCrawlerScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartCrawlerScheduleError {
    fn description(&self) -> &str {
        match *self {
            StartCrawlerScheduleError::EntityNotFound(ref cause) => cause,
            StartCrawlerScheduleError::NoSchedule(ref cause) => cause,
            StartCrawlerScheduleError::OperationTimeout(ref cause) => cause,
            StartCrawlerScheduleError::SchedulerRunning(ref cause) => cause,
            StartCrawlerScheduleError::SchedulerTransitioning(ref cause) => cause,
            StartCrawlerScheduleError::Validation(ref cause) => cause,
            StartCrawlerScheduleError::Credentials(ref err) => err.description(),
            StartCrawlerScheduleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartCrawlerScheduleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartJobRun
#[derive(Debug, PartialEq)]
pub enum StartJobRunError {
    ///<p>Too many jobs are being run concurrently.</p>
    ConcurrentRunsExceeded(String),
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartJobRunError {
    pub fn from_body(body: &str) -> StartJobRunError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentRunsExceededException" => {
                        StartJobRunError::ConcurrentRunsExceeded(String::from(error_message))
                    }
                    "EntityNotFoundException" => {
                        StartJobRunError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        StartJobRunError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        StartJobRunError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        StartJobRunError::OperationTimeout(String::from(error_message))
                    }
                    "ResourceNumberLimitExceededException" => {
                        StartJobRunError::ResourceNumberLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartJobRunError::Validation(error_message.to_string())
                    }
                    _ => StartJobRunError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartJobRunError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartJobRunError {
    fn from(err: serde_json::error::Error) -> StartJobRunError {
        StartJobRunError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartJobRunError {
    fn from(err: CredentialsError) -> StartJobRunError {
        StartJobRunError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartJobRunError {
    fn from(err: HttpDispatchError) -> StartJobRunError {
        StartJobRunError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartJobRunError {
    fn from(err: io::Error) -> StartJobRunError {
        StartJobRunError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartJobRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartJobRunError {
    fn description(&self) -> &str {
        match *self {
            StartJobRunError::ConcurrentRunsExceeded(ref cause) => cause,
            StartJobRunError::EntityNotFound(ref cause) => cause,
            StartJobRunError::InternalService(ref cause) => cause,
            StartJobRunError::InvalidInput(ref cause) => cause,
            StartJobRunError::OperationTimeout(ref cause) => cause,
            StartJobRunError::ResourceNumberLimitExceeded(ref cause) => cause,
            StartJobRunError::Validation(ref cause) => cause,
            StartJobRunError::Credentials(ref err) => err.description(),
            StartJobRunError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartJobRunError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartTrigger
#[derive(Debug, PartialEq)]
pub enum StartTriggerError {
    ///<p>Too many jobs are being run concurrently.</p>
    ConcurrentRunsExceeded(String),
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartTriggerError {
    pub fn from_body(body: &str) -> StartTriggerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentRunsExceededException" => {
                        StartTriggerError::ConcurrentRunsExceeded(String::from(error_message))
                    }
                    "EntityNotFoundException" => {
                        StartTriggerError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        StartTriggerError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        StartTriggerError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        StartTriggerError::OperationTimeout(String::from(error_message))
                    }
                    "ResourceNumberLimitExceededException" => {
                        StartTriggerError::ResourceNumberLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartTriggerError::Validation(error_message.to_string())
                    }
                    _ => StartTriggerError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartTriggerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartTriggerError {
    fn from(err: serde_json::error::Error) -> StartTriggerError {
        StartTriggerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartTriggerError {
    fn from(err: CredentialsError) -> StartTriggerError {
        StartTriggerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartTriggerError {
    fn from(err: HttpDispatchError) -> StartTriggerError {
        StartTriggerError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartTriggerError {
    fn from(err: io::Error) -> StartTriggerError {
        StartTriggerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartTriggerError {
    fn description(&self) -> &str {
        match *self {
            StartTriggerError::ConcurrentRunsExceeded(ref cause) => cause,
            StartTriggerError::EntityNotFound(ref cause) => cause,
            StartTriggerError::InternalService(ref cause) => cause,
            StartTriggerError::InvalidInput(ref cause) => cause,
            StartTriggerError::OperationTimeout(ref cause) => cause,
            StartTriggerError::ResourceNumberLimitExceeded(ref cause) => cause,
            StartTriggerError::Validation(ref cause) => cause,
            StartTriggerError::Credentials(ref err) => err.description(),
            StartTriggerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartTriggerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopCrawler
#[derive(Debug, PartialEq)]
pub enum StopCrawlerError {
    ///<p>The specified crawler is not running.</p>
    CrawlerNotRunning(String),
    ///<p>The specified crawler is stopping.</p>
    CrawlerStopping(String),
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StopCrawlerError {
    pub fn from_body(body: &str) -> StopCrawlerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CrawlerNotRunningException" => {
                        StopCrawlerError::CrawlerNotRunning(String::from(error_message))
                    }
                    "CrawlerStoppingException" => {
                        StopCrawlerError::CrawlerStopping(String::from(error_message))
                    }
                    "EntityNotFoundException" => {
                        StopCrawlerError::EntityNotFound(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        StopCrawlerError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopCrawlerError::Validation(error_message.to_string())
                    }
                    _ => StopCrawlerError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopCrawlerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopCrawlerError {
    fn from(err: serde_json::error::Error) -> StopCrawlerError {
        StopCrawlerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopCrawlerError {
    fn from(err: CredentialsError) -> StopCrawlerError {
        StopCrawlerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopCrawlerError {
    fn from(err: HttpDispatchError) -> StopCrawlerError {
        StopCrawlerError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopCrawlerError {
    fn from(err: io::Error) -> StopCrawlerError {
        StopCrawlerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopCrawlerError {
    fn description(&self) -> &str {
        match *self {
            StopCrawlerError::CrawlerNotRunning(ref cause) => cause,
            StopCrawlerError::CrawlerStopping(ref cause) => cause,
            StopCrawlerError::EntityNotFound(ref cause) => cause,
            StopCrawlerError::OperationTimeout(ref cause) => cause,
            StopCrawlerError::Validation(ref cause) => cause,
            StopCrawlerError::Credentials(ref err) => err.description(),
            StopCrawlerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopCrawlerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopCrawlerSchedule
#[derive(Debug, PartialEq)]
pub enum StopCrawlerScheduleError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>The specified scheduler is not running.</p>
    SchedulerNotRunning(String),
    ///<p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StopCrawlerScheduleError {
    pub fn from_body(body: &str) -> StopCrawlerScheduleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        StopCrawlerScheduleError::EntityNotFound(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        StopCrawlerScheduleError::OperationTimeout(String::from(error_message))
                    }
                    "SchedulerNotRunningException" => {
                        StopCrawlerScheduleError::SchedulerNotRunning(String::from(error_message))
                    }
                    "SchedulerTransitioningException" => StopCrawlerScheduleError::SchedulerTransitioning(String::from(error_message)),
                    "ValidationException" => {
                        StopCrawlerScheduleError::Validation(error_message.to_string())
                    }
                    _ => StopCrawlerScheduleError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopCrawlerScheduleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopCrawlerScheduleError {
    fn from(err: serde_json::error::Error) -> StopCrawlerScheduleError {
        StopCrawlerScheduleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopCrawlerScheduleError {
    fn from(err: CredentialsError) -> StopCrawlerScheduleError {
        StopCrawlerScheduleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopCrawlerScheduleError {
    fn from(err: HttpDispatchError) -> StopCrawlerScheduleError {
        StopCrawlerScheduleError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopCrawlerScheduleError {
    fn from(err: io::Error) -> StopCrawlerScheduleError {
        StopCrawlerScheduleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopCrawlerScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopCrawlerScheduleError {
    fn description(&self) -> &str {
        match *self {
            StopCrawlerScheduleError::EntityNotFound(ref cause) => cause,
            StopCrawlerScheduleError::OperationTimeout(ref cause) => cause,
            StopCrawlerScheduleError::SchedulerNotRunning(ref cause) => cause,
            StopCrawlerScheduleError::SchedulerTransitioning(ref cause) => cause,
            StopCrawlerScheduleError::Validation(ref cause) => cause,
            StopCrawlerScheduleError::Credentials(ref err) => err.description(),
            StopCrawlerScheduleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopCrawlerScheduleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopTrigger
#[derive(Debug, PartialEq)]
pub enum StopTriggerError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StopTriggerError {
    pub fn from_body(body: &str) -> StopTriggerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        StopTriggerError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        StopTriggerError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        StopTriggerError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        StopTriggerError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopTriggerError::Validation(error_message.to_string())
                    }
                    _ => StopTriggerError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopTriggerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopTriggerError {
    fn from(err: serde_json::error::Error) -> StopTriggerError {
        StopTriggerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopTriggerError {
    fn from(err: CredentialsError) -> StopTriggerError {
        StopTriggerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopTriggerError {
    fn from(err: HttpDispatchError) -> StopTriggerError {
        StopTriggerError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopTriggerError {
    fn from(err: io::Error) -> StopTriggerError {
        StopTriggerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopTriggerError {
    fn description(&self) -> &str {
        match *self {
            StopTriggerError::EntityNotFound(ref cause) => cause,
            StopTriggerError::InternalService(ref cause) => cause,
            StopTriggerError::InvalidInput(ref cause) => cause,
            StopTriggerError::OperationTimeout(ref cause) => cause,
            StopTriggerError::Validation(ref cause) => cause,
            StopTriggerError::Credentials(ref err) => err.description(),
            StopTriggerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopTriggerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateClassifier
#[derive(Debug, PartialEq)]
pub enum UpdateClassifierError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>There was a version conflict.</p>
    VersionMismatch(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateClassifierError {
    pub fn from_body(body: &str) -> UpdateClassifierError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        UpdateClassifierError::EntityNotFound(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateClassifierError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        UpdateClassifierError::OperationTimeout(String::from(error_message))
                    }
                    "VersionMismatchException" => {
                        UpdateClassifierError::VersionMismatch(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateClassifierError::Validation(error_message.to_string())
                    }
                    _ => UpdateClassifierError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateClassifierError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateClassifierError {
    fn from(err: serde_json::error::Error) -> UpdateClassifierError {
        UpdateClassifierError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateClassifierError {
    fn from(err: CredentialsError) -> UpdateClassifierError {
        UpdateClassifierError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateClassifierError {
    fn from(err: HttpDispatchError) -> UpdateClassifierError {
        UpdateClassifierError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateClassifierError {
    fn from(err: io::Error) -> UpdateClassifierError {
        UpdateClassifierError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateClassifierError {
    fn description(&self) -> &str {
        match *self {
            UpdateClassifierError::EntityNotFound(ref cause) => cause,
            UpdateClassifierError::InvalidInput(ref cause) => cause,
            UpdateClassifierError::OperationTimeout(ref cause) => cause,
            UpdateClassifierError::VersionMismatch(ref cause) => cause,
            UpdateClassifierError::Validation(ref cause) => cause,
            UpdateClassifierError::Credentials(ref err) => err.description(),
            UpdateClassifierError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateClassifierError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConnection
#[derive(Debug, PartialEq)]
pub enum UpdateConnectionError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateConnectionError {
    pub fn from_body(body: &str) -> UpdateConnectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        UpdateConnectionError::EntityNotFound(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateConnectionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        UpdateConnectionError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateConnectionError::Validation(error_message.to_string())
                    }
                    _ => UpdateConnectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateConnectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateConnectionError {
    fn from(err: serde_json::error::Error) -> UpdateConnectionError {
        UpdateConnectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateConnectionError {
    fn from(err: CredentialsError) -> UpdateConnectionError {
        UpdateConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateConnectionError {
    fn from(err: HttpDispatchError) -> UpdateConnectionError {
        UpdateConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateConnectionError {
    fn from(err: io::Error) -> UpdateConnectionError {
        UpdateConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConnectionError {
    fn description(&self) -> &str {
        match *self {
            UpdateConnectionError::EntityNotFound(ref cause) => cause,
            UpdateConnectionError::InvalidInput(ref cause) => cause,
            UpdateConnectionError::OperationTimeout(ref cause) => cause,
            UpdateConnectionError::Validation(ref cause) => cause,
            UpdateConnectionError::Credentials(ref err) => err.description(),
            UpdateConnectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateConnectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCrawler
#[derive(Debug, PartialEq)]
pub enum UpdateCrawlerError {
    ///<p>The operation cannot be performed because the crawler is already running.</p>
    CrawlerRunning(String),
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>There was a version conflict.</p>
    VersionMismatch(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateCrawlerError {
    pub fn from_body(body: &str) -> UpdateCrawlerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CrawlerRunningException" => {
                        UpdateCrawlerError::CrawlerRunning(String::from(error_message))
                    }
                    "EntityNotFoundException" => {
                        UpdateCrawlerError::EntityNotFound(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateCrawlerError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        UpdateCrawlerError::OperationTimeout(String::from(error_message))
                    }
                    "VersionMismatchException" => {
                        UpdateCrawlerError::VersionMismatch(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateCrawlerError::Validation(error_message.to_string())
                    }
                    _ => UpdateCrawlerError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateCrawlerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateCrawlerError {
    fn from(err: serde_json::error::Error) -> UpdateCrawlerError {
        UpdateCrawlerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCrawlerError {
    fn from(err: CredentialsError) -> UpdateCrawlerError {
        UpdateCrawlerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCrawlerError {
    fn from(err: HttpDispatchError) -> UpdateCrawlerError {
        UpdateCrawlerError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCrawlerError {
    fn from(err: io::Error) -> UpdateCrawlerError {
        UpdateCrawlerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCrawlerError {
    fn description(&self) -> &str {
        match *self {
            UpdateCrawlerError::CrawlerRunning(ref cause) => cause,
            UpdateCrawlerError::EntityNotFound(ref cause) => cause,
            UpdateCrawlerError::InvalidInput(ref cause) => cause,
            UpdateCrawlerError::OperationTimeout(ref cause) => cause,
            UpdateCrawlerError::VersionMismatch(ref cause) => cause,
            UpdateCrawlerError::Validation(ref cause) => cause,
            UpdateCrawlerError::Credentials(ref err) => err.description(),
            UpdateCrawlerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateCrawlerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCrawlerSchedule
#[derive(Debug, PartialEq)]
pub enum UpdateCrawlerScheduleError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    ///<p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
    ///<p>There was a version conflict.</p>
    VersionMismatch(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateCrawlerScheduleError {
    pub fn from_body(body: &str) -> UpdateCrawlerScheduleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        UpdateCrawlerScheduleError::EntityNotFound(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateCrawlerScheduleError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        UpdateCrawlerScheduleError::OperationTimeout(String::from(error_message))
                    }
                    "SchedulerTransitioningException" => UpdateCrawlerScheduleError::SchedulerTransitioning(String::from(error_message)),
                    "VersionMismatchException" => {
                        UpdateCrawlerScheduleError::VersionMismatch(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateCrawlerScheduleError::Validation(error_message.to_string())
                    }
                    _ => UpdateCrawlerScheduleError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateCrawlerScheduleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateCrawlerScheduleError {
    fn from(err: serde_json::error::Error) -> UpdateCrawlerScheduleError {
        UpdateCrawlerScheduleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCrawlerScheduleError {
    fn from(err: CredentialsError) -> UpdateCrawlerScheduleError {
        UpdateCrawlerScheduleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCrawlerScheduleError {
    fn from(err: HttpDispatchError) -> UpdateCrawlerScheduleError {
        UpdateCrawlerScheduleError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCrawlerScheduleError {
    fn from(err: io::Error) -> UpdateCrawlerScheduleError {
        UpdateCrawlerScheduleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCrawlerScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCrawlerScheduleError {
    fn description(&self) -> &str {
        match *self {
            UpdateCrawlerScheduleError::EntityNotFound(ref cause) => cause,
            UpdateCrawlerScheduleError::InvalidInput(ref cause) => cause,
            UpdateCrawlerScheduleError::OperationTimeout(ref cause) => cause,
            UpdateCrawlerScheduleError::SchedulerTransitioning(ref cause) => cause,
            UpdateCrawlerScheduleError::VersionMismatch(ref cause) => cause,
            UpdateCrawlerScheduleError::Validation(ref cause) => cause,
            UpdateCrawlerScheduleError::Credentials(ref err) => err.description(),
            UpdateCrawlerScheduleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateCrawlerScheduleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDatabase
#[derive(Debug, PartialEq)]
pub enum UpdateDatabaseError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateDatabaseError {
    pub fn from_body(body: &str) -> UpdateDatabaseError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        UpdateDatabaseError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateDatabaseError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateDatabaseError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        UpdateDatabaseError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDatabaseError::Validation(error_message.to_string())
                    }
                    _ => UpdateDatabaseError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDatabaseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDatabaseError {
    fn from(err: serde_json::error::Error) -> UpdateDatabaseError {
        UpdateDatabaseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDatabaseError {
    fn from(err: CredentialsError) -> UpdateDatabaseError {
        UpdateDatabaseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDatabaseError {
    fn from(err: HttpDispatchError) -> UpdateDatabaseError {
        UpdateDatabaseError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDatabaseError {
    fn from(err: io::Error) -> UpdateDatabaseError {
        UpdateDatabaseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDatabaseError {
    fn description(&self) -> &str {
        match *self {
            UpdateDatabaseError::EntityNotFound(ref cause) => cause,
            UpdateDatabaseError::InternalService(ref cause) => cause,
            UpdateDatabaseError::InvalidInput(ref cause) => cause,
            UpdateDatabaseError::OperationTimeout(ref cause) => cause,
            UpdateDatabaseError::Validation(ref cause) => cause,
            UpdateDatabaseError::Credentials(ref err) => err.description(),
            UpdateDatabaseError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDatabaseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDevEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateDevEndpointError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateDevEndpointError {
    pub fn from_body(body: &str) -> UpdateDevEndpointError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        UpdateDevEndpointError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateDevEndpointError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateDevEndpointError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        UpdateDevEndpointError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDevEndpointError::Validation(error_message.to_string())
                    }
                    _ => UpdateDevEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDevEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDevEndpointError {
    fn from(err: serde_json::error::Error) -> UpdateDevEndpointError {
        UpdateDevEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDevEndpointError {
    fn from(err: CredentialsError) -> UpdateDevEndpointError {
        UpdateDevEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDevEndpointError {
    fn from(err: HttpDispatchError) -> UpdateDevEndpointError {
        UpdateDevEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDevEndpointError {
    fn from(err: io::Error) -> UpdateDevEndpointError {
        UpdateDevEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            UpdateDevEndpointError::EntityNotFound(ref cause) => cause,
            UpdateDevEndpointError::InternalService(ref cause) => cause,
            UpdateDevEndpointError::InvalidInput(ref cause) => cause,
            UpdateDevEndpointError::OperationTimeout(ref cause) => cause,
            UpdateDevEndpointError::Validation(ref cause) => cause,
            UpdateDevEndpointError::Credentials(ref err) => err.description(),
            UpdateDevEndpointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDevEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateJob
#[derive(Debug, PartialEq)]
pub enum UpdateJobError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateJobError {
    pub fn from_body(body: &str) -> UpdateJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        UpdateJobError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateJobError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateJobError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        UpdateJobError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => UpdateJobError::Validation(error_message.to_string()),
                    _ => UpdateJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateJobError {
    fn from(err: serde_json::error::Error) -> UpdateJobError {
        UpdateJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateJobError {
    fn from(err: CredentialsError) -> UpdateJobError {
        UpdateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateJobError {
    fn from(err: HttpDispatchError) -> UpdateJobError {
        UpdateJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateJobError {
    fn from(err: io::Error) -> UpdateJobError {
        UpdateJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateJobError {
    fn description(&self) -> &str {
        match *self {
            UpdateJobError::EntityNotFound(ref cause) => cause,
            UpdateJobError::InternalService(ref cause) => cause,
            UpdateJobError::InvalidInput(ref cause) => cause,
            UpdateJobError::OperationTimeout(ref cause) => cause,
            UpdateJobError::Validation(ref cause) => cause,
            UpdateJobError::Credentials(ref err) => err.description(),
            UpdateJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePartition
#[derive(Debug, PartialEq)]
pub enum UpdatePartitionError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdatePartitionError {
    pub fn from_body(body: &str) -> UpdatePartitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        UpdatePartitionError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdatePartitionError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdatePartitionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        UpdatePartitionError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdatePartitionError::Validation(error_message.to_string())
                    }
                    _ => UpdatePartitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdatePartitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdatePartitionError {
    fn from(err: serde_json::error::Error) -> UpdatePartitionError {
        UpdatePartitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdatePartitionError {
    fn from(err: CredentialsError) -> UpdatePartitionError {
        UpdatePartitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdatePartitionError {
    fn from(err: HttpDispatchError) -> UpdatePartitionError {
        UpdatePartitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdatePartitionError {
    fn from(err: io::Error) -> UpdatePartitionError {
        UpdatePartitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdatePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePartitionError {
    fn description(&self) -> &str {
        match *self {
            UpdatePartitionError::EntityNotFound(ref cause) => cause,
            UpdatePartitionError::InternalService(ref cause) => cause,
            UpdatePartitionError::InvalidInput(ref cause) => cause,
            UpdatePartitionError::OperationTimeout(ref cause) => cause,
            UpdatePartitionError::Validation(ref cause) => cause,
            UpdatePartitionError::Credentials(ref err) => err.description(),
            UpdatePartitionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdatePartitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTable
#[derive(Debug, PartialEq)]
pub enum UpdateTableError {
    ///<p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateTableError {
    pub fn from_body(body: &str) -> UpdateTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        UpdateTableError::ConcurrentModification(String::from(error_message))
                    }
                    "EntityNotFoundException" => {
                        UpdateTableError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateTableError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateTableError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        UpdateTableError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateTableError::Validation(error_message.to_string())
                    }
                    _ => UpdateTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateTableError {
    fn from(err: serde_json::error::Error) -> UpdateTableError {
        UpdateTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateTableError {
    fn from(err: CredentialsError) -> UpdateTableError {
        UpdateTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTableError {
    fn from(err: HttpDispatchError) -> UpdateTableError {
        UpdateTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTableError {
    fn from(err: io::Error) -> UpdateTableError {
        UpdateTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTableError {
    fn description(&self) -> &str {
        match *self {
            UpdateTableError::ConcurrentModification(ref cause) => cause,
            UpdateTableError::EntityNotFound(ref cause) => cause,
            UpdateTableError::InternalService(ref cause) => cause,
            UpdateTableError::InvalidInput(ref cause) => cause,
            UpdateTableError::OperationTimeout(ref cause) => cause,
            UpdateTableError::Validation(ref cause) => cause,
            UpdateTableError::Credentials(ref err) => err.description(),
            UpdateTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTrigger
#[derive(Debug, PartialEq)]
pub enum UpdateTriggerError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateTriggerError {
    pub fn from_body(body: &str) -> UpdateTriggerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        UpdateTriggerError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateTriggerError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateTriggerError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => {
                        UpdateTriggerError::OperationTimeout(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateTriggerError::Validation(error_message.to_string())
                    }
                    _ => UpdateTriggerError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateTriggerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateTriggerError {
    fn from(err: serde_json::error::Error) -> UpdateTriggerError {
        UpdateTriggerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateTriggerError {
    fn from(err: CredentialsError) -> UpdateTriggerError {
        UpdateTriggerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTriggerError {
    fn from(err: HttpDispatchError) -> UpdateTriggerError {
        UpdateTriggerError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTriggerError {
    fn from(err: io::Error) -> UpdateTriggerError {
        UpdateTriggerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTriggerError {
    fn description(&self) -> &str {
        match *self {
            UpdateTriggerError::EntityNotFound(ref cause) => cause,
            UpdateTriggerError::InternalService(ref cause) => cause,
            UpdateTriggerError::InvalidInput(ref cause) => cause,
            UpdateTriggerError::OperationTimeout(ref cause) => cause,
            UpdateTriggerError::Validation(ref cause) => cause,
            UpdateTriggerError::Credentials(ref err) => err.description(),
            UpdateTriggerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateTriggerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum UpdateUserDefinedFunctionError {
    ///<p>A specified entity does not exist</p>
    EntityNotFound(String),
    ///<p>An internal service error occurred.</p>
    InternalService(String),
    ///<p>The input provided was not valid.</p>
    InvalidInput(String),
    ///<p>The operation timed out.</p>
    OperationTimeout(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateUserDefinedFunctionError {
    pub fn from_body(body: &str) -> UpdateUserDefinedFunctionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EntityNotFoundException" => {
                        UpdateUserDefinedFunctionError::EntityNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateUserDefinedFunctionError::InternalService(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateUserDefinedFunctionError::InvalidInput(String::from(error_message))
                    }
                    "OperationTimeoutException" => UpdateUserDefinedFunctionError::OperationTimeout(String::from(error_message)),
                    "ValidationException" => {
                        UpdateUserDefinedFunctionError::Validation(error_message.to_string())
                    }
                    _ => UpdateUserDefinedFunctionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateUserDefinedFunctionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateUserDefinedFunctionError {
    fn from(err: serde_json::error::Error) -> UpdateUserDefinedFunctionError {
        UpdateUserDefinedFunctionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserDefinedFunctionError {
    fn from(err: CredentialsError) -> UpdateUserDefinedFunctionError {
        UpdateUserDefinedFunctionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserDefinedFunctionError {
    fn from(err: HttpDispatchError) -> UpdateUserDefinedFunctionError {
        UpdateUserDefinedFunctionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUserDefinedFunctionError {
    fn from(err: io::Error) -> UpdateUserDefinedFunctionError {
        UpdateUserDefinedFunctionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            UpdateUserDefinedFunctionError::InternalService(ref cause) => cause,
            UpdateUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            UpdateUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
            UpdateUserDefinedFunctionError::Validation(ref cause) => cause,
            UpdateUserDefinedFunctionError::Credentials(ref err) => err.description(),
            UpdateUserDefinedFunctionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateUserDefinedFunctionError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Glue API. AWS Glue clients implement this trait.
pub trait Glue {
    #[doc="<p>Creates one or more partitions in a batch operation.</p>"]
    fn batch_create_partition
        (&self,
         input: &BatchCreatePartitionRequest)
         -> Result<BatchCreatePartitionResponse, BatchCreatePartitionError>;


    #[doc="<p>Deletes a list of connection definitions from the Data Catalog.</p>"]
    fn batch_delete_connection
        (&self,
         input: &BatchDeleteConnectionRequest)
         -> Result<BatchDeleteConnectionResponse, BatchDeleteConnectionError>;


    #[doc="<p>Deletes one or more partitions in a batch operation.</p>"]
    fn batch_delete_partition
        (&self,
         input: &BatchDeletePartitionRequest)
         -> Result<BatchDeletePartitionResponse, BatchDeletePartitionError>;


    #[doc="<p>Deletes multiple tables at once.</p>"]
    fn batch_delete_table(&self,
                          input: &BatchDeleteTableRequest)
                          -> Result<BatchDeleteTableResponse, BatchDeleteTableError>;


    #[doc="<p>Retrieves partitions in a batch request.</p>"]
    fn batch_get_partition(&self,
                           input: &BatchGetPartitionRequest)
                           -> Result<BatchGetPartitionResponse, BatchGetPartitionError>;


    #[doc="<p>Creates a <code>Classifier</code> in the user's account.</p>"]
    fn create_classifier(&self,
                         input: &CreateClassifierRequest)
                         -> Result<CreateClassifierResponse, CreateClassifierError>;


    #[doc="<p>Creates a connection definition in the Data Catalog.</p>"]
    fn create_connection(&self,
                         input: &CreateConnectionRequest)
                         -> Result<CreateConnectionResponse, CreateConnectionError>;


    #[doc="<p>Creates a new <code>Crawler</code> with specified targets, role, configuration, and optional schedule. At least one crawl target must be specified, in either the <i>s3Targets</i> or the <i>jdbcTargets</i> field.</p>"]
    fn create_crawler(&self,
                      input: &CreateCrawlerRequest)
                      -> Result<CreateCrawlerResponse, CreateCrawlerError>;


    #[doc="<p>Creates a new database in a Data Catalog.</p>"]
    fn create_database(&self,
                       input: &CreateDatabaseRequest)
                       -> Result<CreateDatabaseResponse, CreateDatabaseError>;


    #[doc="<p>Creates a new DevEndpoint.</p>"]
    fn create_dev_endpoint(&self,
                           input: &CreateDevEndpointRequest)
                           -> Result<CreateDevEndpointResponse, CreateDevEndpointError>;


    #[doc="<p>Creates a new job.</p>"]
    fn create_job(&self, input: &CreateJobRequest) -> Result<CreateJobResponse, CreateJobError>;


    #[doc="<p>Creates a new partition.</p>"]
    fn create_partition(&self,
                        input: &CreatePartitionRequest)
                        -> Result<CreatePartitionResponse, CreatePartitionError>;


    #[doc="<p>Transforms a directed acyclic graph (DAG) into a Python script.</p>"]
    fn create_script(&self,
                     input: &CreateScriptRequest)
                     -> Result<CreateScriptResponse, CreateScriptError>;


    #[doc="<p>Creates a new table definition in the Data Catalog.</p>"]
    fn create_table(&self,
                    input: &CreateTableRequest)
                    -> Result<CreateTableResponse, CreateTableError>;


    #[doc="<p>Creates a new trigger.</p>"]
    fn create_trigger(&self,
                      input: &CreateTriggerRequest)
                      -> Result<CreateTriggerResponse, CreateTriggerError>;


    #[doc="<p>Creates a new function definition in the Data Catalog.</p>"]
    fn create_user_defined_function
        (&self,
         input: &CreateUserDefinedFunctionRequest)
         -> Result<CreateUserDefinedFunctionResponse, CreateUserDefinedFunctionError>;


    #[doc="<p>Removes a <code>Classifier</code> from the metadata store.</p>"]
    fn delete_classifier(&self,
                         input: &DeleteClassifierRequest)
                         -> Result<DeleteClassifierResponse, DeleteClassifierError>;


    #[doc="<p>Deletes a connection from the Data Catalog.</p>"]
    fn delete_connection(&self,
                         input: &DeleteConnectionRequest)
                         -> Result<DeleteConnectionResponse, DeleteConnectionError>;


    #[doc="<p>Removes a specified <code>Crawler</code> from the metadata store, unless the <code>Crawler</code> state is <code>RUNNING</code>.</p>"]
    fn delete_crawler(&self,
                      input: &DeleteCrawlerRequest)
                      -> Result<DeleteCrawlerResponse, DeleteCrawlerError>;


    #[doc="<p>Removes a specified Database from a Data Catalog.</p>"]
    fn delete_database(&self,
                       input: &DeleteDatabaseRequest)
                       -> Result<DeleteDatabaseResponse, DeleteDatabaseError>;


    #[doc="<p>Deletes a specified DevEndpoint.</p>"]
    fn delete_dev_endpoint(&self,
                           input: &DeleteDevEndpointRequest)
                           -> Result<DeleteDevEndpointResponse, DeleteDevEndpointError>;


    #[doc="<p>Deletes a specified job.</p>"]
    fn delete_job(&self, input: &DeleteJobRequest) -> Result<DeleteJobResponse, DeleteJobError>;


    #[doc="<p>Deletes a specified partition.</p>"]
    fn delete_partition(&self,
                        input: &DeletePartitionRequest)
                        -> Result<DeletePartitionResponse, DeletePartitionError>;


    #[doc="<p>Removes a table definition from the Data Catalog.</p>"]
    fn delete_table(&self,
                    input: &DeleteTableRequest)
                    -> Result<DeleteTableResponse, DeleteTableError>;


    #[doc="<p>Deletes a specified trigger.</p>"]
    fn delete_trigger(&self,
                      input: &DeleteTriggerRequest)
                      -> Result<DeleteTriggerResponse, DeleteTriggerError>;


    #[doc="<p>Deletes an existing function definition from the Data Catalog.</p>"]
    fn delete_user_defined_function
        (&self,
         input: &DeleteUserDefinedFunctionRequest)
         -> Result<DeleteUserDefinedFunctionResponse, DeleteUserDefinedFunctionError>;


    #[doc="<p>Retrieves the status of a migration operation.</p>"]
    fn get_catalog_import_status
        (&self,
         input: &GetCatalogImportStatusRequest)
         -> Result<GetCatalogImportStatusResponse, GetCatalogImportStatusError>;


    #[doc="<p>Retrieve a <code>Classifier</code> by name.</p>"]
    fn get_classifier(&self,
                      input: &GetClassifierRequest)
                      -> Result<GetClassifierResponse, GetClassifierError>;


    #[doc="<p>Lists all Classifier objects in the metadata store.</p>"]
    fn get_classifiers(&self,
                       input: &GetClassifiersRequest)
                       -> Result<GetClassifiersResponse, GetClassifiersError>;


    #[doc="<p>Retrieves a connection definition from the Data Catalog.</p>"]
    fn get_connection(&self,
                      input: &GetConnectionRequest)
                      -> Result<GetConnectionResponse, GetConnectionError>;


    #[doc="<p>Retrieves a list of connection definitions from the Data Catalog.</p>"]
    fn get_connections(&self,
                       input: &GetConnectionsRequest)
                       -> Result<GetConnectionsResponse, GetConnectionsError>;


    #[doc="<p>Retrieves metadata for a specified <code>Crawler</code>.</p>"]
    fn get_crawler(&self,
                   input: &GetCrawlerRequest)
                   -> Result<GetCrawlerResponse, GetCrawlerError>;


    #[doc="<p>Retrieves metrics about specified crawlers.</p>"]
    fn get_crawler_metrics(&self,
                           input: &GetCrawlerMetricsRequest)
                           -> Result<GetCrawlerMetricsResponse, GetCrawlerMetricsError>;


    #[doc="<p>Retrieves metadata for all <code>Crawlers</code> defined in the customer account.</p>"]
    fn get_crawlers(&self,
                    input: &GetCrawlersRequest)
                    -> Result<GetCrawlersResponse, GetCrawlersError>;


    #[doc="<p>Retrieves the definition of a specified database.</p>"]
    fn get_database(&self,
                    input: &GetDatabaseRequest)
                    -> Result<GetDatabaseResponse, GetDatabaseError>;


    #[doc="<p>Retrieves all Databases defined in a given Data Catalog.</p>"]
    fn get_databases(&self,
                     input: &GetDatabasesRequest)
                     -> Result<GetDatabasesResponse, GetDatabasesError>;


    #[doc="<p>Transforms a Python script into a directed acyclic graph (DAG). </p>"]
    fn get_dataflow_graph(&self,
                          input: &GetDataflowGraphRequest)
                          -> Result<GetDataflowGraphResponse, GetDataflowGraphError>;


    #[doc="<p>Retrieves information about a specified DevEndpoint.</p>"]
    fn get_dev_endpoint(&self,
                        input: &GetDevEndpointRequest)
                        -> Result<GetDevEndpointResponse, GetDevEndpointError>;


    #[doc="<p>Retrieves all the DevEndpoints in this AWS account.</p>"]
    fn get_dev_endpoints(&self,
                         input: &GetDevEndpointsRequest)
                         -> Result<GetDevEndpointsResponse, GetDevEndpointsError>;


    #[doc="<p>Retrieves an existing job definition.</p>"]
    fn get_job(&self, input: &GetJobRequest) -> Result<GetJobResponse, GetJobError>;


    #[doc="<p>Retrieves the metadata for a given job run.</p>"]
    fn get_job_run(&self, input: &GetJobRunRequest) -> Result<GetJobRunResponse, GetJobRunError>;


    #[doc="<p>Retrieves metadata for all runs of a given job.</p>"]
    fn get_job_runs(&self,
                    input: &GetJobRunsRequest)
                    -> Result<GetJobRunsResponse, GetJobRunsError>;


    #[doc="<p>Retrieves all current jobs.</p>"]
    fn get_jobs(&self, input: &GetJobsRequest) -> Result<GetJobsResponse, GetJobsError>;


    #[doc="<p>Creates mappings.</p>"]
    fn get_mapping(&self,
                   input: &GetMappingRequest)
                   -> Result<GetMappingResponse, GetMappingError>;


    #[doc="<p>Retrieves information about a specified partition.</p>"]
    fn get_partition(&self,
                     input: &GetPartitionRequest)
                     -> Result<GetPartitionResponse, GetPartitionError>;


    #[doc="<p>Retrieves information about the partitions in a table.</p>"]
    fn get_partitions(&self,
                      input: &GetPartitionsRequest)
                      -> Result<GetPartitionsResponse, GetPartitionsError>;


    #[doc="<p>Gets a Python script to perform a specified mapping.</p>"]
    fn get_plan(&self, input: &GetPlanRequest) -> Result<GetPlanResponse, GetPlanError>;


    #[doc="<p>Retrieves the <code>Table</code> definition in a Data Catalog for a specified table.</p>"]
    fn get_table(&self, input: &GetTableRequest) -> Result<GetTableResponse, GetTableError>;


    #[doc="<p>Retrieves a list of strings that identify available versions of a specified table.</p>"]
    fn get_table_versions(&self,
                          input: &GetTableVersionsRequest)
                          -> Result<GetTableVersionsResponse, GetTableVersionsError>;


    #[doc="<p>Retrieves the definitions of some or all of the tables in a given <code>Database</code>.</p>"]
    fn get_tables(&self, input: &GetTablesRequest) -> Result<GetTablesResponse, GetTablesError>;


    #[doc="<p>Retrieves the definition of a trigger.</p>"]
    fn get_trigger(&self,
                   input: &GetTriggerRequest)
                   -> Result<GetTriggerResponse, GetTriggerError>;


    #[doc="<p>Gets all the triggers associated with a job.</p>"]
    fn get_triggers(&self,
                    input: &GetTriggersRequest)
                    -> Result<GetTriggersResponse, GetTriggersError>;


    #[doc="<p>Retrieves a specified function definition from the Data Catalog.</p>"]
    fn get_user_defined_function
        (&self,
         input: &GetUserDefinedFunctionRequest)
         -> Result<GetUserDefinedFunctionResponse, GetUserDefinedFunctionError>;


    #[doc="<p>Retrieves a multiple function definitions from the Data Catalog.</p>"]
    fn get_user_defined_functions
        (&self,
         input: &GetUserDefinedFunctionsRequest)
         -> Result<GetUserDefinedFunctionsResponse, GetUserDefinedFunctionsError>;


    #[doc="<p>Imports an existing Athena Data Catalog to AWS Glue</p>"]
    fn import_catalog_to_glue(&self,
                              input: &ImportCatalogToGlueRequest)
                              -> Result<ImportCatalogToGlueResponse, ImportCatalogToGlueError>;


    #[doc="<p>Resets a bookmark entry.</p>"]
    fn reset_job_bookmark(&self,
                          input: &ResetJobBookmarkRequest)
                          -> Result<ResetJobBookmarkResponse, ResetJobBookmarkError>;


    #[doc="<p>Starts a crawl using the specified <code>Crawler</code>, regardless of what is scheduled. If the <code>Crawler</code> is already running, does nothing.</p>"]
    fn start_crawler(&self,
                     input: &StartCrawlerRequest)
                     -> Result<StartCrawlerResponse, StartCrawlerError>;


    #[doc="<p>Changes the schedule state of the specified crawler to <code>SCHEDULED</code>, unless the crawler is already running or the schedule state is already <code>SCHEDULED</code>.</p>"]
    fn start_crawler_schedule
        (&self,
         input: &StartCrawlerScheduleRequest)
         -> Result<StartCrawlerScheduleResponse, StartCrawlerScheduleError>;


    #[doc="<p>Runs a job.</p>"]
    fn start_job_run(&self,
                     input: &StartJobRunRequest)
                     -> Result<StartJobRunResponse, StartJobRunError>;


    #[doc="<p>Starts an existing trigger.</p>"]
    fn start_trigger(&self,
                     input: &StartTriggerRequest)
                     -> Result<StartTriggerResponse, StartTriggerError>;


    #[doc="<p>If the specified <code>Crawler</code> is running, stops the crawl.</p>"]
    fn stop_crawler(&self,
                    input: &StopCrawlerRequest)
                    -> Result<StopCrawlerResponse, StopCrawlerError>;


    #[doc="<p>Sets the schedule state of the specified crawler to <code>NOT_SCHEDULED</code>, but does not stop the crawler if it is already running.</p>"]
    fn stop_crawler_schedule(&self,
                             input: &StopCrawlerScheduleRequest)
                             -> Result<StopCrawlerScheduleResponse, StopCrawlerScheduleError>;


    #[doc="<p>Stops a specified trigger.</p>"]
    fn stop_trigger(&self,
                    input: &StopTriggerRequest)
                    -> Result<StopTriggerResponse, StopTriggerError>;


    #[doc="<p>Modifies an existing <code>Classifier</code>.</p>"]
    fn update_classifier(&self,
                         input: &UpdateClassifierRequest)
                         -> Result<UpdateClassifierResponse, UpdateClassifierError>;


    #[doc="<p>Updates a connection definition in the Data Catalog.</p>"]
    fn update_connection(&self,
                         input: &UpdateConnectionRequest)
                         -> Result<UpdateConnectionResponse, UpdateConnectionError>;


    #[doc="<p>Updates a <code>Crawler</code>. If a <code>Crawler</code> is running, you must stop it using <code>StopCrawler</code> before updating it.</p>"]
    fn update_crawler(&self,
                      input: &UpdateCrawlerRequest)
                      -> Result<UpdateCrawlerResponse, UpdateCrawlerError>;


    #[doc="<p>Updates the schedule of a crawler using a Cron expression. </p>"]
    fn update_crawler_schedule
        (&self,
         input: &UpdateCrawlerScheduleRequest)
         -> Result<UpdateCrawlerScheduleResponse, UpdateCrawlerScheduleError>;


    #[doc="<p>Updates an existing database definition in a Data Catalog.</p>"]
    fn update_database(&self,
                       input: &UpdateDatabaseRequest)
                       -> Result<UpdateDatabaseResponse, UpdateDatabaseError>;


    #[doc="<p>Updates a specified DevEndpoint.</p>"]
    fn update_dev_endpoint(&self,
                           input: &UpdateDevEndpointRequest)
                           -> Result<UpdateDevEndpointResponse, UpdateDevEndpointError>;


    #[doc="<p>Updates an existing job definition.</p>"]
    fn update_job(&self, input: &UpdateJobRequest) -> Result<UpdateJobResponse, UpdateJobError>;


    #[doc="<p>Updates a partition.</p>"]
    fn update_partition(&self,
                        input: &UpdatePartitionRequest)
                        -> Result<UpdatePartitionResponse, UpdatePartitionError>;


    #[doc="<p>Updates a metadata table in the Data Catalog.</p>"]
    fn update_table(&self,
                    input: &UpdateTableRequest)
                    -> Result<UpdateTableResponse, UpdateTableError>;


    #[doc="<p>Updates a trigger definition.</p>"]
    fn update_trigger(&self,
                      input: &UpdateTriggerRequest)
                      -> Result<UpdateTriggerResponse, UpdateTriggerError>;


    #[doc="<p>Updates an existing function definition in the Data Catalog.</p>"]
    fn update_user_defined_function
        (&self,
         input: &UpdateUserDefinedFunctionRequest)
         -> Result<UpdateUserDefinedFunctionResponse, UpdateUserDefinedFunctionError>;
}
/// A client for the AWS Glue API.
pub struct GlueClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> GlueClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        GlueClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Glue for GlueClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Creates one or more partitions in a batch operation.</p>"]
    fn batch_create_partition
        (&self,
         input: &BatchCreatePartitionRequest)
         -> Result<BatchCreatePartitionResponse, BatchCreatePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchCreatePartition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<BatchCreatePartitionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(BatchCreatePartitionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a list of connection definitions from the Data Catalog.</p>"]
    fn batch_delete_connection
        (&self,
         input: &BatchDeleteConnectionRequest)
         -> Result<BatchDeleteConnectionResponse, BatchDeleteConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchDeleteConnection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<BatchDeleteConnectionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(BatchDeleteConnectionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes one or more partitions in a batch operation.</p>"]
    fn batch_delete_partition
        (&self,
         input: &BatchDeletePartitionRequest)
         -> Result<BatchDeletePartitionResponse, BatchDeletePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchDeletePartition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<BatchDeletePartitionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(BatchDeletePartitionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes multiple tables at once.</p>"]
    fn batch_delete_table(&self,
                          input: &BatchDeleteTableRequest)
                          -> Result<BatchDeleteTableResponse, BatchDeleteTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchDeleteTable");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<BatchDeleteTableResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(BatchDeleteTableError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves partitions in a batch request.</p>"]
    fn batch_get_partition(&self,
                           input: &BatchGetPartitionRequest)
                           -> Result<BatchGetPartitionResponse, BatchGetPartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchGetPartition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<BatchGetPartitionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(BatchGetPartitionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a <code>Classifier</code> in the user's account.</p>"]
    fn create_classifier(&self,
                         input: &CreateClassifierRequest)
                         -> Result<CreateClassifierResponse, CreateClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateClassifier");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateClassifierResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateClassifierError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a connection definition in the Data Catalog.</p>"]
    fn create_connection(&self,
                         input: &CreateConnectionRequest)
                         -> Result<CreateConnectionResponse, CreateConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateConnection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateConnectionResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateConnectionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new <code>Crawler</code> with specified targets, role, configuration, and optional schedule. At least one crawl target must be specified, in either the <i>s3Targets</i> or the <i>jdbcTargets</i> field.</p>"]
    fn create_crawler(&self,
                      input: &CreateCrawlerRequest)
                      -> Result<CreateCrawlerResponse, CreateCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateCrawler");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateCrawlerResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateCrawlerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new database in a Data Catalog.</p>"]
    fn create_database(&self,
                       input: &CreateDatabaseRequest)
                       -> Result<CreateDatabaseResponse, CreateDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateDatabase");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateDatabaseResponse>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateDatabaseError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new DevEndpoint.</p>"]
    fn create_dev_endpoint(&self,
                           input: &CreateDevEndpointRequest)
                           -> Result<CreateDevEndpointResponse, CreateDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateDevEndpoint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateDevEndpointResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateDevEndpointError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new job.</p>"]
    fn create_job(&self, input: &CreateJobRequest) -> Result<CreateJobResponse, CreateJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateJob");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateJobResponse>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateJobError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new partition.</p>"]
    fn create_partition(&self,
                        input: &CreatePartitionRequest)
                        -> Result<CreatePartitionResponse, CreatePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreatePartition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreatePartitionResponse>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreatePartitionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Transforms a directed acyclic graph (DAG) into a Python script.</p>"]
    fn create_script(&self,
                     input: &CreateScriptRequest)
                     -> Result<CreateScriptResponse, CreateScriptError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateScript");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateScriptResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateScriptError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new table definition in the Data Catalog.</p>"]
    fn create_table(&self,
                    input: &CreateTableRequest)
                    -> Result<CreateTableResponse, CreateTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateTable");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateTableResponse>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateTableError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new trigger.</p>"]
    fn create_trigger(&self,
                      input: &CreateTriggerRequest)
                      -> Result<CreateTriggerResponse, CreateTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateTrigger");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateTriggerResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateTriggerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new function definition in the Data Catalog.</p>"]
    fn create_user_defined_function
        (&self,
         input: &CreateUserDefinedFunctionRequest)
         -> Result<CreateUserDefinedFunctionResponse, CreateUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateUserDefinedFunction");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateUserDefinedFunctionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateUserDefinedFunctionError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="<p>Removes a <code>Classifier</code> from the metadata store.</p>"]
    fn delete_classifier(&self,
                         input: &DeleteClassifierRequest)
                         -> Result<DeleteClassifierResponse, DeleteClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteClassifier");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteClassifierResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteClassifierError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a connection from the Data Catalog.</p>"]
    fn delete_connection(&self,
                         input: &DeleteConnectionRequest)
                         -> Result<DeleteConnectionResponse, DeleteConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteConnection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteConnectionResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteConnectionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Removes a specified <code>Crawler</code> from the metadata store, unless the <code>Crawler</code> state is <code>RUNNING</code>.</p>"]
    fn delete_crawler(&self,
                      input: &DeleteCrawlerRequest)
                      -> Result<DeleteCrawlerResponse, DeleteCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteCrawler");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteCrawlerResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteCrawlerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Removes a specified Database from a Data Catalog.</p>"]
    fn delete_database(&self,
                       input: &DeleteDatabaseRequest)
                       -> Result<DeleteDatabaseResponse, DeleteDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteDatabase");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteDatabaseResponse>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteDatabaseError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a specified DevEndpoint.</p>"]
    fn delete_dev_endpoint(&self,
                           input: &DeleteDevEndpointRequest)
                           -> Result<DeleteDevEndpointResponse, DeleteDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteDevEndpoint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteDevEndpointResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteDevEndpointError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a specified job.</p>"]
    fn delete_job(&self, input: &DeleteJobRequest) -> Result<DeleteJobResponse, DeleteJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteJob");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteJobResponse>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteJobError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a specified partition.</p>"]
    fn delete_partition(&self,
                        input: &DeletePartitionRequest)
                        -> Result<DeletePartitionResponse, DeletePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeletePartition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeletePartitionResponse>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeletePartitionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Removes a table definition from the Data Catalog.</p>"]
    fn delete_table(&self,
                    input: &DeleteTableRequest)
                    -> Result<DeleteTableResponse, DeleteTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteTable");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteTableResponse>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteTableError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a specified trigger.</p>"]
    fn delete_trigger(&self,
                      input: &DeleteTriggerRequest)
                      -> Result<DeleteTriggerResponse, DeleteTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteTrigger");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteTriggerResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteTriggerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes an existing function definition from the Data Catalog.</p>"]
    fn delete_user_defined_function
        (&self,
         input: &DeleteUserDefinedFunctionRequest)
         -> Result<DeleteUserDefinedFunctionResponse, DeleteUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteUserDefinedFunction");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteUserDefinedFunctionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteUserDefinedFunctionError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the status of a migration operation.</p>"]
    fn get_catalog_import_status
        (&self,
         input: &GetCatalogImportStatusRequest)
         -> Result<GetCatalogImportStatusResponse, GetCatalogImportStatusError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCatalogImportStatus");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetCatalogImportStatusResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCatalogImportStatusError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieve a <code>Classifier</code> by name.</p>"]
    fn get_classifier(&self,
                      input: &GetClassifierRequest)
                      -> Result<GetClassifierResponse, GetClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetClassifier");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetClassifierResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetClassifierError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists all Classifier objects in the metadata store.</p>"]
    fn get_classifiers(&self,
                       input: &GetClassifiersRequest)
                       -> Result<GetClassifiersResponse, GetClassifiersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetClassifiers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetClassifiersResponse>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetClassifiersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves a connection definition from the Data Catalog.</p>"]
    fn get_connection(&self,
                      input: &GetConnectionRequest)
                      -> Result<GetConnectionResponse, GetConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetConnection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetConnectionResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetConnectionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves a list of connection definitions from the Data Catalog.</p>"]
    fn get_connections(&self,
                       input: &GetConnectionsRequest)
                       -> Result<GetConnectionsResponse, GetConnectionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetConnections");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetConnectionsResponse>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetConnectionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves metadata for a specified <code>Crawler</code>.</p>"]
    fn get_crawler(&self,
                   input: &GetCrawlerRequest)
                   -> Result<GetCrawlerResponse, GetCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCrawler");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetCrawlerResponse>(String::from_utf8_lossy(&body)
                                                                  .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCrawlerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves metrics about specified crawlers.</p>"]
    fn get_crawler_metrics(&self,
                           input: &GetCrawlerMetricsRequest)
                           -> Result<GetCrawlerMetricsResponse, GetCrawlerMetricsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCrawlerMetrics");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetCrawlerMetricsResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCrawlerMetricsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves metadata for all <code>Crawlers</code> defined in the customer account.</p>"]
    fn get_crawlers(&self,
                    input: &GetCrawlersRequest)
                    -> Result<GetCrawlersResponse, GetCrawlersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCrawlers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetCrawlersResponse>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCrawlersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the definition of a specified database.</p>"]
    fn get_database(&self,
                    input: &GetDatabaseRequest)
                    -> Result<GetDatabaseResponse, GetDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDatabase");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetDatabaseResponse>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDatabaseError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves all Databases defined in a given Data Catalog.</p>"]
    fn get_databases(&self,
                     input: &GetDatabasesRequest)
                     -> Result<GetDatabasesResponse, GetDatabasesError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDatabases");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetDatabasesResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDatabasesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Transforms a Python script into a directed acyclic graph (DAG). </p>"]
    fn get_dataflow_graph(&self,
                          input: &GetDataflowGraphRequest)
                          -> Result<GetDataflowGraphResponse, GetDataflowGraphError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDataflowGraph");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetDataflowGraphResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDataflowGraphError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves information about a specified DevEndpoint.</p>"]
    fn get_dev_endpoint(&self,
                        input: &GetDevEndpointRequest)
                        -> Result<GetDevEndpointResponse, GetDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDevEndpoint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetDevEndpointResponse>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDevEndpointError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves all the DevEndpoints in this AWS account.</p>"]
    fn get_dev_endpoints(&self,
                         input: &GetDevEndpointsRequest)
                         -> Result<GetDevEndpointsResponse, GetDevEndpointsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDevEndpoints");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetDevEndpointsResponse>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDevEndpointsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves an existing job definition.</p>"]
    fn get_job(&self, input: &GetJobRequest) -> Result<GetJobResponse, GetJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJob");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetJobResponse>(String::from_utf8_lossy(&body).as_ref())
                       .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetJobError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the metadata for a given job run.</p>"]
    fn get_job_run(&self, input: &GetJobRunRequest) -> Result<GetJobRunResponse, GetJobRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJobRun");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetJobRunResponse>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetJobRunError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves metadata for all runs of a given job.</p>"]
    fn get_job_runs(&self,
                    input: &GetJobRunsRequest)
                    -> Result<GetJobRunsResponse, GetJobRunsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJobRuns");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetJobRunsResponse>(String::from_utf8_lossy(&body)
                                                                  .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetJobRunsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves all current jobs.</p>"]
    fn get_jobs(&self, input: &GetJobsRequest) -> Result<GetJobsResponse, GetJobsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJobs");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetJobsResponse>(String::from_utf8_lossy(&body).as_ref())
                       .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetJobsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates mappings.</p>"]
    fn get_mapping(&self,
                   input: &GetMappingRequest)
                   -> Result<GetMappingResponse, GetMappingError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetMapping");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetMappingResponse>(String::from_utf8_lossy(&body)
                                                                  .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetMappingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves information about a specified partition.</p>"]
    fn get_partition(&self,
                     input: &GetPartitionRequest)
                     -> Result<GetPartitionResponse, GetPartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetPartition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetPartitionResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetPartitionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves information about the partitions in a table.</p>"]
    fn get_partitions(&self,
                      input: &GetPartitionsRequest)
                      -> Result<GetPartitionsResponse, GetPartitionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetPartitions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetPartitionsResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetPartitionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets a Python script to perform a specified mapping.</p>"]
    fn get_plan(&self, input: &GetPlanRequest) -> Result<GetPlanResponse, GetPlanError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetPlan");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetPlanResponse>(String::from_utf8_lossy(&body).as_ref())
                       .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetPlanError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the <code>Table</code> definition in a Data Catalog for a specified table.</p>"]
    fn get_table(&self, input: &GetTableRequest) -> Result<GetTableResponse, GetTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTable");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetTableResponse>(String::from_utf8_lossy(&body)
                                                                .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetTableError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves a list of strings that identify available versions of a specified table.</p>"]
    fn get_table_versions(&self,
                          input: &GetTableVersionsRequest)
                          -> Result<GetTableVersionsResponse, GetTableVersionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTableVersions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetTableVersionsResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetTableVersionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the definitions of some or all of the tables in a given <code>Database</code>.</p>"]
    fn get_tables(&self, input: &GetTablesRequest) -> Result<GetTablesResponse, GetTablesError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTables");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetTablesResponse>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetTablesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the definition of a trigger.</p>"]
    fn get_trigger(&self,
                   input: &GetTriggerRequest)
                   -> Result<GetTriggerResponse, GetTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTrigger");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetTriggerResponse>(String::from_utf8_lossy(&body)
                                                                  .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetTriggerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets all the triggers associated with a job.</p>"]
    fn get_triggers(&self,
                    input: &GetTriggersRequest)
                    -> Result<GetTriggersResponse, GetTriggersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTriggers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetTriggersResponse>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetTriggersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves a specified function definition from the Data Catalog.</p>"]
    fn get_user_defined_function
        (&self,
         input: &GetUserDefinedFunctionRequest)
         -> Result<GetUserDefinedFunctionResponse, GetUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetUserDefinedFunction");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetUserDefinedFunctionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetUserDefinedFunctionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves a multiple function definitions from the Data Catalog.</p>"]
    fn get_user_defined_functions
        (&self,
         input: &GetUserDefinedFunctionsRequest)
         -> Result<GetUserDefinedFunctionsResponse, GetUserDefinedFunctionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetUserDefinedFunctions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetUserDefinedFunctionsResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetUserDefinedFunctionsError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }


    #[doc="<p>Imports an existing Athena Data Catalog to AWS Glue</p>"]
    fn import_catalog_to_glue(&self,
                              input: &ImportCatalogToGlueRequest)
                              -> Result<ImportCatalogToGlueResponse, ImportCatalogToGlueError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.ImportCatalogToGlue");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ImportCatalogToGlueResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ImportCatalogToGlueError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Resets a bookmark entry.</p>"]
    fn reset_job_bookmark(&self,
                          input: &ResetJobBookmarkRequest)
                          -> Result<ResetJobBookmarkResponse, ResetJobBookmarkError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.ResetJobBookmark");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ResetJobBookmarkResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ResetJobBookmarkError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Starts a crawl using the specified <code>Crawler</code>, regardless of what is scheduled. If the <code>Crawler</code> is already running, does nothing.</p>"]
    fn start_crawler(&self,
                     input: &StartCrawlerRequest)
                     -> Result<StartCrawlerResponse, StartCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartCrawler");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartCrawlerResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartCrawlerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Changes the schedule state of the specified crawler to <code>SCHEDULED</code>, unless the crawler is already running or the schedule state is already <code>SCHEDULED</code>.</p>"]
    fn start_crawler_schedule
        (&self,
         input: &StartCrawlerScheduleRequest)
         -> Result<StartCrawlerScheduleResponse, StartCrawlerScheduleError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartCrawlerSchedule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartCrawlerScheduleResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartCrawlerScheduleError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Runs a job.</p>"]
    fn start_job_run(&self,
                     input: &StartJobRunRequest)
                     -> Result<StartJobRunResponse, StartJobRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartJobRun");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartJobRunResponse>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartJobRunError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Starts an existing trigger.</p>"]
    fn start_trigger(&self,
                     input: &StartTriggerRequest)
                     -> Result<StartTriggerResponse, StartTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartTrigger");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartTriggerResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartTriggerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>If the specified <code>Crawler</code> is running, stops the crawl.</p>"]
    fn stop_crawler(&self,
                    input: &StopCrawlerRequest)
                    -> Result<StopCrawlerResponse, StopCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StopCrawler");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StopCrawlerResponse>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StopCrawlerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Sets the schedule state of the specified crawler to <code>NOT_SCHEDULED</code>, but does not stop the crawler if it is already running.</p>"]
    fn stop_crawler_schedule(&self,
                             input: &StopCrawlerScheduleRequest)
                             -> Result<StopCrawlerScheduleResponse, StopCrawlerScheduleError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StopCrawlerSchedule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StopCrawlerScheduleResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StopCrawlerScheduleError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Stops a specified trigger.</p>"]
    fn stop_trigger(&self,
                    input: &StopTriggerRequest)
                    -> Result<StopTriggerResponse, StopTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StopTrigger");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StopTriggerResponse>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StopTriggerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Modifies an existing <code>Classifier</code>.</p>"]
    fn update_classifier(&self,
                         input: &UpdateClassifierRequest)
                         -> Result<UpdateClassifierResponse, UpdateClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateClassifier");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateClassifierResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateClassifierError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a connection definition in the Data Catalog.</p>"]
    fn update_connection(&self,
                         input: &UpdateConnectionRequest)
                         -> Result<UpdateConnectionResponse, UpdateConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateConnection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateConnectionResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateConnectionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a <code>Crawler</code>. If a <code>Crawler</code> is running, you must stop it using <code>StopCrawler</code> before updating it.</p>"]
    fn update_crawler(&self,
                      input: &UpdateCrawlerRequest)
                      -> Result<UpdateCrawlerResponse, UpdateCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateCrawler");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateCrawlerResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateCrawlerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates the schedule of a crawler using a Cron expression. </p>"]
    fn update_crawler_schedule
        (&self,
         input: &UpdateCrawlerScheduleRequest)
         -> Result<UpdateCrawlerScheduleResponse, UpdateCrawlerScheduleError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateCrawlerSchedule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateCrawlerScheduleResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateCrawlerScheduleError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates an existing database definition in a Data Catalog.</p>"]
    fn update_database(&self,
                       input: &UpdateDatabaseRequest)
                       -> Result<UpdateDatabaseResponse, UpdateDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateDatabase");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateDatabaseResponse>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateDatabaseError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a specified DevEndpoint.</p>"]
    fn update_dev_endpoint(&self,
                           input: &UpdateDevEndpointRequest)
                           -> Result<UpdateDevEndpointResponse, UpdateDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateDevEndpoint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateDevEndpointResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateDevEndpointError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates an existing job definition.</p>"]
    fn update_job(&self, input: &UpdateJobRequest) -> Result<UpdateJobResponse, UpdateJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateJob");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateJobResponse>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateJobError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a partition.</p>"]
    fn update_partition(&self,
                        input: &UpdatePartitionRequest)
                        -> Result<UpdatePartitionResponse, UpdatePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdatePartition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdatePartitionResponse>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdatePartitionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a metadata table in the Data Catalog.</p>"]
    fn update_table(&self,
                    input: &UpdateTableRequest)
                    -> Result<UpdateTableResponse, UpdateTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateTable");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateTableResponse>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateTableError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a trigger definition.</p>"]
    fn update_trigger(&self,
                      input: &UpdateTriggerRequest)
                      -> Result<UpdateTriggerResponse, UpdateTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateTrigger");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateTriggerResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateTriggerError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates an existing function definition in the Data Catalog.</p>"]
    fn update_user_defined_function
        (&self,
         input: &UpdateUserDefinedFunctionRequest)
         -> Result<UpdateUserDefinedFunctionResponse, UpdateUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateUserDefinedFunction");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateUserDefinedFunctionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateUserDefinedFunctionError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
