
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
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddApplicationCloudWatchLoggingOptionRequest {
    #[doc="<p>The Kinesis Analytics application name.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>Provides the CloudWatch log stream Amazon Resource Name (ARN) and the IAM role ARN. Note: To write application messages to CloudWatch, the IAM role that is used must have the <code>PutLogEvents</code> policy action enabled.</p>"]
    #[serde(rename="CloudWatchLoggingOption")]
    pub cloud_watch_logging_option: CloudWatchLoggingOption,
    #[doc="<p>The version ID of the Kinesis Analytics application.</p>"]
    #[serde(rename="CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddApplicationCloudWatchLoggingOptionResponse;

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddApplicationInputRequest {
    #[doc="<p>Name of your existing Amazon Kinesis Analytics application to which you want to add the streaming source.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>Current version of your Amazon Kinesis Analytics application. You can use the <a>DescribeApplication</a> operation to find the current application version.</p>"]
    #[serde(rename="CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    #[doc="<p/>"]
    #[serde(rename="Input")]
    pub input: Input,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddApplicationInputResponse;

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddApplicationOutputRequest {
    #[doc="<p>Name of the application to which you want to add the output configuration.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>Version of the application to which you want add the output configuration. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned. </p>"]
    #[serde(rename="CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    #[doc="<p>An array of objects, each describing one output configuration. In the output configuration, you specify the name of an in-application stream, a destination (that is, an Amazon Kinesis stream or an Amazon Kinesis Firehose delivery stream), and record the formation to use when writing to the destination.</p>"]
    #[serde(rename="Output")]
    pub output: Output,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddApplicationOutputResponse;

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddApplicationReferenceDataSourceRequest {
    #[doc="<p>Name of an existing application.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>Version of the application for which you are adding the reference data source. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>"]
    #[serde(rename="CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    #[doc="<p>The reference data source can be an object in your Amazon S3 bucket. Amazon Kinesis Analytics reads the object and copies the data into the in-application table that is created. You provide an S3 bucket, object key name, and the resulting in-application table that is created. You must also provide an IAM role with the necessary permissions that Amazon Kinesis Analytics can assume to read the object from your S3 bucket on your behalf.</p>"]
    #[serde(rename="ReferenceDataSource")]
    pub reference_data_source: ReferenceDataSource,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddApplicationReferenceDataSourceResponse;

#[doc="<p>Provides a description of the application, including the application Amazon Resource Name (ARN), status, latest version, and input and output configuration.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ApplicationDetail {
    #[doc="<p>ARN of the application.</p>"]
    #[serde(rename="ApplicationARN")]
    pub application_arn: String,
    #[doc="<p>Returns the application code that you provided to perform data analysis on any of the in-application streams in your application.</p>"]
    #[serde(rename="ApplicationCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_code: Option<String>,
    #[doc="<p>Description of the application.</p>"]
    #[serde(rename="ApplicationDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_description: Option<String>,
    #[doc="<p>Name of the application.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>Status of the application.</p>"]
    #[serde(rename="ApplicationStatus")]
    pub application_status: String,
    #[doc="<p>Provides the current application version.</p>"]
    #[serde(rename="ApplicationVersionId")]
    pub application_version_id: i64,
    #[doc="<p>Describes the CloudWatch log streams that are configured to receive application messages. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html\">Working with Amazon CloudWatch Logs</a>. </p>"]
    #[serde(rename="CloudWatchLoggingOptionDescriptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_option_descriptions:
        Option<Vec<CloudWatchLoggingOptionDescription>>,
    #[doc="<p>Timestamp when the application version was created.</p>"]
    #[serde(rename="CreateTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create_timestamp: Option<f64>,
    #[doc="<p>Describes the application input configuration. For more information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>. </p>"]
    #[serde(rename="InputDescriptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_descriptions: Option<Vec<InputDescription>>,
    #[doc="<p>Timestamp when the application was last updated.</p>"]
    #[serde(rename="LastUpdateTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    #[doc="<p>Describes the application output configuration. For more information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html\">Configuring Application Output</a>. </p>"]
    #[serde(rename="OutputDescriptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_descriptions: Option<Vec<OutputDescription>>,
    #[doc="<p>Describes reference data sources configured for the application. For more information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>. </p>"]
    #[serde(rename="ReferenceDataSourceDescriptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reference_data_source_descriptions: Option<Vec<ReferenceDataSourceDescription>>,
}

#[doc="<p>Provides application summary information, including the application Amazon Resource Name (ARN), name, and status.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ApplicationSummary {
    #[doc="<p>ARN of the application.</p>"]
    #[serde(rename="ApplicationARN")]
    pub application_arn: String,
    #[doc="<p>Name of the application.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>Status of the application.</p>"]
    #[serde(rename="ApplicationStatus")]
    pub application_status: String,
}

#[doc="<p>Describes updates to apply to an existing Amazon Kinesis Analytics application.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ApplicationUpdate {
    #[doc="<p>Describes application code updates.</p>"]
    #[serde(rename="ApplicationCodeUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_code_update: Option<String>,
    #[doc="<p>Describes application CloudWatch logging option updates.</p>"]
    #[serde(rename="CloudWatchLoggingOptionUpdates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_option_updates: Option<Vec<CloudWatchLoggingOptionUpdate>>,
    #[doc="<p>Describes application input configuration updates.</p>"]
    #[serde(rename="InputUpdates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_updates: Option<Vec<InputUpdate>>,
    #[doc="<p>Describes application output configuration updates.</p>"]
    #[serde(rename="OutputUpdates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_updates: Option<Vec<OutputUpdate>>,
    #[doc="<p>Describes application reference data source updates.</p>"]
    #[serde(rename="ReferenceDataSourceUpdates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reference_data_source_updates: Option<Vec<ReferenceDataSourceUpdate>>,
}

#[doc="<p>Provides additional mapping information when the record format uses delimiters, such as CSV. For example, the following sample records use CSV format, where the records use the <i>'\\n'</i> as the row delimiter and a comma (\",\") as the column delimiter: </p> <p> <code>\"name1\", \"address1\" </code> </p> <p> <code>\"name2, \"address2\"</code> </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CSVMappingParameters {
    #[doc="<p>Column delimiter. For example, in a CSV format, a comma (\",\") is the typical column delimiter.</p>"]
    #[serde(rename="RecordColumnDelimiter")]
    pub record_column_delimiter: String,
    #[doc="<p>Row delimiter. For example, in a CSV format, <i>'\\n'</i> is the typical row delimiter.</p>"]
    #[serde(rename="RecordRowDelimiter")]
    pub record_row_delimiter: String,
}

#[doc="<p>Provides a description of CloudWatch logging options, including the log stream Amazon Resource Name (ARN) and the role ARN.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CloudWatchLoggingOption {
    #[doc="<p>ARN of the CloudWatch log to receive application messages.</p>"]
    #[serde(rename="LogStreamARN")]
    pub log_stream_arn: String,
    #[doc="<p>IAM ARN of the role to use to send application messages. Note: To write application messages to CloudWatch, the IAM role that is used must have the <code>PutLogEvents</code> policy action enabled.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
}

#[doc="<p>Description of the CloudWatch logging option.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CloudWatchLoggingOptionDescription {
    #[doc="<p>ID of the CloudWatch logging option description.</p>"]
    #[serde(rename="CloudWatchLoggingOptionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_option_id: Option<String>,
    #[doc="<p>ARN of the CloudWatch log to receive application messages.</p>"]
    #[serde(rename="LogStreamARN")]
    pub log_stream_arn: String,
    #[doc="<p>IAM ARN of the role to use to send application messages. Note: To write application messages to CloudWatch, the IAM role used must have the <code>PutLogEvents</code> policy action enabled.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
}

#[doc="<p>Describes CloudWatch logging option updates.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CloudWatchLoggingOptionUpdate {
    #[doc="<p>ID of the CloudWatch logging option to update</p>"]
    #[serde(rename="CloudWatchLoggingOptionId")]
    pub cloud_watch_logging_option_id: String,
    #[doc="<p>ARN of the CloudWatch log to receive application messages.</p>"]
    #[serde(rename="LogStreamARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_stream_arn_update: Option<String>,
    #[doc="<p>IAM ARN of the role to use to send application messages. Note: To write application messages to CloudWatch, the IAM role used must have the <code>PutLogEvents</code> policy action enabled.</p>"]
    #[serde(rename="RoleARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn_update: Option<String>,
}

#[doc="<p>TBD</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateApplicationRequest {
    #[doc="<p>One or more SQL statements that read input data, transform it, and generate output. For example, you can write a SQL statement that reads data from one in-application stream, generates a running average of the number of advertisement clicks by vendor, and insert resulting rows in another in-application stream using pumps. For more inforamtion about the typical pattern, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-app-code.html\">Application Code</a>. </p> <p>You can provide such series of SQL statements, where output of one statement can be used as the input for the next statement. You store intermediate results by creating in-application streams and pumps.</p> <p>Note that the application code must create the streams with names specified in the <code>Outputs</code>. For example, if your <code>Outputs</code> defines output streams named <code>ExampleOutputStream1</code> and <code>ExampleOutputStream2</code>, then your application code must create these streams. </p>"]
    #[serde(rename="ApplicationCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_code: Option<String>,
    #[doc="<p>Summary description of the application.</p>"]
    #[serde(rename="ApplicationDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_description: Option<String>,
    #[doc="<p>Name of your Amazon Kinesis Analytics application (for example, <code>sample-app</code>).</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>Use this parameter to configure a CloudWatch log stream to monitor application configuration errors. For more information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html\">Working with Amazon CloudWatch Logs</a>.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<Vec<CloudWatchLoggingOption>>,
    #[doc="<p>Use this parameter to configure the application input.</p> <p>You can configure your application to receive input from a single streaming source. In this configuration, you map this streaming source to an in-application stream that is created. Your application code can then query the in-application stream like a table (you can think of it as a constantly updating table).</p> <p>For the streaming source, you provide its Amazon Resource Name (ARN) and format of data on the stream (for example, JSON, CSV, etc). You also must provide an IAM role that Amazon Kinesis Analytics can assume to read this stream on your behalf.</p> <p>To create the in-application stream, you need to specify a schema to transform your data into a schematized version used in SQL. In the schema, you provide the necessary mapping of the data elements in the streaming source to record columns in the in-app stream.</p>"]
    #[serde(rename="Inputs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inputs: Option<Vec<Input>>,
    #[doc="<p>You can configure application output to write data from any of the in-application streams to up to five destinations.</p> <p>These destinations can be Amazon Kinesis streams, Amazon Kinesis Firehose delivery streams, or both.</p> <p>In the configuration, you specify the in-application stream name, the destination stream Amazon Resource Name (ARN), and the format to use when writing data. You must also provide an IAM role that Amazon Kinesis Analytics can assume to write to the destination stream on your behalf.</p> <p>In the output configuration, you also provide the output stream Amazon Resource Name (ARN) and the format of data in the stream (for example, JSON, CSV). You also must provide an IAM role that Amazon Kinesis Analytics can assume to write to this stream on your behalf.</p>"]
    #[serde(rename="Outputs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub outputs: Option<Vec<Output>>,
}

#[doc="<p>TBD</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateApplicationResponse {
    #[doc="<p>In response to your <code>CreateApplication</code> request, Amazon Kinesis Analytics returns a response with a summary of the application it created, including the application Amazon Resource Name (ARN), name, and status.</p>"]
    #[serde(rename="ApplicationSummary")]
    pub application_summary: ApplicationSummary,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteApplicationCloudWatchLoggingOptionRequest {
    #[doc="<p>The Kinesis Analytics application name.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>The <code>CloudWatchLoggingOptionId</code> of the CloudWatch logging option to delete. You can use the <a>DescribeApplication</a> operation to get the <code>CloudWatchLoggingOptionId</code>. </p>"]
    #[serde(rename="CloudWatchLoggingOptionId")]
    pub cloud_watch_logging_option_id: String,
    #[doc="<p>The version ID of the Kinesis Analytics application.</p>"]
    #[serde(rename="CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteApplicationCloudWatchLoggingOptionResponse;

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteApplicationOutputRequest {
    #[doc="<p>Amazon Kinesis Analytics application name.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>Amazon Kinesis Analytics application version. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned. </p>"]
    #[serde(rename="CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    #[doc="<p>The ID of the configuration to delete. Each output configuration that is added to the application, either when the application is created or later using the <a>AddApplicationOutput</a> operation, has a unique ID. You need to provide the ID to uniquely identify the output configuration that you want to delete from the application configuration. You can use the <a>DescribeApplication</a> operation to get the specific <code>OutputId</code>. </p>"]
    #[serde(rename="OutputId")]
    pub output_id: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteApplicationOutputResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteApplicationReferenceDataSourceRequest {
    #[doc="<p>Name of an existing application.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>Version of the application. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>"]
    #[serde(rename="CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    #[doc="<p>ID of the reference data source. When you add a reference data source to your application using the <a>AddApplicationReferenceDataSource</a>, Amazon Kinesis Analytics assigns an ID. You can use the <a>DescribeApplication</a> operation to get the reference ID. </p>"]
    #[serde(rename="ReferenceId")]
    pub reference_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteApplicationReferenceDataSourceResponse;

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteApplicationRequest {
    #[doc="<p>Name of the Amazon Kinesis Analytics application to delete.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p> You can use the <code>DescribeApplication</code> operation to get this value. </p>"]
    #[serde(rename="CreateTimestamp")]
    pub create_timestamp: f64,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteApplicationResponse;

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeApplicationRequest {
    #[doc="<p>Name of the application.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeApplicationResponse {
    #[doc="<p>Provides a description of the application, such as the application Amazon Resource Name (ARN), status, latest version, and input and output configuration details.</p>"]
    #[serde(rename="ApplicationDetail")]
    pub application_detail: ApplicationDetail,
}

#[doc="<p>Describes the data format when records are written to the destination. For more information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html\">Configuring Application Output</a>. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DestinationSchema {
    #[doc="<p>Specifies the format of the records on the output stream.</p>"]
    #[serde(rename="RecordFormatType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_format_type: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DiscoverInputSchemaRequest {
    #[doc="<p>Point at which you want Amazon Kinesis Analytics to start reading records from the specified streaming source discovery purposes.</p>"]
    #[serde(rename="InputStartingPositionConfiguration")]
    pub input_starting_position_configuration: InputStartingPositionConfiguration,
    #[doc="<p>Amazon Resource Name (ARN) of the streaming source.</p>"]
    #[serde(rename="ResourceARN")]
    pub resource_arn: String,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DiscoverInputSchemaResponse {
    #[doc="<p>Schema inferred from the streaming source. It identifies the format of the data in the streaming source and how each data element maps to corresponding columns in the in-application stream that you can create.</p>"]
    #[serde(rename="InputSchema")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_schema: Option<SourceSchema>,
    #[doc="<p>An array of elements, where each element corresponds to a row in a stream record (a stream record can have more than one row).</p>"]
    #[serde(rename="ParsedInputRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parsed_input_records: Option<Vec<Vec<String>>>,
    #[doc="<p>Raw stream data that was sampled to infer the schema.</p>"]
    #[serde(rename="RawInputRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_input_records: Option<Vec<String>>,
}

#[doc="<p>When you configure the application input, you specify the streaming source, the in-application stream name that is created, and the mapping between the two. For more information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct Input {
    #[doc="<p>Describes the number of in-application streams to create. </p> <p>Data from your source will be routed to these in-application input streams.</p> <p> (see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>.</p>"]
    #[serde(rename="InputParallelism")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_parallelism: Option<InputParallelism>,
    #[doc="<p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns in the in-application stream that is being created.</p> <p>Also used to describe the format of the reference data source.</p>"]
    #[serde(rename="InputSchema")]
    pub input_schema: SourceSchema,
    #[doc="<p>If the streaming source is an Amazon Kinesis Firehose delivery stream, identifies the Firehose delivery stream's ARN and an IAM role that enables Amazon Kinesis Analytics to access the stream on your behalf.</p> <p>Note: Either <code>KinesisStreamsInput</code> or <code>KinesisFirehoseInput</code> is required.</p>"]
    #[serde(rename="KinesisFirehoseInput")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_firehose_input: Option<KinesisFirehoseInput>,
    #[doc="<p>If the streaming source is an Amazon Kinesis stream, identifies the stream's Amazon Resource Name (ARN) and an IAM role that enables Amazon Kinesis Analytics to access the stream on your behalf.</p> <p>Note: Either <code>KinesisStreamsInput</code> or <code>KinesisFirehoseInput</code> is required.</p>"]
    #[serde(rename="KinesisStreamsInput")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_streams_input: Option<KinesisStreamsInput>,
    #[doc="<p>Name prefix to use when creating in-application stream. Suppose you specify a prefix \"MyInApplicationStream\". Amazon Kinesis Analytics will then create one or more (as per the <code>InputParallelism</code> count you specified) in-application streams with names \"MyInApplicationStream_001\", \"MyInApplicationStream_002\" and so on. </p>"]
    #[serde(rename="NamePrefix")]
    pub name_prefix: String,
}

#[doc="<p>When you start your application, you provide this configuration, which identifies the input source and the point in the input source at which you want the application to start processing records.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InputConfiguration {
    #[doc="<p>Input source ID. You can get this ID by calling the <a>DescribeApplication</a> operation.</p>"]
    #[serde(rename="Id")]
    pub id: String,
    #[doc="<p>Point at which you want the application to start processing records from the streaming source.</p>"]
    #[serde(rename="InputStartingPositionConfiguration")]
    pub input_starting_position_configuration: InputStartingPositionConfiguration,
}

#[doc="<p>Describes the application input configuration. For more information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InputDescription {
    #[doc="<p>Returns the in-application stream names that are mapped to the stream source.</p>"]
    #[serde(rename="InAppStreamNames")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub in_app_stream_names: Option<Vec<String>>,
    #[doc="<p>Input ID associated with the application input. This is the ID that Amazon Kinesis Analytics assigns to each input configuration you add to your application. </p>"]
    #[serde(rename="InputId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_id: Option<String>,
    #[doc="<p>Describes the configured parallelism (number of in-application streams mapped to the streaming source).</p>"]
    #[serde(rename="InputParallelism")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_parallelism: Option<InputParallelism>,
    #[serde(rename="InputSchema")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_schema: Option<SourceSchema>,
    #[doc="<p>Point at which the application is configured to read from the input stream.</p>"]
    #[serde(rename="InputStartingPositionConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_starting_position_configuration: Option<InputStartingPositionConfiguration>,
    #[doc="<p>If an Amazon Kinesis Firehose delivery stream is configured as a streaming source, provides the Firehose delivery stream's Amazon Resource Name (ARN) and an IAM role that enables Amazon Kinesis Analytics to access the stream on your behalf.</p>"]
    #[serde(rename="KinesisFirehoseInputDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_firehose_input_description: Option<KinesisFirehoseInputDescription>,
    #[doc="<p>If an Amazon Kinesis stream is configured as streaming source, provides Amazon Kinesis stream's ARN and an IAM role that enables Amazon Kinesis Analytics to access the stream on your behalf.</p>"]
    #[serde(rename="KinesisStreamsInputDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_streams_input_description: Option<KinesisStreamsInputDescription>,
    #[doc="<p>In-application name prefix.</p>"]
    #[serde(rename="NamePrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name_prefix: Option<String>,
}

#[doc="<p>Describes the number of in-application streams to create for a given streaming source. For information about parallelism, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct InputParallelism {
    #[doc="<p>Number of in-application streams to create. For more information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html\">Limits</a>. </p>"]
    #[serde(rename="Count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,
}

#[doc="<p>Provides updates to the parallelism count.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InputParallelismUpdate {
    #[doc="<p>Number of in-application streams to create for the specified streaming source.</p>"]
    #[serde(rename="CountUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count_update: Option<i64>,
}

#[doc="<p> Describes updates for the application's input schema. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InputSchemaUpdate {
    #[doc="<p>A list of <code>RecordColumn</code> objects. Each object describes the mapping of the streaming source element to the corresponding column in the in-application stream. </p>"]
    #[serde(rename="RecordColumnUpdates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_column_updates: Option<Vec<RecordColumn>>,
    #[doc="<p>Specifies the encoding of the records in the streaming source. For example, UTF-8.</p>"]
    #[serde(rename="RecordEncodingUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_encoding_update: Option<String>,
    #[doc="<p>Specifies the format of the records on the streaming source.</p>"]
    #[serde(rename="RecordFormatUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_format_update: Option<RecordFormat>,
}

#[doc="<p>Describes the point at which the application reads from the streaming source.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct InputStartingPositionConfiguration {
    #[doc="<p>The starting position on the stream.</p> <ul> <li> <p> <code>NOW</code> - Start reading just after the most recent record in the stream, start at the request timestamp that the customer issued.</p> </li> <li> <p> <code>TRIM_HORIZON</code> - Start reading at the last untrimmed record in the stream, which is the oldest record available in the stream. This option is not available for an Amazon Kinesis Firehose delivery stream.</p> </li> <li> <p> <code>LAST_STOPPED_POINT</code> - Resume reading from where the application last stopped reading.</p> </li> </ul>"]
    #[serde(rename="InputStartingPosition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_starting_position: Option<String>,
}

#[doc="<p>Describes updates to a specific input configuration (identified by the <code>InputId</code> of an application). </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InputUpdate {
    #[doc="<p>Input ID of the application input to be updated.</p>"]
    #[serde(rename="InputId")]
    pub input_id: String,
    #[doc="<p>Describes the parallelism updates (the number in-application streams Amazon Kinesis Analytics creates for the specific streaming source).</p>"]
    #[serde(rename="InputParallelismUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_parallelism_update: Option<InputParallelismUpdate>,
    #[doc="<p>Describes the data format on the streaming source, and how record elements on the streaming source map to columns of the in-application stream that is created.</p>"]
    #[serde(rename="InputSchemaUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_schema_update: Option<InputSchemaUpdate>,
    #[doc="<p>If an Amazon Kinesis Firehose delivery stream is the streaming source to be updated, provides an updated stream Amazon Resource Name (ARN) and IAM role ARN.</p>"]
    #[serde(rename="KinesisFirehoseInputUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_firehose_input_update: Option<KinesisFirehoseInputUpdate>,
    #[doc="<p>If a Amazon Kinesis stream is the streaming source to be updated, provides an updated stream ARN and IAM role ARN.</p>"]
    #[serde(rename="KinesisStreamsInputUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_streams_input_update: Option<KinesisStreamsInputUpdate>,
    #[doc="<p>Name prefix for in-application streams that Amazon Kinesis Analytics creates for the specific streaming source.</p>"]
    #[serde(rename="NamePrefixUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name_prefix_update: Option<String>,
}

#[doc="<p>Provides additional mapping information when JSON is the record format on the streaming source.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct JSONMappingParameters {
    #[doc="<p>Path to the top-level parent that contains the records.</p>"]
    #[serde(rename="RecordRowPath")]
    pub record_row_path: String,
}

#[doc="<p> Identifies an Amazon Kinesis Firehose delivery stream as the streaming source. You provide the Firehose delivery stream's Amazon Resource Name (ARN) and an IAM role ARN that enables Amazon Kinesis Analytics to access the stream on your behalf.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct KinesisFirehoseInput {
    #[doc="<p>ARN of the input Firehose delivery stream.</p>"]
    #[serde(rename="ResourceARN")]
    pub resource_arn: String,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to make sure the role has necessary permissions to access the stream.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
}

#[doc="<p> Describes the Amazon Kinesis Firehose delivery stream that is configured as the streaming source in the application input configuration. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct KinesisFirehoseInputDescription {
    #[doc="<p>Amazon Resource Name (ARN) of the Amazon Kinesis Firehose delivery stream.</p>"]
    #[serde(rename="ResourceARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_arn: Option<String>,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics assumes to access the stream.</p>"]
    #[serde(rename="RoleARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}

#[doc="<p>When updating application input configuration, provides information about an Amazon Kinesis Firehose delivery stream as the streaming source.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct KinesisFirehoseInputUpdate {
    #[doc="<p>ARN of the input Amazon Kinesis Firehose delivery stream to read.</p>"]
    #[serde(rename="ResourceARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_arn_update: Option<String>,
    #[doc="<p>Amazon Resource Name (ARN) of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant necessary permissions to this role.</p>"]
    #[serde(rename="RoleARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn_update: Option<String>,
}

#[doc="<p>When configuring application output, identifies an Amazon Kinesis Firehose delivery stream as the destination. You provide the stream Amazon Resource Name (ARN) and an IAM role that enables Amazon Kinesis Analytics to write to the stream on your behalf.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct KinesisFirehoseOutput {
    #[doc="<p>ARN of the destination Amazon Kinesis Firehose delivery stream to write to.</p>"]
    #[serde(rename="ResourceARN")]
    pub resource_arn: String,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination stream on your behalf. You need to grant the necessary permissions to this role.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
}

#[doc="<p> For an application output, describes the Amazon Kinesis Firehose delivery stream configured as its destination. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct KinesisFirehoseOutputDescription {
    #[doc="<p>Amazon Resource Name (ARN) of the Amazon Kinesis Firehose delivery stream.</p>"]
    #[serde(rename="ResourceARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_arn: Option<String>,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream.</p>"]
    #[serde(rename="RoleARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}

#[doc="<p> When updating an output configuration using the <a>UpdateApplication</a> operation, provides information about an Amazon Kinesis Firehose delivery stream configured as the destination. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct KinesisFirehoseOutputUpdate {
    #[doc="<p>Amazon Resource Name (ARN) of the Amazon Kinesis Firehose delivery stream to write to.</p>"]
    #[serde(rename="ResourceARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_arn_update: Option<String>,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant necessary permissions to this role.</p>"]
    #[serde(rename="RoleARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn_update: Option<String>,
}

#[doc="<p> Identifies an Amazon Kinesis stream as the streaming source. You provide the stream's ARN and an IAM role ARN that enables Amazon Kinesis Analytics to access the stream on your behalf.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct KinesisStreamsInput {
    #[doc="<p>ARN of the input Amazon Kinesis stream to read.</p>"]
    #[serde(rename="ResourceARN")]
    pub resource_arn: String,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant the necessary permissions to this role.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
}

#[doc="<p> Describes the Amazon Kinesis stream that is configured as the streaming source in the application input configuration. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct KinesisStreamsInputDescription {
    #[doc="<p>Amazon Resource Name (ARN) of the Amazon Kinesis stream.</p>"]
    #[serde(rename="ResourceARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_arn: Option<String>,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream.</p>"]
    #[serde(rename="RoleARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}

#[doc="<p>When updating application input configuration, provides information about an Amazon Kinesis stream as the streaming source.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct KinesisStreamsInputUpdate {
    #[doc="<p>Amazon Resource Name (ARN) of the input Amazon Kinesis stream to read.</p>"]
    #[serde(rename="ResourceARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_arn_update: Option<String>,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant the necessary permissions to this role.</p>"]
    #[serde(rename="RoleARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn_update: Option<String>,
}

#[doc="<p>When configuring application output, identifies a Amazon Kinesis stream as the destination. You provide the stream Amazon Resource Name (ARN) and also an IAM role ARN that Amazon Kinesis Analytics can use to write to the stream on your behalf.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct KinesisStreamsOutput {
    #[doc="<p>ARN of the destination Amazon Kinesis stream to write to.</p>"]
    #[serde(rename="ResourceARN")]
    pub resource_arn: String,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination stream on your behalf. You need to grant the necessary permissions to this role.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
}

#[doc="<p> For an application output, describes the Amazon Kinesis stream configured as its destination. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct KinesisStreamsOutputDescription {
    #[doc="<p>Amazon Resource Name (ARN) of the Amazon Kinesis stream.</p>"]
    #[serde(rename="ResourceARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_arn: Option<String>,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream.</p>"]
    #[serde(rename="RoleARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}

#[doc="<p> When updating an output configuration using the <a>UpdateApplication</a> operation, provides information about an Amazon Kinesis stream configured as the destination. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct KinesisStreamsOutputUpdate {
    #[doc="<p>Amazon Resource Name (ARN) of the Amazon Kinesis stream where you want to write the output.</p>"]
    #[serde(rename="ResourceARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_arn_update: Option<String>,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant the necessary permissions to this role.</p>"]
    #[serde(rename="RoleARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn_update: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListApplicationsRequest {
    #[doc="<p>Name of the application to start the list with. When using pagination to retrieve the list, you don't need to specify this parameter in the first request. However, in subsequent requests, you add the last application name from the previous response to get the next page of applications.</p>"]
    #[serde(rename="ExclusiveStartApplicationName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclusive_start_application_name: Option<String>,
    #[doc="<p>Maximum number of applications to list.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListApplicationsResponse {
    #[doc="<p>List of <code>ApplicationSummary</code> objects. </p>"]
    #[serde(rename="ApplicationSummaries")]
    pub application_summaries: Vec<ApplicationSummary>,
    #[doc="<p>Returns true if there are more applications to retrieve.</p>"]
    #[serde(rename="HasMoreApplications")]
    pub has_more_applications: bool,
}

#[doc="<p>When configuring application input at the time of creating or updating an application, provides additional mapping information specific to the record format (such as JSON, CSV, or record fields delimited by some delimiter) on the streaming source.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct MappingParameters {
    #[doc="<p>Provides additional mapping information when the record format uses delimiters (for example, CSV).</p>"]
    #[serde(rename="CSVMappingParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csv_mapping_parameters: Option<CSVMappingParameters>,
    #[doc="<p>Provides additional mapping information when JSON is the record format on the streaming source.</p>"]
    #[serde(rename="JSONMappingParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub json_mapping_parameters: Option<JSONMappingParameters>,
}

#[doc="<p> Describes application output configuration in which you identify an in-application stream and a destination where you want the in-application stream data to be written. The destination can be an Amazon Kinesis stream or an Amazon Kinesis Firehose delivery stream. </p> <p/> <p>For limits on how many destinations an application can write and other limitations, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html\">Limits</a>. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct Output {
    #[serde(rename="DestinationSchema")]
    pub destination_schema: DestinationSchema,
    #[doc="<p>Identifies an Amazon Kinesis Firehose delivery stream as the destination.</p>"]
    #[serde(rename="KinesisFirehoseOutput")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_firehose_output: Option<KinesisFirehoseOutput>,
    #[doc="<p>Identifies an Amazon Kinesis stream as the destination.</p>"]
    #[serde(rename="KinesisStreamsOutput")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_streams_output: Option<KinesisStreamsOutput>,
    #[doc="<p>Name of the in-application stream.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[doc="<p>Describes the application output configuration, which includes the in-application stream name and the destination where the stream data is written. The destination can be an Amazon Kinesis stream or an Amazon Kinesis Firehose delivery stream. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct OutputDescription {
    #[doc="<p>Data format used for writing data to the destination.</p>"]
    #[serde(rename="DestinationSchema")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub destination_schema: Option<DestinationSchema>,
    #[doc="<p>Describes the Amazon Kinesis Firehose delivery stream configured as the destination where output is written.</p>"]
    #[serde(rename="KinesisFirehoseOutputDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_firehose_output_description: Option<KinesisFirehoseOutputDescription>,
    #[doc="<p>Describes Amazon Kinesis stream configured as the destination where output is written.</p>"]
    #[serde(rename="KinesisStreamsOutputDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_streams_output_description: Option<KinesisStreamsOutputDescription>,
    #[doc="<p>Name of the in-application stream configured as output.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>A unique identifier for the output configuration.</p>"]
    #[serde(rename="OutputId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_id: Option<String>,
}

#[doc="<p> Describes updates to the output configuration identified by the <code>OutputId</code>. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct OutputUpdate {
    #[serde(rename="DestinationSchemaUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub destination_schema_update: Option<DestinationSchema>,
    #[doc="<p>Describes a Amazon Kinesis Firehose delivery stream as the destination for the output.</p>"]
    #[serde(rename="KinesisFirehoseOutputUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_firehose_output_update: Option<KinesisFirehoseOutputUpdate>,
    #[doc="<p>Describes an Amazon Kinesis stream as the destination for the output.</p>"]
    #[serde(rename="KinesisStreamsOutputUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_streams_output_update: Option<KinesisStreamsOutputUpdate>,
    #[doc="<p>If you want to specify a different in-application stream for this output configuration, use this field to specify the new in-application stream name.</p>"]
    #[serde(rename="NameUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name_update: Option<String>,
    #[doc="<p>Identifies the specific output configuration that you want to update.</p>"]
    #[serde(rename="OutputId")]
    pub output_id: String,
}

#[doc="<p>Describes the mapping of each data element in the streaming source to the corresponding column in the in-application stream.</p> <p>Also used to describe the format of the reference data source.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RecordColumn {
    #[doc="<p>Reference to the data element in the streaming input of the reference data source.</p>"]
    #[serde(rename="Mapping")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mapping: Option<String>,
    #[doc="<p>Name of the column created in the in-application input stream or reference table.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>Type of column created in the in-application input stream or reference table.</p>"]
    #[serde(rename="SqlType")]
    pub sql_type: String,
}

#[doc="<p> Describes the record format and relevant mapping information that should be applied to schematize the records on the stream. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RecordFormat {
    #[serde(rename="MappingParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mapping_parameters: Option<MappingParameters>,
    #[doc="<p>The type of record format.</p>"]
    #[serde(rename="RecordFormatType")]
    pub record_format_type: String,
}

#[doc="<p>Describes the reference data source by providing the source information (S3 bucket name and object key name), the resulting in-application table name that is created, and the necessary schema to map the data elements in the Amazon S3 object to the in-application table.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ReferenceDataSource {
    #[serde(rename="ReferenceSchema")]
    pub reference_schema: SourceSchema,
    #[serde(rename="S3ReferenceDataSource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_reference_data_source: Option<S3ReferenceDataSource>,
    #[doc="<p>Name of the in-application table to create.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[doc="<p>Describes the reference data source configured for an application.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ReferenceDataSourceDescription {
    #[doc="<p>ID of the reference data source. This is the ID that Amazon Kinesis Analytics assigns when you add the reference data source to your application using the <a>AddApplicationReferenceDataSource</a> operation.</p>"]
    #[serde(rename="ReferenceId")]
    pub reference_id: String,
    #[serde(rename="ReferenceSchema")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reference_schema: Option<SourceSchema>,
    #[doc="<p>Provides the S3 bucket name, the object key name that contains the reference data. It also provides the Amazon Resource Name (ARN) of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object and populate the in-application reference table.</p>"]
    #[serde(rename="S3ReferenceDataSourceDescription")]
    pub s3_reference_data_source_description: S3ReferenceDataSourceDescription,
    #[doc="<p>The in-application table name created by the specific reference data source configuration.</p>"]
    #[serde(rename="TableName")]
    pub table_name: String,
}

#[doc="<p>When you update a reference data source configuration for an application, this object provides all the updated values (such as the source bucket name and object key name), the in-application table name that is created, and updated mapping information that maps the data in the Amazon S3 object to the in-application reference table that is created.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ReferenceDataSourceUpdate {
    #[doc="<p>ID of the reference data source being updated. You can use the <a>DescribeApplication</a> operation to get this value.</p>"]
    #[serde(rename="ReferenceId")]
    pub reference_id: String,
    #[serde(rename="ReferenceSchemaUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reference_schema_update: Option<SourceSchema>,
    #[doc="<p>Describes the S3 bucket name, object key name, and IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object on your behalf and populate the in-application reference table.</p>"]
    #[serde(rename="S3ReferenceDataSourceUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_reference_data_source_update: Option<S3ReferenceDataSourceUpdate>,
    #[doc="<p>In-application table name that is created by this update.</p>"]
    #[serde(rename="TableNameUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_name_update: Option<String>,
}

#[doc="<p>Identifies the S3 bucket and object that contains the reference data. Also identifies the IAM role Amazon Kinesis Analytics can assume to read this object on your behalf.</p> <p>An Amazon Kinesis Analytics application loads reference data only once. If the data changes, you call the <a>UpdateApplication</a> operation to trigger reloading of data into your application.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct S3ReferenceDataSource {
    #[doc="<p>Amazon Resource Name (ARN) of the S3 bucket.</p>"]
    #[serde(rename="BucketARN")]
    pub bucket_arn: String,
    #[doc="<p>Object key name containing reference data.</p>"]
    #[serde(rename="FileKey")]
    pub file_key: String,
    #[doc="<p>ARN of the IAM role that the service can assume to read data on your behalf. This role must have permission for the <code>s3:GetObject</code> action on the object and trust policy that allows Amazon Kinesis Analytics service principal to assume this role.</p>"]
    #[serde(rename="ReferenceRoleARN")]
    pub reference_role_arn: String,
}

#[doc="<p>Provides the bucket name and object key name that stores the reference data.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct S3ReferenceDataSourceDescription {
    #[doc="<p>Amazon Resource Name (ARN) of the S3 bucket.</p>"]
    #[serde(rename="BucketARN")]
    pub bucket_arn: String,
    #[doc="<p>Amazon S3 object key name.</p>"]
    #[serde(rename="FileKey")]
    pub file_key: String,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object on your behalf to populate the in-application reference table.</p>"]
    #[serde(rename="ReferenceRoleARN")]
    pub reference_role_arn: String,
}

#[doc="<p>Describes the S3 bucket name, object key name, and IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object on your behalf and populate the in-application reference table.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct S3ReferenceDataSourceUpdate {
    #[doc="<p>Amazon Resource Name (ARN) of the S3 bucket.</p>"]
    #[serde(rename="BucketARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bucket_arn_update: Option<String>,
    #[doc="<p>Object key name.</p>"]
    #[serde(rename="FileKeyUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub file_key_update: Option<String>,
    #[doc="<p>ARN of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object and populate the in-application.</p>"]
    #[serde(rename="ReferenceRoleARNUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reference_role_arn_update: Option<String>,
}

#[doc="<p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SourceSchema {
    #[doc="<p>A list of <code>RecordColumn</code> objects.</p>"]
    #[serde(rename="RecordColumns")]
    pub record_columns: Vec<RecordColumn>,
    #[doc="<p>Specifies the encoding of the records in the streaming source. For example, UTF-8.</p>"]
    #[serde(rename="RecordEncoding")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_encoding: Option<String>,
    #[doc="<p>Specifies the format of the records on the streaming source.</p>"]
    #[serde(rename="RecordFormat")]
    pub record_format: RecordFormat,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StartApplicationRequest {
    #[doc="<p>Name of the application.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>Identifies the specific input, by ID, that the application starts consuming. Amazon Kinesis Analytics starts reading the streaming source associated with the input. You can also specify where in the streaming source you want Amazon Kinesis Analytics to start reading.</p>"]
    #[serde(rename="InputConfigurations")]
    pub input_configurations: Vec<InputConfiguration>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartApplicationResponse;

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StopApplicationRequest {
    #[doc="<p>Name of the running application to stop.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StopApplicationResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateApplicationRequest {
    #[doc="<p>Name of the Amazon Kinesis Analytics application to update.</p>"]
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[doc="<p>Describes application updates.</p>"]
    #[serde(rename="ApplicationUpdate")]
    pub application_update: ApplicationUpdate,
    #[doc="<p>The current application version ID. You can use the <a>DescribeApplication</a> operation to get this value.</p>"]
    #[serde(rename="CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateApplicationResponse;

/// Errors returned by AddApplicationCloudWatchLoggingOption
#[derive(Debug, PartialEq)]
pub enum AddApplicationCloudWatchLoggingOptionError {
    ///<p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    ///<p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    ///<p>Specified application can't be found.</p>
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


impl AddApplicationCloudWatchLoggingOptionError {
    pub fn from_body(body: &str) -> AddApplicationCloudWatchLoggingOptionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => AddApplicationCloudWatchLoggingOptionError::ConcurrentModification(String::from(error_message)),
                    "InvalidArgumentException" => AddApplicationCloudWatchLoggingOptionError::InvalidArgument(String::from(error_message)),
                    "ResourceInUseException" => AddApplicationCloudWatchLoggingOptionError::ResourceInUse(String::from(error_message)),
                    "ResourceNotFoundException" => AddApplicationCloudWatchLoggingOptionError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        AddApplicationCloudWatchLoggingOptionError::Validation(error_message
                                                                                   .to_string())
                    }
                    _ => AddApplicationCloudWatchLoggingOptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddApplicationCloudWatchLoggingOptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddApplicationCloudWatchLoggingOptionError {
    fn from(err: serde_json::error::Error) -> AddApplicationCloudWatchLoggingOptionError {
        AddApplicationCloudWatchLoggingOptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddApplicationCloudWatchLoggingOptionError {
    fn from(err: CredentialsError) -> AddApplicationCloudWatchLoggingOptionError {
        AddApplicationCloudWatchLoggingOptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddApplicationCloudWatchLoggingOptionError {
    fn from(err: HttpDispatchError) -> AddApplicationCloudWatchLoggingOptionError {
        AddApplicationCloudWatchLoggingOptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddApplicationCloudWatchLoggingOptionError {
    fn from(err: io::Error) -> AddApplicationCloudWatchLoggingOptionError {
        AddApplicationCloudWatchLoggingOptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddApplicationCloudWatchLoggingOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddApplicationCloudWatchLoggingOptionError {
    fn description(&self) -> &str {
        match *self {
            AddApplicationCloudWatchLoggingOptionError::ConcurrentModification(ref cause) => cause,
            AddApplicationCloudWatchLoggingOptionError::InvalidArgument(ref cause) => cause,
            AddApplicationCloudWatchLoggingOptionError::ResourceInUse(ref cause) => cause,
            AddApplicationCloudWatchLoggingOptionError::ResourceNotFound(ref cause) => cause,
            AddApplicationCloudWatchLoggingOptionError::Validation(ref cause) => cause,
            AddApplicationCloudWatchLoggingOptionError::Credentials(ref err) => err.description(),
            AddApplicationCloudWatchLoggingOptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddApplicationCloudWatchLoggingOptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddApplicationInput
#[derive(Debug, PartialEq)]
pub enum AddApplicationInputError {
    ///<p>User-provided application code (query) is invalid. This can be a simple syntax error.</p>
    CodeValidation(String),
    ///<p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    ///<p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    ///<p>Specified application can't be found.</p>
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


impl AddApplicationInputError {
    pub fn from_body(body: &str) -> AddApplicationInputError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CodeValidationException" => {
                        AddApplicationInputError::CodeValidation(String::from(error_message))
                    }
                    "ConcurrentModificationException" => AddApplicationInputError::ConcurrentModification(String::from(error_message)),
                    "InvalidArgumentException" => {
                        AddApplicationInputError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        AddApplicationInputError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AddApplicationInputError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddApplicationInputError::Validation(error_message.to_string())
                    }
                    _ => AddApplicationInputError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddApplicationInputError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddApplicationInputError {
    fn from(err: serde_json::error::Error) -> AddApplicationInputError {
        AddApplicationInputError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddApplicationInputError {
    fn from(err: CredentialsError) -> AddApplicationInputError {
        AddApplicationInputError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddApplicationInputError {
    fn from(err: HttpDispatchError) -> AddApplicationInputError {
        AddApplicationInputError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddApplicationInputError {
    fn from(err: io::Error) -> AddApplicationInputError {
        AddApplicationInputError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddApplicationInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddApplicationInputError {
    fn description(&self) -> &str {
        match *self {
            AddApplicationInputError::CodeValidation(ref cause) => cause,
            AddApplicationInputError::ConcurrentModification(ref cause) => cause,
            AddApplicationInputError::InvalidArgument(ref cause) => cause,
            AddApplicationInputError::ResourceInUse(ref cause) => cause,
            AddApplicationInputError::ResourceNotFound(ref cause) => cause,
            AddApplicationInputError::Validation(ref cause) => cause,
            AddApplicationInputError::Credentials(ref err) => err.description(),
            AddApplicationInputError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddApplicationInputError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddApplicationOutput
#[derive(Debug, PartialEq)]
pub enum AddApplicationOutputError {
    ///<p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    ///<p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    ///<p>Specified application can't be found.</p>
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


impl AddApplicationOutputError {
    pub fn from_body(body: &str) -> AddApplicationOutputError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => AddApplicationOutputError::ConcurrentModification(String::from(error_message)),
                    "InvalidArgumentException" => {
                        AddApplicationOutputError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        AddApplicationOutputError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AddApplicationOutputError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddApplicationOutputError::Validation(error_message.to_string())
                    }
                    _ => AddApplicationOutputError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddApplicationOutputError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddApplicationOutputError {
    fn from(err: serde_json::error::Error) -> AddApplicationOutputError {
        AddApplicationOutputError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddApplicationOutputError {
    fn from(err: CredentialsError) -> AddApplicationOutputError {
        AddApplicationOutputError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddApplicationOutputError {
    fn from(err: HttpDispatchError) -> AddApplicationOutputError {
        AddApplicationOutputError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddApplicationOutputError {
    fn from(err: io::Error) -> AddApplicationOutputError {
        AddApplicationOutputError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddApplicationOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddApplicationOutputError {
    fn description(&self) -> &str {
        match *self {
            AddApplicationOutputError::ConcurrentModification(ref cause) => cause,
            AddApplicationOutputError::InvalidArgument(ref cause) => cause,
            AddApplicationOutputError::ResourceInUse(ref cause) => cause,
            AddApplicationOutputError::ResourceNotFound(ref cause) => cause,
            AddApplicationOutputError::Validation(ref cause) => cause,
            AddApplicationOutputError::Credentials(ref err) => err.description(),
            AddApplicationOutputError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddApplicationOutputError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddApplicationReferenceDataSource
#[derive(Debug, PartialEq)]
pub enum AddApplicationReferenceDataSourceError {
    ///<p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    ///<p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    ///<p>Specified application can't be found.</p>
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


impl AddApplicationReferenceDataSourceError {
    pub fn from_body(body: &str) -> AddApplicationReferenceDataSourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => AddApplicationReferenceDataSourceError::ConcurrentModification(String::from(error_message)),
                    "InvalidArgumentException" => AddApplicationReferenceDataSourceError::InvalidArgument(String::from(error_message)),
                    "ResourceInUseException" => AddApplicationReferenceDataSourceError::ResourceInUse(String::from(error_message)),
                    "ResourceNotFoundException" => AddApplicationReferenceDataSourceError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        AddApplicationReferenceDataSourceError::Validation(error_message
                                                                               .to_string())
                    }
                    _ => AddApplicationReferenceDataSourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddApplicationReferenceDataSourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddApplicationReferenceDataSourceError {
    fn from(err: serde_json::error::Error) -> AddApplicationReferenceDataSourceError {
        AddApplicationReferenceDataSourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddApplicationReferenceDataSourceError {
    fn from(err: CredentialsError) -> AddApplicationReferenceDataSourceError {
        AddApplicationReferenceDataSourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddApplicationReferenceDataSourceError {
    fn from(err: HttpDispatchError) -> AddApplicationReferenceDataSourceError {
        AddApplicationReferenceDataSourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddApplicationReferenceDataSourceError {
    fn from(err: io::Error) -> AddApplicationReferenceDataSourceError {
        AddApplicationReferenceDataSourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddApplicationReferenceDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddApplicationReferenceDataSourceError {
    fn description(&self) -> &str {
        match *self {
            AddApplicationReferenceDataSourceError::ConcurrentModification(ref cause) => cause,
            AddApplicationReferenceDataSourceError::InvalidArgument(ref cause) => cause,
            AddApplicationReferenceDataSourceError::ResourceInUse(ref cause) => cause,
            AddApplicationReferenceDataSourceError::ResourceNotFound(ref cause) => cause,
            AddApplicationReferenceDataSourceError::Validation(ref cause) => cause,
            AddApplicationReferenceDataSourceError::Credentials(ref err) => err.description(),
            AddApplicationReferenceDataSourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddApplicationReferenceDataSourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    ///<p>User-provided application code (query) is invalid. This can be a simple syntax error.</p>
    CodeValidation(String),
    ///<p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    ///<p>Exceeded the number of applications allowed.</p>
    LimitExceeded(String),
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateApplicationError {
    pub fn from_body(body: &str) -> CreateApplicationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CodeValidationException" => {
                        CreateApplicationError::CodeValidation(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        CreateApplicationError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateApplicationError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        CreateApplicationError::ResourceInUse(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateApplicationError::Validation(error_message.to_string())
                    }
                    _ => CreateApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateApplicationError {
    fn from(err: serde_json::error::Error) -> CreateApplicationError {
        CreateApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateApplicationError {
    fn from(err: CredentialsError) -> CreateApplicationError {
        CreateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateApplicationError {
    fn from(err: HttpDispatchError) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateApplicationError {
    fn from(err: io::Error) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApplicationError {
    fn description(&self) -> &str {
        match *self {
            CreateApplicationError::CodeValidation(ref cause) => cause,
            CreateApplicationError::InvalidArgument(ref cause) => cause,
            CreateApplicationError::LimitExceeded(ref cause) => cause,
            CreateApplicationError::ResourceInUse(ref cause) => cause,
            CreateApplicationError::Validation(ref cause) => cause,
            CreateApplicationError::Credentials(ref err) => err.description(),
            CreateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplication
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationError {
    ///<p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    ///<p>Specified application can't be found.</p>
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


impl DeleteApplicationError {
    pub fn from_body(body: &str) -> DeleteApplicationError {
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
                        DeleteApplicationError::ConcurrentModification(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteApplicationError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteApplicationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteApplicationError::Validation(error_message.to_string())
                    }
                    _ => DeleteApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApplicationError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationError {
        DeleteApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationError {
    fn from(err: CredentialsError) -> DeleteApplicationError {
        DeleteApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationError {
    fn from(err: HttpDispatchError) -> DeleteApplicationError {
        DeleteApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationError {
    fn from(err: io::Error) -> DeleteApplicationError {
        DeleteApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationError::ConcurrentModification(ref cause) => cause,
            DeleteApplicationError::ResourceInUse(ref cause) => cause,
            DeleteApplicationError::ResourceNotFound(ref cause) => cause,
            DeleteApplicationError::Validation(ref cause) => cause,
            DeleteApplicationError::Credentials(ref err) => err.description(),
            DeleteApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplicationCloudWatchLoggingOption
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationCloudWatchLoggingOptionError {
    ///<p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    ///<p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    ///<p>Specified application can't be found.</p>
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


impl DeleteApplicationCloudWatchLoggingOptionError {
    pub fn from_body(body: &str) -> DeleteApplicationCloudWatchLoggingOptionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => DeleteApplicationCloudWatchLoggingOptionError::ConcurrentModification(String::from(error_message)),
                    "InvalidArgumentException" => DeleteApplicationCloudWatchLoggingOptionError::InvalidArgument(String::from(error_message)),
                    "ResourceInUseException" => DeleteApplicationCloudWatchLoggingOptionError::ResourceInUse(String::from(error_message)),
                    "ResourceNotFoundException" => DeleteApplicationCloudWatchLoggingOptionError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => DeleteApplicationCloudWatchLoggingOptionError::Validation(error_message.to_string()),
                    _ => DeleteApplicationCloudWatchLoggingOptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApplicationCloudWatchLoggingOptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApplicationCloudWatchLoggingOptionError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationCloudWatchLoggingOptionError {
        DeleteApplicationCloudWatchLoggingOptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationCloudWatchLoggingOptionError {
    fn from(err: CredentialsError) -> DeleteApplicationCloudWatchLoggingOptionError {
        DeleteApplicationCloudWatchLoggingOptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationCloudWatchLoggingOptionError {
    fn from(err: HttpDispatchError) -> DeleteApplicationCloudWatchLoggingOptionError {
        DeleteApplicationCloudWatchLoggingOptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationCloudWatchLoggingOptionError {
    fn from(err: io::Error) -> DeleteApplicationCloudWatchLoggingOptionError {
        DeleteApplicationCloudWatchLoggingOptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationCloudWatchLoggingOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationCloudWatchLoggingOptionError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationCloudWatchLoggingOptionError::ConcurrentModification(ref cause) => {
                cause
            }
            DeleteApplicationCloudWatchLoggingOptionError::InvalidArgument(ref cause) => cause,
            DeleteApplicationCloudWatchLoggingOptionError::ResourceInUse(ref cause) => cause,
            DeleteApplicationCloudWatchLoggingOptionError::ResourceNotFound(ref cause) => cause,
            DeleteApplicationCloudWatchLoggingOptionError::Validation(ref cause) => cause,
            DeleteApplicationCloudWatchLoggingOptionError::Credentials(ref err) => {
                err.description()
            }
            DeleteApplicationCloudWatchLoggingOptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationCloudWatchLoggingOptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplicationOutput
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationOutputError {
    ///<p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    ///<p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    ///<p>Specified application can't be found.</p>
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


impl DeleteApplicationOutputError {
    pub fn from_body(body: &str) -> DeleteApplicationOutputError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => DeleteApplicationOutputError::ConcurrentModification(String::from(error_message)),
                    "InvalidArgumentException" => {
                        DeleteApplicationOutputError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteApplicationOutputError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteApplicationOutputError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteApplicationOutputError::Validation(error_message.to_string())
                    }
                    _ => DeleteApplicationOutputError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApplicationOutputError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApplicationOutputError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationOutputError {
        DeleteApplicationOutputError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationOutputError {
    fn from(err: CredentialsError) -> DeleteApplicationOutputError {
        DeleteApplicationOutputError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationOutputError {
    fn from(err: HttpDispatchError) -> DeleteApplicationOutputError {
        DeleteApplicationOutputError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationOutputError {
    fn from(err: io::Error) -> DeleteApplicationOutputError {
        DeleteApplicationOutputError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationOutputError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationOutputError::ConcurrentModification(ref cause) => cause,
            DeleteApplicationOutputError::InvalidArgument(ref cause) => cause,
            DeleteApplicationOutputError::ResourceInUse(ref cause) => cause,
            DeleteApplicationOutputError::ResourceNotFound(ref cause) => cause,
            DeleteApplicationOutputError::Validation(ref cause) => cause,
            DeleteApplicationOutputError::Credentials(ref err) => err.description(),
            DeleteApplicationOutputError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationOutputError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplicationReferenceDataSource
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationReferenceDataSourceError {
    ///<p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    ///<p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    ///<p>Specified application can't be found.</p>
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


impl DeleteApplicationReferenceDataSourceError {
    pub fn from_body(body: &str) -> DeleteApplicationReferenceDataSourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => DeleteApplicationReferenceDataSourceError::ConcurrentModification(String::from(error_message)),
                    "InvalidArgumentException" => DeleteApplicationReferenceDataSourceError::InvalidArgument(String::from(error_message)),
                    "ResourceInUseException" => DeleteApplicationReferenceDataSourceError::ResourceInUse(String::from(error_message)),
                    "ResourceNotFoundException" => DeleteApplicationReferenceDataSourceError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DeleteApplicationReferenceDataSourceError::Validation(error_message
                                                                                  .to_string())
                    }
                    _ => DeleteApplicationReferenceDataSourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApplicationReferenceDataSourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApplicationReferenceDataSourceError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationReferenceDataSourceError {
        DeleteApplicationReferenceDataSourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationReferenceDataSourceError {
    fn from(err: CredentialsError) -> DeleteApplicationReferenceDataSourceError {
        DeleteApplicationReferenceDataSourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationReferenceDataSourceError {
    fn from(err: HttpDispatchError) -> DeleteApplicationReferenceDataSourceError {
        DeleteApplicationReferenceDataSourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationReferenceDataSourceError {
    fn from(err: io::Error) -> DeleteApplicationReferenceDataSourceError {
        DeleteApplicationReferenceDataSourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationReferenceDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationReferenceDataSourceError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationReferenceDataSourceError::ConcurrentModification(ref cause) => cause,
            DeleteApplicationReferenceDataSourceError::InvalidArgument(ref cause) => cause,
            DeleteApplicationReferenceDataSourceError::ResourceInUse(ref cause) => cause,
            DeleteApplicationReferenceDataSourceError::ResourceNotFound(ref cause) => cause,
            DeleteApplicationReferenceDataSourceError::Validation(ref cause) => cause,
            DeleteApplicationReferenceDataSourceError::Credentials(ref err) => err.description(),
            DeleteApplicationReferenceDataSourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationReferenceDataSourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeApplication
#[derive(Debug, PartialEq)]
pub enum DescribeApplicationError {
    ///<p>Specified application can't be found.</p>
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


impl DescribeApplicationError {
    pub fn from_body(body: &str) -> DescribeApplicationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeApplicationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeApplicationError::Validation(error_message.to_string())
                    }
                    _ => DescribeApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeApplicationError {
    fn from(err: serde_json::error::Error) -> DescribeApplicationError {
        DescribeApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeApplicationError {
    fn from(err: CredentialsError) -> DescribeApplicationError {
        DescribeApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeApplicationError {
    fn from(err: HttpDispatchError) -> DescribeApplicationError {
        DescribeApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeApplicationError {
    fn from(err: io::Error) -> DescribeApplicationError {
        DescribeApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeApplicationError {
    fn description(&self) -> &str {
        match *self {
            DescribeApplicationError::ResourceNotFound(ref cause) => cause,
            DescribeApplicationError::Validation(ref cause) => cause,
            DescribeApplicationError::Credentials(ref err) => err.description(),
            DescribeApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DiscoverInputSchema
#[derive(Debug, PartialEq)]
pub enum DiscoverInputSchemaError {
    ///<p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    ///<p>Discovery failed to get a record from the streaming source because of the Amazon Kinesis Streams ProvisionedThroughputExceededException. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/APIReference/API_GetRecords.html">GetRecords</a> in the Amazon Kinesis Streams API Reference.</p>
    ResourceProvisionedThroughputExceeded(String),
    ///<p>The service is unavailable, back off and retry the operation. </p>
    ServiceUnavailable(String),
    ///<p>Data format is not valid, Amazon Kinesis Analytics is not able to detect schema for the given streaming source.</p>
    UnableToDetectSchema(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DiscoverInputSchemaError {
    pub fn from_body(body: &str) -> DiscoverInputSchemaError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => {
                        DiscoverInputSchemaError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceProvisionedThroughputExceededException" => DiscoverInputSchemaError::ResourceProvisionedThroughputExceeded(String::from(error_message)),
                    "ServiceUnavailableException" => {
                        DiscoverInputSchemaError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnableToDetectSchemaException" => {
                        DiscoverInputSchemaError::UnableToDetectSchema(String::from(error_message))
                    }
                    "ValidationException" => {
                        DiscoverInputSchemaError::Validation(error_message.to_string())
                    }
                    _ => DiscoverInputSchemaError::Unknown(String::from(body)),
                }
            }
            Err(_) => DiscoverInputSchemaError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DiscoverInputSchemaError {
    fn from(err: serde_json::error::Error) -> DiscoverInputSchemaError {
        DiscoverInputSchemaError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DiscoverInputSchemaError {
    fn from(err: CredentialsError) -> DiscoverInputSchemaError {
        DiscoverInputSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DiscoverInputSchemaError {
    fn from(err: HttpDispatchError) -> DiscoverInputSchemaError {
        DiscoverInputSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for DiscoverInputSchemaError {
    fn from(err: io::Error) -> DiscoverInputSchemaError {
        DiscoverInputSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DiscoverInputSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DiscoverInputSchemaError {
    fn description(&self) -> &str {
        match *self {
            DiscoverInputSchemaError::InvalidArgument(ref cause) => cause,
            DiscoverInputSchemaError::ResourceProvisionedThroughputExceeded(ref cause) => cause,
            DiscoverInputSchemaError::ServiceUnavailable(ref cause) => cause,
            DiscoverInputSchemaError::UnableToDetectSchema(ref cause) => cause,
            DiscoverInputSchemaError::Validation(ref cause) => cause,
            DiscoverInputSchemaError::Credentials(ref err) => err.description(),
            DiscoverInputSchemaError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DiscoverInputSchemaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListApplicationsError {
    pub fn from_body(body: &str) -> ListApplicationsError {
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
                        ListApplicationsError::Validation(error_message.to_string())
                    }
                    _ => ListApplicationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListApplicationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListApplicationsError {
    fn from(err: serde_json::error::Error) -> ListApplicationsError {
        ListApplicationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListApplicationsError {
    fn from(err: CredentialsError) -> ListApplicationsError {
        ListApplicationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListApplicationsError {
    fn from(err: HttpDispatchError) -> ListApplicationsError {
        ListApplicationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListApplicationsError {
    fn from(err: io::Error) -> ListApplicationsError {
        ListApplicationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationsError {
    fn description(&self) -> &str {
        match *self {
            ListApplicationsError::Validation(ref cause) => cause,
            ListApplicationsError::Credentials(ref err) => err.description(),
            ListApplicationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListApplicationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartApplication
#[derive(Debug, PartialEq)]
pub enum StartApplicationError {
    ///<p>User-provided application configuration is not valid.</p>
    InvalidApplicationConfiguration(String),
    ///<p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    ///<p>Specified application can't be found.</p>
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


impl StartApplicationError {
    pub fn from_body(body: &str) -> StartApplicationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidApplicationConfigurationException" => StartApplicationError::InvalidApplicationConfiguration(String::from(error_message)),
                    "InvalidArgumentException" => {
                        StartApplicationError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        StartApplicationError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StartApplicationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartApplicationError::Validation(error_message.to_string())
                    }
                    _ => StartApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartApplicationError {
    fn from(err: serde_json::error::Error) -> StartApplicationError {
        StartApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartApplicationError {
    fn from(err: CredentialsError) -> StartApplicationError {
        StartApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartApplicationError {
    fn from(err: HttpDispatchError) -> StartApplicationError {
        StartApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartApplicationError {
    fn from(err: io::Error) -> StartApplicationError {
        StartApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartApplicationError {
    fn description(&self) -> &str {
        match *self {
            StartApplicationError::InvalidApplicationConfiguration(ref cause) => cause,
            StartApplicationError::InvalidArgument(ref cause) => cause,
            StartApplicationError::ResourceInUse(ref cause) => cause,
            StartApplicationError::ResourceNotFound(ref cause) => cause,
            StartApplicationError::Validation(ref cause) => cause,
            StartApplicationError::Credentials(ref err) => err.description(),
            StartApplicationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopApplication
#[derive(Debug, PartialEq)]
pub enum StopApplicationError {
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    ///<p>Specified application can't be found.</p>
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


impl StopApplicationError {
    pub fn from_body(body: &str) -> StopApplicationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceInUseException" => {
                        StopApplicationError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StopApplicationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopApplicationError::Validation(error_message.to_string())
                    }
                    _ => StopApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopApplicationError {
    fn from(err: serde_json::error::Error) -> StopApplicationError {
        StopApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopApplicationError {
    fn from(err: CredentialsError) -> StopApplicationError {
        StopApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopApplicationError {
    fn from(err: HttpDispatchError) -> StopApplicationError {
        StopApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopApplicationError {
    fn from(err: io::Error) -> StopApplicationError {
        StopApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopApplicationError {
    fn description(&self) -> &str {
        match *self {
            StopApplicationError::ResourceInUse(ref cause) => cause,
            StopApplicationError::ResourceNotFound(ref cause) => cause,
            StopApplicationError::Validation(ref cause) => cause,
            StopApplicationError::Credentials(ref err) => err.description(),
            StopApplicationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    ///<p>User-provided application code (query) is invalid. This can be a simple syntax error.</p>
    CodeValidation(String),
    ///<p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    ///<p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    ///<p>Application is not available for this operation.</p>
    ResourceInUse(String),
    ///<p>Specified application can't be found.</p>
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


impl UpdateApplicationError {
    pub fn from_body(body: &str) -> UpdateApplicationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CodeValidationException" => {
                        UpdateApplicationError::CodeValidation(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        UpdateApplicationError::ConcurrentModification(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        UpdateApplicationError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        UpdateApplicationError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateApplicationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateApplicationError::Validation(error_message.to_string())
                    }
                    _ => UpdateApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateApplicationError {
    fn from(err: serde_json::error::Error) -> UpdateApplicationError {
        UpdateApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApplicationError {
    fn from(err: CredentialsError) -> UpdateApplicationError {
        UpdateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApplicationError {
    fn from(err: HttpDispatchError) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApplicationError {
    fn from(err: io::Error) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationError::CodeValidation(ref cause) => cause,
            UpdateApplicationError::ConcurrentModification(ref cause) => cause,
            UpdateApplicationError::InvalidArgument(ref cause) => cause,
            UpdateApplicationError::ResourceInUse(ref cause) => cause,
            UpdateApplicationError::ResourceNotFound(ref cause) => cause,
            UpdateApplicationError::Validation(ref cause) => cause,
            UpdateApplicationError::Credentials(ref err) => err.description(),
            UpdateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Kinesis Analytics API. Kinesis Analytics clients implement this trait.
pub trait KinesisAnalytics {
    #[doc="<p>Adds a CloudWatch log stream to monitor application configuration errors. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html\">Working with Amazon CloudWatch Logs</a>.</p>"]
    fn add_application_cloud_watch_logging_option(&self, input: &AddApplicationCloudWatchLoggingOptionRequest)  -> Result<AddApplicationCloudWatchLoggingOptionResponse, AddApplicationCloudWatchLoggingOptionError>;


    #[doc="<p> Adds a streaming source to your Amazon Kinesis application. For conceptual information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>. </p> <p>You can add a streaming source either when you create an application or you can use this operation to add a streaming source after you create an application. For more information, see <a>CreateApplication</a>.</p> <p>Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationInput</code> action.</p>"]
    fn add_application_input(&self,
                             input: &AddApplicationInputRequest)
                             -> Result<AddApplicationInputResponse, AddApplicationInputError>;


    #[doc="<p>Adds an external destination to your Amazon Kinesis Analytics application.</p> <p>If you want Amazon Kinesis Analytics to deliver data from an in-application stream within your application to an external destination (such as an Amazon Kinesis stream or a Firehose delivery stream), you add the relevant configuration to your application using this operation. You can configure one or more outputs for your application. Each output configuration maps an in-application stream and an external destination.</p> <p> You can use one of the output configurations to deliver data from your in-application error stream to an external destination so that you can analyze the errors. For conceptual information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html\">Understanding Application Output (Destination)</a>. </p> <p> Note that any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version.</p> <p>For the limits on the number of application inputs and outputs you can configure, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html\">Limits</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action.</p>"]
    fn add_application_output
        (&self,
         input: &AddApplicationOutputRequest)
         -> Result<AddApplicationOutputResponse, AddApplicationOutputError>;


    #[doc="<p>Adds a reference data source to an existing application.</p> <p>Amazon Kinesis Analytics reads reference data (that is, an Amazon S3 object) and creates an in-application table within your application. In the request, you provide the source (S3 bucket name and object key name), name of the in-application table to create, and the necessary mapping information that describes how data in Amazon S3 object maps to columns in the resulting in-application table.</p> <p> For conceptual information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>. For the limits on data sources you can add to your application, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html\">Limits</a>. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action. </p>"]
    fn add_application_reference_data_source
        (&self,
         input: &AddApplicationReferenceDataSourceRequest)
         -> Result<AddApplicationReferenceDataSourceResponse,
                   AddApplicationReferenceDataSourceError>;


    #[doc="<p> Creates an Amazon Kinesis Analytics application. You can configure each application with one streaming source as input, application code to process the input, and up to five streaming destinations where you want Amazon Kinesis Analytics to write the output data from your application. For an overview, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works.html\">How it Works</a>. </p> <p>In the input configuration, you map the streaming source to an in-application stream, which you can think of as a constantly updating table. In the mapping, you must provide a schema for the in-application stream and map each data column in the in-application stream to a data element in the streaming source.</p> <p>Your application code is one or more SQL statements that read input data, transform it, and generate output. Your application code can create one or more SQL artifacts like SQL streams or pumps.</p> <p>In the output configuration, you can configure the application to write data from in-application streams created in your applications to up to five streaming destinations.</p> <p> To read data from your source stream or write data to destination streams, Amazon Kinesis Analytics needs your permissions. You grant these permissions by creating IAM roles. This operation requires permissions to perform the <code>kinesisanalytics:CreateApplication</code> action. </p> <p> For introductory exercises to create an Amazon Kinesis Analytics application, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/getting-started.html\">Getting Started</a>. </p>"]
    fn create_application(&self,
                          input: &CreateApplicationRequest)
                          -> Result<CreateApplicationResponse, CreateApplicationError>;


    #[doc="<p>Deletes the specified application. Amazon Kinesis Analytics halts application execution and deletes the application, including any application artifacts (such as in-application streams, reference table, and application code).</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplication</code> action.</p>"]
    fn delete_application(&self,
                          input: &DeleteApplicationRequest)
                          -> Result<DeleteApplicationResponse, DeleteApplicationError>;


    #[doc="<p>Deletes a CloudWatch log stream from an application. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html\">Working with Amazon CloudWatch Logs</a>.</p>"]
    fn delete_application_cloud_watch_logging_option(&self, input: &DeleteApplicationCloudWatchLoggingOptionRequest)  -> Result<DeleteApplicationCloudWatchLoggingOptionResponse, DeleteApplicationCloudWatchLoggingOptionError>;


    #[doc="<p>Deletes output destination configuration from your application configuration. Amazon Kinesis Analytics will no longer write data from the corresponding in-application stream to the external output destination.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplicationOutput</code> action.</p>"]
    fn delete_application_output
        (&self,
         input: &DeleteApplicationOutputRequest)
         -> Result<DeleteApplicationOutputResponse, DeleteApplicationOutputError>;


    #[doc="<p>Deletes a reference data source configuration from the specified application configuration.</p> <p>If the application is running, Amazon Kinesis Analytics immediately removes the in-application table that you created using the <a>AddApplicationReferenceDataSource</a> operation. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics.DeleteApplicationReferenceDataSource</code> action.</p>"]
    fn delete_application_reference_data_source(&self, input: &DeleteApplicationReferenceDataSourceRequest)  -> Result<DeleteApplicationReferenceDataSourceResponse, DeleteApplicationReferenceDataSourceError>;


    #[doc="<p>Returns information about a specific Amazon Kinesis Analytics application.</p> <p>If you want to retrieve a list of all applications in your account, use the <a>ListApplications</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DescribeApplication</code> action. You can use <code>DescribeApplication</code> to get the current application versionId, which you need to call other operations such as <code>Update</code>. </p>"]
    fn describe_application(&self,
                            input: &DescribeApplicationRequest)
                            -> Result<DescribeApplicationResponse, DescribeApplicationError>;


    #[doc="<p>Infers a schema by evaluating sample records on the specified streaming source (Amazon Kinesis stream or Amazon Kinesis Firehose delivery stream). In the response, the operation returns the inferred schema and also the sample records that the operation used to infer the schema.</p> <p> You can use the inferred schema when configuring a streaming source for your application. For conceptual information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>. Note that when you create an application using the Amazon Kinesis Analytics console, the console uses this operation to infer a schema and show it in the console user interface. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:DiscoverInputSchema</code> action. </p>"]
    fn discover_input_schema(&self,
                             input: &DiscoverInputSchemaRequest)
                             -> Result<DiscoverInputSchemaResponse, DiscoverInputSchemaError>;


    #[doc="<p>Returns a list of Amazon Kinesis Analytics applications in your account. For each application, the response includes the application name, Amazon Resource Name (ARN), and status. If the response returns the <code>HasMoreApplications</code> value as true, you can send another request by adding the <code>ExclusiveStartApplicationName</code> in the request body, and set the value of this to the last application name from the previous response. </p> <p>If you want detailed information about a specific application, use <a>DescribeApplication</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:ListApplications</code> action.</p>"]
    fn list_applications(&self,
                         input: &ListApplicationsRequest)
                         -> Result<ListApplicationsResponse, ListApplicationsError>;


    #[doc="<p>Starts the specified Amazon Kinesis Analytics application. After creating an application, you must exclusively call this operation to start your application.</p> <p>After the application starts, it begins consuming the input data, processes it, and writes the output to the configured destination.</p> <p> The application status must be <code>READY</code> for you to start an application. You can get the application status in the console or using the <a>DescribeApplication</a> operation.</p> <p>After you start the application, you can stop the application from processing the input by calling the <a>StopApplication</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StartApplication</code> action.</p>"]
    fn start_application(&self,
                         input: &StartApplicationRequest)
                         -> Result<StartApplicationResponse, StartApplicationError>;


    #[doc="<p>Stops the application from processing input data. You can stop an application only if it is in the running state. You can use the <a>DescribeApplication</a> operation to find the application state. After the application is stopped, Amazon Kinesis Analytics stops reading data from the input, the application stops processing data, and there is no output written to the destination. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StopApplication</code> action.</p>"]
    fn stop_application(&self,
                        input: &StopApplicationRequest)
                        -> Result<StopApplicationResponse, StopApplicationError>;


    #[doc="<p>Updates an existing Amazon Kinesis Analytics application. Using this API, you can update application code, input configuration, and output configuration. </p> <p>Note that Amazon Kinesis Analytics updates the <code>CurrentApplicationVersionId</code> each time you update your application. </p> <p>This operation requires permission for the <code>kinesisanalytics:UpdateApplication</code> action.</p>"]
    fn update_application(&self,
                          input: &UpdateApplicationRequest)
                          -> Result<UpdateApplicationResponse, UpdateApplicationError>;
}
/// A client for the Kinesis Analytics API.
pub struct KinesisAnalyticsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> KinesisAnalyticsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        KinesisAnalyticsClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> KinesisAnalytics for KinesisAnalyticsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Adds a CloudWatch log stream to monitor application configuration errors. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html\">Working with Amazon CloudWatch Logs</a>.</p>"]
fn add_application_cloud_watch_logging_option(&self, input: &AddApplicationCloudWatchLoggingOptionRequest)  -> Result<AddApplicationCloudWatchLoggingOptionResponse, AddApplicationCloudWatchLoggingOptionError>{
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.AddApplicationCloudWatchLoggingOption");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AddApplicationCloudWatchLoggingOptionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AddApplicationCloudWatchLoggingOptionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Adds a streaming source to your Amazon Kinesis application. For conceptual information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>. </p> <p>You can add a streaming source either when you create an application or you can use this operation to add a streaming source after you create an application. For more information, see <a>CreateApplication</a>.</p> <p>Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationInput</code> action.</p>"]
    fn add_application_input(&self,
                             input: &AddApplicationInputRequest)
                             -> Result<AddApplicationInputResponse, AddApplicationInputError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.AddApplicationInput");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AddApplicationInputResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AddApplicationInputError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Adds an external destination to your Amazon Kinesis Analytics application.</p> <p>If you want Amazon Kinesis Analytics to deliver data from an in-application stream within your application to an external destination (such as an Amazon Kinesis stream or a Firehose delivery stream), you add the relevant configuration to your application using this operation. You can configure one or more outputs for your application. Each output configuration maps an in-application stream and an external destination.</p> <p> You can use one of the output configurations to deliver data from your in-application error stream to an external destination so that you can analyze the errors. For conceptual information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html\">Understanding Application Output (Destination)</a>. </p> <p> Note that any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version.</p> <p>For the limits on the number of application inputs and outputs you can configure, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html\">Limits</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action.</p>"]
    fn add_application_output
        (&self,
         input: &AddApplicationOutputRequest)
         -> Result<AddApplicationOutputResponse, AddApplicationOutputError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.AddApplicationOutput");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AddApplicationOutputResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AddApplicationOutputError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Adds a reference data source to an existing application.</p> <p>Amazon Kinesis Analytics reads reference data (that is, an Amazon S3 object) and creates an in-application table within your application. In the request, you provide the source (S3 bucket name and object key name), name of the in-application table to create, and the necessary mapping information that describes how data in Amazon S3 object maps to columns in the resulting in-application table.</p> <p> For conceptual information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>. For the limits on data sources you can add to your application, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html\">Limits</a>. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action. </p>"]
    fn add_application_reference_data_source
        (&self,
         input: &AddApplicationReferenceDataSourceRequest)
         -> Result<AddApplicationReferenceDataSourceResponse,
                   AddApplicationReferenceDataSourceError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.AddApplicationReferenceDataSource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AddApplicationReferenceDataSourceResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AddApplicationReferenceDataSourceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> Creates an Amazon Kinesis Analytics application. You can configure each application with one streaming source as input, application code to process the input, and up to five streaming destinations where you want Amazon Kinesis Analytics to write the output data from your application. For an overview, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works.html\">How it Works</a>. </p> <p>In the input configuration, you map the streaming source to an in-application stream, which you can think of as a constantly updating table. In the mapping, you must provide a schema for the in-application stream and map each data column in the in-application stream to a data element in the streaming source.</p> <p>Your application code is one or more SQL statements that read input data, transform it, and generate output. Your application code can create one or more SQL artifacts like SQL streams or pumps.</p> <p>In the output configuration, you can configure the application to write data from in-application streams created in your applications to up to five streaming destinations.</p> <p> To read data from your source stream or write data to destination streams, Amazon Kinesis Analytics needs your permissions. You grant these permissions by creating IAM roles. This operation requires permissions to perform the <code>kinesisanalytics:CreateApplication</code> action. </p> <p> For introductory exercises to create an Amazon Kinesis Analytics application, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/getting-started.html\">Getting Started</a>. </p>"]
    fn create_application(&self,
                          input: &CreateApplicationRequest)
                          -> Result<CreateApplicationResponse, CreateApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.CreateApplication");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateApplicationResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateApplicationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified application. Amazon Kinesis Analytics halts application execution and deletes the application, including any application artifacts (such as in-application streams, reference table, and application code).</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplication</code> action.</p>"]
    fn delete_application(&self,
                          input: &DeleteApplicationRequest)
                          -> Result<DeleteApplicationResponse, DeleteApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.DeleteApplication");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteApplicationResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteApplicationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a CloudWatch log stream from an application. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html\">Working with Amazon CloudWatch Logs</a>.</p>"]
fn delete_application_cloud_watch_logging_option(&self, input: &DeleteApplicationCloudWatchLoggingOptionRequest)  -> Result<DeleteApplicationCloudWatchLoggingOptionResponse, DeleteApplicationCloudWatchLoggingOptionError>{
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.DeleteApplicationCloudWatchLoggingOption");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteApplicationCloudWatchLoggingOptionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteApplicationCloudWatchLoggingOptionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes output destination configuration from your application configuration. Amazon Kinesis Analytics will no longer write data from the corresponding in-application stream to the external output destination.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplicationOutput</code> action.</p>"]
    fn delete_application_output
        (&self,
         input: &DeleteApplicationOutputRequest)
         -> Result<DeleteApplicationOutputResponse, DeleteApplicationOutputError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.DeleteApplicationOutput");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteApplicationOutputResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteApplicationOutputError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a reference data source configuration from the specified application configuration.</p> <p>If the application is running, Amazon Kinesis Analytics immediately removes the in-application table that you created using the <a>AddApplicationReferenceDataSource</a> operation. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics.DeleteApplicationReferenceDataSource</code> action.</p>"]
fn delete_application_reference_data_source(&self, input: &DeleteApplicationReferenceDataSourceRequest)  -> Result<DeleteApplicationReferenceDataSourceResponse, DeleteApplicationReferenceDataSourceError>{
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.DeleteApplicationReferenceDataSource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteApplicationReferenceDataSourceResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteApplicationReferenceDataSourceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about a specific Amazon Kinesis Analytics application.</p> <p>If you want to retrieve a list of all applications in your account, use the <a>ListApplications</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DescribeApplication</code> action. You can use <code>DescribeApplication</code> to get the current application versionId, which you need to call other operations such as <code>Update</code>. </p>"]
    fn describe_application(&self,
                            input: &DescribeApplicationRequest)
                            -> Result<DescribeApplicationResponse, DescribeApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.DescribeApplication");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeApplicationResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeApplicationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Infers a schema by evaluating sample records on the specified streaming source (Amazon Kinesis stream or Amazon Kinesis Firehose delivery stream). In the response, the operation returns the inferred schema and also the sample records that the operation used to infer the schema.</p> <p> You can use the inferred schema when configuring a streaming source for your application. For conceptual information, see <a href=\"http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html\">Configuring Application Input</a>. Note that when you create an application using the Amazon Kinesis Analytics console, the console uses this operation to infer a schema and show it in the console user interface. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:DiscoverInputSchema</code> action. </p>"]
    fn discover_input_schema(&self,
                             input: &DiscoverInputSchemaRequest)
                             -> Result<DiscoverInputSchemaResponse, DiscoverInputSchemaError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.DiscoverInputSchema");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DiscoverInputSchemaResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DiscoverInputSchemaError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a list of Amazon Kinesis Analytics applications in your account. For each application, the response includes the application name, Amazon Resource Name (ARN), and status. If the response returns the <code>HasMoreApplications</code> value as true, you can send another request by adding the <code>ExclusiveStartApplicationName</code> in the request body, and set the value of this to the last application name from the previous response. </p> <p>If you want detailed information about a specific application, use <a>DescribeApplication</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:ListApplications</code> action.</p>"]
    fn list_applications(&self,
                         input: &ListApplicationsRequest)
                         -> Result<ListApplicationsResponse, ListApplicationsError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "KinesisAnalytics_20150814.ListApplications");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListApplicationsResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListApplicationsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Starts the specified Amazon Kinesis Analytics application. After creating an application, you must exclusively call this operation to start your application.</p> <p>After the application starts, it begins consuming the input data, processes it, and writes the output to the configured destination.</p> <p> The application status must be <code>READY</code> for you to start an application. You can get the application status in the console or using the <a>DescribeApplication</a> operation.</p> <p>After you start the application, you can stop the application from processing the input by calling the <a>StopApplication</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StartApplication</code> action.</p>"]
    fn start_application(&self,
                         input: &StartApplicationRequest)
                         -> Result<StartApplicationResponse, StartApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "KinesisAnalytics_20150814.StartApplication");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartApplicationResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartApplicationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Stops the application from processing input data. You can stop an application only if it is in the running state. You can use the <a>DescribeApplication</a> operation to find the application state. After the application is stopped, Amazon Kinesis Analytics stops reading data from the input, the application stops processing data, and there is no output written to the destination. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StopApplication</code> action.</p>"]
    fn stop_application(&self,
                        input: &StopApplicationRequest)
                        -> Result<StopApplicationResponse, StopApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "KinesisAnalytics_20150814.StopApplication");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StopApplicationResponse>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StopApplicationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates an existing Amazon Kinesis Analytics application. Using this API, you can update application code, input configuration, and output configuration. </p> <p>Note that Amazon Kinesis Analytics updates the <code>CurrentApplicationVersionId</code> each time you update your application. </p> <p>This operation requires permission for the <code>kinesisanalytics:UpdateApplication</code> action.</p>"]
    fn update_application(&self,
                          input: &UpdateApplicationRequest)
                          -> Result<UpdateApplicationResponse, UpdateApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "KinesisAnalytics_20150814.UpdateApplication");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateApplicationResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateApplicationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
