
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
#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AcceptMatchInput {
    #[doc="<p>Player response to the proposed match.</p>"]
    #[serde(rename="AcceptanceType")]
    pub acceptance_type: String,
    #[doc="<p>Unique identifier for a player delivering the response. This parameter can include one or multiple player IDs.</p>"]
    #[serde(rename="PlayerIds")]
    pub player_ids: Vec<String>,
    #[doc="<p>Unique identifier for a matchmaking ticket. The ticket must be in status <code>REQUIRES_ACCEPTANCE</code>; otherwise this request will fail.</p>"]
    #[serde(rename="TicketId")]
    pub ticket_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AcceptMatchOutput;

#[doc="<p>Properties describing a fleet alias.</p> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Alias {
    #[doc="<p>Unique identifier for an alias; alias ARNs are unique across all regions.</p>"]
    #[serde(rename="AliasArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias_arn: Option<String>,
    #[doc="<p>Unique identifier for an alias; alias IDs are unique within a region.</p>"]
    #[serde(rename="AliasId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias_id: Option<String>,
    #[doc="<p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>Human-readable description of an alias.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Time stamp indicating when this data object was last modified. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="LastUpdatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[doc="<p>Descriptive label that is associated with an alias. Alias names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Alias configuration for the alias, including routing type and settings.</p>"]
    #[serde(rename="RoutingStrategy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub routing_strategy: Option<RoutingStrategy>,
}

#[doc="<p>Values for use in <a>Player</a> attribute type:value pairs. This object lets you specify an attribute value using any of the valid data types: string, number, string array or data map. Each <code>AttributeValue</code> object can use only one of the available properties.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct AttributeValue {
    #[doc="<p>For number values, expressed as double.</p>"]
    #[serde(rename="N")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub n: Option<f64>,
    #[doc="<p>For single string values. Maximum string length is 100 characters.</p>"]
    #[serde(rename="S")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s: Option<String>,
    #[doc="<p>For a map of up to 10 type:value pairs. Maximum length for each string value is 100 characters. </p>"]
    #[serde(rename="SDM")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sdm: Option<::std::collections::HashMap<String, f64>>,
    #[doc="<p>For a list of up to 10 strings. Maximum length for each string is 100 characters. Duplicate values are not recognized; all occurances of the the repeated value after the first of a repeated value are ignored.</p>"]
    #[serde(rename="SL")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sl: Option<Vec<String>>,
}

#[doc="<p>Temporary access credentials used for uploading game build files to Amazon GameLift. They are valid for a limited time. If they expire before you upload your game build, get a new set by calling <a>RequestUploadCredentials</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AwsCredentials {
    #[doc="<p>Temporary key allowing access to the Amazon GameLift S3 account.</p>"]
    #[serde(rename="AccessKeyId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_key_id: Option<String>,
    #[doc="<p>Temporary secret key allowing access to the Amazon GameLift S3 account.</p>"]
    #[serde(rename="SecretAccessKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret_access_key: Option<String>,
    #[doc="<p>Token used to associate a specific build ID with the files uploaded using these credentials.</p>"]
    #[serde(rename="SessionToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session_token: Option<String>,
}

#[doc="<p>Properties describing a game build.</p> <p>Build-related operations include:</p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Build {
    #[doc="<p>Unique identifier for a build.</p>"]
    #[serde(rename="BuildId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build_id: Option<String>,
    #[doc="<p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>Descriptive label that is associated with a build. Build names do not need to be unique. It can be set using <a>CreateBuild</a> or <a>UpdateBuild</a>.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Operating system that the game server binaries are built to run on. This value determines the type of fleet resources that you can use for this build.</p>"]
    #[serde(rename="OperatingSystem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operating_system: Option<String>,
    #[doc="<p>File size of the uploaded game build, expressed in bytes. When the build status is <code>INITIALIZED</code>, this value is 0.</p>"]
    #[serde(rename="SizeOnDisk")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size_on_disk: Option<i64>,
    #[doc="<p>Current status of the build.</p> <p>Possible build statuses include the following:</p> <ul> <li> <p> <b>INITIALIZED</b> – A new build has been defined, but no files have been uploaded. You cannot create fleets for builds that are in this status. When a build is successfully created, the build status is set to this value. </p> </li> <li> <p> <b>READY</b> – The game build has been successfully uploaded. You can now create new fleets for this build.</p> </li> <li> <p> <b>FAILED</b> – The game build upload failed. You cannot create new fleets for this build. </p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>Version that is associated with this build. Version strings do not need to be unique. This value can be set using <a>CreateBuild</a> or <a>UpdateBuild</a>.</p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateAliasInput {
    #[doc="<p>Human-readable description of an alias.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Descriptive label that is associated with an alias. Alias names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>Object that specifies the fleet and routing type to use for the alias.</p>"]
    #[serde(rename="RoutingStrategy")]
    pub routing_strategy: RoutingStrategy,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateAliasOutput {
    #[doc="<p>Object that describes the newly created alias record.</p>"]
    #[serde(rename="Alias")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias: Option<Alias>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateBuildInput {
    #[doc="<p>Descriptive label that is associated with a build. Build names do not need to be unique. You can use <a>UpdateBuild</a> to change this value later. </p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Operating system that the game server binaries are built to run on. This value determines the type of fleet resources that you can use for this build. If your game build contains multiple executables, they all must run on the same operating system.</p>"]
    #[serde(rename="OperatingSystem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operating_system: Option<String>,
    #[doc="<p>Amazon S3 location of the game build files to be uploaded. The S3 bucket must be owned by the same AWS account that you're using to manage Amazon GameLift. It also must in the same region that you want to create a new build in. Before calling <code>CreateBuild</code> with this location, you must allow Amazon GameLift to access your Amazon S3 bucket (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-cli-uploading.html#gamelift-build-cli-uploading-create-build\">Create a Build with Files in Amazon S3</a>).</p>"]
    #[serde(rename="StorageLocation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_location: Option<S3Location>,
    #[doc="<p>Version that is associated with this build. Version strings do not need to be unique. You can use <a>UpdateBuild</a> to change this value later. </p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateBuildOutput {
    #[doc="<p>The newly created build record, including a unique build ID and status. </p>"]
    #[serde(rename="Build")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build: Option<Build>,
    #[doc="<p>Amazon S3 location specified in the request.</p>"]
    #[serde(rename="StorageLocation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_location: Option<S3Location>,
    #[doc="<p>This element is not currently in use.</p>"]
    #[serde(rename="UploadCredentials")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub upload_credentials: Option<AwsCredentials>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateFleetInput {
    #[doc="<p>Unique identifier for a build to be deployed on the new fleet. The build must have been successfully uploaded to Amazon GameLift and be in a <code>READY</code> status. This fleet setting cannot be changed once the fleet is created.</p>"]
    #[serde(rename="BuildId")]
    pub build_id: String,
    #[doc="<p>Human-readable description of a fleet.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Range of IP addresses and port settings that permit inbound traffic to access server processes running on the fleet. If no inbound permissions are set, including both IP address range and port range, the server processes in the fleet cannot accept connections. You can specify one or more sets of permissions for a fleet.</p>"]
    #[serde(rename="EC2InboundPermissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec2_inbound_permissions: Option<Vec<IpPermission>>,
    #[doc="<p>Name of an EC2 instance type that is supported in Amazon GameLift. A fleet instance type determines the computing resources of each instance in the fleet, including CPU, memory, storage, and networking capacity. Amazon GameLift supports the following EC2 instance types. See <a href=\"http://aws.amazon.com/ec2/instance-types/\">Amazon EC2 Instance Types</a> for detailed descriptions.</p>"]
    #[serde(rename="EC2InstanceType")]
    pub ec2_instance_type: String,
    #[doc="<p>This parameter is no longer used. Instead, to specify where Amazon GameLift should store log files once a server process shuts down, use the Amazon GameLift server API <code>ProcessReady()</code> and specify one or more directory paths in <code>logParameters</code>. See more information in the <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api-ref.html#gamelift-sdk-server-api-ref-dataypes-process\">Server API Reference</a>. </p>"]
    #[serde(rename="LogPaths")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_paths: Option<Vec<String>>,
    #[doc="<p>Names of metric groups to add this fleet to. Use an existing metric group name to add this fleet to the group. Or use a new name to create a new metric group. A fleet can only be included in one metric group at a time.</p>"]
    #[serde(rename="MetricGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric_groups: Option<Vec<String>>,
    #[doc="<p>Descriptive label that is associated with a fleet. Fleet names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>Game session protection policy to apply to all instances in this fleet. If this parameter is not set, instances in this fleet default to no protection. You can change a fleet's protection policy using UpdateFleetAttributes, but this change will only affect sessions created after the policy change. You can also set protection for individual instances using <a>UpdateGameSession</a>.</p> <ul> <li> <p> <b>NoProtection</b> – The game session can be terminated during a scale-down event.</p> </li> <li> <p> <b>FullProtection</b> – If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li> </ul>"]
    #[serde(rename="NewGameSessionProtectionPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_game_session_protection_policy: Option<String>,
    #[doc="<p>Policy that limits the number of game sessions an individual player can create over a span of time for this fleet.</p>"]
    #[serde(rename="ResourceCreationLimitPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_creation_limit_policy: Option<ResourceCreationLimitPolicy>,
    #[doc="<p>Instructions for launching server processes on each instance in the fleet. The run-time configuration for a fleet has a collection of server process configurations, one for each type of server process to run on an instance. A server process configuration specifies the location of the server executable, launch parameters, and the number of concurrent processes with that configuration to maintain on each instance. A CreateFleet request must include a run-time configuration with at least one server process configuration; otherwise the request fails with an invalid request exception. (This parameter replaces the parameters <code>ServerLaunchPath</code> and <code>ServerLaunchParameters</code>; requests that contain values for these parameters instead of a run-time configuration will continue to work.) </p>"]
    #[serde(rename="RuntimeConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runtime_configuration: Option<RuntimeConfiguration>,
    #[doc="<p>This parameter is no longer used. Instead, specify server launch parameters in the <code>RuntimeConfiguration</code> parameter. (Requests that specify a server launch path and launch parameters instead of a run-time configuration will continue to work.)</p>"]
    #[serde(rename="ServerLaunchParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_launch_parameters: Option<String>,
    #[doc="<p>This parameter is no longer used. Instead, specify a server launch path using the <code>RuntimeConfiguration</code> parameter. (Requests that specify a server launch path and launch parameters instead of a run-time configuration will continue to work.)</p>"]
    #[serde(rename="ServerLaunchPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_launch_path: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateFleetOutput {
    #[doc="<p>Properties for the newly created fleet.</p>"]
    #[serde(rename="FleetAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_attributes: Option<FleetAttributes>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateGameSessionInput {
    #[doc="<p>Unique identifier for an alias associated with the fleet to create a game session in. Each request must reference either a fleet ID or alias ID, but not both.</p>"]
    #[serde(rename="AliasId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias_id: Option<String>,
    #[doc="<p>Unique identifier for a player or entity creating the game session. This ID is used to enforce a resource protection policy (if one exists) that limits the number of concurrent active game sessions one player can have.</p>"]
    #[serde(rename="CreatorId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creator_id: Option<String>,
    #[doc="<p>Unique identifier for a fleet to create a game session in. Each request must reference either a fleet ID or alias ID, but not both.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Set of developer-defined properties for a game session, formatted as a set of type:value pairs. These properties are included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>).</p>"]
    #[serde(rename="GameProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    #[doc="<p>Set of developer-defined game session properties, formatted as a single string value. This data is included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>).</p>"]
    #[serde(rename="GameSessionData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_data: Option<String>,
    #[doc="<p> <i>This parameter is no longer preferred. Please use <code>IdempotencyToken</code> instead.</i> Custom string that uniquely identifies a request for a new game session. Maximum token length is 48 characters. If provided, this string is included in the new game session's ID. (A game session ID has the following format: <code>arn:aws:gamelift:&lt;region&gt;::gamesession/&lt;fleet ID&gt;/&lt;custom ID string or idempotency token&gt;</code>.) </p>"]
    #[serde(rename="GameSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_id: Option<String>,
    #[doc="<p>Custom string that uniquely identifies a request for a new game session. Maximum token length is 48 characters. If provided, this string is included in the new game session's ID. (A game session ID has the following format: <code>arn:aws:gamelift:&lt;region&gt;::gamesession/&lt;fleet ID&gt;/&lt;custom ID string or idempotency token&gt;</code>.) Idempotency tokens remain in use for 30 days after a game session has ended; game session objects are retained for this time period and then deleted.</p>"]
    #[serde(rename="IdempotencyToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub idempotency_token: Option<String>,
    #[doc="<p>Maximum number of players that can be connected simultaneously to the game session.</p>"]
    #[serde(rename="MaximumPlayerSessionCount")]
    pub maximum_player_session_count: i64,
    #[doc="<p>Descriptive label that is associated with a game session. Session names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateGameSessionOutput {
    #[doc="<p>Object that describes the newly created game session record.</p>"]
    #[serde(rename="GameSession")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session: Option<GameSession>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateGameSessionQueueInput {
    #[doc="<p>List of fleets that can be used to fulfill game session placement requests in the queue. Fleets are identified by either a fleet ARN or a fleet alias ARN. Destinations are listed in default preference order.</p>"]
    #[serde(rename="Destinations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub destinations: Option<Vec<GameSessionQueueDestination>>,
    #[doc="<p>Descriptive label that is associated with game session queue. Queue names must be unique within each region.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>Collection of latency policies to apply when processing game sessions placement requests with player latency information. Multiple policies are evaluated in order of the maximum latency value, starting with the lowest latency values. With just one policy, it is enforced at the start of the game session placement for the duration period. With multiple policies, each policy is enforced consecutively for its duration period. For example, a queue might enforce a 60-second policy followed by a 120-second policy, and then no policy for the remainder of the placement. A player latency policy must set a value for MaximumIndividualPlayerLatencyMilliseconds; if none is set, this API requests will fail.</p>"]
    #[serde(rename="PlayerLatencyPolicies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_latency_policies: Option<Vec<PlayerLatencyPolicy>>,
    #[doc="<p>Maximum time, in seconds, that a new game session placement request remains in the queue. When a request exceeds this time, the game session placement changes to a <code>TIMED_OUT</code> status.</p>"]
    #[serde(rename="TimeoutInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateGameSessionQueueOutput {
    #[doc="<p>Object that describes the newly created game session queue.</p>"]
    #[serde(rename="GameSessionQueue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_queue: Option<GameSessionQueue>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateMatchmakingConfigurationInput {
    #[doc="<p>Flag that determines whether or not a match that was created with this configuration must be accepted by the matched players. To require acceptance, set to TRUE.</p>"]
    #[serde(rename="AcceptanceRequired")]
    pub acceptance_required: bool,
    #[doc="<p>Length of time (in seconds) to wait for players to accept a proposed match. If any player rejects the match or fails to accept before the timeout, the ticket continues to look for an acceptable match.</p>"]
    #[serde(rename="AcceptanceTimeoutSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acceptance_timeout_seconds: Option<i64>,
    #[doc="<p>Number of player slots in a match to keep open for future players. For example, if the configuration's rule set specifies a match for a single 12-person team, and the additional player count is set to 2, only 10 players are selected for the match.</p>"]
    #[serde(rename="AdditionalPlayerCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_player_count: Option<i64>,
    #[doc="<p>Information to attached to all events related to the matchmaking configuration. </p>"]
    #[serde(rename="CustomEventData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_event_data: Option<String>,
    #[doc="<p>Meaningful description of the matchmaking configuration. </p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Set of developer-defined properties for a game session, formatted as a set of type:value pairs. These properties are included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match. </p>"]
    #[serde(rename="GameProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    #[doc="<p>Set of developer-defined game session properties, formatted as a single string value. This data is included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match.</p>"]
    #[serde(rename="GameSessionData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_data: Option<String>,
    #[doc="<p>Amazon Resource Name (<a href=\"http://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html\">ARN</a>) that is assigned to a game session queue and uniquely identifies it. Format is <code>arn:aws:gamelift:&lt;region&gt;::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912</code>. These queues are used when placing game sessions for matches that are created with this matchmaking configuration. Queues can be located in any region.</p>"]
    #[serde(rename="GameSessionQueueArns")]
    pub game_session_queue_arns: Vec<String>,
    #[doc="<p>Unique identifier for a matchmaking configuration. This name is used to identify the configuration associated with a matchmaking request or ticket.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>SNS topic ARN that is set up to receive matchmaking notifications.</p>"]
    #[serde(rename="NotificationTarget")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notification_target: Option<String>,
    #[doc="<p>Maximum duration, in seconds, that a matchmaking ticket can remain in process before timing out. Requests that time out can be resubmitted as needed.</p>"]
    #[serde(rename="RequestTimeoutSeconds")]
    pub request_timeout_seconds: i64,
    #[doc="<p>Unique identifier for a matchmaking rule set to use with this configuration. A matchmaking configuration can only use rule sets that are defined in the same region.</p>"]
    #[serde(rename="RuleSetName")]
    pub rule_set_name: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateMatchmakingConfigurationOutput {
    #[doc="<p>Object that describes the newly created matchmaking configuration.</p>"]
    #[serde(rename="Configuration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configuration: Option<MatchmakingConfiguration>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateMatchmakingRuleSetInput {
    #[doc="<p>Unique identifier for a matchmaking rule set. This name is used to identify the rule set associated with a matchmaking configuration.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>Collection of matchmaking rules, formatted as a JSON string. (Note that comments are not allowed in JSON, but most elements support a description field.)</p>"]
    #[serde(rename="RuleSetBody")]
    pub rule_set_body: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateMatchmakingRuleSetOutput {
    #[doc="<p>Object that describes the newly created matchmaking rule set.</p>"]
    #[serde(rename="RuleSet")]
    pub rule_set: MatchmakingRuleSet,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreatePlayerSessionInput {
    #[doc="<p>Unique identifier for the game session to add a player to.</p>"]
    #[serde(rename="GameSessionId")]
    pub game_session_id: String,
    #[doc="<p>Developer-defined information related to a player. Amazon GameLift does not use this data, so it can be formatted as needed for use in the game.</p>"]
    #[serde(rename="PlayerData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_data: Option<String>,
    #[doc="<p>Unique identifier for a player. Player IDs are developer-defined.</p>"]
    #[serde(rename="PlayerId")]
    pub player_id: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreatePlayerSessionOutput {
    #[doc="<p>Object that describes the newly created player session record.</p>"]
    #[serde(rename="PlayerSession")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_session: Option<PlayerSession>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreatePlayerSessionsInput {
    #[doc="<p>Unique identifier for the game session to add players to.</p>"]
    #[serde(rename="GameSessionId")]
    pub game_session_id: String,
    #[doc="<p>Map of string pairs, each specifying a player ID and a set of developer-defined information related to the player. Amazon GameLift does not use this data, so it can be formatted as needed for use in the game. Player data strings for player IDs not included in the <code>PlayerIds</code> parameter are ignored. </p>"]
    #[serde(rename="PlayerDataMap")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_data_map: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>List of unique identifiers for the players to be added.</p>"]
    #[serde(rename="PlayerIds")]
    pub player_ids: Vec<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreatePlayerSessionsOutput {
    #[doc="<p>Collection of player session objects created for the added players.</p>"]
    #[serde(rename="PlayerSessions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_sessions: Option<Vec<PlayerSession>>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteAliasInput {
    #[doc="<p>Unique identifier for a fleet alias. Specify the alias you want to delete.</p>"]
    #[serde(rename="AliasId")]
    pub alias_id: String,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteBuildInput {
    #[doc="<p>Unique identifier for a build to delete.</p>"]
    #[serde(rename="BuildId")]
    pub build_id: String,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteFleetInput {
    #[doc="<p>Unique identifier for a fleet to be deleted.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteGameSessionQueueInput {
    #[doc="<p>Descriptive label that is associated with game session queue. Queue names must be unique within each region.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteGameSessionQueueOutput;

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteMatchmakingConfigurationInput {
    #[doc="<p>Unique identifier for a matchmaking configuration</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteMatchmakingConfigurationOutput;

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteScalingPolicyInput {
    #[doc="<p>Unique identifier for a fleet to be deleted.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
    #[doc="<p>Descriptive label that is associated with a scaling policy. Policy names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeAliasInput {
    #[doc="<p>Unique identifier for a fleet alias. Specify the alias you want to retrieve.</p>"]
    #[serde(rename="AliasId")]
    pub alias_id: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeAliasOutput {
    #[doc="<p>Object that contains the requested alias.</p>"]
    #[serde(rename="Alias")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias: Option<Alias>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeBuildInput {
    #[doc="<p>Unique identifier for a build to retrieve properties for.</p>"]
    #[serde(rename="BuildId")]
    pub build_id: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeBuildOutput {
    #[doc="<p>Set of properties describing the requested build.</p>"]
    #[serde(rename="Build")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build: Option<Build>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEC2InstanceLimitsInput {
    #[doc="<p>Name of an EC2 instance type that is supported in Amazon GameLift. A fleet instance type determines the computing resources of each instance in the fleet, including CPU, memory, storage, and networking capacity. Amazon GameLift supports the following EC2 instance types. See <a href=\"http://aws.amazon.com/ec2/instance-types/\">Amazon EC2 Instance Types</a> for detailed descriptions. Leave this parameter blank to retrieve limits for all types.</p>"]
    #[serde(rename="EC2InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec2_instance_type: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEC2InstanceLimitsOutput {
    #[doc="<p>Object that contains the maximum number of instances for the specified instance type.</p>"]
    #[serde(rename="EC2InstanceLimits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec2_instance_limits: Option<Vec<EC2InstanceLimit>>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeFleetAttributesInput {
    #[doc="<p>Unique identifier for a fleet(s) to retrieve attributes for. To request attributes for all fleets, leave this parameter empty.</p>"]
    #[serde(rename="FleetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_ids: Option<Vec<String>>,
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeFleetAttributesOutput {
    #[doc="<p>Collection of objects containing attribute metadata for each requested fleet ID.</p>"]
    #[serde(rename="FleetAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_attributes: Option<Vec<FleetAttributes>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeFleetCapacityInput {
    #[doc="<p>Unique identifier for a fleet(s) to retrieve capacity information for. To request capacity information for all fleets, leave this parameter empty.</p>"]
    #[serde(rename="FleetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_ids: Option<Vec<String>>,
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeFleetCapacityOutput {
    #[doc="<p>Collection of objects containing capacity information for each requested fleet ID. Leave this parameter empty to retrieve capacity information for all fleets.</p>"]
    #[serde(rename="FleetCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_capacity: Option<Vec<FleetCapacity>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeFleetEventsInput {
    #[doc="<p>Most recent date to retrieve event logs for. If no end time is specified, this call returns entries from the specified start time up to the present. Format is a number expressed in Unix time as milliseconds (ex: \"1469498468.057\").</p>"]
    #[serde(rename="EndTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_time: Option<f64>,
    #[doc="<p>Unique identifier for a fleet to get event logs for.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Earliest date to retrieve event logs for. If no start time is specified, this call returns entries starting from when the fleet was created to the specified end time. Format is a number expressed in Unix time as milliseconds (ex: \"1469498468.057\").</p>"]
    #[serde(rename="StartTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_time: Option<f64>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeFleetEventsOutput {
    #[doc="<p>Collection of objects containing event log entries for the specified fleet.</p>"]
    #[serde(rename="Events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeFleetPortSettingsInput {
    #[doc="<p>Unique identifier for a fleet to retrieve port settings for.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeFleetPortSettingsOutput {
    #[doc="<p>Object that contains port settings for the requested fleet ID.</p>"]
    #[serde(rename="InboundPermissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inbound_permissions: Option<Vec<IpPermission>>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeFleetUtilizationInput {
    #[doc="<p>Unique identifier for a fleet(s) to retrieve utilization data for. To request utilization data for all fleets, leave this parameter empty.</p>"]
    #[serde(rename="FleetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_ids: Option<Vec<String>>,
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeFleetUtilizationOutput {
    #[doc="<p>Collection of objects containing utilization information for each requested fleet ID.</p>"]
    #[serde(rename="FleetUtilization")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_utilization: Option<Vec<FleetUtilization>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeGameSessionDetailsInput {
    #[doc="<p>Unique identifier for an alias associated with the fleet to retrieve all game sessions for.</p>"]
    #[serde(rename="AliasId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias_id: Option<String>,
    #[doc="<p>Unique identifier for a fleet to retrieve all game sessions active on the fleet.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Unique identifier for the game session to retrieve.</p>"]
    #[serde(rename="GameSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_id: Option<String>,
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Game session status to filter results on. Possible game session statuses include <code>ACTIVE</code>, <code>TERMINATED</code>, <code>ACTIVATING</code> and <code>TERMINATING</code> (the last two are transitory). </p>"]
    #[serde(rename="StatusFilter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_filter: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeGameSessionDetailsOutput {
    #[doc="<p>Collection of objects containing game session properties and the protection policy currently in force for each session matching the request.</p>"]
    #[serde(rename="GameSessionDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_details: Option<Vec<GameSessionDetail>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeGameSessionPlacementInput {
    #[doc="<p>Unique identifier for a game session placement to retrieve.</p>"]
    #[serde(rename="PlacementId")]
    pub placement_id: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeGameSessionPlacementOutput {
    #[doc="<p>Object that describes the requested game session placement.</p>"]
    #[serde(rename="GameSessionPlacement")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_placement: Option<GameSessionPlacement>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeGameSessionQueuesInput {
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>List of queue names to retrieve information for. To request settings for all queues, leave this parameter empty.</p>"]
    #[serde(rename="Names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<Vec<String>>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeGameSessionQueuesOutput {
    #[doc="<p>Collection of objects that describes the requested game session queues.</p>"]
    #[serde(rename="GameSessionQueues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_queues: Option<Vec<GameSessionQueue>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeGameSessionsInput {
    #[doc="<p>Unique identifier for an alias associated with the fleet to retrieve all game sessions for. </p>"]
    #[serde(rename="AliasId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias_id: Option<String>,
    #[doc="<p>Unique identifier for a fleet to retrieve all game sessions for.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Unique identifier for the game session to retrieve. You can use either a <code>GameSessionId</code> or <code>GameSessionArn</code> value. </p>"]
    #[serde(rename="GameSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_id: Option<String>,
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Game session status to filter results on. Possible game session statuses include <code>ACTIVE</code>, <code>TERMINATED</code>, <code>ACTIVATING</code>, and <code>TERMINATING</code> (the last two are transitory). </p>"]
    #[serde(rename="StatusFilter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_filter: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeGameSessionsOutput {
    #[doc="<p>Collection of objects containing game session properties for each session matching the request.</p>"]
    #[serde(rename="GameSessions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_sessions: Option<Vec<GameSession>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeInstancesInput {
    #[doc="<p>Unique identifier for a fleet to retrieve instance information for.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
    #[doc="<p>Unique identifier for an instance to retrieve. Specify an instance ID or leave blank to retrieve all instances in the fleet.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeInstancesOutput {
    #[doc="<p>Collection of objects containing properties for each instance returned.</p>"]
    #[serde(rename="Instances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instances: Option<Vec<Instance>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeMatchmakingConfigurationsInput {
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. This parameter is limited to 10.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Unique identifier for a matchmaking configuration(s) to retrieve. To request all existing configurations, leave this parameter empty.</p>"]
    #[serde(rename="Names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<Vec<String>>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Unique identifier for a matchmaking rule set. Use this parameter to retrieve all matchmaking configurations that use this rule set.</p>"]
    #[serde(rename="RuleSetName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_set_name: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeMatchmakingConfigurationsOutput {
    #[doc="<p>Collection of requested matchmaking configuration objects.</p>"]
    #[serde(rename="Configurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configurations: Option<Vec<MatchmakingConfiguration>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeMatchmakingInput {
    #[doc="<p>Unique identifier for a matchmaking ticket. To request all existing tickets, leave this parameter empty.</p>"]
    #[serde(rename="TicketIds")]
    pub ticket_ids: Vec<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeMatchmakingOutput {
    #[doc="<p>Collection of existing matchmaking ticket objects matching the request.</p>"]
    #[serde(rename="TicketList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ticket_list: Option<Vec<MatchmakingTicket>>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeMatchmakingRuleSetsInput {
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Unique identifier for a matchmaking rule set. This name is used to identify the rule set associated with a matchmaking configuration.</p>"]
    #[serde(rename="Names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<Vec<String>>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeMatchmakingRuleSetsOutput {
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Collection of requested matchmaking rule set objects. </p>"]
    #[serde(rename="RuleSets")]
    pub rule_sets: Vec<MatchmakingRuleSet>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribePlayerSessionsInput {
    #[doc="<p>Unique identifier for the game session to retrieve player sessions for.</p>"]
    #[serde(rename="GameSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_id: Option<String>,
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. If a player session ID is specified, this parameter is ignored.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value. If a player session ID is specified, this parameter is ignored.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Unique identifier for a player to retrieve player sessions for.</p>"]
    #[serde(rename="PlayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_id: Option<String>,
    #[doc="<p>Unique identifier for a player session to retrieve.</p>"]
    #[serde(rename="PlayerSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_session_id: Option<String>,
    #[doc="<p>Player session status to filter results on.</p> <p>Possible player session statuses include the following:</p> <ul> <li> <p> <b>RESERVED</b> – The player session request has been received, but the player has not yet connected to the server process and/or been validated. </p> </li> <li> <p> <b>ACTIVE</b> – The player has been validated by the server process and is currently connected.</p> </li> <li> <p> <b>COMPLETED</b> – The player connection has been dropped.</p> </li> <li> <p> <b>TIMEDOUT</b> – A player session request was received, but the player did not connect and/or was not validated within the time-out limit (60 seconds).</p> </li> </ul>"]
    #[serde(rename="PlayerSessionStatusFilter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_session_status_filter: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribePlayerSessionsOutput {
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Collection of objects containing properties for each player session that matches the request.</p>"]
    #[serde(rename="PlayerSessions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_sessions: Option<Vec<PlayerSession>>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeRuntimeConfigurationInput {
    #[doc="<p>Unique identifier for a fleet to get the run-time configuration for.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeRuntimeConfigurationOutput {
    #[doc="<p>Instructions describing how server processes should be launched and maintained on each instance in the fleet.</p>"]
    #[serde(rename="RuntimeConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runtime_configuration: Option<RuntimeConfiguration>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeScalingPoliciesInput {
    #[doc="<p>Unique identifier for a fleet to retrieve scaling policies for.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Scaling policy status to filter results on. A scaling policy is only in force when in an <code>ACTIVE</code> status.</p> <ul> <li> <p> <b>ACTIVE</b> – The scaling policy is currently in force.</p> </li> <li> <p> <b>UPDATEREQUESTED</b> – A request to update the scaling policy has been received.</p> </li> <li> <p> <b>UPDATING</b> – A change is being made to the scaling policy.</p> </li> <li> <p> <b>DELETEREQUESTED</b> – A request to delete the scaling policy has been received.</p> </li> <li> <p> <b>DELETING</b> – The scaling policy is being deleted.</p> </li> <li> <p> <b>DELETED</b> – The scaling policy has been deleted.</p> </li> <li> <p> <b>ERROR</b> – An error occurred in creating the policy. It should be removed and recreated.</p> </li> </ul>"]
    #[serde(rename="StatusFilter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_filter: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeScalingPoliciesOutput {
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Collection of objects containing the scaling policies matching the request.</p>"]
    #[serde(rename="ScalingPolicies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scaling_policies: Option<Vec<ScalingPolicy>>,
}

#[doc="<p>Player information for use when creating player sessions using a game session placement request with <a>StartGameSessionPlacement</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DesiredPlayerSession {
    #[doc="<p>Developer-defined information related to a player. Amazon GameLift does not use this data, so it can be formatted as needed for use in the game.</p>"]
    #[serde(rename="PlayerData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_data: Option<String>,
    #[doc="<p>Unique identifier for a player to associate with the player session.</p>"]
    #[serde(rename="PlayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_id: Option<String>,
}

#[doc="<p>Current status of fleet capacity. The number of active instances should match or be in the process of matching the number of desired instances. Pending and terminating counts are non-zero only if fleet capacity is adjusting to an <a>UpdateFleetCapacity</a> request, or if access to resources is temporarily affected.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EC2InstanceCounts {
    #[doc="<p>Actual number of active instances in the fleet.</p>"]
    #[serde(rename="ACTIVE")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<i64>,
    #[doc="<p>Ideal number of active instances in the fleet.</p>"]
    #[serde(rename="DESIRED")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired: Option<i64>,
    #[doc="<p>Number of active instances in the fleet that are not currently hosting a game session.</p>"]
    #[serde(rename="IDLE")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub idle: Option<i64>,
    #[doc="<p>Maximum value allowed for the fleet's instance count.</p>"]
    #[serde(rename="MAXIMUM")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum: Option<i64>,
    #[doc="<p>Minimum value allowed for the fleet's instance count.</p>"]
    #[serde(rename="MINIMUM")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minimum: Option<i64>,
    #[doc="<p>Number of instances in the fleet that are starting but not yet active.</p>"]
    #[serde(rename="PENDING")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pending: Option<i64>,
    #[doc="<p>Number of instances in the fleet that are no longer active but haven't yet been terminated.</p>"]
    #[serde(rename="TERMINATING")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub terminating: Option<i64>,
}

#[doc="<p>Maximum number of instances allowed based on the Amazon Elastic Compute Cloud (Amazon EC2) instance type. Instance limits can be retrieved by calling <a>DescribeEC2InstanceLimits</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EC2InstanceLimit {
    #[doc="<p>Number of instances of the specified type that are currently in use by this AWS account.</p>"]
    #[serde(rename="CurrentInstances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_instances: Option<i64>,
    #[doc="<p>Name of an EC2 instance type that is supported in Amazon GameLift. A fleet instance type determines the computing resources of each instance in the fleet, including CPU, memory, storage, and networking capacity. Amazon GameLift supports the following EC2 instance types. See <a href=\"http://aws.amazon.com/ec2/instance-types/\">Amazon EC2 Instance Types</a> for detailed descriptions.</p>"]
    #[serde(rename="EC2InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec2_instance_type: Option<String>,
    #[doc="<p>Number of instances allowed.</p>"]
    #[serde(rename="InstanceLimit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_limit: Option<i64>,
}

#[doc="<p>Log entry describing an event that involves Amazon GameLift resources (such as a fleet). In addition to tracking activity, event codes and messages can provide additional information for troubleshooting and debugging problems.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Event {
    #[doc="<p>Type of event being logged. The following events are currently in use:</p> <ul> <li> <p>General events:</p> <ul> <li> <p> <b>GENERIC_EVENT</b> – An unspecified event has occurred.</p> </li> </ul> </li> <li> <p>Fleet creation events:</p> <ul> <li> <p> <b>FLEET_CREATED</b> – A fleet record was successfully created with a status of <code>NEW</code>. Event messaging includes the fleet ID.</p> </li> <li> <p> <b>FLEET_STATE_DOWNLOADING</b> – Fleet status changed from <code>NEW</code> to <code>DOWNLOADING</code>. The compressed build has started downloading to a fleet instance for installation.</p> </li> <li> <p> <b>FLEET_BINARY_DOWNLOAD_FAILED</b> – The build failed to download to the fleet instance.</p> </li> <li> <p> <b>FLEET_CREATION_EXTRACTING_BUILD</b> – The game server build was successfully downloaded to an instance, and the build files are now being extracted from the uploaded build and saved to an instance. Failure at this stage prevents a fleet from moving to <code>ACTIVE</code> status. Logs for this stage display a list of the files that are extracted and saved on the instance. Access the logs by using the URL in <i>PreSignedLogUrl</i>.</p> </li> <li> <p> <b>FLEET_CREATION_RUNNING_INSTALLER</b> – The game server build files were successfully extracted, and the Amazon GameLift is now running the build's install script (if one is included). Failure in this stage prevents a fleet from moving to <code>ACTIVE</code> status. Logs for this stage list the installation steps and whether or not the install completed successfully. Access the logs by using the URL in <i>PreSignedLogUrl</i>. </p> </li> <li> <p> <b>FLEET_CREATION_VALIDATING_RUNTIME_CONFIG</b> – The build process was successful, and the Amazon GameLift is now verifying that the game server launch paths, which are specified in the fleet's run-time configuration, exist. If any listed launch path exists, Amazon GameLift tries to launch a game server process and waits for the process to report ready. Failures in this stage prevent a fleet from moving to <code>ACTIVE</code> status. Logs for this stage list the launch paths in the run-time configuration and indicate whether each is found. Access the logs by using the URL in <i>PreSignedLogUrl</i>. </p> </li> <li> <p> <b>FLEET_STATE_VALIDATING</b> – Fleet status changed from <code>DOWNLOADING</code> to <code>VALIDATING</code>.</p> </li> <li> <p> <b>FLEET_VALIDATION_LAUNCH_PATH_NOT_FOUND</b> – Validation of the run-time configuration failed because the executable specified in a launch path does not exist on the instance.</p> </li> <li> <p> <b>FLEET_STATE_BUILDING</b> – Fleet status changed from <code>VALIDATING</code> to <code>BUILDING</code>.</p> </li> <li> <p> <b>FLEET_VALIDATION_EXECUTABLE_RUNTIME_FAILURE</b> – Validation of the run-time configuration failed because the executable specified in a launch path failed to run on the fleet instance.</p> </li> <li> <p> <b>FLEET_STATE_ACTIVATING</b> – Fleet status changed from <code>BUILDING</code> to <code>ACTIVATING</code>. </p> </li> <li> <p> <b>FLEET_ACTIVATION_FAILED</b> - The fleet failed to successfully complete one of the steps in the fleet activation process. This event code indicates that the game build was successfully downloaded to a fleet instance, built, and validated, but was not able to start a server process. A possible reason for failure is that the game server is not reporting \"process ready\" to the Amazon GameLift service.</p> </li> <li> <p> <b>FLEET_STATE_ACTIVE</b> – The fleet's status changed from <code>ACTIVATING</code> to <code>ACTIVE</code>. The fleet is now ready to host game sessions.</p> </li> </ul> </li> <li> <p>Other fleet events:</p> <ul> <li> <p> <b>FLEET_SCALING_EVENT</b> – A change was made to the fleet's capacity settings (desired instances, minimum/maximum scaling limits). Event messaging includes the new capacity settings.</p> </li> <li> <p> <b>FLEET_NEW_GAME_SESSION_PROTECTION_POLICY_UPDATED</b> – A change was made to the fleet's game session protection policy setting. Event messaging includes both the old and new policy setting. </p> </li> <li> <p> <b>FLEET_DELETED</b> – A request to delete a fleet was initiated.</p> </li> </ul> </li> </ul>"]
    #[serde(rename="EventCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_code: Option<String>,
    #[doc="<p>Unique identifier for a fleet event.</p>"]
    #[serde(rename="EventId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_id: Option<String>,
    #[doc="<p>Time stamp indicating when this event occurred. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="EventTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_time: Option<f64>,
    #[doc="<p>Additional information related to the event.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[doc="<p>Location of stored logs with additional detail that is related to the event. This is useful for debugging issues. The URL is valid for 15 minutes. You can also access fleet creation logs through the Amazon GameLift console.</p>"]
    #[serde(rename="PreSignedLogUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pre_signed_log_url: Option<String>,
    #[doc="<p>Unique identifier for an event resource, such as a fleet ID.</p>"]
    #[serde(rename="ResourceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_id: Option<String>,
}

#[doc="<p>General properties describing a fleet.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct FleetAttributes {
    #[doc="<p>Unique identifier for a build.</p>"]
    #[serde(rename="BuildId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build_id: Option<String>,
    #[doc="<p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>Human-readable description of the fleet.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Identifier for a fleet that is unique across all regions.</p>"]
    #[serde(rename="FleetArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_arn: Option<String>,
    #[doc="<p>Unique identifier for a fleet.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Location of default log files. When a server process is shut down, Amazon GameLift captures and stores any log files in this location. These logs are in addition to game session logs; see more on game session logs in the <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-api-server-code\">Amazon GameLift Developer Guide</a>. If no default log path for a fleet is specified, Amazon GameLift automatically uploads logs that are stored on each instance at <code>C:\\game\\logs</code> (for Windows) or <code>/local/game/logs</code> (for Linux). Use the Amazon GameLift console to access stored logs. </p>"]
    #[serde(rename="LogPaths")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_paths: Option<Vec<String>>,
    #[doc="<p>Names of metric groups that this fleet is included in. In Amazon CloudWatch, you can view metrics for an individual fleet or aggregated metrics for fleets that are in a fleet metric group. A fleet can be included in only one metric group at a time.</p>"]
    #[serde(rename="MetricGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric_groups: Option<Vec<String>>,
    #[doc="<p>Descriptive label that is associated with a fleet. Fleet names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Type of game session protection to set for all new instances started in the fleet.</p> <ul> <li> <p> <b>NoProtection</b> – The game session can be terminated during a scale-down event.</p> </li> <li> <p> <b>FullProtection</b> – If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li> </ul>"]
    #[serde(rename="NewGameSessionProtectionPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_game_session_protection_policy: Option<String>,
    #[doc="<p>Operating system of the fleet's computing resources. A fleet's operating system depends on the OS specified for the build that is deployed on this fleet.</p>"]
    #[serde(rename="OperatingSystem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operating_system: Option<String>,
    #[doc="<p>Fleet policy to limit the number of game sessions an individual player can create over a span of time.</p>"]
    #[serde(rename="ResourceCreationLimitPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_creation_limit_policy: Option<ResourceCreationLimitPolicy>,
    #[doc="<p>Game server launch parameters specified for fleets created before 2016-08-04 (or AWS SDK v. 0.12.16). Server launch parameters for fleets created after this date are specified in the fleet's <a>RuntimeConfiguration</a>.</p>"]
    #[serde(rename="ServerLaunchParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_launch_parameters: Option<String>,
    #[doc="<p>Path to a game server executable in the fleet's build, specified for fleets created before 2016-08-04 (or AWS SDK v. 0.12.16). Server launch paths for fleets created after this date are specified in the fleet's <a>RuntimeConfiguration</a>.</p>"]
    #[serde(rename="ServerLaunchPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_launch_path: Option<String>,
    #[doc="<p>Current status of the fleet.</p> <p>Possible fleet statuses include the following:</p> <ul> <li> <p> <b>NEW</b> – A new fleet has been defined and desired instances is set to 1. </p> </li> <li> <p> <b>DOWNLOADING/VALIDATING/BUILDING/ACTIVATING</b> – Amazon GameLift is setting up the new fleet, creating new instances with the game build and starting server processes.</p> </li> <li> <p> <b>ACTIVE</b> – Hosts can now accept game sessions.</p> </li> <li> <p> <b>ERROR</b> – An error occurred when downloading, validating, building, or activating the fleet.</p> </li> <li> <p> <b>DELETING</b> – Hosts are responding to a delete fleet request.</p> </li> <li> <p> <b>TERMINATED</b> – The fleet no longer exists.</p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>Time stamp indicating when this data object was terminated. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="TerminationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub termination_time: Option<f64>,
}

#[doc="<p>Information about the fleet's capacity. Fleet capacity is measured in EC2 instances. By default, new fleets have a capacity of one instance, but can be updated as needed. The maximum number of instances for a fleet is determined by the fleet's instance type.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct FleetCapacity {
    #[doc="<p>Unique identifier for a fleet.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Current status of fleet capacity.</p>"]
    #[serde(rename="InstanceCounts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_counts: Option<EC2InstanceCounts>,
    #[doc="<p>Name of an EC2 instance type that is supported in Amazon GameLift. A fleet instance type determines the computing resources of each instance in the fleet, including CPU, memory, storage, and networking capacity. Amazon GameLift supports the following EC2 instance types. See <a href=\"http://aws.amazon.com/ec2/instance-types/\">Amazon EC2 Instance Types</a> for detailed descriptions.</p>"]
    #[serde(rename="InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type: Option<String>,
}

#[doc="<p>Current status of fleet utilization, including the number of game and player sessions being hosted.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct FleetUtilization {
    #[doc="<p>Number of active game sessions currently being hosted on all instances in the fleet.</p>"]
    #[serde(rename="ActiveGameSessionCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_game_session_count: Option<i64>,
    #[doc="<p>Number of server processes in an <code>ACTIVE</code> status currently running across all instances in the fleet</p>"]
    #[serde(rename="ActiveServerProcessCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_server_process_count: Option<i64>,
    #[doc="<p>Number of active player sessions currently being hosted on all instances in the fleet.</p>"]
    #[serde(rename="CurrentPlayerSessionCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_player_session_count: Option<i64>,
    #[doc="<p>Unique identifier for a fleet.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Maximum players allowed across all game sessions currently being hosted on all instances in the fleet.</p>"]
    #[serde(rename="MaximumPlayerSessionCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_player_session_count: Option<i64>,
}

#[doc="<p>Set of key-value pairs that contain information about a game session. When included in a game session request, these properties communicate details to be used when setting up the new game session, such as to specify a game mode, level, or map. Game properties are passed to the game server process when initiating a new game session; the server process uses the properties as appropriate. For more information, see the <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-client-api.html#gamelift-sdk-client-api-create\"> Amazon GameLift Developer Guide</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct GameProperty {
    #[doc="<p>Game property identifier.</p>"]
    #[serde(rename="Key")]
    pub key: String,
    #[doc="<p>Game property value.</p>"]
    #[serde(rename="Value")]
    pub value: String,
}

#[doc="<p>Properties describing a game session.</p> <p>A game session in ACTIVE status can host players. When a game session ends, its status is set to <code>TERMINATED</code>. </p> <p>Once the session ends, the game session object is retained for 30 days. This means you can reuse idempotency token values after this time. Game session logs are retained for 14 days.</p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GameSession {
    #[doc="<p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>Unique identifier for a player. This ID is used to enforce a resource protection policy (if one exists), that limits the number of game sessions a player can create.</p>"]
    #[serde(rename="CreatorId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creator_id: Option<String>,
    #[doc="<p>Number of players currently in the game session.</p>"]
    #[serde(rename="CurrentPlayerSessionCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_player_session_count: Option<i64>,
    #[doc="<p>Unique identifier for a fleet the game session is running on.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Set of developer-defined properties for a game session, formatted as a set of type:value pairs. These properties are included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>).</p>"]
    #[serde(rename="GameProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    #[doc="<p>Set of developer-defined game session properties, formatted as a single string value. This data is included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>).</p>"]
    #[serde(rename="GameSessionData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_data: Option<String>,
    #[doc="<p>Unique identifier for the game session. A game session ID has the following format: <code>arn:aws:gamelift:&lt;region&gt;::gamesession/&lt;fleet ID&gt;/&lt;custom ID string or idempotency token&gt;</code>.</p>"]
    #[serde(rename="GameSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_id: Option<String>,
    #[doc="<p>IP address of the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number.</p>"]
    #[serde(rename="IpAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_address: Option<String>,
    #[doc="<p>Maximum number of players that can be connected simultaneously to the game session.</p>"]
    #[serde(rename="MaximumPlayerSessionCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_player_session_count: Option<i64>,
    #[doc="<p>Descriptive label that is associated with a game session. Session names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Indicates whether or not the game session is accepting new players.</p>"]
    #[serde(rename="PlayerSessionCreationPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_session_creation_policy: Option<String>,
    #[doc="<p>Port number for the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number.</p>"]
    #[serde(rename="Port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<i64>,
    #[doc="<p>Current status of the game session. A game session must have an <code>ACTIVE</code> status to have player sessions.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>Time stamp indicating when this data object was terminated. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="TerminationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub termination_time: Option<f64>,
}

#[doc="<p>Connection information for the new game session that is created with matchmaking. (with <a>StartMatchmaking</a>). Once a match is set, the FlexMatch engine places the match and creates a new game session for it. This information, including the game session endpoint and player sessions for each player in the original matchmaking request, is added to the <a>MatchmakingTicket</a>, which can be retrieved by calling <a>DescribeMatchmaking</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GameSessionConnectionInfo {
    #[doc="<p>Amazon Resource Name (<a href=\"http://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html\">ARN</a>) that is assigned to a game session and uniquely identifies it.</p>"]
    #[serde(rename="GameSessionArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_arn: Option<String>,
    #[doc="<p>IP address of the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number.</p>"]
    #[serde(rename="IpAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_address: Option<String>,
    #[doc="<p>Collection of player session IDs, one for each player ID that was included in the original matchmaking request. </p>"]
    #[serde(rename="MatchedPlayerSessions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub matched_player_sessions: Option<Vec<MatchedPlayerSession>>,
    #[doc="<p>Port number for the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number.</p>"]
    #[serde(rename="Port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<i64>,
}

#[doc="<p>A game session's properties plus the protection policy currently in force.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GameSessionDetail {
    #[doc="<p>Object that describes a game session.</p>"]
    #[serde(rename="GameSession")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session: Option<GameSession>,
    #[doc="<p>Current status of protection for the game session.</p> <ul> <li> <p> <b>NoProtection</b> – The game session can be terminated during a scale-down event.</p> </li> <li> <p> <b>FullProtection</b> – If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li> </ul>"]
    #[serde(rename="ProtectionPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection_policy: Option<String>,
}

#[doc="<p>Object that describes a <a>StartGameSessionPlacement</a> request. This object includes the full details of the original request plus the current status and start/end time stamps.</p> <p>Game session placement-related operations include:</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GameSessionPlacement {
    #[doc="<p>Time stamp indicating when this request was completed, canceled, or timed out.</p>"]
    #[serde(rename="EndTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_time: Option<f64>,
    #[doc="<p>Set of developer-defined properties for a game session, formatted as a set of type:value pairs. These properties are included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>).</p>"]
    #[serde(rename="GameProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    #[doc="<p>Identifier for the game session created by this placement request. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). This identifier is unique across all regions. You can use this value as a <code>GameSessionId</code> value as needed.</p>"]
    #[serde(rename="GameSessionArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_arn: Option<String>,
    #[doc="<p>Set of developer-defined game session properties, formatted as a single string value. This data is included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>).</p>"]
    #[serde(rename="GameSessionData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_data: Option<String>,
    #[doc="<p>Unique identifier for the game session. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>"]
    #[serde(rename="GameSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_id: Option<String>,
    #[doc="<p>Descriptive label that is associated with a game session. Session names do not need to be unique.</p>"]
    #[serde(rename="GameSessionName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_name: Option<String>,
    #[doc="<p>Descriptive label that is associated with game session queue. Queue names must be unique within each region.</p>"]
    #[serde(rename="GameSessionQueueName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_queue_name: Option<String>,
    #[doc="<p>Name of the region where the game session created by this placement request is running. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>"]
    #[serde(rename="GameSessionRegion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_region: Option<String>,
    #[doc="<p>IP address of the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). </p>"]
    #[serde(rename="IpAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_address: Option<String>,
    #[doc="<p>Maximum number of players that can be connected simultaneously to the game session.</p>"]
    #[serde(rename="MaximumPlayerSessionCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_player_session_count: Option<i64>,
    #[doc="<p>Collection of information on player sessions created in response to the game session placement request. These player sessions are created only once a new game session is successfully placed (placement status is <code>FULFILLED</code>). This information includes the player ID (as provided in the placement request) and the corresponding player session ID. Retrieve full player sessions by calling <a>DescribePlayerSessions</a> with the player session ID.</p>"]
    #[serde(rename="PlacedPlayerSessions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placed_player_sessions: Option<Vec<PlacedPlayerSession>>,
    #[doc="<p>Unique identifier for a game session placement.</p>"]
    #[serde(rename="PlacementId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placement_id: Option<String>,
    #[doc="<p>Set of values, expressed in milliseconds, indicating the amount of latency that a player experiences when connected to AWS regions.</p>"]
    #[serde(rename="PlayerLatencies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_latencies: Option<Vec<PlayerLatency>>,
    #[doc="<p>Port number for the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>"]
    #[serde(rename="Port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<i64>,
    #[doc="<p>Time stamp indicating when this request was placed in the queue. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="StartTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_time: Option<f64>,
    #[doc="<p>Current status of the game session placement request.</p> <ul> <li> <p> <b>PENDING</b> – The placement request is currently in the queue waiting to be processed.</p> </li> <li> <p> <b>FULFILLED</b> – A new game session and player sessions (if requested) have been successfully created. Values for <i>GameSessionArn</i> and <i>GameSessionRegion</i> are available. </p> </li> <li> <p> <b>CANCELLED</b> – The placement request was canceled with a call to <a>StopGameSessionPlacement</a>.</p> </li> <li> <p> <b>TIMED_OUT</b> – A new game session was not successfully created before the time limit expired. You can resubmit the placement request as needed.</p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[doc="<p>Configuration of a queue that is used to process game session placement requests. The queue configuration identifies several game features:</p> <ul> <li> <p>The destinations where a new game session can potentially be hosted. Amazon GameLift tries these destinations in an order based on either the queue's default order or player latency information, if provided in a placement request. With latency information, Amazon GameLift can place game sessions where the majority of players are reporting the lowest possible latency. </p> </li> <li> <p>The length of time that placement requests can wait in the queue before timing out. </p> </li> <li> <p>A set of optional latency policies that protect individual players from high latencies, preventing game sessions from being placed where any individual player is reporting latency higher than a policy's maximum.</p> </li> </ul> <p>Queue-related operations include:</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GameSessionQueue {
    #[doc="<p>List of fleets that can be used to fulfill game session placement requests in the queue. Fleets are identified by either a fleet ARN or a fleet alias ARN. Destinations are listed in default preference order.</p>"]
    #[serde(rename="Destinations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub destinations: Option<Vec<GameSessionQueueDestination>>,
    #[doc="<p>Amazon Resource Name (<a href=\"http://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html\">ARN</a>) that is assigned to a game session queue and uniquely identifies it. Format is <code>arn:aws:gamelift:&lt;region&gt;::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912</code>.</p>"]
    #[serde(rename="GameSessionQueueArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_queue_arn: Option<String>,
    #[doc="<p>Descriptive label that is associated with game session queue. Queue names must be unique within each region.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Collection of latency policies to apply when processing game sessions placement requests with player latency information. Multiple policies are evaluated in order of the maximum latency value, starting with the lowest latency values. With just one policy, it is enforced at the start of the game session placement for the duration period. With multiple policies, each policy is enforced consecutively for its duration period. For example, a queue might enforce a 60-second policy followed by a 120-second policy, and then no policy for the remainder of the placement. </p>"]
    #[serde(rename="PlayerLatencyPolicies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_latency_policies: Option<Vec<PlayerLatencyPolicy>>,
    #[doc="<p>Maximum time, in seconds, that a new game session placement request remains in the queue. When a request exceeds this time, the game session placement changes to a <code>TIMED_OUT</code> status.</p>"]
    #[serde(rename="TimeoutInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

#[doc="<p>Fleet designated in a game session queue. Requests for new game sessions in the queue are fulfilled by starting a new game session on any destination configured for a queue. </p> <p>Queue-related operations include:</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct GameSessionQueueDestination {
    #[doc="<p>Amazon Resource Name (ARN) assigned to fleet or fleet alias. ARNs, which include a fleet ID or alias ID and a region name, provide a unique identifier across all regions. </p>"]
    #[serde(rename="DestinationArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub destination_arn: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetGameSessionLogUrlInput {
    #[doc="<p>Unique identifier for the game session to get logs for.</p>"]
    #[serde(rename="GameSessionId")]
    pub game_session_id: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetGameSessionLogUrlOutput {
    #[doc="<p>Location of the requested game session logs, available for download.</p>"]
    #[serde(rename="PreSignedUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pre_signed_url: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetInstanceAccessInput {
    #[doc="<p>Unique identifier for a fleet that contains the instance you want access to. The fleet can be in any of the following statuses: <code>ACTIVATING</code>, <code>ACTIVE</code>, or <code>ERROR</code>. Fleets with an <code>ERROR</code> status may be accessible for a short time before they are deleted.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
    #[doc="<p>Unique identifier for an instance you want to get access to. You can access an instance in any status.</p>"]
    #[serde(rename="InstanceId")]
    pub instance_id: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetInstanceAccessOutput {
    #[doc="<p>Object that contains connection information for a fleet instance, including IP address and access credentials.</p>"]
    #[serde(rename="InstanceAccess")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_access: Option<InstanceAccess>,
}

#[doc="<p>Properties that describe an instance of a virtual computing resource that hosts one or more game servers. A fleet may contain zero or more instances.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Instance {
    #[doc="<p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>Unique identifier for a fleet that the instance is in.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Unique identifier for an instance.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>IP address assigned to the instance.</p>"]
    #[serde(rename="IpAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_address: Option<String>,
    #[doc="<p>Operating system that is running on this instance. </p>"]
    #[serde(rename="OperatingSystem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operating_system: Option<String>,
    #[doc="<p>Current status of the instance. Possible statuses include the following:</p> <ul> <li> <p> <b>PENDING</b> – The instance is in the process of being created and launching server processes as defined in the fleet's run-time configuration. </p> </li> <li> <p> <b>ACTIVE</b> – The instance has been successfully created and at least one server process has successfully launched and reported back to Amazon GameLift that it is ready to host a game session. The instance is now considered ready to host game sessions. </p> </li> <li> <p> <b>TERMINATING</b> – The instance is in the process of shutting down. This may happen to reduce capacity during a scaling down event or to recycle resources in the event of a problem.</p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>EC2 instance type that defines the computing resources of this instance. </p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[doc="<p>Information required to remotely connect to a fleet instance. Access is requested by calling <a>GetInstanceAccess</a>. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceAccess {
    #[doc="<p>Credentials required to access the instance.</p>"]
    #[serde(rename="Credentials")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub credentials: Option<InstanceCredentials>,
    #[doc="<p>Unique identifier for a fleet containing the instance being accessed.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Unique identifier for an instance being accessed.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>IP address assigned to the instance.</p>"]
    #[serde(rename="IpAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_address: Option<String>,
    #[doc="<p>Operating system that is running on the instance.</p>"]
    #[serde(rename="OperatingSystem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operating_system: Option<String>,
}

#[doc="<p>Set of credentials required to remotely access a fleet instance. Access credentials are requested by calling <a>GetInstanceAccess</a> and returned in an <a>InstanceAccess</a> object.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceCredentials {
    #[doc="<p>Secret string. For Windows instances, the secret is a password for use with Windows Remote Desktop. For Linux instances, it is a private key (which must be saved as a <code>.pem</code> file) for use with SSH.</p>"]
    #[serde(rename="Secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<String>,
    #[doc="<p>User login string.</p>"]
    #[serde(rename="UserName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,
}

#[doc="<p>A range of IP addresses and port settings that allow inbound traffic to connect to server processes on Amazon GameLift. Each game session hosted on a fleet is assigned a unique combination of IP address and port number, which must fall into the fleet's allowed ranges. This combination is included in the <a>GameSession</a> object. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct IpPermission {
    #[doc="<p>Starting value for a range of allowed port numbers.</p>"]
    #[serde(rename="FromPort")]
    pub from_port: i64,
    #[doc="<p>Range of allowed IP addresses. This value must be expressed in CIDR notation. Example: \"<code>000.000.000.000/[subnet mask]</code>\" or optionally the shortened version \"<code>0.0.0.0/[subnet mask]</code>\".</p>"]
    #[serde(rename="IpRange")]
    pub ip_range: String,
    #[doc="<p>Network communication protocol used by the fleet.</p>"]
    #[serde(rename="Protocol")]
    pub protocol: String,
    #[doc="<p>Ending value for a range of allowed port numbers. Port numbers are end-inclusive. This value must be higher than <code>FromPort</code>.</p>"]
    #[serde(rename="ToPort")]
    pub to_port: i64,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListAliasesInput {
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Descriptive label that is associated with an alias. Alias names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Type of routing to filter results on. Use this parameter to retrieve only aliases of a certain type. To retrieve all aliases, leave this parameter empty.</p> <p>Possible routing types include the following:</p> <ul> <li> <p> <b>SIMPLE</b> – The alias resolves to one specific fleet. Use this type when routing to active fleets.</p> </li> <li> <p> <b>TERMINAL</b> – The alias does not resolve to a fleet but instead can be used to display a message to the user. A terminal alias throws a TerminalRoutingStrategyException with the <a>RoutingStrategy</a> message embedded.</p> </li> </ul>"]
    #[serde(rename="RoutingStrategyType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub routing_strategy_type: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListAliasesOutput {
    #[doc="<p>Collection of alias records that match the list request.</p>"]
    #[serde(rename="Aliases")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListBuildsInput {
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Build status to filter results by. To retrieve all builds, leave this parameter empty.</p> <p>Possible build statuses include the following:</p> <ul> <li> <p> <b>INITIALIZED</b> – A new build has been defined, but no files have been uploaded. You cannot create fleets for builds that are in this status. When a build is successfully created, the build status is set to this value. </p> </li> <li> <p> <b>READY</b> – The game build has been successfully uploaded. You can now create new fleets for this build.</p> </li> <li> <p> <b>FAILED</b> – The game build upload failed. You cannot create new fleets for this build. </p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListBuildsOutput {
    #[doc="<p>Collection of build records that match the request.</p>"]
    #[serde(rename="Builds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub builds: Option<Vec<Build>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListFleetsInput {
    #[doc="<p>Unique identifier for a build to return fleets for. Use this parameter to return only fleets using the specified build. To retrieve all fleets, leave this parameter empty.</p>"]
    #[serde(rename="BuildId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build_id: Option<String>,
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListFleetsOutput {
    #[doc="<p>Set of fleet IDs matching the list request. You can retrieve additional information about all returned fleets by passing this result set to a call to <a>DescribeFleetAttributes</a>, <a>DescribeFleetCapacity</a>, or <a>DescribeFleetUtilization</a>.</p>"]
    #[serde(rename="FleetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_ids: Option<Vec<String>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>New player session created as a result of a successful FlexMatch match. A successful match automatically creates new player sessions for every player ID in the original matchmaking request. </p> <p>When players connect to the match's game session, they must include both player ID and player session ID in order to claim their assigned player slot.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct MatchedPlayerSession {
    #[doc="<p>Unique identifier for a player </p>"]
    #[serde(rename="PlayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_id: Option<String>,
    #[doc="<p>Unique identifier for a player session</p>"]
    #[serde(rename="PlayerSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_session_id: Option<String>,
}

#[doc="<p>Guidelines for use with FlexMatch to match players into games. All matchmaking requests must specify a matchmaking configuration.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct MatchmakingConfiguration {
    #[doc="<p>Flag that determines whether or not a match that was created with this configuration must be accepted by the matched players. To require acceptance, set to TRUE.</p>"]
    #[serde(rename="AcceptanceRequired")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acceptance_required: Option<bool>,
    #[doc="<p>Length of time (in seconds) to wait for players to accept a proposed match. If any player rejects the match or fails to accept before the timeout, the ticket continues to look for an acceptable match.</p>"]
    #[serde(rename="AcceptanceTimeoutSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acceptance_timeout_seconds: Option<i64>,
    #[doc="<p>Number of player slots in a match to keep open for future players. For example, if the configuration's rule set specifies a match for a single 12-person team, and the additional player count is set to 2, only 10 players are selected for the match.</p>"]
    #[serde(rename="AdditionalPlayerCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_player_count: Option<i64>,
    #[doc="<p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>Information to attached to all events related to the matchmaking configuration. </p>"]
    #[serde(rename="CustomEventData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_event_data: Option<String>,
    #[doc="<p>Descriptive label that is associated with matchmaking configuration.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Set of developer-defined properties for a game session, formatted as a set of type:value pairs. These properties are included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match. </p>"]
    #[serde(rename="GameProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    #[doc="<p>Set of developer-defined game session properties, formatted as a single string value. This data is included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match. </p>"]
    #[serde(rename="GameSessionData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_data: Option<String>,
    #[doc="<p>Amazon Resource Name (<a href=\"http://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html\">ARN</a>) that is assigned to a game session queue and uniquely identifies it. Format is <code>arn:aws:gamelift:&lt;region&gt;::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912</code>. These queues are used when placing game sessions for matches that are created with this matchmaking configuration. Queues can be located in any region.</p>"]
    #[serde(rename="GameSessionQueueArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_queue_arns: Option<Vec<String>>,
    #[doc="<p>Unique identifier for a matchmaking configuration. This name is used to identify the configuration associated with a matchmaking request or ticket.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>SNS topic ARN that is set up to receive matchmaking notifications.</p>"]
    #[serde(rename="NotificationTarget")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notification_target: Option<String>,
    #[doc="<p>Maximum duration, in seconds, that a matchmaking ticket can remain in process before timing out. Requests that time out can be resubmitted as needed.</p>"]
    #[serde(rename="RequestTimeoutSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub request_timeout_seconds: Option<i64>,
    #[doc="<p>Unique identifier for a matchmaking rule set to use with this configuration. A matchmaking configuration can only use rule sets that are defined in the same region.</p>"]
    #[serde(rename="RuleSetName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_set_name: Option<String>,
}

#[doc="<p>Set of rule statements, used with FlexMatch, that determine how to build a certain kind of player match. Each rule set describes a type of group to be created and defines the parameters for acceptable player matches. Rule sets are used in <a>MatchmakingConfiguration</a> objects.</p> <p>A rule set may define the following elements for a match. For detailed information and examples showing how to construct a rule set, see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/match-rules.html\">Create Matchmaking Rules for Your Game</a>. </p> <ul> <li> <p>Teams -- Required. A rule set must define one or multiple teams for the match and set minimum and maximum team sizes. For example, a rule set might describe a 4x4 match that requires all eight slots to be filled. </p> </li> <li> <p>Player attributes -- Optional. These attributes specify a set of player characteristics to evaluate when looking for a match. Matchmaking requests that use a rule set with player attributes must provide the corresponding attribute values. For example, an attribute might specify a player's skill or level.</p> </li> <li> <p>Rules -- Optional. Rules define how to evaluate potential players for a match based on player attributes. A rule might specify minimum requirements for individual players--such as each player must meet a certain skill level, or may describe an entire group--such as all teams must be evenly matched or have at least one player in a certain role. </p> </li> <li> <p>Expansions -- Optional. Expansions allow you to relax the rules after a period of time if no acceptable matches are found. This feature lets you balance getting players into games in a reasonable amount of time instead of making them wait indefinitely for the best possible match. For example, you might use an expansion to increase the maximum skill variance between players after 30 seconds.</p> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct MatchmakingRuleSet {
    #[doc="<p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>Collection of matchmaking rules, formatted as a JSON string. (Note that comments14 are not allowed in JSON, but most elements support a description field.)</p>"]
    #[serde(rename="RuleSetBody")]
    pub rule_set_body: String,
    #[doc="<p>Unique identifier for a matchmaking rule set</p>"]
    #[serde(rename="RuleSetName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_set_name: Option<String>,
}

#[doc="<p>Ticket generated to track the progress of a matchmaking request. Each ticket is uniquely identified by a ticket ID, supplied by the requester, when creating a matchmaking request with <a>StartMatchmaking</a>. Tickets can be retrieved by calling <a>DescribeMatchmaking</a> with the ticket ID.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct MatchmakingTicket {
    #[doc="<p>Name of the <a>MatchmakingConfiguration</a> that is used with this ticket. Matchmaking configurations determine how players are grouped into a match and how a new game session is created for the match.</p>"]
    #[serde(rename="ConfigurationName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configuration_name: Option<String>,
    #[doc="<p>Identifier and connection information of the game session created for the match. This information is added to the ticket only after the matchmaking request has been successfully completed.</p>"]
    #[serde(rename="GameSessionConnectionInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_connection_info: Option<GameSessionConnectionInfo>,
    #[doc="<p>A set of <code>Player</code> objects, each representing a player to find matches for. Players are identified by a unique player ID and may include latency data for use during matchmaking. If the ticket is in status <code>COMPLETED</code>, the <code>Player</code> objects include the team the players were assigned to in the resulting match.</p>"]
    #[serde(rename="Players")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub players: Option<Vec<Player>>,
    #[doc="<p>Time stamp indicating when this matchmaking request was received. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="StartTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_time: Option<f64>,
    #[doc="<p>Current status of the matchmaking request.</p> <ul> <li> <p> <b>QUEUED</b> – The matchmaking request has been received and is currently waiting to be processed.</p> </li> <li> <p> <b>SEARCHING</b> – The matchmaking request is currently being processed. </p> </li> <li> <p> <b>REQUIRES_ACCEPTANCE</b> – A match has been proposed and the players must accept the match (see <a>AcceptMatch</a>). This status is used only with requests that use a matchmaking configuration with a player acceptance requirement.</p> </li> <li> <p> <b>PLACING</b> – The FlexMatch engine has matched players and is in the process of placing a new game session for the match.</p> </li> <li> <p> <b>COMPLETED</b> – Players have been matched and a game session is ready to host the players. A ticket in this state contains the necessary connection information for players.</p> </li> <li> <p> <b>FAILED</b> – The matchmaking request was not completed. Tickets with players who fail to accept a proposed match are placed in <code>FAILED</code> status; new matchmaking requests can be submitted for these players.</p> </li> <li> <p> <b>CANCELLED</b> – The matchmaking request was canceled with a call to <a>StopMatchmaking</a>.</p> </li> <li> <p> <b>TIMED_OUT</b> – The matchmaking request was not completed within the duration specified in the matchmaking configuration. Matchmaking requests that time out can be resubmitted.</p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>Additional information about the current status.</p>"]
    #[serde(rename="StatusMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_message: Option<String>,
    #[doc="<p>Code to explain the current status. For example, a status reason may indicate when a ticket has returned to <code>SEARCHING</code> status after a proposed match fails to receive player acceptances.</p>"]
    #[serde(rename="StatusReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_reason: Option<String>,
    #[doc="<p>Unique identifier for a matchmaking ticket.</p>"]
    #[serde(rename="TicketId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ticket_id: Option<String>,
}

#[doc="<p>Information about a player session that was created as part of a <a>StartGameSessionPlacement</a> request. This object contains only the player ID and player session ID. To retrieve full details on a player session, call <a>DescribePlayerSessions</a> with the player session ID.</p> <p>Player-session-related operations include:</p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PlacedPlayerSession {
    #[doc="<p>Unique identifier for a player that is associated with this player session.</p>"]
    #[serde(rename="PlayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_id: Option<String>,
    #[doc="<p>Unique identifier for a player session.</p>"]
    #[serde(rename="PlayerSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_session_id: Option<String>,
}

#[doc="<p>Object used in matchmaking to represent a player. When starting a matchmaking request, a player has a player ID and may have latency data. Team information is added after a match has been successfully completed.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Player {
    #[doc="<p>Set of values, expressed in milliseconds, indicating the amount of latency that a player experiences when connected to AWS regions. If this property is present, FlexMatch considers placing the match only in regions that are included in the object map. If not present (that is, null), FlexMatch ignores latency issues and may place the match in any region in the queue.</p> <note> <p>If this property contains an empty map, FlexMatch assumes that no regions are available to the player. In this scenario, the ticket is not matchable and always times out unless canceled. </p> </note>"]
    #[serde(rename="LatencyInMs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub latency_in_ms: Option<::std::collections::HashMap<String, i64>>,
    #[doc="<p>Collection of name:value pairs containing player information for use in matchmaking. Player attribute names need to match <i>playerAttributes</i> names in the rule set being used. Example: <code>\"PlayerAttributes\": {\"skill\": {\"N\": \"23\"}, \"gameMode\": {\"S\": \"deathmatch\"}}</code>.</p>"]
    #[serde(rename="PlayerAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_attributes: Option<::std::collections::HashMap<String, AttributeValue>>,
    #[doc="<p>Unique identifier for a player</p>"]
    #[serde(rename="PlayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_id: Option<String>,
    #[doc="<p>Name of the team that the player is assigned to in a match. Team names are defined in a matchmaking rule set.</p>"]
    #[serde(rename="Team")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub team: Option<String>,
}

#[doc="<p>Regional latency information for a player, used when requesting a new game session with <a>StartGameSessionPlacement</a>. This value indicates the amount of time lag that exists when the player is connected to a fleet in the specified region. The relative difference between a player's latency values for multiple regions are used to determine which fleets are best suited to place a new game session for the player. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PlayerLatency {
    #[doc="<p>Amount of time that represents the time lag experienced by the player when connected to the specified region.</p>"]
    #[serde(rename="LatencyInMilliseconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub latency_in_milliseconds: Option<f32>,
    #[doc="<p>Unique identifier for a player associated with the latency data.</p>"]
    #[serde(rename="PlayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_id: Option<String>,
    #[doc="<p>Name of the region that is associated with the latency value.</p>"]
    #[serde(rename="RegionIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region_identifier: Option<String>,
}

#[doc="<p>Queue setting that determines the highest latency allowed for individual players when placing a game session. When a latency policy is in force, a game session cannot be placed at any destination in a region where a player is reporting latency higher than the cap. Latency policies are only enforced when the placement request contains player latency information.</p> <p>Queue-related operations include:</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PlayerLatencyPolicy {
    #[doc="<p>The maximum latency value that is allowed for any player, in milliseconds. All policies must have a value set for this property.</p>"]
    #[serde(rename="MaximumIndividualPlayerLatencyMilliseconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_individual_player_latency_milliseconds: Option<i64>,
    #[doc="<p>The length of time, in seconds, that the policy is enforced while placing a new game session. A null value for this property means that the policy is enforced until the queue times out.</p>"]
    #[serde(rename="PolicyDurationSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_duration_seconds: Option<i64>,
}

#[doc="<p>Properties describing a player session. Player session objects are created either by creating a player session for a specific game session, or as part of a game session placement. A player session represents either a player reservation for a game session (status <code>RESERVED</code>) or actual player activity in a game session (status <code>ACTIVE</code>). A player session object (including player data) is automatically passed to a game session when the player connects to the game session and is validated.</p> <p>When a player disconnects, the player session status changes to <code>COMPLETED</code>. Once the session ends, the player session object is retained for 30 days and then removed.</p> <p>Player-session-related operations include:</p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PlayerSession {
    #[doc="<p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<f64>,
    #[doc="<p>Unique identifier for a fleet that the player's game session is running on.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Unique identifier for the game session that the player session is connected to.</p>"]
    #[serde(rename="GameSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_id: Option<String>,
    #[doc="<p>IP address of the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number.</p>"]
    #[serde(rename="IpAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_address: Option<String>,
    #[doc="<p>Developer-defined information related to a player. Amazon GameLift does not use this data, so it can be formatted as needed for use in the game. </p>"]
    #[serde(rename="PlayerData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_data: Option<String>,
    #[doc="<p>Unique identifier for a player that is associated with this player session.</p>"]
    #[serde(rename="PlayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_id: Option<String>,
    #[doc="<p>Unique identifier for a player session.</p>"]
    #[serde(rename="PlayerSessionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_session_id: Option<String>,
    #[doc="<p>Port number for the game session. To connect to a Amazon GameLift server process, an app needs both the IP address and port number.</p>"]
    #[serde(rename="Port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<i64>,
    #[doc="<p>Current status of the player session.</p> <p>Possible player session statuses include the following:</p> <ul> <li> <p> <b>RESERVED</b> – The player session request has been received, but the player has not yet connected to the server process and/or been validated. </p> </li> <li> <p> <b>ACTIVE</b> – The player has been validated by the server process and is currently connected.</p> </li> <li> <p> <b>COMPLETED</b> – The player connection has been dropped.</p> </li> <li> <p> <b>TIMEDOUT</b> – A player session request was received, but the player did not connect and/or was not validated within the time-out limit (60 seconds).</p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>Time stamp indicating when this data object was terminated. Format is a number expressed in Unix time as milliseconds (for example \"1469498468.057\").</p>"]
    #[serde(rename="TerminationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub termination_time: Option<f64>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutScalingPolicyInput {
    #[doc="<p>Comparison operator to use when measuring the metric against the threshold value.</p>"]
    #[serde(rename="ComparisonOperator")]
    pub comparison_operator: String,
    #[doc="<p>Length of time (in minutes) the metric must be at or beyond the threshold before a scaling event is triggered.</p>"]
    #[serde(rename="EvaluationPeriods")]
    pub evaluation_periods: i64,
    #[doc="<p>Unique identifier for a fleet to apply this policy to.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
    #[doc="<p>Name of the Amazon GameLift-defined metric that is used to trigger an adjustment.</p> <ul> <li> <p> <b>ActivatingGameSessions</b> – number of game sessions in the process of being created (game session status = <code>ACTIVATING</code>).</p> </li> <li> <p> <b>ActiveGameSessions</b> – number of game sessions currently running (game session status = <code>ACTIVE</code>).</p> </li> <li> <p> <b>CurrentPlayerSessions</b> – number of active or reserved player sessions (player session status = <code>ACTIVE</code> or <code>RESERVED</code>). </p> </li> <li> <p> <b>AvailablePlayerSessions</b> – number of player session slots currently available in active game sessions across the fleet, calculated by subtracting a game session's current player session count from its maximum player session count. This number includes game sessions that are not currently accepting players (game session <code>PlayerSessionCreationPolicy</code> = <code>DENY_ALL</code>).</p> </li> <li> <p> <b>ActiveInstances</b> – number of instances currently running a game session.</p> </li> <li> <p> <b>IdleInstances</b> – number of instances not currently running a game session.</p> </li> </ul>"]
    #[serde(rename="MetricName")]
    pub metric_name: String,
    #[doc="<p>Descriptive label that is associated with a scaling policy. Policy names do not need to be unique. A fleet can have only one scaling policy with the same name.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>Amount of adjustment to make, based on the scaling adjustment type.</p>"]
    #[serde(rename="ScalingAdjustment")]
    pub scaling_adjustment: i64,
    #[doc="<p>Type of adjustment to make to a fleet's instance count (see <a>FleetCapacity</a>):</p> <ul> <li> <p> <b>ChangeInCapacity</b> – add (or subtract) the scaling adjustment value from the current instance count. Positive values scale up while negative values scale down.</p> </li> <li> <p> <b>ExactCapacity</b> – set the instance count to the scaling adjustment value.</p> </li> <li> <p> <b>PercentChangeInCapacity</b> – increase or reduce the current instance count by the scaling adjustment, read as a percentage. Positive values scale up while negative values scale down; for example, a value of \"-10\" scales the fleet down by 10%.</p> </li> </ul>"]
    #[serde(rename="ScalingAdjustmentType")]
    pub scaling_adjustment_type: String,
    #[doc="<p>Metric value used to trigger a scaling event.</p>"]
    #[serde(rename="Threshold")]
    pub threshold: f64,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutScalingPolicyOutput {
    #[doc="<p>Descriptive label that is associated with a scaling policy. Policy names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RequestUploadCredentialsInput {
    #[doc="<p>Unique identifier for a build to get credentials for.</p>"]
    #[serde(rename="BuildId")]
    pub build_id: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RequestUploadCredentialsOutput {
    #[doc="<p>Amazon S3 path and key, identifying where the game build files are stored.</p>"]
    #[serde(rename="StorageLocation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_location: Option<S3Location>,
    #[doc="<p>AWS credentials required when uploading a game build to the storage location. These credentials have a limited lifespan and are valid only for the build they were issued for.</p>"]
    #[serde(rename="UploadCredentials")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub upload_credentials: Option<AwsCredentials>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ResolveAliasInput {
    #[doc="<p>Unique identifier for the alias you want to resolve.</p>"]
    #[serde(rename="AliasId")]
    pub alias_id: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ResolveAliasOutput {
    #[doc="<p>Fleet identifier that is associated with the requested alias.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
}

#[doc="<p>Policy that limits the number of game sessions a player can create on the same fleet. This optional policy gives game owners control over how players can consume available game server resources. A resource creation policy makes the following statement: \"An individual player can create a maximum number of new game sessions within a specified time period\".</p> <p>The policy is evaluated when a player tries to create a new game session. For example, with a policy of 10 new game sessions and a time period of 60 minutes, on receiving a <code>CreateGameSession</code> request, Amazon GameLift checks that the player (identified by <code>CreatorId</code>) has created fewer than 10 game sessions in the past 60 minutes.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ResourceCreationLimitPolicy {
    #[doc="<p>Maximum number of game sessions that an individual can create during the policy period. </p>"]
    #[serde(rename="NewGameSessionsPerCreator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_game_sessions_per_creator: Option<i64>,
    #[doc="<p>Time span used in evaluating the resource creation limit policy. </p>"]
    #[serde(rename="PolicyPeriodInMinutes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_period_in_minutes: Option<i64>,
}

#[doc="<p>Routing configuration for a fleet alias.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RoutingStrategy {
    #[doc="<p>Unique identifier for a fleet that the alias points to.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Message text to be used with a terminal routing strategy.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[doc="<p>Type of routing strategy.</p> <p>Possible routing types include the following:</p> <ul> <li> <p> <b>SIMPLE</b> – The alias resolves to one specific fleet. Use this type when routing to active fleets.</p> </li> <li> <p> <b>TERMINAL</b> – The alias does not resolve to a fleet but instead can be used to display a message to the user. A terminal alias throws a TerminalRoutingStrategyException with the <a>RoutingStrategy</a> message embedded.</p> </li> </ul>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[doc="<p>A collection of server process configurations that describe what processes to run on each instance in a fleet. All fleets must have a run-time configuration. Each instance in the fleet launches the server processes specified in the run-time configuration and launches new ones as existing processes end. Each instance regularly checks for an updated run-time configuration and follows the new instructions. </p> <p>The run-time configuration enables the instances in a fleet to run multiple processes simultaneously. Potential scenarios are as follows: (1) Run multiple processes of a single game server executable to maximize usage of your hosting resources. (2) Run one or more processes of different build executables, such as your game server executable and a related program, or two or more different versions of a game server. (3) Run multiple processes of a single game server but with different launch parameters, for example to run one process on each instance in debug mode.</p> <p>A Amazon GameLift instance is limited to 50 processes running simultaneously. A run-time configuration must specify fewer than this limit. To calculate the total number of processes specified in a run-time configuration, add the values of the <code>ConcurrentExecutions</code> parameter for each <code> <a>ServerProcess</a> </code> object in the run-time configuration.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RuntimeConfiguration {
    #[doc="<p>Maximum amount of time (in seconds) that a game session can remain in status <code>ACTIVATING</code>. If the game session is not active before the timeout, activation is terminated and the game session status is changed to <code>TERMINATED</code>.</p>"]
    #[serde(rename="GameSessionActivationTimeoutSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_activation_timeout_seconds: Option<i64>,
    #[doc="<p>Maximum number of game sessions with status <code>ACTIVATING</code> to allow on an instance simultaneously. This setting limits the amount of instance resources that can be used for new game activations at any one time.</p>"]
    #[serde(rename="MaxConcurrentGameSessionActivations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_concurrent_game_session_activations: Option<i64>,
    #[doc="<p>Collection of server process configurations that describe which server processes to run on each instance in a fleet.</p>"]
    #[serde(rename="ServerProcesses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_processes: Option<Vec<ServerProcess>>,
}

#[doc="<p>Location in Amazon Simple Storage Service (Amazon S3) where build files can be stored for access by Amazon GameLift. This location is specified in a <a>CreateBuild</a> request. For more details, see the <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-cli-uploading.html#gamelift-build-cli-uploading-create-build\">Create a Build with Files in Amazon S3</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct S3Location {
    #[doc="<p>Amazon S3 bucket identifier. This is the name of your S3 bucket.</p>"]
    #[serde(rename="Bucket")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bucket: Option<String>,
    #[doc="<p>Name of the zip file containing your build files. </p>"]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,
    #[doc="<p>Amazon Resource Name (<a href=\"http://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html\">ARN</a>) for the access role that allows Amazon GameLift to access your S3 bucket.</p>"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}

#[doc="<p>Rule that controls how a fleet is scaled. Scaling policies are uniquely identified by the combination of name and fleet ID.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ScalingPolicy {
    #[doc="<p>Comparison operator to use when measuring a metric against the threshold value.</p>"]
    #[serde(rename="ComparisonOperator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comparison_operator: Option<String>,
    #[doc="<p>Length of time (in minutes) the metric must be at or beyond the threshold before a scaling event is triggered.</p>"]
    #[serde(rename="EvaluationPeriods")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub evaluation_periods: Option<i64>,
    #[doc="<p>Unique identifier for a fleet that is associated with this scaling policy.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Name of the Amazon GameLift-defined metric that is used to trigger an adjustment.</p> <ul> <li> <p> <b>ActivatingGameSessions</b> – number of game sessions in the process of being created (game session status = <code>ACTIVATING</code>).</p> </li> <li> <p> <b>ActiveGameSessions</b> – number of game sessions currently running (game session status = <code>ACTIVE</code>).</p> </li> <li> <p> <b>CurrentPlayerSessions</b> – number of active or reserved player sessions (player session status = <code>ACTIVE</code> or <code>RESERVED</code>). </p> </li> <li> <p> <b>AvailablePlayerSessions</b> – number of player session slots currently available in active game sessions across the fleet, calculated by subtracting a game session's current player session count from its maximum player session count. This number does include game sessions that are not currently accepting players (game session <code>PlayerSessionCreationPolicy</code> = <code>DENY_ALL</code>).</p> </li> <li> <p> <b>ActiveInstances</b> – number of instances currently running a game session.</p> </li> <li> <p> <b>IdleInstances</b> – number of instances not currently running a game session.</p> </li> </ul>"]
    #[serde(rename="MetricName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric_name: Option<String>,
    #[doc="<p>Descriptive label that is associated with a scaling policy. Policy names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Amount of adjustment to make, based on the scaling adjustment type.</p>"]
    #[serde(rename="ScalingAdjustment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scaling_adjustment: Option<i64>,
    #[doc="<p>Type of adjustment to make to a fleet's instance count (see <a>FleetCapacity</a>):</p> <ul> <li> <p> <b>ChangeInCapacity</b> – add (or subtract) the scaling adjustment value from the current instance count. Positive values scale up while negative values scale down.</p> </li> <li> <p> <b>ExactCapacity</b> – set the instance count to the scaling adjustment value.</p> </li> <li> <p> <b>PercentChangeInCapacity</b> – increase or reduce the current instance count by the scaling adjustment, read as a percentage. Positive values scale up while negative values scale down.</p> </li> </ul>"]
    #[serde(rename="ScalingAdjustmentType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scaling_adjustment_type: Option<String>,
    #[doc="<p>Current status of the scaling policy. The scaling policy is only in force when in an <code>ACTIVE</code> status.</p> <ul> <li> <p> <b>ACTIVE</b> – The scaling policy is currently in force.</p> </li> <li> <p> <b>UPDATE_REQUESTED</b> – A request to update the scaling policy has been received.</p> </li> <li> <p> <b>UPDATING</b> – A change is being made to the scaling policy.</p> </li> <li> <p> <b>DELETE_REQUESTED</b> – A request to delete the scaling policy has been received.</p> </li> <li> <p> <b>DELETING</b> – The scaling policy is being deleted.</p> </li> <li> <p> <b>DELETED</b> – The scaling policy has been deleted.</p> </li> <li> <p> <b>ERROR</b> – An error occurred in creating the policy. It should be removed and recreated.</p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>Metric value used to trigger a scaling event.</p>"]
    #[serde(rename="Threshold")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub threshold: Option<f64>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SearchGameSessionsInput {
    #[doc="<p>Unique identifier for an alias associated with the fleet to search for active game sessions. Each request must reference either a fleet ID or alias ID, but not both.</p>"]
    #[serde(rename="AliasId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias_id: Option<String>,
    #[doc="<p>String containing the search criteria for the session search. If no filter expression is included, the request returns results for all game sessions in the fleet that are in <code>ACTIVE</code> status.</p> <p>A filter expression can contain one or multiple conditions. Each condition consists of the following:</p> <ul> <li> <p> <b>Operand</b> -- Name of a game session attribute. Valid values are <code>gameSessionName</code>, <code>gameSessionId</code>, <code>creationTimeMillis</code>, <code>playerSessionCount</code>, <code>maximumSessions</code>, <code>hasAvailablePlayerSessions</code>.</p> </li> <li> <p> <b>Comparator</b> -- Valid comparators are: <code>=</code>, <code>&lt;&gt;</code>, <code>&lt;</code>, <code>&gt;</code>, <code>&lt;=</code>, <code>&gt;=</code>. </p> </li> <li> <p> <b>Value</b> -- Value to be searched for. Values can be numbers, boolean values (true/false) or strings. String values are case sensitive, enclosed in single quotes. Special characters must be escaped. Boolean and string values can only be used with the comparators <code>=</code> and <code>&lt;&gt;</code>. For example, the following filter expression searches on <code>gameSessionName</code>: \"<code>FilterExpression\": \"gameSessionName = 'Matt\\\\'s Awesome Game 1'\"</code>. </p> </li> </ul> <p>To chain multiple conditions in a single expression, use the logical keywords <code>AND</code>, <code>OR</code>, and <code>NOT</code> and parentheses as needed. For example: <code>x AND y AND NOT z</code>, <code>NOT (x OR y)</code>.</p> <p>Session search evaluates conditions from left to right using the following precedence rules:</p> <ol> <li> <p> <code>=</code>, <code>&lt;&gt;</code>, <code>&lt;</code>, <code>&gt;</code>, <code>&lt;=</code>, <code>&gt;=</code> </p> </li> <li> <p>Parentheses</p> </li> <li> <p>NOT</p> </li> <li> <p>AND</p> </li> <li> <p>OR</p> </li> </ol> <p>For example, this filter expression retrieves game sessions hosting at least ten players that have an open player slot: <code>\"maximumSessions&gt;=10 AND hasAvailablePlayerSessions=true\"</code>. </p>"]
    #[serde(rename="FilterExpression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter_expression: Option<String>,
    #[doc="<p>Unique identifier for a fleet to search for active game sessions. Each request must reference either a fleet ID or alias ID, but not both.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
    #[doc="<p>Maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. The maximum number of results returned is 20, even if this value is not set or is set higher than 20. </p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this action. To start at the beginning of the result set, do not specify a value.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Instructions on how to sort the search results. If no sort expression is included, the request returns results in random order. A sort expression consists of the following elements:</p> <ul> <li> <p> <b>Operand</b> -- Name of a game session attribute. Valid values are <code>gameSessionName</code>, <code>gameSessionId</code>, <code>creationTimeMillis</code>, <code>playerSessionCount</code>, <code>maximumSessions</code>, <code>hasAvailablePlayerSessions</code>.</p> </li> <li> <p> <b>Order</b> -- Valid sort orders are <code>ASC</code> (ascending) and <code>DESC</code> (descending).</p> </li> </ul> <p>For example, this sort expression returns the oldest active sessions first: <code>\"SortExpression\": \"creationTimeMillis ASC\"</code>. Results with a null value for the sort operand are returned at the end of the list.</p>"]
    #[serde(rename="SortExpression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_expression: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SearchGameSessionsOutput {
    #[doc="<p>Collection of objects containing game session properties for each session matching the request.</p>"]
    #[serde(rename="GameSessions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_sessions: Option<Vec<GameSession>>,
    #[doc="<p>Token that indicates where to resume retrieving results on the next call to this action. If no token is returned, these results represent the end of the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>A set of instructions for launching server processes on each instance in a fleet. Each instruction set identifies the location of the server executable, optional launch parameters, and the number of server processes with this configuration to maintain concurrently on the instance. Server process configurations make up a fleet's <code> <a>RuntimeConfiguration</a> </code>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ServerProcess {
    #[doc="<p>Number of server processes using this configuration to run concurrently on an instance.</p>"]
    #[serde(rename="ConcurrentExecutions")]
    pub concurrent_executions: i64,
    #[doc="<p>Location of the server executable in a game build. All game builds are installed on instances at the root : for Windows instances <code>C:\\game</code>, and for Linux instances <code>/local/game</code>. A Windows game build with an executable file located at <code>MyGame\\latest\\server.exe</code> must have a launch path of \"<code>C:\\game\\MyGame\\latest\\server.exe</code>\". A Linux game build with an executable file located at <code>MyGame/latest/server.exe</code> must have a launch path of \"<code>/local/game/MyGame/latest/server.exe</code>\". </p>"]
    #[serde(rename="LaunchPath")]
    pub launch_path: String,
    #[doc="<p>Optional list of parameters to pass to the server executable on launch.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StartGameSessionPlacementInput {
    #[doc="<p>Set of information on each player to create a player session for.</p>"]
    #[serde(rename="DesiredPlayerSessions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_player_sessions: Option<Vec<DesiredPlayerSession>>,
    #[doc="<p>Set of developer-defined properties for a game session, formatted as a set of type:value pairs. These properties are included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>).</p>"]
    #[serde(rename="GameProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    #[doc="<p>Set of developer-defined game session properties, formatted as a single string value. This data is included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>).</p>"]
    #[serde(rename="GameSessionData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_data: Option<String>,
    #[doc="<p>Descriptive label that is associated with a game session. Session names do not need to be unique.</p>"]
    #[serde(rename="GameSessionName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_name: Option<String>,
    #[doc="<p>Name of the queue to use to place the new game session.</p>"]
    #[serde(rename="GameSessionQueueName")]
    pub game_session_queue_name: String,
    #[doc="<p>Maximum number of players that can be connected simultaneously to the game session.</p>"]
    #[serde(rename="MaximumPlayerSessionCount")]
    pub maximum_player_session_count: i64,
    #[doc="<p>Unique identifier to assign to the new game session placement. This value is developer-defined. The value must be unique across all regions and cannot be reused unless you are resubmitting a canceled or timed-out placement request.</p>"]
    #[serde(rename="PlacementId")]
    pub placement_id: String,
    #[doc="<p>Set of values, expressed in milliseconds, indicating the amount of latency that a player experiences when connected to AWS regions. This information is used to try to place the new game session where it can offer the best possible gameplay experience for the players. </p>"]
    #[serde(rename="PlayerLatencies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_latencies: Option<Vec<PlayerLatency>>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartGameSessionPlacementOutput {
    #[doc="<p>Object that describes the newly created game session placement. This object includes all the information provided in the request, as well as start/end time stamps and placement status. </p>"]
    #[serde(rename="GameSessionPlacement")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_placement: Option<GameSessionPlacement>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StartMatchmakingInput {
    #[doc="<p>Name of the matchmaking configuration to use for this request. Matchmaking configurations must exist in the same region as this request.</p>"]
    #[serde(rename="ConfigurationName")]
    pub configuration_name: String,
    #[doc="<p>Information on each player to be matched. This information must include a player ID, and may contain player attributes and latency data to be used in the matchmaking process. After a successful match, <code>Player</code> objects contain the name of the team the player is assigned to.</p>"]
    #[serde(rename="Players")]
    pub players: Vec<Player>,
    #[doc="<p>Unique identifier for a matchmaking ticket. Use this identifier to track the matchmaking ticket status and retrieve match results.</p>"]
    #[serde(rename="TicketId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ticket_id: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartMatchmakingOutput {
    #[doc="<p>Ticket representing the matchmaking request. This object include the information included in the request, ticket status, and match results as generated during the matchmaking process.</p>"]
    #[serde(rename="MatchmakingTicket")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub matchmaking_ticket: Option<MatchmakingTicket>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StopGameSessionPlacementInput {
    #[doc="<p>Unique identifier for a game session placement to cancel.</p>"]
    #[serde(rename="PlacementId")]
    pub placement_id: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StopGameSessionPlacementOutput {
    #[doc="<p>Object that describes the canceled game session placement, with <code>CANCELLED</code> status and an end time stamp. </p>"]
    #[serde(rename="GameSessionPlacement")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_placement: Option<GameSessionPlacement>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StopMatchmakingInput {
    #[doc="<p>Unique identifier for a matchmaking ticket.</p>"]
    #[serde(rename="TicketId")]
    pub ticket_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StopMatchmakingOutput;

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateAliasInput {
    #[doc="<p>Unique identifier for a fleet alias. Specify the alias you want to update.</p>"]
    #[serde(rename="AliasId")]
    pub alias_id: String,
    #[doc="<p>Human-readable description of an alias.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Descriptive label that is associated with an alias. Alias names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Object that specifies the fleet and routing type to use for the alias.</p>"]
    #[serde(rename="RoutingStrategy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub routing_strategy: Option<RoutingStrategy>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateAliasOutput {
    #[doc="<p>Object that contains the updated alias configuration.</p>"]
    #[serde(rename="Alias")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias: Option<Alias>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateBuildInput {
    #[doc="<p>Unique identifier for a build to update.</p>"]
    #[serde(rename="BuildId")]
    pub build_id: String,
    #[doc="<p>Descriptive label that is associated with a build. Build names do not need to be unique. </p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Version that is associated with this build. Version strings do not need to be unique.</p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateBuildOutput {
    #[doc="<p>Object that contains the updated build record.</p>"]
    #[serde(rename="Build")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build: Option<Build>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateFleetAttributesInput {
    #[doc="<p>Human-readable description of a fleet.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Unique identifier for a fleet to update attribute metadata for.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
    #[doc="<p>Names of metric groups to include this fleet in. Amazon CloudWatch uses a fleet metric group is to aggregate metrics from multiple fleets. Use an existing metric group name to add this fleet to the group. Or use a new name to create a new metric group. A fleet can only be included in one metric group at a time.</p>"]
    #[serde(rename="MetricGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric_groups: Option<Vec<String>>,
    #[doc="<p>Descriptive label that is associated with a fleet. Fleet names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Game session protection policy to apply to all new instances created in this fleet. Instances that already exist are not affected. You can set protection for individual instances using <a>UpdateGameSession</a>.</p> <ul> <li> <p> <b>NoProtection</b> – The game session can be terminated during a scale-down event.</p> </li> <li> <p> <b>FullProtection</b> – If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li> </ul>"]
    #[serde(rename="NewGameSessionProtectionPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_game_session_protection_policy: Option<String>,
    #[doc="<p>Policy that limits the number of game sessions an individual player can create over a span of time. </p>"]
    #[serde(rename="ResourceCreationLimitPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_creation_limit_policy: Option<ResourceCreationLimitPolicy>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateFleetAttributesOutput {
    #[doc="<p>Unique identifier for a fleet that was updated.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateFleetCapacityInput {
    #[doc="<p>Number of EC2 instances you want this fleet to host.</p>"]
    #[serde(rename="DesiredInstances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_instances: Option<i64>,
    #[doc="<p>Unique identifier for a fleet to update capacity for.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
    #[doc="<p>Maximum value allowed for the fleet's instance count. Default if not set is 1.</p>"]
    #[serde(rename="MaxSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_size: Option<i64>,
    #[doc="<p>Minimum value allowed for the fleet's instance count. Default if not set is 0.</p>"]
    #[serde(rename="MinSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_size: Option<i64>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateFleetCapacityOutput {
    #[doc="<p>Unique identifier for a fleet that was updated.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateFleetPortSettingsInput {
    #[doc="<p>Unique identifier for a fleet to update port settings for.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
    #[doc="<p>Collection of port settings to be added to the fleet record.</p>"]
    #[serde(rename="InboundPermissionAuthorizations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inbound_permission_authorizations: Option<Vec<IpPermission>>,
    #[doc="<p>Collection of port settings to be removed from the fleet record.</p>"]
    #[serde(rename="InboundPermissionRevocations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inbound_permission_revocations: Option<Vec<IpPermission>>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateFleetPortSettingsOutput {
    #[doc="<p>Unique identifier for a fleet that was updated.</p>"]
    #[serde(rename="FleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fleet_id: Option<String>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateGameSessionInput {
    #[doc="<p>Unique identifier for the game session to update.</p>"]
    #[serde(rename="GameSessionId")]
    pub game_session_id: String,
    #[doc="<p>Maximum number of players that can be connected simultaneously to the game session.</p>"]
    #[serde(rename="MaximumPlayerSessionCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_player_session_count: Option<i64>,
    #[doc="<p>Descriptive label that is associated with a game session. Session names do not need to be unique.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Policy determining whether or not the game session accepts new players.</p>"]
    #[serde(rename="PlayerSessionCreationPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_session_creation_policy: Option<String>,
    #[doc="<p>Game session protection policy to apply to this game session only.</p> <ul> <li> <p> <b>NoProtection</b> – The game session can be terminated during a scale-down event.</p> </li> <li> <p> <b>FullProtection</b> – If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li> </ul>"]
    #[serde(rename="ProtectionPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection_policy: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateGameSessionOutput {
    #[doc="<p>Object that contains the updated game session metadata.</p>"]
    #[serde(rename="GameSession")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session: Option<GameSession>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateGameSessionQueueInput {
    #[doc="<p>List of fleets that can be used to fulfill game session placement requests in the queue. Fleets are identified by either a fleet ARN or a fleet alias ARN. Destinations are listed in default preference order. When updating this list, provide a complete list of destinations.</p>"]
    #[serde(rename="Destinations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub destinations: Option<Vec<GameSessionQueueDestination>>,
    #[doc="<p>Descriptive label that is associated with game session queue. Queue names must be unique within each region.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>Collection of latency policies to apply when processing game sessions placement requests with player latency information. Multiple policies are evaluated in order of the maximum latency value, starting with the lowest latency values. With just one policy, it is enforced at the start of the game session placement for the duration period. With multiple policies, each policy is enforced consecutively for its duration period. For example, a queue might enforce a 60-second policy followed by a 120-second policy, and then no policy for the remainder of the placement. When updating policies, provide a complete collection of policies.</p>"]
    #[serde(rename="PlayerLatencyPolicies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_latency_policies: Option<Vec<PlayerLatencyPolicy>>,
    #[doc="<p>Maximum time, in seconds, that a new game session placement request remains in the queue. When a request exceeds this time, the game session placement changes to a <code>TIMED_OUT</code> status.</p>"]
    #[serde(rename="TimeoutInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateGameSessionQueueOutput {
    #[doc="<p>Object that describes the newly updated game session queue.</p>"]
    #[serde(rename="GameSessionQueue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_queue: Option<GameSessionQueue>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateMatchmakingConfigurationInput {
    #[doc="<p>Flag that determines whether or not a match that was created with this configuration must be accepted by the matched players. To require acceptance, set to TRUE.</p>"]
    #[serde(rename="AcceptanceRequired")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acceptance_required: Option<bool>,
    #[doc="<p>Length of time (in seconds) to wait for players to accept a proposed match. If any player rejects the match or fails to accept before the timeout, the ticket continues to look for an acceptable match.</p>"]
    #[serde(rename="AcceptanceTimeoutSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acceptance_timeout_seconds: Option<i64>,
    #[doc="<p>Number of player slots in a match to keep open for future players. For example, if the configuration's rule set specifies a match for a single 12-person team, and the additional player count is set to 2, only 10 players are selected for the match.</p>"]
    #[serde(rename="AdditionalPlayerCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_player_count: Option<i64>,
    #[doc="<p>Information to attached to all events related to the matchmaking configuration. </p>"]
    #[serde(rename="CustomEventData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_event_data: Option<String>,
    #[doc="<p>Descriptive label that is associated with matchmaking configuration.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Set of developer-defined properties for a game session, formatted as a set of type:value pairs. These properties are included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match. </p>"]
    #[serde(rename="GameProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    #[doc="<p>Set of developer-defined game session properties, formatted as a single string value. This data is included in the <a>GameSession</a> object, which is passed to the game server with a request to start a new game session (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession\">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match. </p>"]
    #[serde(rename="GameSessionData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_data: Option<String>,
    #[doc="<p>Amazon Resource Name (<a href=\"http://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html\">ARN</a>) that is assigned to a game session queue and uniquely identifies it. Format is <code>arn:aws:gamelift:&lt;region&gt;::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912</code>. These queues are used when placing game sessions for matches that are created with this matchmaking configuration. Queues can be located in any region.</p>"]
    #[serde(rename="GameSessionQueueArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub game_session_queue_arns: Option<Vec<String>>,
    #[doc="<p>Unique identifier for a matchmaking configuration to update.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>SNS topic ARN that is set up to receive matchmaking notifications. See <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/match-notification.html\"> Setting up Notifications for Matchmaking</a> for more information.</p>"]
    #[serde(rename="NotificationTarget")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notification_target: Option<String>,
    #[doc="<p>Maximum duration, in seconds, that a matchmaking ticket can remain in process before timing out. Requests that time out can be resubmitted as needed.</p>"]
    #[serde(rename="RequestTimeoutSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub request_timeout_seconds: Option<i64>,
    #[doc="<p>Unique identifier for a matchmaking rule set to use with this configuration. A matchmaking configuration can only use rule sets that are defined in the same region.</p>"]
    #[serde(rename="RuleSetName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_set_name: Option<String>,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateMatchmakingConfigurationOutput {
    #[doc="<p>Object that describes the updated matchmaking configuration.</p>"]
    #[serde(rename="Configuration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configuration: Option<MatchmakingConfiguration>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateRuntimeConfigurationInput {
    #[doc="<p>Unique identifier for a fleet to update run-time configuration for.</p>"]
    #[serde(rename="FleetId")]
    pub fleet_id: String,
    #[doc="<p>Instructions for launching server processes on each instance in the fleet. The run-time configuration for a fleet has a collection of server process configurations, one for each type of server process to run on an instance. A server process configuration specifies the location of the server executable, launch parameters, and the number of concurrent processes with that configuration to maintain on each instance.</p>"]
    #[serde(rename="RuntimeConfiguration")]
    pub runtime_configuration: RuntimeConfiguration,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateRuntimeConfigurationOutput {
    #[doc="<p>The run-time configuration currently in force. If the update was successful, this object matches the one in the request.</p>"]
    #[serde(rename="RuntimeConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runtime_configuration: Option<RuntimeConfiguration>,
}

#[doc="<p>Represents the input for a request action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ValidateMatchmakingRuleSetInput {
    #[doc="<p>Collection of matchmaking rules to validate, formatted as a JSON string.</p>"]
    #[serde(rename="RuleSetBody")]
    pub rule_set_body: String,
}

#[doc="<p>Represents the returned data in response to a request action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ValidateMatchmakingRuleSetOutput {
    #[doc="<p>Response indicating whether or not the rule set is valid.</p>"]
    #[serde(rename="Valid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub valid: Option<bool>,
}

/// Errors returned by AcceptMatch
#[derive(Debug, PartialEq)]
pub enum AcceptMatchError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The requested operation is not supported in the region specified.</p>
    UnsupportedRegion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AcceptMatchError {
    pub fn from_body(body: &str) -> AcceptMatchError {
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
                        AcceptMatchError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AcceptMatchError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => AcceptMatchError::NotFound(String::from(error_message)),
                    "UnsupportedRegionException" => {
                        AcceptMatchError::UnsupportedRegion(String::from(error_message))
                    }
                    "ValidationException" => {
                        AcceptMatchError::Validation(error_message.to_string())
                    }
                    _ => AcceptMatchError::Unknown(String::from(body)),
                }
            }
            Err(_) => AcceptMatchError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AcceptMatchError {
    fn from(err: serde_json::error::Error) -> AcceptMatchError {
        AcceptMatchError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AcceptMatchError {
    fn from(err: CredentialsError) -> AcceptMatchError {
        AcceptMatchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AcceptMatchError {
    fn from(err: HttpDispatchError) -> AcceptMatchError {
        AcceptMatchError::HttpDispatch(err)
    }
}
impl From<io::Error> for AcceptMatchError {
    fn from(err: io::Error) -> AcceptMatchError {
        AcceptMatchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AcceptMatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcceptMatchError {
    fn description(&self) -> &str {
        match *self {
            AcceptMatchError::InternalService(ref cause) => cause,
            AcceptMatchError::InvalidRequest(ref cause) => cause,
            AcceptMatchError::NotFound(ref cause) => cause,
            AcceptMatchError::UnsupportedRegion(ref cause) => cause,
            AcceptMatchError::Validation(ref cause) => cause,
            AcceptMatchError::Credentials(ref err) => err.description(),
            AcceptMatchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AcceptMatchError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    ///<p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateAliasError {
    pub fn from_body(body: &str) -> CreateAliasError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConflictException" => CreateAliasError::Conflict(String::from(error_message)),
                    "InternalServiceException" => {
                        CreateAliasError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateAliasError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateAliasError::LimitExceeded(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateAliasError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateAliasError::Validation(error_message.to_string())
                    }
                    _ => CreateAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAliasError {
    fn from(err: serde_json::error::Error) -> CreateAliasError {
        CreateAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAliasError {
    fn from(err: CredentialsError) -> CreateAliasError {
        CreateAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAliasError {
    fn from(err: HttpDispatchError) -> CreateAliasError {
        CreateAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAliasError {
    fn from(err: io::Error) -> CreateAliasError {
        CreateAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAliasError {
    fn description(&self) -> &str {
        match *self {
            CreateAliasError::Conflict(ref cause) => cause,
            CreateAliasError::InternalService(ref cause) => cause,
            CreateAliasError::InvalidRequest(ref cause) => cause,
            CreateAliasError::LimitExceeded(ref cause) => cause,
            CreateAliasError::Unauthorized(ref cause) => cause,
            CreateAliasError::Validation(ref cause) => cause,
            CreateAliasError::Credentials(ref err) => err.description(),
            CreateAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateBuild
#[derive(Debug, PartialEq)]
pub enum CreateBuildError {
    ///<p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateBuildError {
    pub fn from_body(body: &str) -> CreateBuildError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConflictException" => CreateBuildError::Conflict(String::from(error_message)),
                    "InternalServiceException" => {
                        CreateBuildError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateBuildError::InvalidRequest(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateBuildError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateBuildError::Validation(error_message.to_string())
                    }
                    _ => CreateBuildError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateBuildError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateBuildError {
    fn from(err: serde_json::error::Error) -> CreateBuildError {
        CreateBuildError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBuildError {
    fn from(err: CredentialsError) -> CreateBuildError {
        CreateBuildError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBuildError {
    fn from(err: HttpDispatchError) -> CreateBuildError {
        CreateBuildError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBuildError {
    fn from(err: io::Error) -> CreateBuildError {
        CreateBuildError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBuildError {
    fn description(&self) -> &str {
        match *self {
            CreateBuildError::Conflict(ref cause) => cause,
            CreateBuildError::InternalService(ref cause) => cause,
            CreateBuildError::InvalidRequest(ref cause) => cause,
            CreateBuildError::Unauthorized(ref cause) => cause,
            CreateBuildError::Validation(ref cause) => cause,
            CreateBuildError::Credentials(ref err) => err.description(),
            CreateBuildError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBuildError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFleet
#[derive(Debug, PartialEq)]
pub enum CreateFleetError {
    ///<p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateFleetError {
    pub fn from_body(body: &str) -> CreateFleetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConflictException" => CreateFleetError::Conflict(String::from(error_message)),
                    "InternalServiceException" => {
                        CreateFleetError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateFleetError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateFleetError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => CreateFleetError::NotFound(String::from(error_message)),
                    "UnauthorizedException" => {
                        CreateFleetError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateFleetError::Validation(error_message.to_string())
                    }
                    _ => CreateFleetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateFleetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateFleetError {
    fn from(err: serde_json::error::Error) -> CreateFleetError {
        CreateFleetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateFleetError {
    fn from(err: CredentialsError) -> CreateFleetError {
        CreateFleetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateFleetError {
    fn from(err: HttpDispatchError) -> CreateFleetError {
        CreateFleetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateFleetError {
    fn from(err: io::Error) -> CreateFleetError {
        CreateFleetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFleetError {
    fn description(&self) -> &str {
        match *self {
            CreateFleetError::Conflict(ref cause) => cause,
            CreateFleetError::InternalService(ref cause) => cause,
            CreateFleetError::InvalidRequest(ref cause) => cause,
            CreateFleetError::LimitExceeded(ref cause) => cause,
            CreateFleetError::NotFound(ref cause) => cause,
            CreateFleetError::Unauthorized(ref cause) => cause,
            CreateFleetError::Validation(ref cause) => cause,
            CreateFleetError::Credentials(ref err) => err.description(),
            CreateFleetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGameSession
#[derive(Debug, PartialEq)]
pub enum CreateGameSessionError {
    ///<p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    ///<p>The specified fleet has no available instances to fulfill a <code>CreateGameSession</code> request. Clients can retry such requests immediately or after a waiting period.</p>
    FleetCapacityExceeded(String),
    ///<p>A game session with this custom ID string already exists in this fleet. Resolve this conflict before retrying this request.</p>
    IdempotentParameterMismatch(String),
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateGameSessionError {
    pub fn from_body(body: &str) -> CreateGameSessionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConflictException" => {
                        CreateGameSessionError::Conflict(String::from(error_message))
                    }
                    "FleetCapacityExceededException" => {
                        CreateGameSessionError::FleetCapacityExceeded(String::from(error_message))
                    }
                    "IdempotentParameterMismatchException" => CreateGameSessionError::IdempotentParameterMismatch(String::from(error_message)),
                    "InternalServiceException" => {
                        CreateGameSessionError::InternalService(String::from(error_message))
                    }
                    "InvalidFleetStatusException" => {
                        CreateGameSessionError::InvalidFleetStatus(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateGameSessionError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateGameSessionError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateGameSessionError::NotFound(String::from(error_message))
                    }
                    "TerminalRoutingStrategyException" => {
                        CreateGameSessionError::TerminalRoutingStrategy(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateGameSessionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateGameSessionError::Validation(error_message.to_string())
                    }
                    _ => CreateGameSessionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateGameSessionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateGameSessionError {
    fn from(err: serde_json::error::Error) -> CreateGameSessionError {
        CreateGameSessionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateGameSessionError {
    fn from(err: CredentialsError) -> CreateGameSessionError {
        CreateGameSessionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateGameSessionError {
    fn from(err: HttpDispatchError) -> CreateGameSessionError {
        CreateGameSessionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateGameSessionError {
    fn from(err: io::Error) -> CreateGameSessionError {
        CreateGameSessionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateGameSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGameSessionError {
    fn description(&self) -> &str {
        match *self {
            CreateGameSessionError::Conflict(ref cause) => cause,
            CreateGameSessionError::FleetCapacityExceeded(ref cause) => cause,
            CreateGameSessionError::IdempotentParameterMismatch(ref cause) => cause,
            CreateGameSessionError::InternalService(ref cause) => cause,
            CreateGameSessionError::InvalidFleetStatus(ref cause) => cause,
            CreateGameSessionError::InvalidRequest(ref cause) => cause,
            CreateGameSessionError::LimitExceeded(ref cause) => cause,
            CreateGameSessionError::NotFound(ref cause) => cause,
            CreateGameSessionError::TerminalRoutingStrategy(ref cause) => cause,
            CreateGameSessionError::Unauthorized(ref cause) => cause,
            CreateGameSessionError::Validation(ref cause) => cause,
            CreateGameSessionError::Credentials(ref err) => err.description(),
            CreateGameSessionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateGameSessionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGameSessionQueue
#[derive(Debug, PartialEq)]
pub enum CreateGameSessionQueueError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateGameSessionQueueError {
    pub fn from_body(body: &str) -> CreateGameSessionQueueError {
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
                        CreateGameSessionQueueError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateGameSessionQueueError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateGameSessionQueueError::LimitExceeded(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateGameSessionQueueError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateGameSessionQueueError::Validation(error_message.to_string())
                    }
                    _ => CreateGameSessionQueueError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateGameSessionQueueError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateGameSessionQueueError {
    fn from(err: serde_json::error::Error) -> CreateGameSessionQueueError {
        CreateGameSessionQueueError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateGameSessionQueueError {
    fn from(err: CredentialsError) -> CreateGameSessionQueueError {
        CreateGameSessionQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateGameSessionQueueError {
    fn from(err: HttpDispatchError) -> CreateGameSessionQueueError {
        CreateGameSessionQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateGameSessionQueueError {
    fn from(err: io::Error) -> CreateGameSessionQueueError {
        CreateGameSessionQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateGameSessionQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGameSessionQueueError {
    fn description(&self) -> &str {
        match *self {
            CreateGameSessionQueueError::InternalService(ref cause) => cause,
            CreateGameSessionQueueError::InvalidRequest(ref cause) => cause,
            CreateGameSessionQueueError::LimitExceeded(ref cause) => cause,
            CreateGameSessionQueueError::Unauthorized(ref cause) => cause,
            CreateGameSessionQueueError::Validation(ref cause) => cause,
            CreateGameSessionQueueError::Credentials(ref err) => err.description(),
            CreateGameSessionQueueError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateGameSessionQueueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateMatchmakingConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateMatchmakingConfigurationError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The requested operation is not supported in the region specified.</p>
    UnsupportedRegion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateMatchmakingConfigurationError {
    pub fn from_body(body: &str) -> CreateMatchmakingConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => CreateMatchmakingConfigurationError::InternalService(String::from(error_message)),
                    "InvalidRequestException" => CreateMatchmakingConfigurationError::InvalidRequest(String::from(error_message)),
                    "LimitExceededException" => CreateMatchmakingConfigurationError::LimitExceeded(String::from(error_message)),
                    "NotFoundException" => {
                        CreateMatchmakingConfigurationError::NotFound(String::from(error_message))
                    }
                    "UnsupportedRegionException" => CreateMatchmakingConfigurationError::UnsupportedRegion(String::from(error_message)),
                    "ValidationException" => {
                        CreateMatchmakingConfigurationError::Validation(error_message.to_string())
                    }
                    _ => CreateMatchmakingConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateMatchmakingConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateMatchmakingConfigurationError {
    fn from(err: serde_json::error::Error) -> CreateMatchmakingConfigurationError {
        CreateMatchmakingConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateMatchmakingConfigurationError {
    fn from(err: CredentialsError) -> CreateMatchmakingConfigurationError {
        CreateMatchmakingConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateMatchmakingConfigurationError {
    fn from(err: HttpDispatchError) -> CreateMatchmakingConfigurationError {
        CreateMatchmakingConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateMatchmakingConfigurationError {
    fn from(err: io::Error) -> CreateMatchmakingConfigurationError {
        CreateMatchmakingConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateMatchmakingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMatchmakingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            CreateMatchmakingConfigurationError::InternalService(ref cause) => cause,
            CreateMatchmakingConfigurationError::InvalidRequest(ref cause) => cause,
            CreateMatchmakingConfigurationError::LimitExceeded(ref cause) => cause,
            CreateMatchmakingConfigurationError::NotFound(ref cause) => cause,
            CreateMatchmakingConfigurationError::UnsupportedRegion(ref cause) => cause,
            CreateMatchmakingConfigurationError::Validation(ref cause) => cause,
            CreateMatchmakingConfigurationError::Credentials(ref err) => err.description(),
            CreateMatchmakingConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateMatchmakingConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateMatchmakingRuleSet
#[derive(Debug, PartialEq)]
pub enum CreateMatchmakingRuleSetError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation is not supported in the region specified.</p>
    UnsupportedRegion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateMatchmakingRuleSetError {
    pub fn from_body(body: &str) -> CreateMatchmakingRuleSetError {
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
                        CreateMatchmakingRuleSetError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateMatchmakingRuleSetError::InvalidRequest(String::from(error_message))
                    }
                    "UnsupportedRegionException" => CreateMatchmakingRuleSetError::UnsupportedRegion(String::from(error_message)),
                    "ValidationException" => {
                        CreateMatchmakingRuleSetError::Validation(error_message.to_string())
                    }
                    _ => CreateMatchmakingRuleSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateMatchmakingRuleSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateMatchmakingRuleSetError {
    fn from(err: serde_json::error::Error) -> CreateMatchmakingRuleSetError {
        CreateMatchmakingRuleSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateMatchmakingRuleSetError {
    fn from(err: CredentialsError) -> CreateMatchmakingRuleSetError {
        CreateMatchmakingRuleSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateMatchmakingRuleSetError {
    fn from(err: HttpDispatchError) -> CreateMatchmakingRuleSetError {
        CreateMatchmakingRuleSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateMatchmakingRuleSetError {
    fn from(err: io::Error) -> CreateMatchmakingRuleSetError {
        CreateMatchmakingRuleSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateMatchmakingRuleSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMatchmakingRuleSetError {
    fn description(&self) -> &str {
        match *self {
            CreateMatchmakingRuleSetError::InternalService(ref cause) => cause,
            CreateMatchmakingRuleSetError::InvalidRequest(ref cause) => cause,
            CreateMatchmakingRuleSetError::UnsupportedRegion(ref cause) => cause,
            CreateMatchmakingRuleSetError::Validation(ref cause) => cause,
            CreateMatchmakingRuleSetError::Credentials(ref err) => err.description(),
            CreateMatchmakingRuleSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateMatchmakingRuleSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePlayerSession
#[derive(Debug, PartialEq)]
pub enum CreatePlayerSessionError {
    ///<p>The game instance is currently full and cannot allow the requested player(s) to join. Clients can retry such requests immediately or after a waiting period.</p>
    GameSessionFull(String),
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the game instance. Resolve the conflict before retrying.</p>
    InvalidGameSessionStatus(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreatePlayerSessionError {
    pub fn from_body(body: &str) -> CreatePlayerSessionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "GameSessionFullException" => {
                        CreatePlayerSessionError::GameSessionFull(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreatePlayerSessionError::InternalService(String::from(error_message))
                    }
                    "InvalidGameSessionStatusException" => CreatePlayerSessionError::InvalidGameSessionStatus(String::from(error_message)),
                    "InvalidRequestException" => {
                        CreatePlayerSessionError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreatePlayerSessionError::NotFound(String::from(error_message))
                    }
                    "TerminalRoutingStrategyException" => CreatePlayerSessionError::TerminalRoutingStrategy(String::from(error_message)),
                    "UnauthorizedException" => {
                        CreatePlayerSessionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreatePlayerSessionError::Validation(error_message.to_string())
                    }
                    _ => CreatePlayerSessionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePlayerSessionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePlayerSessionError {
    fn from(err: serde_json::error::Error) -> CreatePlayerSessionError {
        CreatePlayerSessionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePlayerSessionError {
    fn from(err: CredentialsError) -> CreatePlayerSessionError {
        CreatePlayerSessionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePlayerSessionError {
    fn from(err: HttpDispatchError) -> CreatePlayerSessionError {
        CreatePlayerSessionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePlayerSessionError {
    fn from(err: io::Error) -> CreatePlayerSessionError {
        CreatePlayerSessionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePlayerSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePlayerSessionError {
    fn description(&self) -> &str {
        match *self {
            CreatePlayerSessionError::GameSessionFull(ref cause) => cause,
            CreatePlayerSessionError::InternalService(ref cause) => cause,
            CreatePlayerSessionError::InvalidGameSessionStatus(ref cause) => cause,
            CreatePlayerSessionError::InvalidRequest(ref cause) => cause,
            CreatePlayerSessionError::NotFound(ref cause) => cause,
            CreatePlayerSessionError::TerminalRoutingStrategy(ref cause) => cause,
            CreatePlayerSessionError::Unauthorized(ref cause) => cause,
            CreatePlayerSessionError::Validation(ref cause) => cause,
            CreatePlayerSessionError::Credentials(ref err) => err.description(),
            CreatePlayerSessionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePlayerSessionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePlayerSessions
#[derive(Debug, PartialEq)]
pub enum CreatePlayerSessionsError {
    ///<p>The game instance is currently full and cannot allow the requested player(s) to join. Clients can retry such requests immediately or after a waiting period.</p>
    GameSessionFull(String),
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the game instance. Resolve the conflict before retrying.</p>
    InvalidGameSessionStatus(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreatePlayerSessionsError {
    pub fn from_body(body: &str) -> CreatePlayerSessionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "GameSessionFullException" => {
                        CreatePlayerSessionsError::GameSessionFull(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreatePlayerSessionsError::InternalService(String::from(error_message))
                    }
                    "InvalidGameSessionStatusException" => CreatePlayerSessionsError::InvalidGameSessionStatus(String::from(error_message)),
                    "InvalidRequestException" => {
                        CreatePlayerSessionsError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreatePlayerSessionsError::NotFound(String::from(error_message))
                    }
                    "TerminalRoutingStrategyException" => CreatePlayerSessionsError::TerminalRoutingStrategy(String::from(error_message)),
                    "UnauthorizedException" => {
                        CreatePlayerSessionsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreatePlayerSessionsError::Validation(error_message.to_string())
                    }
                    _ => CreatePlayerSessionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePlayerSessionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePlayerSessionsError {
    fn from(err: serde_json::error::Error) -> CreatePlayerSessionsError {
        CreatePlayerSessionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePlayerSessionsError {
    fn from(err: CredentialsError) -> CreatePlayerSessionsError {
        CreatePlayerSessionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePlayerSessionsError {
    fn from(err: HttpDispatchError) -> CreatePlayerSessionsError {
        CreatePlayerSessionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePlayerSessionsError {
    fn from(err: io::Error) -> CreatePlayerSessionsError {
        CreatePlayerSessionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePlayerSessionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePlayerSessionsError {
    fn description(&self) -> &str {
        match *self {
            CreatePlayerSessionsError::GameSessionFull(ref cause) => cause,
            CreatePlayerSessionsError::InternalService(ref cause) => cause,
            CreatePlayerSessionsError::InvalidGameSessionStatus(ref cause) => cause,
            CreatePlayerSessionsError::InvalidRequest(ref cause) => cause,
            CreatePlayerSessionsError::NotFound(ref cause) => cause,
            CreatePlayerSessionsError::TerminalRoutingStrategy(ref cause) => cause,
            CreatePlayerSessionsError::Unauthorized(ref cause) => cause,
            CreatePlayerSessionsError::Validation(ref cause) => cause,
            CreatePlayerSessionsError::Credentials(ref err) => err.description(),
            CreatePlayerSessionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePlayerSessionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAlias
#[derive(Debug, PartialEq)]
pub enum DeleteAliasError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteAliasError {
    pub fn from_body(body: &str) -> DeleteAliasError {
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
                        DeleteAliasError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteAliasError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => DeleteAliasError::NotFound(String::from(error_message)),
                    "UnauthorizedException" => {
                        DeleteAliasError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteAliasError::Validation(error_message.to_string())
                    }
                    _ => DeleteAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAliasError {
    fn from(err: serde_json::error::Error) -> DeleteAliasError {
        DeleteAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAliasError {
    fn from(err: CredentialsError) -> DeleteAliasError {
        DeleteAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAliasError {
    fn from(err: HttpDispatchError) -> DeleteAliasError {
        DeleteAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAliasError {
    fn from(err: io::Error) -> DeleteAliasError {
        DeleteAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAliasError {
    fn description(&self) -> &str {
        match *self {
            DeleteAliasError::InternalService(ref cause) => cause,
            DeleteAliasError::InvalidRequest(ref cause) => cause,
            DeleteAliasError::NotFound(ref cause) => cause,
            DeleteAliasError::Unauthorized(ref cause) => cause,
            DeleteAliasError::Validation(ref cause) => cause,
            DeleteAliasError::Credentials(ref err) => err.description(),
            DeleteAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBuild
#[derive(Debug, PartialEq)]
pub enum DeleteBuildError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBuildError {
    pub fn from_body(body: &str) -> DeleteBuildError {
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
                        DeleteBuildError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteBuildError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => DeleteBuildError::NotFound(String::from(error_message)),
                    "UnauthorizedException" => {
                        DeleteBuildError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteBuildError::Validation(error_message.to_string())
                    }
                    _ => DeleteBuildError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBuildError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteBuildError {
    fn from(err: serde_json::error::Error) -> DeleteBuildError {
        DeleteBuildError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBuildError {
    fn from(err: CredentialsError) -> DeleteBuildError {
        DeleteBuildError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBuildError {
    fn from(err: HttpDispatchError) -> DeleteBuildError {
        DeleteBuildError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBuildError {
    fn from(err: io::Error) -> DeleteBuildError {
        DeleteBuildError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBuildError {
    fn description(&self) -> &str {
        match *self {
            DeleteBuildError::InternalService(ref cause) => cause,
            DeleteBuildError::InvalidRequest(ref cause) => cause,
            DeleteBuildError::NotFound(ref cause) => cause,
            DeleteBuildError::Unauthorized(ref cause) => cause,
            DeleteBuildError::Validation(ref cause) => cause,
            DeleteBuildError::Credentials(ref err) => err.description(),
            DeleteBuildError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBuildError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFleet
#[derive(Debug, PartialEq)]
pub enum DeleteFleetError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteFleetError {
    pub fn from_body(body: &str) -> DeleteFleetError {
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
                        DeleteFleetError::InternalService(String::from(error_message))
                    }
                    "InvalidFleetStatusException" => {
                        DeleteFleetError::InvalidFleetStatus(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteFleetError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => DeleteFleetError::NotFound(String::from(error_message)),
                    "UnauthorizedException" => {
                        DeleteFleetError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteFleetError::Validation(error_message.to_string())
                    }
                    _ => DeleteFleetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteFleetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteFleetError {
    fn from(err: serde_json::error::Error) -> DeleteFleetError {
        DeleteFleetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFleetError {
    fn from(err: CredentialsError) -> DeleteFleetError {
        DeleteFleetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFleetError {
    fn from(err: HttpDispatchError) -> DeleteFleetError {
        DeleteFleetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFleetError {
    fn from(err: io::Error) -> DeleteFleetError {
        DeleteFleetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFleetError {
    fn description(&self) -> &str {
        match *self {
            DeleteFleetError::InternalService(ref cause) => cause,
            DeleteFleetError::InvalidFleetStatus(ref cause) => cause,
            DeleteFleetError::InvalidRequest(ref cause) => cause,
            DeleteFleetError::NotFound(ref cause) => cause,
            DeleteFleetError::Unauthorized(ref cause) => cause,
            DeleteFleetError::Validation(ref cause) => cause,
            DeleteFleetError::Credentials(ref err) => err.description(),
            DeleteFleetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteGameSessionQueue
#[derive(Debug, PartialEq)]
pub enum DeleteGameSessionQueueError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteGameSessionQueueError {
    pub fn from_body(body: &str) -> DeleteGameSessionQueueError {
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
                        DeleteGameSessionQueueError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteGameSessionQueueError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteGameSessionQueueError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteGameSessionQueueError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteGameSessionQueueError::Validation(error_message.to_string())
                    }
                    _ => DeleteGameSessionQueueError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteGameSessionQueueError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteGameSessionQueueError {
    fn from(err: serde_json::error::Error) -> DeleteGameSessionQueueError {
        DeleteGameSessionQueueError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteGameSessionQueueError {
    fn from(err: CredentialsError) -> DeleteGameSessionQueueError {
        DeleteGameSessionQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteGameSessionQueueError {
    fn from(err: HttpDispatchError) -> DeleteGameSessionQueueError {
        DeleteGameSessionQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteGameSessionQueueError {
    fn from(err: io::Error) -> DeleteGameSessionQueueError {
        DeleteGameSessionQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteGameSessionQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGameSessionQueueError {
    fn description(&self) -> &str {
        match *self {
            DeleteGameSessionQueueError::InternalService(ref cause) => cause,
            DeleteGameSessionQueueError::InvalidRequest(ref cause) => cause,
            DeleteGameSessionQueueError::NotFound(ref cause) => cause,
            DeleteGameSessionQueueError::Unauthorized(ref cause) => cause,
            DeleteGameSessionQueueError::Validation(ref cause) => cause,
            DeleteGameSessionQueueError::Credentials(ref err) => err.description(),
            DeleteGameSessionQueueError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteGameSessionQueueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMatchmakingConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteMatchmakingConfigurationError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The requested operation is not supported in the region specified.</p>
    UnsupportedRegion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteMatchmakingConfigurationError {
    pub fn from_body(body: &str) -> DeleteMatchmakingConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => DeleteMatchmakingConfigurationError::InternalService(String::from(error_message)),
                    "InvalidRequestException" => DeleteMatchmakingConfigurationError::InvalidRequest(String::from(error_message)),
                    "NotFoundException" => {
                        DeleteMatchmakingConfigurationError::NotFound(String::from(error_message))
                    }
                    "UnsupportedRegionException" => DeleteMatchmakingConfigurationError::UnsupportedRegion(String::from(error_message)),
                    "ValidationException" => {
                        DeleteMatchmakingConfigurationError::Validation(error_message.to_string())
                    }
                    _ => DeleteMatchmakingConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteMatchmakingConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteMatchmakingConfigurationError {
    fn from(err: serde_json::error::Error) -> DeleteMatchmakingConfigurationError {
        DeleteMatchmakingConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteMatchmakingConfigurationError {
    fn from(err: CredentialsError) -> DeleteMatchmakingConfigurationError {
        DeleteMatchmakingConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteMatchmakingConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteMatchmakingConfigurationError {
        DeleteMatchmakingConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteMatchmakingConfigurationError {
    fn from(err: io::Error) -> DeleteMatchmakingConfigurationError {
        DeleteMatchmakingConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteMatchmakingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMatchmakingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteMatchmakingConfigurationError::InternalService(ref cause) => cause,
            DeleteMatchmakingConfigurationError::InvalidRequest(ref cause) => cause,
            DeleteMatchmakingConfigurationError::NotFound(ref cause) => cause,
            DeleteMatchmakingConfigurationError::UnsupportedRegion(ref cause) => cause,
            DeleteMatchmakingConfigurationError::Validation(ref cause) => cause,
            DeleteMatchmakingConfigurationError::Credentials(ref err) => err.description(),
            DeleteMatchmakingConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteMatchmakingConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteScalingPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteScalingPolicyError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteScalingPolicyError {
    pub fn from_body(body: &str) -> DeleteScalingPolicyError {
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
                        DeleteScalingPolicyError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteScalingPolicyError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteScalingPolicyError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteScalingPolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteScalingPolicyError::Validation(error_message.to_string())
                    }
                    _ => DeleteScalingPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteScalingPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteScalingPolicyError {
    fn from(err: serde_json::error::Error) -> DeleteScalingPolicyError {
        DeleteScalingPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteScalingPolicyError {
    fn from(err: CredentialsError) -> DeleteScalingPolicyError {
        DeleteScalingPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteScalingPolicyError {
    fn from(err: HttpDispatchError) -> DeleteScalingPolicyError {
        DeleteScalingPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteScalingPolicyError {
    fn from(err: io::Error) -> DeleteScalingPolicyError {
        DeleteScalingPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteScalingPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteScalingPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteScalingPolicyError::InternalService(ref cause) => cause,
            DeleteScalingPolicyError::InvalidRequest(ref cause) => cause,
            DeleteScalingPolicyError::NotFound(ref cause) => cause,
            DeleteScalingPolicyError::Unauthorized(ref cause) => cause,
            DeleteScalingPolicyError::Validation(ref cause) => cause,
            DeleteScalingPolicyError::Credentials(ref err) => err.description(),
            DeleteScalingPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteScalingPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAlias
#[derive(Debug, PartialEq)]
pub enum DescribeAliasError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeAliasError {
    pub fn from_body(body: &str) -> DescribeAliasError {
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
                        DescribeAliasError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeAliasError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeAliasError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeAliasError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAliasError::Validation(error_message.to_string())
                    }
                    _ => DescribeAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAliasError {
    fn from(err: serde_json::error::Error) -> DescribeAliasError {
        DescribeAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAliasError {
    fn from(err: CredentialsError) -> DescribeAliasError {
        DescribeAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAliasError {
    fn from(err: HttpDispatchError) -> DescribeAliasError {
        DescribeAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAliasError {
    fn from(err: io::Error) -> DescribeAliasError {
        DescribeAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAliasError {
    fn description(&self) -> &str {
        match *self {
            DescribeAliasError::InternalService(ref cause) => cause,
            DescribeAliasError::InvalidRequest(ref cause) => cause,
            DescribeAliasError::NotFound(ref cause) => cause,
            DescribeAliasError::Unauthorized(ref cause) => cause,
            DescribeAliasError::Validation(ref cause) => cause,
            DescribeAliasError::Credentials(ref err) => err.description(),
            DescribeAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBuild
#[derive(Debug, PartialEq)]
pub enum DescribeBuildError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeBuildError {
    pub fn from_body(body: &str) -> DescribeBuildError {
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
                        DescribeBuildError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeBuildError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeBuildError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeBuildError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeBuildError::Validation(error_message.to_string())
                    }
                    _ => DescribeBuildError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeBuildError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeBuildError {
    fn from(err: serde_json::error::Error) -> DescribeBuildError {
        DescribeBuildError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBuildError {
    fn from(err: CredentialsError) -> DescribeBuildError {
        DescribeBuildError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBuildError {
    fn from(err: HttpDispatchError) -> DescribeBuildError {
        DescribeBuildError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBuildError {
    fn from(err: io::Error) -> DescribeBuildError {
        DescribeBuildError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBuildError {
    fn description(&self) -> &str {
        match *self {
            DescribeBuildError::InternalService(ref cause) => cause,
            DescribeBuildError::InvalidRequest(ref cause) => cause,
            DescribeBuildError::NotFound(ref cause) => cause,
            DescribeBuildError::Unauthorized(ref cause) => cause,
            DescribeBuildError::Validation(ref cause) => cause,
            DescribeBuildError::Credentials(ref err) => err.description(),
            DescribeBuildError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeBuildError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEC2InstanceLimits
#[derive(Debug, PartialEq)]
pub enum DescribeEC2InstanceLimitsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEC2InstanceLimitsError {
    pub fn from_body(body: &str) -> DescribeEC2InstanceLimitsError {
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
                        DescribeEC2InstanceLimitsError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeEC2InstanceLimitsError::InvalidRequest(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeEC2InstanceLimitsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEC2InstanceLimitsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEC2InstanceLimitsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEC2InstanceLimitsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEC2InstanceLimitsError {
    fn from(err: serde_json::error::Error) -> DescribeEC2InstanceLimitsError {
        DescribeEC2InstanceLimitsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEC2InstanceLimitsError {
    fn from(err: CredentialsError) -> DescribeEC2InstanceLimitsError {
        DescribeEC2InstanceLimitsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEC2InstanceLimitsError {
    fn from(err: HttpDispatchError) -> DescribeEC2InstanceLimitsError {
        DescribeEC2InstanceLimitsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEC2InstanceLimitsError {
    fn from(err: io::Error) -> DescribeEC2InstanceLimitsError {
        DescribeEC2InstanceLimitsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEC2InstanceLimitsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEC2InstanceLimitsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEC2InstanceLimitsError::InternalService(ref cause) => cause,
            DescribeEC2InstanceLimitsError::InvalidRequest(ref cause) => cause,
            DescribeEC2InstanceLimitsError::Unauthorized(ref cause) => cause,
            DescribeEC2InstanceLimitsError::Validation(ref cause) => cause,
            DescribeEC2InstanceLimitsError::Credentials(ref err) => err.description(),
            DescribeEC2InstanceLimitsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEC2InstanceLimitsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFleetAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeFleetAttributesError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeFleetAttributesError {
    pub fn from_body(body: &str) -> DescribeFleetAttributesError {
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
                        DescribeFleetAttributesError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeFleetAttributesError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeFleetAttributesError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeFleetAttributesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeFleetAttributesError::Validation(error_message.to_string())
                    }
                    _ => DescribeFleetAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeFleetAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeFleetAttributesError {
    fn from(err: serde_json::error::Error) -> DescribeFleetAttributesError {
        DescribeFleetAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeFleetAttributesError {
    fn from(err: CredentialsError) -> DescribeFleetAttributesError {
        DescribeFleetAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeFleetAttributesError {
    fn from(err: HttpDispatchError) -> DescribeFleetAttributesError {
        DescribeFleetAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeFleetAttributesError {
    fn from(err: io::Error) -> DescribeFleetAttributesError {
        DescribeFleetAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeFleetAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFleetAttributesError {
    fn description(&self) -> &str {
        match *self {
            DescribeFleetAttributesError::InternalService(ref cause) => cause,
            DescribeFleetAttributesError::InvalidRequest(ref cause) => cause,
            DescribeFleetAttributesError::NotFound(ref cause) => cause,
            DescribeFleetAttributesError::Unauthorized(ref cause) => cause,
            DescribeFleetAttributesError::Validation(ref cause) => cause,
            DescribeFleetAttributesError::Credentials(ref err) => err.description(),
            DescribeFleetAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeFleetAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFleetCapacity
#[derive(Debug, PartialEq)]
pub enum DescribeFleetCapacityError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeFleetCapacityError {
    pub fn from_body(body: &str) -> DescribeFleetCapacityError {
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
                        DescribeFleetCapacityError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeFleetCapacityError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeFleetCapacityError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeFleetCapacityError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeFleetCapacityError::Validation(error_message.to_string())
                    }
                    _ => DescribeFleetCapacityError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeFleetCapacityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeFleetCapacityError {
    fn from(err: serde_json::error::Error) -> DescribeFleetCapacityError {
        DescribeFleetCapacityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeFleetCapacityError {
    fn from(err: CredentialsError) -> DescribeFleetCapacityError {
        DescribeFleetCapacityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeFleetCapacityError {
    fn from(err: HttpDispatchError) -> DescribeFleetCapacityError {
        DescribeFleetCapacityError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeFleetCapacityError {
    fn from(err: io::Error) -> DescribeFleetCapacityError {
        DescribeFleetCapacityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeFleetCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFleetCapacityError {
    fn description(&self) -> &str {
        match *self {
            DescribeFleetCapacityError::InternalService(ref cause) => cause,
            DescribeFleetCapacityError::InvalidRequest(ref cause) => cause,
            DescribeFleetCapacityError::NotFound(ref cause) => cause,
            DescribeFleetCapacityError::Unauthorized(ref cause) => cause,
            DescribeFleetCapacityError::Validation(ref cause) => cause,
            DescribeFleetCapacityError::Credentials(ref err) => err.description(),
            DescribeFleetCapacityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeFleetCapacityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFleetEvents
#[derive(Debug, PartialEq)]
pub enum DescribeFleetEventsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeFleetEventsError {
    pub fn from_body(body: &str) -> DescribeFleetEventsError {
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
                        DescribeFleetEventsError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeFleetEventsError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeFleetEventsError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeFleetEventsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeFleetEventsError::Validation(error_message.to_string())
                    }
                    _ => DescribeFleetEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeFleetEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeFleetEventsError {
    fn from(err: serde_json::error::Error) -> DescribeFleetEventsError {
        DescribeFleetEventsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeFleetEventsError {
    fn from(err: CredentialsError) -> DescribeFleetEventsError {
        DescribeFleetEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeFleetEventsError {
    fn from(err: HttpDispatchError) -> DescribeFleetEventsError {
        DescribeFleetEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeFleetEventsError {
    fn from(err: io::Error) -> DescribeFleetEventsError {
        DescribeFleetEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeFleetEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFleetEventsError {
    fn description(&self) -> &str {
        match *self {
            DescribeFleetEventsError::InternalService(ref cause) => cause,
            DescribeFleetEventsError::InvalidRequest(ref cause) => cause,
            DescribeFleetEventsError::NotFound(ref cause) => cause,
            DescribeFleetEventsError::Unauthorized(ref cause) => cause,
            DescribeFleetEventsError::Validation(ref cause) => cause,
            DescribeFleetEventsError::Credentials(ref err) => err.description(),
            DescribeFleetEventsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeFleetEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFleetPortSettings
#[derive(Debug, PartialEq)]
pub enum DescribeFleetPortSettingsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeFleetPortSettingsError {
    pub fn from_body(body: &str) -> DescribeFleetPortSettingsError {
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
                        DescribeFleetPortSettingsError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeFleetPortSettingsError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeFleetPortSettingsError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeFleetPortSettingsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeFleetPortSettingsError::Validation(error_message.to_string())
                    }
                    _ => DescribeFleetPortSettingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeFleetPortSettingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeFleetPortSettingsError {
    fn from(err: serde_json::error::Error) -> DescribeFleetPortSettingsError {
        DescribeFleetPortSettingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeFleetPortSettingsError {
    fn from(err: CredentialsError) -> DescribeFleetPortSettingsError {
        DescribeFleetPortSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeFleetPortSettingsError {
    fn from(err: HttpDispatchError) -> DescribeFleetPortSettingsError {
        DescribeFleetPortSettingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeFleetPortSettingsError {
    fn from(err: io::Error) -> DescribeFleetPortSettingsError {
        DescribeFleetPortSettingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeFleetPortSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFleetPortSettingsError {
    fn description(&self) -> &str {
        match *self {
            DescribeFleetPortSettingsError::InternalService(ref cause) => cause,
            DescribeFleetPortSettingsError::InvalidRequest(ref cause) => cause,
            DescribeFleetPortSettingsError::NotFound(ref cause) => cause,
            DescribeFleetPortSettingsError::Unauthorized(ref cause) => cause,
            DescribeFleetPortSettingsError::Validation(ref cause) => cause,
            DescribeFleetPortSettingsError::Credentials(ref err) => err.description(),
            DescribeFleetPortSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeFleetPortSettingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFleetUtilization
#[derive(Debug, PartialEq)]
pub enum DescribeFleetUtilizationError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeFleetUtilizationError {
    pub fn from_body(body: &str) -> DescribeFleetUtilizationError {
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
                        DescribeFleetUtilizationError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeFleetUtilizationError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeFleetUtilizationError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeFleetUtilizationError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeFleetUtilizationError::Validation(error_message.to_string())
                    }
                    _ => DescribeFleetUtilizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeFleetUtilizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeFleetUtilizationError {
    fn from(err: serde_json::error::Error) -> DescribeFleetUtilizationError {
        DescribeFleetUtilizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeFleetUtilizationError {
    fn from(err: CredentialsError) -> DescribeFleetUtilizationError {
        DescribeFleetUtilizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeFleetUtilizationError {
    fn from(err: HttpDispatchError) -> DescribeFleetUtilizationError {
        DescribeFleetUtilizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeFleetUtilizationError {
    fn from(err: io::Error) -> DescribeFleetUtilizationError {
        DescribeFleetUtilizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeFleetUtilizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFleetUtilizationError {
    fn description(&self) -> &str {
        match *self {
            DescribeFleetUtilizationError::InternalService(ref cause) => cause,
            DescribeFleetUtilizationError::InvalidRequest(ref cause) => cause,
            DescribeFleetUtilizationError::NotFound(ref cause) => cause,
            DescribeFleetUtilizationError::Unauthorized(ref cause) => cause,
            DescribeFleetUtilizationError::Validation(ref cause) => cause,
            DescribeFleetUtilizationError::Credentials(ref err) => err.description(),
            DescribeFleetUtilizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeFleetUtilizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeGameSessionDetails
#[derive(Debug, PartialEq)]
pub enum DescribeGameSessionDetailsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeGameSessionDetailsError {
    pub fn from_body(body: &str) -> DescribeGameSessionDetailsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => DescribeGameSessionDetailsError::InternalService(String::from(error_message)),
                    "InvalidRequestException" => {
                        DescribeGameSessionDetailsError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeGameSessionDetailsError::NotFound(String::from(error_message))
                    }
                    "TerminalRoutingStrategyException" => DescribeGameSessionDetailsError::TerminalRoutingStrategy(String::from(error_message)),
                    "UnauthorizedException" => {
                        DescribeGameSessionDetailsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeGameSessionDetailsError::Validation(error_message.to_string())
                    }
                    _ => DescribeGameSessionDetailsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeGameSessionDetailsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeGameSessionDetailsError {
    fn from(err: serde_json::error::Error) -> DescribeGameSessionDetailsError {
        DescribeGameSessionDetailsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeGameSessionDetailsError {
    fn from(err: CredentialsError) -> DescribeGameSessionDetailsError {
        DescribeGameSessionDetailsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeGameSessionDetailsError {
    fn from(err: HttpDispatchError) -> DescribeGameSessionDetailsError {
        DescribeGameSessionDetailsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeGameSessionDetailsError {
    fn from(err: io::Error) -> DescribeGameSessionDetailsError {
        DescribeGameSessionDetailsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeGameSessionDetailsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeGameSessionDetailsError {
    fn description(&self) -> &str {
        match *self {
            DescribeGameSessionDetailsError::InternalService(ref cause) => cause,
            DescribeGameSessionDetailsError::InvalidRequest(ref cause) => cause,
            DescribeGameSessionDetailsError::NotFound(ref cause) => cause,
            DescribeGameSessionDetailsError::TerminalRoutingStrategy(ref cause) => cause,
            DescribeGameSessionDetailsError::Unauthorized(ref cause) => cause,
            DescribeGameSessionDetailsError::Validation(ref cause) => cause,
            DescribeGameSessionDetailsError::Credentials(ref err) => err.description(),
            DescribeGameSessionDetailsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeGameSessionDetailsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeGameSessionPlacement
#[derive(Debug, PartialEq)]
pub enum DescribeGameSessionPlacementError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeGameSessionPlacementError {
    pub fn from_body(body: &str) -> DescribeGameSessionPlacementError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => DescribeGameSessionPlacementError::InternalService(String::from(error_message)),
                    "InvalidRequestException" => DescribeGameSessionPlacementError::InvalidRequest(String::from(error_message)),
                    "NotFoundException" => {
                        DescribeGameSessionPlacementError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeGameSessionPlacementError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeGameSessionPlacementError::Validation(error_message.to_string())
                    }
                    _ => DescribeGameSessionPlacementError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeGameSessionPlacementError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeGameSessionPlacementError {
    fn from(err: serde_json::error::Error) -> DescribeGameSessionPlacementError {
        DescribeGameSessionPlacementError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeGameSessionPlacementError {
    fn from(err: CredentialsError) -> DescribeGameSessionPlacementError {
        DescribeGameSessionPlacementError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeGameSessionPlacementError {
    fn from(err: HttpDispatchError) -> DescribeGameSessionPlacementError {
        DescribeGameSessionPlacementError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeGameSessionPlacementError {
    fn from(err: io::Error) -> DescribeGameSessionPlacementError {
        DescribeGameSessionPlacementError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeGameSessionPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeGameSessionPlacementError {
    fn description(&self) -> &str {
        match *self {
            DescribeGameSessionPlacementError::InternalService(ref cause) => cause,
            DescribeGameSessionPlacementError::InvalidRequest(ref cause) => cause,
            DescribeGameSessionPlacementError::NotFound(ref cause) => cause,
            DescribeGameSessionPlacementError::Unauthorized(ref cause) => cause,
            DescribeGameSessionPlacementError::Validation(ref cause) => cause,
            DescribeGameSessionPlacementError::Credentials(ref err) => err.description(),
            DescribeGameSessionPlacementError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeGameSessionPlacementError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeGameSessionQueues
#[derive(Debug, PartialEq)]
pub enum DescribeGameSessionQueuesError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeGameSessionQueuesError {
    pub fn from_body(body: &str) -> DescribeGameSessionQueuesError {
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
                        DescribeGameSessionQueuesError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeGameSessionQueuesError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeGameSessionQueuesError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeGameSessionQueuesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeGameSessionQueuesError::Validation(error_message.to_string())
                    }
                    _ => DescribeGameSessionQueuesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeGameSessionQueuesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeGameSessionQueuesError {
    fn from(err: serde_json::error::Error) -> DescribeGameSessionQueuesError {
        DescribeGameSessionQueuesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeGameSessionQueuesError {
    fn from(err: CredentialsError) -> DescribeGameSessionQueuesError {
        DescribeGameSessionQueuesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeGameSessionQueuesError {
    fn from(err: HttpDispatchError) -> DescribeGameSessionQueuesError {
        DescribeGameSessionQueuesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeGameSessionQueuesError {
    fn from(err: io::Error) -> DescribeGameSessionQueuesError {
        DescribeGameSessionQueuesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeGameSessionQueuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeGameSessionQueuesError {
    fn description(&self) -> &str {
        match *self {
            DescribeGameSessionQueuesError::InternalService(ref cause) => cause,
            DescribeGameSessionQueuesError::InvalidRequest(ref cause) => cause,
            DescribeGameSessionQueuesError::NotFound(ref cause) => cause,
            DescribeGameSessionQueuesError::Unauthorized(ref cause) => cause,
            DescribeGameSessionQueuesError::Validation(ref cause) => cause,
            DescribeGameSessionQueuesError::Credentials(ref err) => err.description(),
            DescribeGameSessionQueuesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeGameSessionQueuesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeGameSessions
#[derive(Debug, PartialEq)]
pub enum DescribeGameSessionsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeGameSessionsError {
    pub fn from_body(body: &str) -> DescribeGameSessionsError {
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
                        DescribeGameSessionsError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeGameSessionsError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeGameSessionsError::NotFound(String::from(error_message))
                    }
                    "TerminalRoutingStrategyException" => DescribeGameSessionsError::TerminalRoutingStrategy(String::from(error_message)),
                    "UnauthorizedException" => {
                        DescribeGameSessionsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeGameSessionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeGameSessionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeGameSessionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeGameSessionsError {
    fn from(err: serde_json::error::Error) -> DescribeGameSessionsError {
        DescribeGameSessionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeGameSessionsError {
    fn from(err: CredentialsError) -> DescribeGameSessionsError {
        DescribeGameSessionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeGameSessionsError {
    fn from(err: HttpDispatchError) -> DescribeGameSessionsError {
        DescribeGameSessionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeGameSessionsError {
    fn from(err: io::Error) -> DescribeGameSessionsError {
        DescribeGameSessionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeGameSessionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeGameSessionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeGameSessionsError::InternalService(ref cause) => cause,
            DescribeGameSessionsError::InvalidRequest(ref cause) => cause,
            DescribeGameSessionsError::NotFound(ref cause) => cause,
            DescribeGameSessionsError::TerminalRoutingStrategy(ref cause) => cause,
            DescribeGameSessionsError::Unauthorized(ref cause) => cause,
            DescribeGameSessionsError::Validation(ref cause) => cause,
            DescribeGameSessionsError::Credentials(ref err) => err.description(),
            DescribeGameSessionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeGameSessionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstances
#[derive(Debug, PartialEq)]
pub enum DescribeInstancesError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeInstancesError {
    pub fn from_body(body: &str) -> DescribeInstancesError {
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
                        DescribeInstancesError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeInstancesError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeInstancesError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeInstancesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeInstancesError::Validation(error_message.to_string())
                    }
                    _ => DescribeInstancesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeInstancesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeInstancesError {
    fn from(err: serde_json::error::Error) -> DescribeInstancesError {
        DescribeInstancesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeInstancesError {
    fn from(err: CredentialsError) -> DescribeInstancesError {
        DescribeInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInstancesError {
    fn from(err: HttpDispatchError) -> DescribeInstancesError {
        DescribeInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInstancesError {
    fn from(err: io::Error) -> DescribeInstancesError {
        DescribeInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstancesError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstancesError::InternalService(ref cause) => cause,
            DescribeInstancesError::InvalidRequest(ref cause) => cause,
            DescribeInstancesError::NotFound(ref cause) => cause,
            DescribeInstancesError::Unauthorized(ref cause) => cause,
            DescribeInstancesError::Validation(ref cause) => cause,
            DescribeInstancesError::Credentials(ref err) => err.description(),
            DescribeInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMatchmaking
#[derive(Debug, PartialEq)]
pub enum DescribeMatchmakingError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation is not supported in the region specified.</p>
    UnsupportedRegion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeMatchmakingError {
    pub fn from_body(body: &str) -> DescribeMatchmakingError {
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
                        DescribeMatchmakingError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeMatchmakingError::InvalidRequest(String::from(error_message))
                    }
                    "UnsupportedRegionException" => {
                        DescribeMatchmakingError::UnsupportedRegion(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeMatchmakingError::Validation(error_message.to_string())
                    }
                    _ => DescribeMatchmakingError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMatchmakingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMatchmakingError {
    fn from(err: serde_json::error::Error) -> DescribeMatchmakingError {
        DescribeMatchmakingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMatchmakingError {
    fn from(err: CredentialsError) -> DescribeMatchmakingError {
        DescribeMatchmakingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMatchmakingError {
    fn from(err: HttpDispatchError) -> DescribeMatchmakingError {
        DescribeMatchmakingError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMatchmakingError {
    fn from(err: io::Error) -> DescribeMatchmakingError {
        DescribeMatchmakingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMatchmakingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMatchmakingError {
    fn description(&self) -> &str {
        match *self {
            DescribeMatchmakingError::InternalService(ref cause) => cause,
            DescribeMatchmakingError::InvalidRequest(ref cause) => cause,
            DescribeMatchmakingError::UnsupportedRegion(ref cause) => cause,
            DescribeMatchmakingError::Validation(ref cause) => cause,
            DescribeMatchmakingError::Credentials(ref err) => err.description(),
            DescribeMatchmakingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMatchmakingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMatchmakingConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeMatchmakingConfigurationsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation is not supported in the region specified.</p>
    UnsupportedRegion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeMatchmakingConfigurationsError {
    pub fn from_body(body: &str) -> DescribeMatchmakingConfigurationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => DescribeMatchmakingConfigurationsError::InternalService(String::from(error_message)),
                    "InvalidRequestException" => DescribeMatchmakingConfigurationsError::InvalidRequest(String::from(error_message)),
                    "UnsupportedRegionException" => DescribeMatchmakingConfigurationsError::UnsupportedRegion(String::from(error_message)),
                    "ValidationException" => {
                        DescribeMatchmakingConfigurationsError::Validation(error_message
                                                                               .to_string())
                    }
                    _ => DescribeMatchmakingConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMatchmakingConfigurationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMatchmakingConfigurationsError {
    fn from(err: serde_json::error::Error) -> DescribeMatchmakingConfigurationsError {
        DescribeMatchmakingConfigurationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMatchmakingConfigurationsError {
    fn from(err: CredentialsError) -> DescribeMatchmakingConfigurationsError {
        DescribeMatchmakingConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMatchmakingConfigurationsError {
    fn from(err: HttpDispatchError) -> DescribeMatchmakingConfigurationsError {
        DescribeMatchmakingConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMatchmakingConfigurationsError {
    fn from(err: io::Error) -> DescribeMatchmakingConfigurationsError {
        DescribeMatchmakingConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMatchmakingConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMatchmakingConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMatchmakingConfigurationsError::InternalService(ref cause) => cause,
            DescribeMatchmakingConfigurationsError::InvalidRequest(ref cause) => cause,
            DescribeMatchmakingConfigurationsError::UnsupportedRegion(ref cause) => cause,
            DescribeMatchmakingConfigurationsError::Validation(ref cause) => cause,
            DescribeMatchmakingConfigurationsError::Credentials(ref err) => err.description(),
            DescribeMatchmakingConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMatchmakingConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMatchmakingRuleSets
#[derive(Debug, PartialEq)]
pub enum DescribeMatchmakingRuleSetsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The requested operation is not supported in the region specified.</p>
    UnsupportedRegion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeMatchmakingRuleSetsError {
    pub fn from_body(body: &str) -> DescribeMatchmakingRuleSetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => DescribeMatchmakingRuleSetsError::InternalService(String::from(error_message)),
                    "InvalidRequestException" => DescribeMatchmakingRuleSetsError::InvalidRequest(String::from(error_message)),
                    "NotFoundException" => {
                        DescribeMatchmakingRuleSetsError::NotFound(String::from(error_message))
                    }
                    "UnsupportedRegionException" => DescribeMatchmakingRuleSetsError::UnsupportedRegion(String::from(error_message)),
                    "ValidationException" => {
                        DescribeMatchmakingRuleSetsError::Validation(error_message.to_string())
                    }
                    _ => DescribeMatchmakingRuleSetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMatchmakingRuleSetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMatchmakingRuleSetsError {
    fn from(err: serde_json::error::Error) -> DescribeMatchmakingRuleSetsError {
        DescribeMatchmakingRuleSetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMatchmakingRuleSetsError {
    fn from(err: CredentialsError) -> DescribeMatchmakingRuleSetsError {
        DescribeMatchmakingRuleSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMatchmakingRuleSetsError {
    fn from(err: HttpDispatchError) -> DescribeMatchmakingRuleSetsError {
        DescribeMatchmakingRuleSetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMatchmakingRuleSetsError {
    fn from(err: io::Error) -> DescribeMatchmakingRuleSetsError {
        DescribeMatchmakingRuleSetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMatchmakingRuleSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMatchmakingRuleSetsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMatchmakingRuleSetsError::InternalService(ref cause) => cause,
            DescribeMatchmakingRuleSetsError::InvalidRequest(ref cause) => cause,
            DescribeMatchmakingRuleSetsError::NotFound(ref cause) => cause,
            DescribeMatchmakingRuleSetsError::UnsupportedRegion(ref cause) => cause,
            DescribeMatchmakingRuleSetsError::Validation(ref cause) => cause,
            DescribeMatchmakingRuleSetsError::Credentials(ref err) => err.description(),
            DescribeMatchmakingRuleSetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMatchmakingRuleSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePlayerSessions
#[derive(Debug, PartialEq)]
pub enum DescribePlayerSessionsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribePlayerSessionsError {
    pub fn from_body(body: &str) -> DescribePlayerSessionsError {
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
                        DescribePlayerSessionsError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribePlayerSessionsError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribePlayerSessionsError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribePlayerSessionsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribePlayerSessionsError::Validation(error_message.to_string())
                    }
                    _ => DescribePlayerSessionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribePlayerSessionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribePlayerSessionsError {
    fn from(err: serde_json::error::Error) -> DescribePlayerSessionsError {
        DescribePlayerSessionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribePlayerSessionsError {
    fn from(err: CredentialsError) -> DescribePlayerSessionsError {
        DescribePlayerSessionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePlayerSessionsError {
    fn from(err: HttpDispatchError) -> DescribePlayerSessionsError {
        DescribePlayerSessionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePlayerSessionsError {
    fn from(err: io::Error) -> DescribePlayerSessionsError {
        DescribePlayerSessionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribePlayerSessionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePlayerSessionsError {
    fn description(&self) -> &str {
        match *self {
            DescribePlayerSessionsError::InternalService(ref cause) => cause,
            DescribePlayerSessionsError::InvalidRequest(ref cause) => cause,
            DescribePlayerSessionsError::NotFound(ref cause) => cause,
            DescribePlayerSessionsError::Unauthorized(ref cause) => cause,
            DescribePlayerSessionsError::Validation(ref cause) => cause,
            DescribePlayerSessionsError::Credentials(ref err) => err.description(),
            DescribePlayerSessionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribePlayerSessionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRuntimeConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeRuntimeConfigurationError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeRuntimeConfigurationError {
    pub fn from_body(body: &str) -> DescribeRuntimeConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => DescribeRuntimeConfigurationError::InternalService(String::from(error_message)),
                    "InvalidRequestException" => DescribeRuntimeConfigurationError::InvalidRequest(String::from(error_message)),
                    "NotFoundException" => {
                        DescribeRuntimeConfigurationError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeRuntimeConfigurationError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeRuntimeConfigurationError::Validation(error_message.to_string())
                    }
                    _ => DescribeRuntimeConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeRuntimeConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeRuntimeConfigurationError {
    fn from(err: serde_json::error::Error) -> DescribeRuntimeConfigurationError {
        DescribeRuntimeConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeRuntimeConfigurationError {
    fn from(err: CredentialsError) -> DescribeRuntimeConfigurationError {
        DescribeRuntimeConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRuntimeConfigurationError {
    fn from(err: HttpDispatchError) -> DescribeRuntimeConfigurationError {
        DescribeRuntimeConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeRuntimeConfigurationError {
    fn from(err: io::Error) -> DescribeRuntimeConfigurationError {
        DescribeRuntimeConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeRuntimeConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRuntimeConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DescribeRuntimeConfigurationError::InternalService(ref cause) => cause,
            DescribeRuntimeConfigurationError::InvalidRequest(ref cause) => cause,
            DescribeRuntimeConfigurationError::NotFound(ref cause) => cause,
            DescribeRuntimeConfigurationError::Unauthorized(ref cause) => cause,
            DescribeRuntimeConfigurationError::Validation(ref cause) => cause,
            DescribeRuntimeConfigurationError::Credentials(ref err) => err.description(),
            DescribeRuntimeConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeRuntimeConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeScalingPolicies
#[derive(Debug, PartialEq)]
pub enum DescribeScalingPoliciesError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeScalingPoliciesError {
    pub fn from_body(body: &str) -> DescribeScalingPoliciesError {
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
                        DescribeScalingPoliciesError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeScalingPoliciesError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeScalingPoliciesError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeScalingPoliciesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeScalingPoliciesError::Validation(error_message.to_string())
                    }
                    _ => DescribeScalingPoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeScalingPoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeScalingPoliciesError {
    fn from(err: serde_json::error::Error) -> DescribeScalingPoliciesError {
        DescribeScalingPoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeScalingPoliciesError {
    fn from(err: CredentialsError) -> DescribeScalingPoliciesError {
        DescribeScalingPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeScalingPoliciesError {
    fn from(err: HttpDispatchError) -> DescribeScalingPoliciesError {
        DescribeScalingPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeScalingPoliciesError {
    fn from(err: io::Error) -> DescribeScalingPoliciesError {
        DescribeScalingPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeScalingPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeScalingPoliciesError {
    fn description(&self) -> &str {
        match *self {
            DescribeScalingPoliciesError::InternalService(ref cause) => cause,
            DescribeScalingPoliciesError::InvalidRequest(ref cause) => cause,
            DescribeScalingPoliciesError::NotFound(ref cause) => cause,
            DescribeScalingPoliciesError::Unauthorized(ref cause) => cause,
            DescribeScalingPoliciesError::Validation(ref cause) => cause,
            DescribeScalingPoliciesError::Credentials(ref err) => err.description(),
            DescribeScalingPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeScalingPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGameSessionLogUrl
#[derive(Debug, PartialEq)]
pub enum GetGameSessionLogUrlError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetGameSessionLogUrlError {
    pub fn from_body(body: &str) -> GetGameSessionLogUrlError {
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
                        GetGameSessionLogUrlError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetGameSessionLogUrlError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetGameSessionLogUrlError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetGameSessionLogUrlError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetGameSessionLogUrlError::Validation(error_message.to_string())
                    }
                    _ => GetGameSessionLogUrlError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetGameSessionLogUrlError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetGameSessionLogUrlError {
    fn from(err: serde_json::error::Error) -> GetGameSessionLogUrlError {
        GetGameSessionLogUrlError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGameSessionLogUrlError {
    fn from(err: CredentialsError) -> GetGameSessionLogUrlError {
        GetGameSessionLogUrlError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGameSessionLogUrlError {
    fn from(err: HttpDispatchError) -> GetGameSessionLogUrlError {
        GetGameSessionLogUrlError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGameSessionLogUrlError {
    fn from(err: io::Error) -> GetGameSessionLogUrlError {
        GetGameSessionLogUrlError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGameSessionLogUrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGameSessionLogUrlError {
    fn description(&self) -> &str {
        match *self {
            GetGameSessionLogUrlError::InternalService(ref cause) => cause,
            GetGameSessionLogUrlError::InvalidRequest(ref cause) => cause,
            GetGameSessionLogUrlError::NotFound(ref cause) => cause,
            GetGameSessionLogUrlError::Unauthorized(ref cause) => cause,
            GetGameSessionLogUrlError::Validation(ref cause) => cause,
            GetGameSessionLogUrlError::Credentials(ref err) => err.description(),
            GetGameSessionLogUrlError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetGameSessionLogUrlError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceAccess
#[derive(Debug, PartialEq)]
pub enum GetInstanceAccessError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetInstanceAccessError {
    pub fn from_body(body: &str) -> GetInstanceAccessError {
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
                        GetInstanceAccessError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetInstanceAccessError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetInstanceAccessError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetInstanceAccessError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInstanceAccessError::Validation(error_message.to_string())
                    }
                    _ => GetInstanceAccessError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInstanceAccessError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInstanceAccessError {
    fn from(err: serde_json::error::Error) -> GetInstanceAccessError {
        GetInstanceAccessError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstanceAccessError {
    fn from(err: CredentialsError) -> GetInstanceAccessError {
        GetInstanceAccessError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstanceAccessError {
    fn from(err: HttpDispatchError) -> GetInstanceAccessError {
        GetInstanceAccessError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstanceAccessError {
    fn from(err: io::Error) -> GetInstanceAccessError {
        GetInstanceAccessError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstanceAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceAccessError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceAccessError::InternalService(ref cause) => cause,
            GetInstanceAccessError::InvalidRequest(ref cause) => cause,
            GetInstanceAccessError::NotFound(ref cause) => cause,
            GetInstanceAccessError::Unauthorized(ref cause) => cause,
            GetInstanceAccessError::Validation(ref cause) => cause,
            GetInstanceAccessError::Credentials(ref err) => err.description(),
            GetInstanceAccessError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetInstanceAccessError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAliases
#[derive(Debug, PartialEq)]
pub enum ListAliasesError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListAliasesError {
    pub fn from_body(body: &str) -> ListAliasesError {
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
                        ListAliasesError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListAliasesError::InvalidRequest(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListAliasesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAliasesError::Validation(error_message.to_string())
                    }
                    _ => ListAliasesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAliasesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAliasesError {
    fn from(err: serde_json::error::Error) -> ListAliasesError {
        ListAliasesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAliasesError {
    fn from(err: CredentialsError) -> ListAliasesError {
        ListAliasesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAliasesError {
    fn from(err: HttpDispatchError) -> ListAliasesError {
        ListAliasesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAliasesError {
    fn from(err: io::Error) -> ListAliasesError {
        ListAliasesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAliasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAliasesError {
    fn description(&self) -> &str {
        match *self {
            ListAliasesError::InternalService(ref cause) => cause,
            ListAliasesError::InvalidRequest(ref cause) => cause,
            ListAliasesError::Unauthorized(ref cause) => cause,
            ListAliasesError::Validation(ref cause) => cause,
            ListAliasesError::Credentials(ref err) => err.description(),
            ListAliasesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListAliasesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBuilds
#[derive(Debug, PartialEq)]
pub enum ListBuildsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListBuildsError {
    pub fn from_body(body: &str) -> ListBuildsError {
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
                        ListBuildsError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListBuildsError::InvalidRequest(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListBuildsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => ListBuildsError::Validation(error_message.to_string()),
                    _ => ListBuildsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBuildsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListBuildsError {
    fn from(err: serde_json::error::Error) -> ListBuildsError {
        ListBuildsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBuildsError {
    fn from(err: CredentialsError) -> ListBuildsError {
        ListBuildsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBuildsError {
    fn from(err: HttpDispatchError) -> ListBuildsError {
        ListBuildsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBuildsError {
    fn from(err: io::Error) -> ListBuildsError {
        ListBuildsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBuildsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBuildsError {
    fn description(&self) -> &str {
        match *self {
            ListBuildsError::InternalService(ref cause) => cause,
            ListBuildsError::InvalidRequest(ref cause) => cause,
            ListBuildsError::Unauthorized(ref cause) => cause,
            ListBuildsError::Validation(ref cause) => cause,
            ListBuildsError::Credentials(ref err) => err.description(),
            ListBuildsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListBuildsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFleets
#[derive(Debug, PartialEq)]
pub enum ListFleetsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListFleetsError {
    pub fn from_body(body: &str) -> ListFleetsError {
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
                        ListFleetsError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListFleetsError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => ListFleetsError::NotFound(String::from(error_message)),
                    "UnauthorizedException" => {
                        ListFleetsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => ListFleetsError::Validation(error_message.to_string()),
                    _ => ListFleetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListFleetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListFleetsError {
    fn from(err: serde_json::error::Error) -> ListFleetsError {
        ListFleetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFleetsError {
    fn from(err: CredentialsError) -> ListFleetsError {
        ListFleetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFleetsError {
    fn from(err: HttpDispatchError) -> ListFleetsError {
        ListFleetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFleetsError {
    fn from(err: io::Error) -> ListFleetsError {
        ListFleetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFleetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFleetsError {
    fn description(&self) -> &str {
        match *self {
            ListFleetsError::InternalService(ref cause) => cause,
            ListFleetsError::InvalidRequest(ref cause) => cause,
            ListFleetsError::NotFound(ref cause) => cause,
            ListFleetsError::Unauthorized(ref cause) => cause,
            ListFleetsError::Validation(ref cause) => cause,
            ListFleetsError::Credentials(ref err) => err.description(),
            ListFleetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListFleetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutScalingPolicy
#[derive(Debug, PartialEq)]
pub enum PutScalingPolicyError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutScalingPolicyError {
    pub fn from_body(body: &str) -> PutScalingPolicyError {
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
                        PutScalingPolicyError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        PutScalingPolicyError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        PutScalingPolicyError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        PutScalingPolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutScalingPolicyError::Validation(error_message.to_string())
                    }
                    _ => PutScalingPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutScalingPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutScalingPolicyError {
    fn from(err: serde_json::error::Error) -> PutScalingPolicyError {
        PutScalingPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutScalingPolicyError {
    fn from(err: CredentialsError) -> PutScalingPolicyError {
        PutScalingPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutScalingPolicyError {
    fn from(err: HttpDispatchError) -> PutScalingPolicyError {
        PutScalingPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutScalingPolicyError {
    fn from(err: io::Error) -> PutScalingPolicyError {
        PutScalingPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutScalingPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutScalingPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutScalingPolicyError::InternalService(ref cause) => cause,
            PutScalingPolicyError::InvalidRequest(ref cause) => cause,
            PutScalingPolicyError::NotFound(ref cause) => cause,
            PutScalingPolicyError::Unauthorized(ref cause) => cause,
            PutScalingPolicyError::Validation(ref cause) => cause,
            PutScalingPolicyError::Credentials(ref err) => err.description(),
            PutScalingPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutScalingPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RequestUploadCredentials
#[derive(Debug, PartialEq)]
pub enum RequestUploadCredentialsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RequestUploadCredentialsError {
    pub fn from_body(body: &str) -> RequestUploadCredentialsError {
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
                        RequestUploadCredentialsError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RequestUploadCredentialsError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        RequestUploadCredentialsError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        RequestUploadCredentialsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        RequestUploadCredentialsError::Validation(error_message.to_string())
                    }
                    _ => RequestUploadCredentialsError::Unknown(String::from(body)),
                }
            }
            Err(_) => RequestUploadCredentialsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RequestUploadCredentialsError {
    fn from(err: serde_json::error::Error) -> RequestUploadCredentialsError {
        RequestUploadCredentialsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RequestUploadCredentialsError {
    fn from(err: CredentialsError) -> RequestUploadCredentialsError {
        RequestUploadCredentialsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RequestUploadCredentialsError {
    fn from(err: HttpDispatchError) -> RequestUploadCredentialsError {
        RequestUploadCredentialsError::HttpDispatch(err)
    }
}
impl From<io::Error> for RequestUploadCredentialsError {
    fn from(err: io::Error) -> RequestUploadCredentialsError {
        RequestUploadCredentialsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RequestUploadCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RequestUploadCredentialsError {
    fn description(&self) -> &str {
        match *self {
            RequestUploadCredentialsError::InternalService(ref cause) => cause,
            RequestUploadCredentialsError::InvalidRequest(ref cause) => cause,
            RequestUploadCredentialsError::NotFound(ref cause) => cause,
            RequestUploadCredentialsError::Unauthorized(ref cause) => cause,
            RequestUploadCredentialsError::Validation(ref cause) => cause,
            RequestUploadCredentialsError::Credentials(ref err) => err.description(),
            RequestUploadCredentialsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RequestUploadCredentialsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ResolveAlias
#[derive(Debug, PartialEq)]
pub enum ResolveAliasError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ResolveAliasError {
    pub fn from_body(body: &str) -> ResolveAliasError {
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
                        ResolveAliasError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ResolveAliasError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => ResolveAliasError::NotFound(String::from(error_message)),
                    "TerminalRoutingStrategyException" => {
                        ResolveAliasError::TerminalRoutingStrategy(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ResolveAliasError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ResolveAliasError::Validation(error_message.to_string())
                    }
                    _ => ResolveAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => ResolveAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ResolveAliasError {
    fn from(err: serde_json::error::Error) -> ResolveAliasError {
        ResolveAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ResolveAliasError {
    fn from(err: CredentialsError) -> ResolveAliasError {
        ResolveAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResolveAliasError {
    fn from(err: HttpDispatchError) -> ResolveAliasError {
        ResolveAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResolveAliasError {
    fn from(err: io::Error) -> ResolveAliasError {
        ResolveAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResolveAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResolveAliasError {
    fn description(&self) -> &str {
        match *self {
            ResolveAliasError::InternalService(ref cause) => cause,
            ResolveAliasError::InvalidRequest(ref cause) => cause,
            ResolveAliasError::NotFound(ref cause) => cause,
            ResolveAliasError::TerminalRoutingStrategy(ref cause) => cause,
            ResolveAliasError::Unauthorized(ref cause) => cause,
            ResolveAliasError::Validation(ref cause) => cause,
            ResolveAliasError::Credentials(ref err) => err.description(),
            ResolveAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ResolveAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchGameSessions
#[derive(Debug, PartialEq)]
pub enum SearchGameSessionsError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SearchGameSessionsError {
    pub fn from_body(body: &str) -> SearchGameSessionsError {
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
                        SearchGameSessionsError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        SearchGameSessionsError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        SearchGameSessionsError::NotFound(String::from(error_message))
                    }
                    "TerminalRoutingStrategyException" => SearchGameSessionsError::TerminalRoutingStrategy(String::from(error_message)),
                    "UnauthorizedException" => {
                        SearchGameSessionsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        SearchGameSessionsError::Validation(error_message.to_string())
                    }
                    _ => SearchGameSessionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchGameSessionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchGameSessionsError {
    fn from(err: serde_json::error::Error) -> SearchGameSessionsError {
        SearchGameSessionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchGameSessionsError {
    fn from(err: CredentialsError) -> SearchGameSessionsError {
        SearchGameSessionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchGameSessionsError {
    fn from(err: HttpDispatchError) -> SearchGameSessionsError {
        SearchGameSessionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchGameSessionsError {
    fn from(err: io::Error) -> SearchGameSessionsError {
        SearchGameSessionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchGameSessionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchGameSessionsError {
    fn description(&self) -> &str {
        match *self {
            SearchGameSessionsError::InternalService(ref cause) => cause,
            SearchGameSessionsError::InvalidRequest(ref cause) => cause,
            SearchGameSessionsError::NotFound(ref cause) => cause,
            SearchGameSessionsError::TerminalRoutingStrategy(ref cause) => cause,
            SearchGameSessionsError::Unauthorized(ref cause) => cause,
            SearchGameSessionsError::Validation(ref cause) => cause,
            SearchGameSessionsError::Credentials(ref err) => err.description(),
            SearchGameSessionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SearchGameSessionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartGameSessionPlacement
#[derive(Debug, PartialEq)]
pub enum StartGameSessionPlacementError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartGameSessionPlacementError {
    pub fn from_body(body: &str) -> StartGameSessionPlacementError {
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
                        StartGameSessionPlacementError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        StartGameSessionPlacementError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        StartGameSessionPlacementError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        StartGameSessionPlacementError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartGameSessionPlacementError::Validation(error_message.to_string())
                    }
                    _ => StartGameSessionPlacementError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartGameSessionPlacementError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartGameSessionPlacementError {
    fn from(err: serde_json::error::Error) -> StartGameSessionPlacementError {
        StartGameSessionPlacementError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartGameSessionPlacementError {
    fn from(err: CredentialsError) -> StartGameSessionPlacementError {
        StartGameSessionPlacementError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartGameSessionPlacementError {
    fn from(err: HttpDispatchError) -> StartGameSessionPlacementError {
        StartGameSessionPlacementError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartGameSessionPlacementError {
    fn from(err: io::Error) -> StartGameSessionPlacementError {
        StartGameSessionPlacementError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartGameSessionPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartGameSessionPlacementError {
    fn description(&self) -> &str {
        match *self {
            StartGameSessionPlacementError::InternalService(ref cause) => cause,
            StartGameSessionPlacementError::InvalidRequest(ref cause) => cause,
            StartGameSessionPlacementError::NotFound(ref cause) => cause,
            StartGameSessionPlacementError::Unauthorized(ref cause) => cause,
            StartGameSessionPlacementError::Validation(ref cause) => cause,
            StartGameSessionPlacementError::Credentials(ref err) => err.description(),
            StartGameSessionPlacementError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartGameSessionPlacementError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartMatchmaking
#[derive(Debug, PartialEq)]
pub enum StartMatchmakingError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The requested operation is not supported in the region specified.</p>
    UnsupportedRegion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartMatchmakingError {
    pub fn from_body(body: &str) -> StartMatchmakingError {
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
                        StartMatchmakingError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        StartMatchmakingError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        StartMatchmakingError::NotFound(String::from(error_message))
                    }
                    "UnsupportedRegionException" => {
                        StartMatchmakingError::UnsupportedRegion(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartMatchmakingError::Validation(error_message.to_string())
                    }
                    _ => StartMatchmakingError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartMatchmakingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartMatchmakingError {
    fn from(err: serde_json::error::Error) -> StartMatchmakingError {
        StartMatchmakingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartMatchmakingError {
    fn from(err: CredentialsError) -> StartMatchmakingError {
        StartMatchmakingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartMatchmakingError {
    fn from(err: HttpDispatchError) -> StartMatchmakingError {
        StartMatchmakingError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartMatchmakingError {
    fn from(err: io::Error) -> StartMatchmakingError {
        StartMatchmakingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartMatchmakingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartMatchmakingError {
    fn description(&self) -> &str {
        match *self {
            StartMatchmakingError::InternalService(ref cause) => cause,
            StartMatchmakingError::InvalidRequest(ref cause) => cause,
            StartMatchmakingError::NotFound(ref cause) => cause,
            StartMatchmakingError::UnsupportedRegion(ref cause) => cause,
            StartMatchmakingError::Validation(ref cause) => cause,
            StartMatchmakingError::Credentials(ref err) => err.description(),
            StartMatchmakingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartMatchmakingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopGameSessionPlacement
#[derive(Debug, PartialEq)]
pub enum StopGameSessionPlacementError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StopGameSessionPlacementError {
    pub fn from_body(body: &str) -> StopGameSessionPlacementError {
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
                        StopGameSessionPlacementError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        StopGameSessionPlacementError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        StopGameSessionPlacementError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        StopGameSessionPlacementError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopGameSessionPlacementError::Validation(error_message.to_string())
                    }
                    _ => StopGameSessionPlacementError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopGameSessionPlacementError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopGameSessionPlacementError {
    fn from(err: serde_json::error::Error) -> StopGameSessionPlacementError {
        StopGameSessionPlacementError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopGameSessionPlacementError {
    fn from(err: CredentialsError) -> StopGameSessionPlacementError {
        StopGameSessionPlacementError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopGameSessionPlacementError {
    fn from(err: HttpDispatchError) -> StopGameSessionPlacementError {
        StopGameSessionPlacementError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopGameSessionPlacementError {
    fn from(err: io::Error) -> StopGameSessionPlacementError {
        StopGameSessionPlacementError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopGameSessionPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopGameSessionPlacementError {
    fn description(&self) -> &str {
        match *self {
            StopGameSessionPlacementError::InternalService(ref cause) => cause,
            StopGameSessionPlacementError::InvalidRequest(ref cause) => cause,
            StopGameSessionPlacementError::NotFound(ref cause) => cause,
            StopGameSessionPlacementError::Unauthorized(ref cause) => cause,
            StopGameSessionPlacementError::Validation(ref cause) => cause,
            StopGameSessionPlacementError::Credentials(ref err) => err.description(),
            StopGameSessionPlacementError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopGameSessionPlacementError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopMatchmaking
#[derive(Debug, PartialEq)]
pub enum StopMatchmakingError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The requested operation is not supported in the region specified.</p>
    UnsupportedRegion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StopMatchmakingError {
    pub fn from_body(body: &str) -> StopMatchmakingError {
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
                        StopMatchmakingError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        StopMatchmakingError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        StopMatchmakingError::NotFound(String::from(error_message))
                    }
                    "UnsupportedRegionException" => {
                        StopMatchmakingError::UnsupportedRegion(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopMatchmakingError::Validation(error_message.to_string())
                    }
                    _ => StopMatchmakingError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopMatchmakingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopMatchmakingError {
    fn from(err: serde_json::error::Error) -> StopMatchmakingError {
        StopMatchmakingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopMatchmakingError {
    fn from(err: CredentialsError) -> StopMatchmakingError {
        StopMatchmakingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopMatchmakingError {
    fn from(err: HttpDispatchError) -> StopMatchmakingError {
        StopMatchmakingError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopMatchmakingError {
    fn from(err: io::Error) -> StopMatchmakingError {
        StopMatchmakingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopMatchmakingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopMatchmakingError {
    fn description(&self) -> &str {
        match *self {
            StopMatchmakingError::InternalService(ref cause) => cause,
            StopMatchmakingError::InvalidRequest(ref cause) => cause,
            StopMatchmakingError::NotFound(ref cause) => cause,
            StopMatchmakingError::UnsupportedRegion(ref cause) => cause,
            StopMatchmakingError::Validation(ref cause) => cause,
            StopMatchmakingError::Credentials(ref err) => err.description(),
            StopMatchmakingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopMatchmakingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAlias
#[derive(Debug, PartialEq)]
pub enum UpdateAliasError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateAliasError {
    pub fn from_body(body: &str) -> UpdateAliasError {
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
                        UpdateAliasError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateAliasError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => UpdateAliasError::NotFound(String::from(error_message)),
                    "UnauthorizedException" => {
                        UpdateAliasError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateAliasError::Validation(error_message.to_string())
                    }
                    _ => UpdateAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateAliasError {
    fn from(err: serde_json::error::Error) -> UpdateAliasError {
        UpdateAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAliasError {
    fn from(err: CredentialsError) -> UpdateAliasError {
        UpdateAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAliasError {
    fn from(err: HttpDispatchError) -> UpdateAliasError {
        UpdateAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAliasError {
    fn from(err: io::Error) -> UpdateAliasError {
        UpdateAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAliasError {
    fn description(&self) -> &str {
        match *self {
            UpdateAliasError::InternalService(ref cause) => cause,
            UpdateAliasError::InvalidRequest(ref cause) => cause,
            UpdateAliasError::NotFound(ref cause) => cause,
            UpdateAliasError::Unauthorized(ref cause) => cause,
            UpdateAliasError::Validation(ref cause) => cause,
            UpdateAliasError::Credentials(ref err) => err.description(),
            UpdateAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateBuild
#[derive(Debug, PartialEq)]
pub enum UpdateBuildError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateBuildError {
    pub fn from_body(body: &str) -> UpdateBuildError {
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
                        UpdateBuildError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateBuildError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => UpdateBuildError::NotFound(String::from(error_message)),
                    "UnauthorizedException" => {
                        UpdateBuildError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateBuildError::Validation(error_message.to_string())
                    }
                    _ => UpdateBuildError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateBuildError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateBuildError {
    fn from(err: serde_json::error::Error) -> UpdateBuildError {
        UpdateBuildError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateBuildError {
    fn from(err: CredentialsError) -> UpdateBuildError {
        UpdateBuildError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateBuildError {
    fn from(err: HttpDispatchError) -> UpdateBuildError {
        UpdateBuildError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateBuildError {
    fn from(err: io::Error) -> UpdateBuildError {
        UpdateBuildError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBuildError {
    fn description(&self) -> &str {
        match *self {
            UpdateBuildError::InternalService(ref cause) => cause,
            UpdateBuildError::InvalidRequest(ref cause) => cause,
            UpdateBuildError::NotFound(ref cause) => cause,
            UpdateBuildError::Unauthorized(ref cause) => cause,
            UpdateBuildError::Validation(ref cause) => cause,
            UpdateBuildError::Credentials(ref err) => err.description(),
            UpdateBuildError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateBuildError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFleetAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateFleetAttributesError {
    ///<p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateFleetAttributesError {
    pub fn from_body(body: &str) -> UpdateFleetAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConflictException" => {
                        UpdateFleetAttributesError::Conflict(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateFleetAttributesError::InternalService(String::from(error_message))
                    }
                    "InvalidFleetStatusException" => {
                        UpdateFleetAttributesError::InvalidFleetStatus(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateFleetAttributesError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateFleetAttributesError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateFleetAttributesError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateFleetAttributesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateFleetAttributesError::Validation(error_message.to_string())
                    }
                    _ => UpdateFleetAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateFleetAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateFleetAttributesError {
    fn from(err: serde_json::error::Error) -> UpdateFleetAttributesError {
        UpdateFleetAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFleetAttributesError {
    fn from(err: CredentialsError) -> UpdateFleetAttributesError {
        UpdateFleetAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFleetAttributesError {
    fn from(err: HttpDispatchError) -> UpdateFleetAttributesError {
        UpdateFleetAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFleetAttributesError {
    fn from(err: io::Error) -> UpdateFleetAttributesError {
        UpdateFleetAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFleetAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFleetAttributesError {
    fn description(&self) -> &str {
        match *self {
            UpdateFleetAttributesError::Conflict(ref cause) => cause,
            UpdateFleetAttributesError::InternalService(ref cause) => cause,
            UpdateFleetAttributesError::InvalidFleetStatus(ref cause) => cause,
            UpdateFleetAttributesError::InvalidRequest(ref cause) => cause,
            UpdateFleetAttributesError::LimitExceeded(ref cause) => cause,
            UpdateFleetAttributesError::NotFound(ref cause) => cause,
            UpdateFleetAttributesError::Unauthorized(ref cause) => cause,
            UpdateFleetAttributesError::Validation(ref cause) => cause,
            UpdateFleetAttributesError::Credentials(ref err) => err.description(),
            UpdateFleetAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateFleetAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFleetCapacity
#[derive(Debug, PartialEq)]
pub enum UpdateFleetCapacityError {
    ///<p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateFleetCapacityError {
    pub fn from_body(body: &str) -> UpdateFleetCapacityError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConflictException" => {
                        UpdateFleetCapacityError::Conflict(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateFleetCapacityError::InternalService(String::from(error_message))
                    }
                    "InvalidFleetStatusException" => {
                        UpdateFleetCapacityError::InvalidFleetStatus(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateFleetCapacityError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateFleetCapacityError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateFleetCapacityError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateFleetCapacityError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateFleetCapacityError::Validation(error_message.to_string())
                    }
                    _ => UpdateFleetCapacityError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateFleetCapacityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateFleetCapacityError {
    fn from(err: serde_json::error::Error) -> UpdateFleetCapacityError {
        UpdateFleetCapacityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFleetCapacityError {
    fn from(err: CredentialsError) -> UpdateFleetCapacityError {
        UpdateFleetCapacityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFleetCapacityError {
    fn from(err: HttpDispatchError) -> UpdateFleetCapacityError {
        UpdateFleetCapacityError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFleetCapacityError {
    fn from(err: io::Error) -> UpdateFleetCapacityError {
        UpdateFleetCapacityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFleetCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFleetCapacityError {
    fn description(&self) -> &str {
        match *self {
            UpdateFleetCapacityError::Conflict(ref cause) => cause,
            UpdateFleetCapacityError::InternalService(ref cause) => cause,
            UpdateFleetCapacityError::InvalidFleetStatus(ref cause) => cause,
            UpdateFleetCapacityError::InvalidRequest(ref cause) => cause,
            UpdateFleetCapacityError::LimitExceeded(ref cause) => cause,
            UpdateFleetCapacityError::NotFound(ref cause) => cause,
            UpdateFleetCapacityError::Unauthorized(ref cause) => cause,
            UpdateFleetCapacityError::Validation(ref cause) => cause,
            UpdateFleetCapacityError::Credentials(ref err) => err.description(),
            UpdateFleetCapacityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateFleetCapacityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFleetPortSettings
#[derive(Debug, PartialEq)]
pub enum UpdateFleetPortSettingsError {
    ///<p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateFleetPortSettingsError {
    pub fn from_body(body: &str) -> UpdateFleetPortSettingsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConflictException" => {
                        UpdateFleetPortSettingsError::Conflict(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateFleetPortSettingsError::InternalService(String::from(error_message))
                    }
                    "InvalidFleetStatusException" => UpdateFleetPortSettingsError::InvalidFleetStatus(String::from(error_message)),
                    "InvalidRequestException" => {
                        UpdateFleetPortSettingsError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateFleetPortSettingsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateFleetPortSettingsError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateFleetPortSettingsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateFleetPortSettingsError::Validation(error_message.to_string())
                    }
                    _ => UpdateFleetPortSettingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateFleetPortSettingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateFleetPortSettingsError {
    fn from(err: serde_json::error::Error) -> UpdateFleetPortSettingsError {
        UpdateFleetPortSettingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFleetPortSettingsError {
    fn from(err: CredentialsError) -> UpdateFleetPortSettingsError {
        UpdateFleetPortSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFleetPortSettingsError {
    fn from(err: HttpDispatchError) -> UpdateFleetPortSettingsError {
        UpdateFleetPortSettingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFleetPortSettingsError {
    fn from(err: io::Error) -> UpdateFleetPortSettingsError {
        UpdateFleetPortSettingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFleetPortSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFleetPortSettingsError {
    fn description(&self) -> &str {
        match *self {
            UpdateFleetPortSettingsError::Conflict(ref cause) => cause,
            UpdateFleetPortSettingsError::InternalService(ref cause) => cause,
            UpdateFleetPortSettingsError::InvalidFleetStatus(ref cause) => cause,
            UpdateFleetPortSettingsError::InvalidRequest(ref cause) => cause,
            UpdateFleetPortSettingsError::LimitExceeded(ref cause) => cause,
            UpdateFleetPortSettingsError::NotFound(ref cause) => cause,
            UpdateFleetPortSettingsError::Unauthorized(ref cause) => cause,
            UpdateFleetPortSettingsError::Validation(ref cause) => cause,
            UpdateFleetPortSettingsError::Credentials(ref err) => err.description(),
            UpdateFleetPortSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateFleetPortSettingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGameSession
#[derive(Debug, PartialEq)]
pub enum UpdateGameSessionError {
    ///<p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the game instance. Resolve the conflict before retrying.</p>
    InvalidGameSessionStatus(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateGameSessionError {
    pub fn from_body(body: &str) -> UpdateGameSessionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConflictException" => {
                        UpdateGameSessionError::Conflict(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateGameSessionError::InternalService(String::from(error_message))
                    }
                    "InvalidGameSessionStatusException" => UpdateGameSessionError::InvalidGameSessionStatus(String::from(error_message)),
                    "InvalidRequestException" => {
                        UpdateGameSessionError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateGameSessionError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateGameSessionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateGameSessionError::Validation(error_message.to_string())
                    }
                    _ => UpdateGameSessionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateGameSessionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateGameSessionError {
    fn from(err: serde_json::error::Error) -> UpdateGameSessionError {
        UpdateGameSessionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGameSessionError {
    fn from(err: CredentialsError) -> UpdateGameSessionError {
        UpdateGameSessionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGameSessionError {
    fn from(err: HttpDispatchError) -> UpdateGameSessionError {
        UpdateGameSessionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGameSessionError {
    fn from(err: io::Error) -> UpdateGameSessionError {
        UpdateGameSessionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGameSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGameSessionError {
    fn description(&self) -> &str {
        match *self {
            UpdateGameSessionError::Conflict(ref cause) => cause,
            UpdateGameSessionError::InternalService(ref cause) => cause,
            UpdateGameSessionError::InvalidGameSessionStatus(ref cause) => cause,
            UpdateGameSessionError::InvalidRequest(ref cause) => cause,
            UpdateGameSessionError::NotFound(ref cause) => cause,
            UpdateGameSessionError::Unauthorized(ref cause) => cause,
            UpdateGameSessionError::Validation(ref cause) => cause,
            UpdateGameSessionError::Credentials(ref err) => err.description(),
            UpdateGameSessionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateGameSessionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGameSessionQueue
#[derive(Debug, PartialEq)]
pub enum UpdateGameSessionQueueError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateGameSessionQueueError {
    pub fn from_body(body: &str) -> UpdateGameSessionQueueError {
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
                        UpdateGameSessionQueueError::InternalService(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateGameSessionQueueError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateGameSessionQueueError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateGameSessionQueueError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateGameSessionQueueError::Validation(error_message.to_string())
                    }
                    _ => UpdateGameSessionQueueError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateGameSessionQueueError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateGameSessionQueueError {
    fn from(err: serde_json::error::Error) -> UpdateGameSessionQueueError {
        UpdateGameSessionQueueError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGameSessionQueueError {
    fn from(err: CredentialsError) -> UpdateGameSessionQueueError {
        UpdateGameSessionQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGameSessionQueueError {
    fn from(err: HttpDispatchError) -> UpdateGameSessionQueueError {
        UpdateGameSessionQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGameSessionQueueError {
    fn from(err: io::Error) -> UpdateGameSessionQueueError {
        UpdateGameSessionQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGameSessionQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGameSessionQueueError {
    fn description(&self) -> &str {
        match *self {
            UpdateGameSessionQueueError::InternalService(ref cause) => cause,
            UpdateGameSessionQueueError::InvalidRequest(ref cause) => cause,
            UpdateGameSessionQueueError::NotFound(ref cause) => cause,
            UpdateGameSessionQueueError::Unauthorized(ref cause) => cause,
            UpdateGameSessionQueueError::Validation(ref cause) => cause,
            UpdateGameSessionQueueError::Credentials(ref err) => err.description(),
            UpdateGameSessionQueueError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateGameSessionQueueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMatchmakingConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateMatchmakingConfigurationError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The requested operation is not supported in the region specified.</p>
    UnsupportedRegion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateMatchmakingConfigurationError {
    pub fn from_body(body: &str) -> UpdateMatchmakingConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => UpdateMatchmakingConfigurationError::InternalService(String::from(error_message)),
                    "InvalidRequestException" => UpdateMatchmakingConfigurationError::InvalidRequest(String::from(error_message)),
                    "NotFoundException" => {
                        UpdateMatchmakingConfigurationError::NotFound(String::from(error_message))
                    }
                    "UnsupportedRegionException" => UpdateMatchmakingConfigurationError::UnsupportedRegion(String::from(error_message)),
                    "ValidationException" => {
                        UpdateMatchmakingConfigurationError::Validation(error_message.to_string())
                    }
                    _ => UpdateMatchmakingConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateMatchmakingConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateMatchmakingConfigurationError {
    fn from(err: serde_json::error::Error) -> UpdateMatchmakingConfigurationError {
        UpdateMatchmakingConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateMatchmakingConfigurationError {
    fn from(err: CredentialsError) -> UpdateMatchmakingConfigurationError {
        UpdateMatchmakingConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateMatchmakingConfigurationError {
    fn from(err: HttpDispatchError) -> UpdateMatchmakingConfigurationError {
        UpdateMatchmakingConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateMatchmakingConfigurationError {
    fn from(err: io::Error) -> UpdateMatchmakingConfigurationError {
        UpdateMatchmakingConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateMatchmakingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMatchmakingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateMatchmakingConfigurationError::InternalService(ref cause) => cause,
            UpdateMatchmakingConfigurationError::InvalidRequest(ref cause) => cause,
            UpdateMatchmakingConfigurationError::NotFound(ref cause) => cause,
            UpdateMatchmakingConfigurationError::UnsupportedRegion(ref cause) => cause,
            UpdateMatchmakingConfigurationError::Validation(ref cause) => cause,
            UpdateMatchmakingConfigurationError::Credentials(ref err) => err.description(),
            UpdateMatchmakingConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateMatchmakingConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRuntimeConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateRuntimeConfigurationError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    ///<p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateRuntimeConfigurationError {
    pub fn from_body(body: &str) -> UpdateRuntimeConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => UpdateRuntimeConfigurationError::InternalService(String::from(error_message)),
                    "InvalidFleetStatusException" => UpdateRuntimeConfigurationError::InvalidFleetStatus(String::from(error_message)),
                    "InvalidRequestException" => {
                        UpdateRuntimeConfigurationError::InvalidRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateRuntimeConfigurationError::NotFound(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateRuntimeConfigurationError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateRuntimeConfigurationError::Validation(error_message.to_string())
                    }
                    _ => UpdateRuntimeConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRuntimeConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRuntimeConfigurationError {
    fn from(err: serde_json::error::Error) -> UpdateRuntimeConfigurationError {
        UpdateRuntimeConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRuntimeConfigurationError {
    fn from(err: CredentialsError) -> UpdateRuntimeConfigurationError {
        UpdateRuntimeConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRuntimeConfigurationError {
    fn from(err: HttpDispatchError) -> UpdateRuntimeConfigurationError {
        UpdateRuntimeConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateRuntimeConfigurationError {
    fn from(err: io::Error) -> UpdateRuntimeConfigurationError {
        UpdateRuntimeConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateRuntimeConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRuntimeConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateRuntimeConfigurationError::InternalService(ref cause) => cause,
            UpdateRuntimeConfigurationError::InvalidFleetStatus(ref cause) => cause,
            UpdateRuntimeConfigurationError::InvalidRequest(ref cause) => cause,
            UpdateRuntimeConfigurationError::NotFound(ref cause) => cause,
            UpdateRuntimeConfigurationError::Unauthorized(ref cause) => cause,
            UpdateRuntimeConfigurationError::Validation(ref cause) => cause,
            UpdateRuntimeConfigurationError::Credentials(ref err) => err.description(),
            UpdateRuntimeConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateRuntimeConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ValidateMatchmakingRuleSet
#[derive(Debug, PartialEq)]
pub enum ValidateMatchmakingRuleSetError {
    ///<p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    ///<p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    ///<p>The requested operation is not supported in the region specified.</p>
    UnsupportedRegion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ValidateMatchmakingRuleSetError {
    pub fn from_body(body: &str) -> ValidateMatchmakingRuleSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServiceException" => ValidateMatchmakingRuleSetError::InternalService(String::from(error_message)),
                    "InvalidRequestException" => {
                        ValidateMatchmakingRuleSetError::InvalidRequest(String::from(error_message))
                    }
                    "UnsupportedRegionException" => ValidateMatchmakingRuleSetError::UnsupportedRegion(String::from(error_message)),
                    "ValidationException" => {
                        ValidateMatchmakingRuleSetError::Validation(error_message.to_string())
                    }
                    _ => ValidateMatchmakingRuleSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => ValidateMatchmakingRuleSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ValidateMatchmakingRuleSetError {
    fn from(err: serde_json::error::Error) -> ValidateMatchmakingRuleSetError {
        ValidateMatchmakingRuleSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ValidateMatchmakingRuleSetError {
    fn from(err: CredentialsError) -> ValidateMatchmakingRuleSetError {
        ValidateMatchmakingRuleSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ValidateMatchmakingRuleSetError {
    fn from(err: HttpDispatchError) -> ValidateMatchmakingRuleSetError {
        ValidateMatchmakingRuleSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for ValidateMatchmakingRuleSetError {
    fn from(err: io::Error) -> ValidateMatchmakingRuleSetError {
        ValidateMatchmakingRuleSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ValidateMatchmakingRuleSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ValidateMatchmakingRuleSetError {
    fn description(&self) -> &str {
        match *self {
            ValidateMatchmakingRuleSetError::InternalService(ref cause) => cause,
            ValidateMatchmakingRuleSetError::InvalidRequest(ref cause) => cause,
            ValidateMatchmakingRuleSetError::UnsupportedRegion(ref cause) => cause,
            ValidateMatchmakingRuleSetError::Validation(ref cause) => cause,
            ValidateMatchmakingRuleSetError::Credentials(ref err) => err.description(),
            ValidateMatchmakingRuleSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ValidateMatchmakingRuleSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon GameLift API. Amazon GameLift clients implement this trait.
pub trait GameLift {
    #[doc="<p>Registers a player's acceptance or rejection of a proposed FlexMatch match. A matchmaking configuration may require player acceptance; if so, then matches built with that configuration cannot be completed unless all players accept the proposed match within a specified time limit. </p> <p>When FlexMatch builds a match, all the matchmaking tickets involved in the proposed match are placed into status <code>REQUIRES_ACCEPTANCE</code>. This is a trigger for your game to get acceptance from all players in the ticket. Acceptances are only valid for tickets when they are in this status; all other acceptances result in an error.</p> <p>To register acceptance, specify the ticket ID, a response, and one or more players. Once all players have registered acceptance, the matchmaking tickets advance to status <code>PLACING</code>, where a new game session is created for the match. </p> <p>If any player rejects the match, or if acceptances are not received before a specified timeout, the proposed match is dropped. The matchmaking tickets are then handled in one of two ways: For tickets where all players accepted the match, the ticket status is returned to <code>SEARCHING</code> to find a new match. For tickets where one or more players failed to accept the match, the ticket status is set to <code>FAILED</code>, and processing is terminated. A new matchmaking request for these players can be submitted as needed. </p> <p>Matchmaking-related operations include:</p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> </ul>"]
    fn accept_match(&self,
                    input: &AcceptMatchInput)
                    -> Result<AcceptMatchOutput, AcceptMatchError>;


    #[doc="<p>Creates an alias for a fleet. In most situations, you can use an alias ID in place of a fleet ID. By using a fleet alias instead of a specific fleet ID, you can switch gameplay and players to a new fleet without changing your game client or other game components. For example, for games in production, using an alias allows you to seamlessly redirect your player base to a new game server update. </p> <p>Amazon GameLift supports two types of routing strategies for aliases: simple and terminal. A simple alias points to an active fleet. A terminal alias is used to display messaging or link to a URL instead of routing players to an active fleet. For example, you might use a terminal alias when a game version is no longer supported and you want to direct players to an upgrade site. </p> <p>To create a fleet alias, specify an alias name, routing strategy, and optional description. Each simple alias can point to only one fleet, but a fleet can have multiple aliases. If successful, a new alias record is returned, including an alias ID, which you can reference when creating a game session. You can reassign an alias to another fleet by calling <code>UpdateAlias</code>.</p> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn create_alias(&self,
                    input: &CreateAliasInput)
                    -> Result<CreateAliasOutput, CreateAliasError>;


    #[doc="<p>Creates a new Amazon GameLift build from a set of game server binary files stored in an Amazon Simple Storage Service (Amazon S3) location. To use this API call, create a <code>.zip</code> file containing all of the files for the build and store it in an Amazon S3 bucket under your AWS account. For help on packaging your build files and creating a build, see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html\">Uploading Your Game to Amazon GameLift</a>.</p> <important> <p>Use this API action ONLY if you are storing your game build files in an Amazon S3 bucket. To create a build using files stored locally, use the CLI command <a href=\"http://docs.aws.amazon.com/cli/latest/reference/gamelift/upload-build.html\"> <code>upload-build</code> </a>, which uploads the build files from a file location you specify.</p> </important> <p>To create a new build using <code>CreateBuild</code>, identify the storage location and operating system of your game build. You also have the option of specifying a build name and version. If successful, this action creates a new build record with an unique build ID and in <code>INITIALIZED</code> status. Use the API call <a>DescribeBuild</a> to check the status of your build. A build must be in <code>READY</code> status before it can be used to create fleets to host your game.</p> <p>Build-related operations include:</p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul>"]
    fn create_build(&self,
                    input: &CreateBuildInput)
                    -> Result<CreateBuildOutput, CreateBuildError>;


    #[doc="<p>Creates a new fleet to run your game servers. A fleet is a set of Amazon Elastic Compute Cloud (Amazon EC2) instances, each of which can run multiple server processes to host game sessions. You configure a fleet to create instances with certain hardware specifications (see <a href=\"http://aws.amazon.com/ec2/instance-types/\">Amazon EC2 Instance Types</a> for more information), and deploy a specified game build to each instance. A newly created fleet passes through several statuses; once it reaches the <code>ACTIVE</code> status, it can begin hosting game sessions.</p> <p>To create a new fleet, you must specify the following: (1) fleet name, (2) build ID of an uploaded game build, (3) an EC2 instance type, and (4) a run-time configuration that describes which server processes to run on each instance in the fleet. (Although the run-time configuration is not a required parameter, the fleet cannot be successfully activated without it.)</p> <p>You can also configure the new fleet with the following settings:</p> <ul> <li> <p>Fleet description</p> </li> <li> <p>Access permissions for inbound traffic</p> </li> <li> <p>Fleet-wide game session protection</p> </li> <li> <p>Resource creation limit</p> </li> </ul> <p>If you use Amazon CloudWatch for metrics, you can add the new fleet to a metric group. This allows you to view aggregated metrics for a set of fleets. Once you specify a metric group, the new fleet's metrics are included in the metric group's data.</p> <p>If the CreateFleet call is successful, Amazon GameLift performs the following tasks:</p> <ul> <li> <p>Creates a fleet record and sets the status to <code>NEW</code> (followed by other statuses as the fleet is activated).</p> </li> <li> <p>Sets the fleet's target capacity to 1 (desired instances), which causes Amazon GameLift to start one new EC2 instance.</p> </li> <li> <p>Starts launching server processes on the instance. If the fleet is configured to run multiple server processes per instance, Amazon GameLift staggers each launch by a few seconds.</p> </li> <li> <p>Begins writing events to the fleet event log, which can be accessed in the Amazon GameLift console.</p> </li> <li> <p>Sets the fleet's status to <code>ACTIVE</code> as soon as one server process in the fleet is ready to host a game session.</p> </li> </ul> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn create_fleet(&self,
                    input: &CreateFleetInput)
                    -> Result<CreateFleetOutput, CreateFleetError>;


    #[doc="<p>Creates a multiplayer game session for players. This action creates a game session record and assigns an available server process in the specified fleet to host the game session. A fleet must have an <code>ACTIVE</code> status before a game session can be created in it.</p> <p>To create a game session, specify either fleet ID or alias ID and indicate a maximum number of players to allow in the game session. You can also provide a name and game-specific properties for this game session. If successful, a <a>GameSession</a> object is returned containing the game session properties and other settings you specified.</p> <p> <b>Idempotency tokens.</b> You can add a token that uniquely identifies game session requests. This is useful for ensuring that game session requests are idempotent. Multiple requests with the same idempotency token are processed only once; subsequent requests return the original result. All response values are the same with the exception of game session status, which may change.</p> <p> <b>Resource creation limits.</b> If you are creating a game session on a fleet with a resource creation limit policy in force, then you must specify a creator ID. Without this ID, Amazon GameLift has no way to evaluate the policy for this new game session request.</p> <p> <b>Player acceptance policy.</b> By default, newly created game sessions are open to new players. You can restrict new player access by using <a>UpdateGameSession</a> to change the game session's player session creation policy.</p> <p> <b>Game session logs.</b> Logs are retained for all active game sessions for 14 days. To access the logs, call <a>GetGameSessionLogUrl</a> to download the log files.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn create_game_session(&self,
                           input: &CreateGameSessionInput)
                           -> Result<CreateGameSessionOutput, CreateGameSessionError>;


    #[doc="<p>Establishes a new queue for processing requests to place new game sessions. A queue identifies where new game sessions can be hosted -- by specifying a list of destinations (fleets or aliases) -- and how long requests can wait in the queue before timing out. You can set up a queue to try to place game sessions on fleets in multiple regions. To add placement requests to a queue, call <a>StartGameSessionPlacement</a> and reference the queue name.</p> <p> <b>Destination order.</b> When processing a request for a game session, Amazon GameLift tries each destination in order until it finds one with available resources to host the new game session. A queue's default order is determined by how destinations are listed. The default order is overridden when a game session placement request provides player latency information. Player latency information enables Amazon GameLift to prioritize destinations where players report the lowest average latency, as a result placing the new game session where the majority of players will have the best possible gameplay experience.</p> <p> <b>Player latency policies.</b> For placement requests containing player latency information, use player latency policies to protect individual players from very high latencies. With a latency cap, even when a destination can deliver a low latency for most players, the game is not placed where any individual player is reporting latency higher than a policy's maximum. A queue can have multiple latency policies, which are enforced consecutively starting with the policy with the lowest latency cap. Use multiple policies to gradually relax latency controls; for example, you might set a policy with a low latency cap for the first 60 seconds, a second policy with a higher cap for the next 60 seconds, etc. </p> <p>To create a new queue, provide a name, timeout value, a list of destinations and, if desired, a set of latency policies. If successful, a new queue object is returned.</p> <p>Queue-related operations include:</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul>"]
    fn create_game_session_queue
        (&self,
         input: &CreateGameSessionQueueInput)
         -> Result<CreateGameSessionQueueOutput, CreateGameSessionQueueError>;


    #[doc="<p>Defines a new matchmaking configuration for use with FlexMatch. A matchmaking configuration sets out guidelines for matching players and getting the matches into games. You can set up multiple matchmaking configurations to handle the scenarios needed for your game. Each matchmaking request (<a>StartMatchmaking</a>) specifies a configuration for the match and provides player attributes to support the configuration being used. </p> <p>To create a matchmaking configuration, at a minimum you must specify the following: configuration name; a rule set that governs how to evaluate players and find acceptable matches; a game session queue to use when placing a new game session for the match; and the maximum time allowed for a matchmaking attempt.</p> <p> <b>Player acceptance</b> -- In each configuration, you have the option to require that all players accept participation in a proposed match. To enable this feature, set <i>AcceptanceRequired</i> to true and specify a time limit for player acceptance. Players have the option to accept or reject a proposed match, and a match does not move ahead to game session placement unless all matched players accept. </p> <p> <b>Matchmaking status notification</b> -- There are two ways to track the progress of matchmaking tickets: (1) polling ticket status with <a>DescribeMatchmaking</a>; or (2) receiving notifications with Amazon Simple Notification Service (SNS). To use notifications, you first need to set up an SNS topic to receive the notifications, and provide the topic ARN in the matchmaking configuration (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/match-notification.html\"> Setting up Notifications for Matchmaking</a>). Since notifications promise only \"best effort\" delivery, we recommend calling <code>DescribeMatchmaking</code> if no notifications are received within 30 seconds.</p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn create_matchmaking_configuration
        (&self,
         input: &CreateMatchmakingConfigurationInput)
         -> Result<CreateMatchmakingConfigurationOutput, CreateMatchmakingConfigurationError>;


    #[doc="<p>Creates a new rule set for FlexMatch matchmaking. A rule set describes the type of match to create, such as the number and size of teams, and sets the parameters for acceptable player matches, such as minimum skill level or character type. Rule sets are used in matchmaking configurations, which define how matchmaking requests are handled. Each <a>MatchmakingConfiguration</a> uses one rule set; you can set up multiple rule sets to handle the scenarios that suit your game (such as for different game modes), and create a separate matchmaking configuration for each rule set. See additional information on rule set content in the <a>MatchmakingRuleSet</a> structure. For help creating rule sets, including useful examples, see the topic <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/match-intro.html\"> Adding FlexMatch to Your Game</a>.</p> <p>Once created, matchmaking rule sets cannot be changed or deleted, so we recommend checking the rule set syntax using <a>ValidateMatchmakingRuleSet</a>before creating the rule set.</p> <p>To create a matchmaking rule set, provide the set of rules and a unique name. Rule sets must be defined in the same region as the matchmaking configuration they will be used with. Rule sets cannot be edited or deleted. If you need to change a rule set, create a new one with the necessary edits and then update matchmaking configurations to use the new rule set.</p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn create_matchmaking_rule_set
        (&self,
         input: &CreateMatchmakingRuleSetInput)
         -> Result<CreateMatchmakingRuleSetOutput, CreateMatchmakingRuleSetError>;


    #[doc="<p>Adds a player to a game session and creates a player session record. Before a player can be added, a game session must have an <code>ACTIVE</code> status, have a creation policy of <code>ALLOW_ALL</code>, and have an open player slot. To add a group of players to a game session, use <a>CreatePlayerSessions</a>.</p> <p>To create a player session, specify a game session ID, player ID, and optionally a string of player data. If successful, the player is added to the game session and a new <a>PlayerSession</a> object is returned. Player sessions cannot be updated. </p> <p> <i>Available in Amazon GameLift Local.</i> </p> <p>Player-session-related operations include:</p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn create_player_session(&self,
                             input: &CreatePlayerSessionInput)
                             -> Result<CreatePlayerSessionOutput, CreatePlayerSessionError>;


    #[doc="<p>Adds a group of players to a game session. This action is useful with a team matching feature. Before players can be added, a game session must have an <code>ACTIVE</code> status, have a creation policy of <code>ALLOW_ALL</code>, and have an open player slot. To add a single player to a game session, use <a>CreatePlayerSession</a>.</p> <p>To create player sessions, specify a game session ID, a list of player IDs, and optionally a set of player data strings. If successful, the players are added to the game session and a set of new <a>PlayerSession</a> objects is returned. Player sessions cannot be updated.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <p>Player-session-related operations include:</p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn create_player_sessions(&self,
                              input: &CreatePlayerSessionsInput)
                              -> Result<CreatePlayerSessionsOutput, CreatePlayerSessionsError>;


    #[doc="<p>Deletes an alias. This action removes all record of the alias. Game clients attempting to access a server process using the deleted alias receive an error. To delete an alias, specify the alias ID to be deleted.</p> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn delete_alias(&self, input: &DeleteAliasInput) -> Result<(), DeleteAliasError>;


    #[doc="<p>Deletes a build. This action permanently deletes the build record and any uploaded build files.</p> <p>To delete a build, specify its ID. Deleting a build does not affect the status of any active fleets using the build, but you can no longer create new fleets with the deleted build.</p> <p>Build-related operations include:</p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul>"]
    fn delete_build(&self, input: &DeleteBuildInput) -> Result<(), DeleteBuildError>;


    #[doc="<p>Deletes everything related to a fleet. Before deleting a fleet, you must set the fleet's desired capacity to zero. See <a>UpdateFleetCapacity</a>.</p> <p>This action removes the fleet's resources and the fleet record. Once a fleet is deleted, you can no longer use that fleet.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn delete_fleet(&self, input: &DeleteFleetInput) -> Result<(), DeleteFleetError>;


    #[doc="<p>Deletes a game session queue. This action means that any <a>StartGameSessionPlacement</a> requests that reference this queue will fail. To delete a queue, specify the queue name.</p> <p>Queue-related operations include:</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul>"]
    fn delete_game_session_queue
        (&self,
         input: &DeleteGameSessionQueueInput)
         -> Result<DeleteGameSessionQueueOutput, DeleteGameSessionQueueError>;


    #[doc="<p>Permanently removes a FlexMatch matchmaking configuration. To delete, specify the configuration name. A matchmaking configuration cannot be deleted if it is being used in any active matchmaking tickets.</p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn delete_matchmaking_configuration
        (&self,
         input: &DeleteMatchmakingConfigurationInput)
         -> Result<DeleteMatchmakingConfigurationOutput, DeleteMatchmakingConfigurationError>;


    #[doc="<p>Deletes a fleet scaling policy. This action means that the policy is no longer in force and removes all record of it. To delete a scaling policy, specify both the scaling policy name and the fleet ID it is associated with.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn delete_scaling_policy(&self,
                             input: &DeleteScalingPolicyInput)
                             -> Result<(), DeleteScalingPolicyError>;


    #[doc="<p>Retrieves properties for an alias. This operation returns all alias metadata and settings. To get an alias's target fleet ID only, use <code>ResolveAlias</code>. </p> <p>To get alias properties, specify the alias ID. If successful, the requested alias record is returned.</p> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn describe_alias(&self,
                      input: &DescribeAliasInput)
                      -> Result<DescribeAliasOutput, DescribeAliasError>;


    #[doc="<p>Retrieves properties for a build. To get a build record, specify a build ID. If successful, an object containing the build properties is returned.</p> <p>Build-related operations include:</p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul>"]
    fn describe_build(&self,
                      input: &DescribeBuildInput)
                      -> Result<DescribeBuildOutput, DescribeBuildError>;


    #[doc="<p>Retrieves the following information for the specified EC2 instance type:</p> <ul> <li> <p>maximum number of instances allowed per AWS account (service limit)</p> </li> <li> <p>current usage level for the AWS account</p> </li> </ul> <p>Service limits vary depending on region. Available regions for Amazon GameLift can be found in the AWS Management Console for Amazon GameLift (see the drop-down list in the upper right corner).</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_ec2_instance_limits
        (&self,
         input: &DescribeEC2InstanceLimitsInput)
         -> Result<DescribeEC2InstanceLimitsOutput, DescribeEC2InstanceLimitsError>;


    #[doc="<p>Retrieves fleet properties, including metadata, status, and configuration, for one or more fleets. You can request attributes for all fleets, or specify a list of one or more fleet IDs. When requesting multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetAttributes</a> object is returned for each requested fleet ID. When specifying a list of fleet IDs, attribute objects are returned only for fleets that currently exist. </p> <note> <p>Some API actions may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed.</p> </note> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_fleet_attributes
        (&self,
         input: &DescribeFleetAttributesInput)
         -> Result<DescribeFleetAttributesOutput, DescribeFleetAttributesError>;


    #[doc="<p>Retrieves the current status of fleet capacity for one or more fleets. This information includes the number of instances that have been requested for the fleet and the number currently active. You can request capacity for all fleets, or specify a list of one or more fleet IDs. When requesting multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetCapacity</a> object is returned for each requested fleet ID. When specifying a list of fleet IDs, attribute objects are returned only for fleets that currently exist. </p> <note> <p>Some API actions may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed.</p> </note> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_fleet_capacity
        (&self,
         input: &DescribeFleetCapacityInput)
         -> Result<DescribeFleetCapacityOutput, DescribeFleetCapacityError>;


    #[doc="<p>Retrieves entries from the specified fleet's event log. You can specify a time range to limit the result set. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a collection of event log entries matching the request are returned.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_fleet_events(&self,
                             input: &DescribeFleetEventsInput)
                             -> Result<DescribeFleetEventsOutput, DescribeFleetEventsError>;


    #[doc="<p>Retrieves the inbound connection permissions for a fleet. Connection permissions include a range of IP addresses and port settings that incoming traffic can use to access server processes in the fleet. To get a fleet's inbound connection permissions, specify a fleet ID. If successful, a collection of <a>IpPermission</a> objects is returned for the requested fleet ID. If the requested fleet has been deleted, the result set is empty.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_fleet_port_settings
        (&self,
         input: &DescribeFleetPortSettingsInput)
         -> Result<DescribeFleetPortSettingsOutput, DescribeFleetPortSettingsError>;


    #[doc="<p>Retrieves utilization statistics for one or more fleets. You can request utilization data for all fleets, or specify a list of one or more fleet IDs. When requesting multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetUtilization</a> object is returned for each requested fleet ID. When specifying a list of fleet IDs, utilization objects are returned only for fleets that currently exist. </p> <note> <p>Some API actions may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed.</p> </note> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_fleet_utilization
        (&self,
         input: &DescribeFleetUtilizationInput)
         -> Result<DescribeFleetUtilizationOutput, DescribeFleetUtilizationError>;


    #[doc="<p>Retrieves properties, including the protection policy in force, for one or more game sessions. This action can be used in several ways: (1) provide a <code>GameSessionId</code> or <code>GameSessionArn</code> to request details for a specific game session; (2) provide either a <code>FleetId</code> or an <code>AliasId</code> to request properties for all game sessions running on a fleet. </p> <p>To get game session record(s), specify just one of the following: game session ID, fleet ID, or alias ID. You can filter this request by game session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSessionDetail</a> object is returned for each session matching the request.</p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn describe_game_session_details
        (&self,
         input: &DescribeGameSessionDetailsInput)
         -> Result<DescribeGameSessionDetailsOutput, DescribeGameSessionDetailsError>;


    #[doc="<p>Retrieves properties and current status of a game session placement request. To get game session placement details, specify the placement ID. If successful, a <a>GameSessionPlacement</a> object is returned.</p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn describe_game_session_placement
        (&self,
         input: &DescribeGameSessionPlacementInput)
         -> Result<DescribeGameSessionPlacementOutput, DescribeGameSessionPlacementError>;


    #[doc="<p>Retrieves the properties for one or more game session queues. When requesting multiple queues, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSessionQueue</a> object is returned for each requested queue. When specifying a list of queues, objects are returned only for queues that currently exist in the region.</p> <p>Queue-related operations include:</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul>"]
    fn describe_game_session_queues
        (&self,
         input: &DescribeGameSessionQueuesInput)
         -> Result<DescribeGameSessionQueuesOutput, DescribeGameSessionQueuesError>;


    #[doc="<p>Retrieves a set of one or more game sessions. Request a specific game session or request all game sessions on a fleet. Alternatively, use <a>SearchGameSessions</a> to request a set of active game sessions that are filtered by certain criteria. To retrieve protection policy settings for game sessions, use <a>DescribeGameSessionDetails</a>.</p> <p>To get game sessions, specify one of the following: game session ID, fleet ID, or alias ID. You can filter this request by game session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSession</a> object is returned for each game session matching the request.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn describe_game_sessions(&self,
                              input: &DescribeGameSessionsInput)
                              -> Result<DescribeGameSessionsOutput, DescribeGameSessionsError>;


    #[doc="<p>Retrieves information about a fleet's instances, including instance IDs. Use this action to get details on all instances in the fleet or get details on one specific instance.</p> <p>To get a specific instance, specify fleet ID and instance ID. To get all instances in a fleet, specify a fleet ID only. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, an <a>Instance</a> object is returned for each result.</p>"]
    fn describe_instances(&self,
                          input: &DescribeInstancesInput)
                          -> Result<DescribeInstancesOutput, DescribeInstancesError>;


    #[doc="<p>Retrieves a set of one or more matchmaking tickets. Use this operation to retrieve ticket information, including status and--once a successful match is made--acquire connection information for the resulting new game session. </p> <p>You can use this operation to track the progress of matchmaking requests (through polling) as an alternative to using event notifications. See more details on tracking matchmaking requests through polling or notifications in <a>StartMatchmaking</a>. </p> <p>You can request data for a one or a list of ticket IDs. If the request is successful, a ticket object is returned for each requested ID. When specifying a list of ticket IDs, objects are returned only for tickets that currently exist. </p> <p>Matchmaking-related operations include:</p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> </ul>"]
    fn describe_matchmaking(&self,
                            input: &DescribeMatchmakingInput)
                            -> Result<DescribeMatchmakingOutput, DescribeMatchmakingError>;


    #[doc="<p>Retrieves the details of FlexMatch matchmaking configurations. with this operation, you have the following options: (1) retrieve all existing configurations, (2) provide the names of one or more configurations to retrieve, or (3) retrieve all configurations that use a specified rule set name. When requesting multiple items, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a configuration is returned for each requested name. When specifying a list of names, only configurations that currently exist are returned. </p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn describe_matchmaking_configurations
        (&self,
         input: &DescribeMatchmakingConfigurationsInput)
         -> Result<DescribeMatchmakingConfigurationsOutput, DescribeMatchmakingConfigurationsError>;


    #[doc="<p>Retrieves the details for FlexMatch matchmaking rule sets. You can request all existing rule sets for the region, or provide a list of one or more rule set names. When requesting multiple items, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a rule set is returned for each requested name. </p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn describe_matchmaking_rule_sets
        (&self,
         input: &DescribeMatchmakingRuleSetsInput)
         -> Result<DescribeMatchmakingRuleSetsOutput, DescribeMatchmakingRuleSetsError>;


    #[doc="<p>Retrieves properties for one or more player sessions. This action can be used in several ways: (1) provide a <code>PlayerSessionId</code> to request properties for a specific player session; (2) provide a <code>GameSessionId</code> to request properties for all player sessions in the specified game session; (3) provide a <code>PlayerId</code> to request properties for all player sessions of a specified player. </p> <p>To get game session record(s), specify only one of the following: a player session ID, a game session ID, or a player ID. You can filter this request by player session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>PlayerSession</a> object is returned for each session matching the request.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <p>Player-session-related operations include:</p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn describe_player_sessions
        (&self,
         input: &DescribePlayerSessionsInput)
         -> Result<DescribePlayerSessionsOutput, DescribePlayerSessionsError>;


    #[doc="<p>Retrieves the current run-time configuration for the specified fleet. The run-time configuration tells Amazon GameLift how to launch server processes on instances in the fleet.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_runtime_configuration
        (&self,
         input: &DescribeRuntimeConfigurationInput)
         -> Result<DescribeRuntimeConfigurationOutput, DescribeRuntimeConfigurationError>;


    #[doc="<p>Retrieves all scaling policies applied to a fleet.</p> <p>To get a fleet's scaling policies, specify the fleet ID. You can filter this request by policy status, such as to retrieve only active scaling policies. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, set of <a>ScalingPolicy</a> objects is returned for the fleet.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_scaling_policies
        (&self,
         input: &DescribeScalingPoliciesInput)
         -> Result<DescribeScalingPoliciesOutput, DescribeScalingPoliciesError>;


    #[doc="<p>Retrieves the location of stored game session logs for a specified game session. When a game session is terminated, Amazon GameLift automatically stores the logs in Amazon S3 and retains them for 14 days. Use this URL to download the logs.</p> <note> <p>See the <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_gamelift\">AWS Service Limits</a> page for maximum log file sizes. Log files that exceed this limit are not saved.</p> </note> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn get_game_session_log_url
        (&self,
         input: &GetGameSessionLogUrlInput)
         -> Result<GetGameSessionLogUrlOutput, GetGameSessionLogUrlError>;


    #[doc="<p>Requests remote access to a fleet instance. Remote access is useful for debugging, gathering benchmarking data, or watching activity in real time. </p> <p>Access requires credentials that match the operating system of the instance. For a Windows instance, Amazon GameLift returns a user name and password as strings for use with a Windows Remote Desktop client. For a Linux instance, Amazon GameLift returns a user name and RSA private key, also as strings, for use with an SSH client. The private key must be saved in the proper format to a <code>.pem</code> file before using. If you're making this request using the AWS CLI, saving the secret can be handled as part of the GetInstanceAccess request. (See the example later in this topic). For more information on remote access, see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-remote-access.html\">Remotely Accessing an Instance</a>.</p> <p>To request access to a specific instance, specify the IDs of the instance and the fleet it belongs to. If successful, an <a>InstanceAccess</a> object is returned containing the instance's IP address and a set of credentials.</p>"]
    fn get_instance_access(&self,
                           input: &GetInstanceAccessInput)
                           -> Result<GetInstanceAccessOutput, GetInstanceAccessError>;


    #[doc="<p>Retrieves all aliases for this AWS account. You can filter the result set by alias name and/or routing strategy type. Use the pagination parameters to retrieve results in sequential pages.</p> <note> <p>Returned aliases are not listed in any particular order.</p> </note> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn list_aliases(&self,
                    input: &ListAliasesInput)
                    -> Result<ListAliasesOutput, ListAliasesError>;


    #[doc="<p>Retrieves build records for all builds associated with the AWS account in use. You can limit results to builds that are in a specific status by using the <code>Status</code> parameter. Use the pagination parameters to retrieve results in a set of sequential pages. </p> <note> <p>Build records are not listed in any particular order.</p> </note> <p>Build-related operations include:</p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul>"]
    fn list_builds(&self, input: &ListBuildsInput) -> Result<ListBuildsOutput, ListBuildsError>;


    #[doc="<p>Retrieves a collection of fleet records for this AWS account. You can filter the result set by build ID. Use the pagination parameters to retrieve results in sequential pages.</p> <note> <p>Fleet records are not listed in any particular order.</p> </note> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn list_fleets(&self, input: &ListFleetsInput) -> Result<ListFleetsOutput, ListFleetsError>;


    #[doc="<p>Creates or updates a scaling policy for a fleet. An active scaling policy prompts Amazon GameLift to track a certain metric for a fleet and automatically change the fleet's capacity in specific circumstances. Each scaling policy contains one rule statement. Fleets can have multiple scaling policies in force simultaneously.</p> <p>A scaling policy rule statement has the following structure:</p> <p>If <code>[MetricName]</code> is <code>[ComparisonOperator]</code> <code>[Threshold]</code> for <code>[EvaluationPeriods]</code> minutes, then <code>[ScalingAdjustmentType]</code> to/by <code>[ScalingAdjustment]</code>.</p> <p>For example, this policy: \"If the number of idle instances exceeds 20 for more than 15 minutes, then reduce the fleet capacity by 10 instances\" could be implemented as the following rule statement:</p> <p>If [IdleInstances] is [GreaterThanOrEqualToThreshold] [20] for [15] minutes, then [ChangeInCapacity] by [-10].</p> <p>To create or update a scaling policy, specify a unique combination of name and fleet ID, and set the rule values. All parameters for this action are required. If successful, the policy name is returned. Scaling policies cannot be suspended or made inactive. To stop enforcing a scaling policy, call <a>DeleteScalingPolicy</a>.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn put_scaling_policy(&self,
                          input: &PutScalingPolicyInput)
                          -> Result<PutScalingPolicyOutput, PutScalingPolicyError>;


    #[doc="<p> <i>This API call is not currently in use. </i> Retrieves a fresh set of upload credentials and the assigned Amazon S3 storage location for a specific build. Valid credentials are required to upload your game build files to Amazon S3.</p>"]
    fn request_upload_credentials
        (&self,
         input: &RequestUploadCredentialsInput)
         -> Result<RequestUploadCredentialsOutput, RequestUploadCredentialsError>;


    #[doc="<p>Retrieves the fleet ID that a specified alias is currently pointing to.</p> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn resolve_alias(&self,
                     input: &ResolveAliasInput)
                     -> Result<ResolveAliasOutput, ResolveAliasError>;


    #[doc="<p>Retrieves a set of game sessions that match a set of search criteria and sorts them in a specified order. A game session search is limited to a single fleet. Search results include only game sessions that are in <code>ACTIVE</code> status. If you need to retrieve game sessions with a status other than active, use <a>DescribeGameSessions</a>. If you need to retrieve the protection policy for each game session, use <a>DescribeGameSessionDetails</a>.</p> <p>You can search or sort by the following game session attributes:</p> <ul> <li> <p> <b>gameSessionId</b> -- Unique identifier for the game session. You can use either a <code>GameSessionId</code> or <code>GameSessionArn</code> value. </p> </li> <li> <p> <b>gameSessionName</b> -- Name assigned to a game session. This value is set when requesting a new game session with <a>CreateGameSession</a> or updating with <a>UpdateGameSession</a>. Game session names do not need to be unique to a game session.</p> </li> <li> <p> <b>creationTimeMillis</b> -- Value indicating when a game session was created. It is expressed in Unix time as milliseconds.</p> </li> <li> <p> <b>playerSessionCount</b> -- Number of players currently connected to a game session. This value changes rapidly as players join the session or drop out.</p> </li> <li> <p> <b>maximumSessions</b> -- Maximum number of player sessions allowed for a game session. This value is set when requesting a new game session with <a>CreateGameSession</a> or updating with <a>UpdateGameSession</a>.</p> </li> <li> <p> <b>hasAvailablePlayerSessions</b> -- Boolean value indicating whether a game session has reached its maximum number of players. When searching with this attribute, the search value must be <code>true</code> or <code>false</code>. It is highly recommended that all search requests include this filter attribute to optimize search performance and return only sessions that players can join. </p> </li> </ul> <p>To search or sort, specify either a fleet ID or an alias ID, and provide a search filter expression, a sort expression, or both. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a collection of <a>GameSession</a> objects matching the request is returned.</p> <note> <p>Returned values for <code>playerSessionCount</code> and <code>hasAvailablePlayerSessions</code> change quickly as players join sessions and others drop out. Results should be considered a snapshot in time. Be sure to refresh search results often, and handle sessions that fill up before a player can join. </p> </note> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn search_game_sessions(&self,
                            input: &SearchGameSessionsInput)
                            -> Result<SearchGameSessionsOutput, SearchGameSessionsError>;


    #[doc="<p>Places a request for a new game session in a queue (see <a>CreateGameSessionQueue</a>). When processing a placement request, Amazon GameLift searches for available resources on the queue's destinations, scanning each until it finds resources or the placement request times out.</p> <p>A game session placement request can also request player sessions. When a new game session is successfully created, Amazon GameLift creates a player session for each player included in the request.</p> <p>When placing a game session, by default Amazon GameLift tries each fleet in the order they are listed in the queue configuration. Ideally, a queue's destinations are listed in preference order.</p> <p>Alternatively, when requesting a game session with players, you can also provide latency data for each player in relevant regions. Latency data indicates the performance lag a player experiences when connected to a fleet in the region. Amazon GameLift uses latency data to reorder the list of destinations to place the game session in a region with minimal lag. If latency data is provided for multiple players, Amazon GameLift calculates each region's average lag for all players and reorders to get the best game play across all players. </p> <p>To place a new game session request, specify the following:</p> <ul> <li> <p>The queue name and a set of game session properties and settings</p> </li> <li> <p>A unique ID (such as a UUID) for the placement. You use this ID to track the status of the placement request</p> </li> <li> <p>(Optional) A set of IDs and player data for each player you want to join to the new game session</p> </li> <li> <p>Latency data for all players (if you want to optimize game play for the players)</p> </li> </ul> <p>If successful, a new game session placement is created.</p> <p>To track the status of a placement request, call <a>DescribeGameSessionPlacement</a> and check the request's status. If the status is <code>FULFILLED</code>, a new game session has been created and a game session ARN and region are referenced. If the placement request times out, you can resubmit the request or retry it with a different queue. </p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn start_game_session_placement
        (&self,
         input: &StartGameSessionPlacementInput)
         -> Result<StartGameSessionPlacementOutput, StartGameSessionPlacementError>;


    #[doc="<p>Uses FlexMatch to create a game match for a group of players based on custom matchmaking rules, and starts a new game for the matched players. Each matchmaking request specifies the type of match to build (team configuration, rules for an acceptable match, etc.). The request also specifies the players to find a match for and where to host the new game session for optimal performance. A matchmaking request might start with a single player or a group of players who want to play together. FlexMatch finds additional players as needed to fill the match. Match type, rules, and the queue used to place a new game session are defined in a <code>MatchmakingConfiguration</code>. For complete information on setting up and using FlexMatch, see the topic <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/match-intro.html\"> Adding FlexMatch to Your Game</a>.</p> <p>To start matchmaking, provide a unique ticket ID, specify a matchmaking configuration, and include the players to be matched. You must also include a set of player attributes relevant for the matchmaking configuration. If successful, a matchmaking ticket is returned with status set to <code>QUEUED</code>. Track the status of the ticket to respond as needed and acquire game session connection information for sucessfully completed matches.</p> <p> <b>Tracking ticket status</b> -- A couple of options are available for tracking the status of matchmaking requests: </p> <ul> <li> <p>Polling -- Call <code>DescribeMatchmaking</code>. This operation returns the full ticket object, including current status and (for completed tickets) game session connection info. We recommend polling no more than once every 10 seconds.</p> </li> <li> <p>Notifications -- Get event notifications for changes in ticket status using Amazon Simple Notification Service (SNS). Notifications are easy to set up (see <a>CreateMatchmakingConfiguration</a>) and typically deliver match status changes faster and more efficiently than polling. We recommend that you use polling to back up to notifications (since delivery is not guaranteed) and call <code>DescribeMatchmaking</code> only when notifications are not received within 30 seconds.</p> </li> </ul> <p> <b>Processing a matchmaking request</b> -- FlexMatch handles a matchmaking request as follows: </p> <ol> <li> <p>Your client code submits a <code>StartMatchmaking</code> request for one or more players and tracks the status of the request ticket. </p> </li> <li> <p>FlexMatch uses this ticket and others in process to build an acceptable match. When a potential match is identified, all tickets in the proposed match are advanced to the next status. </p> </li> <li> <p>If the match requires player acceptance (set in the matchmaking configuration), the tickets move into status <code>REQUIRES_ACCEPTANCE</code>. This status triggers your client code to solicit acceptance from all players in every ticket involved in the match, and then call <a>AcceptMatch</a> for each player. If any player rejects or fails to accept the match before a specified timeout, the proposed match is dropped (see <code>AcceptMatch</code> for more details).</p> </li> <li> <p>Once a match is proposed and accepted, the matchmaking tickets move into status <code>PLACING</code>. FlexMatch locates resources for a new game session using the game session queue (set in the matchmaking configuration) and creates the game session based on the match data. </p> </li> <li> <p>When the match is successfully placed, the matchmaking tickets move into <code>COMPLETED</code> status. Connection information (including game session endpoint and player session) is added to the matchmaking tickets. Matched players can use the connection information to join the game. </p> </li> </ol> <p>Matchmaking-related operations include:</p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> </ul>"]
    fn start_matchmaking(&self,
                         input: &StartMatchmakingInput)
                         -> Result<StartMatchmakingOutput, StartMatchmakingError>;


    #[doc="<p>Cancels a game session placement that is in <code>PENDING</code> status. To stop a placement, provide the placement ID values. If successful, the placement is moved to <code>CANCELLED</code> status.</p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn stop_game_session_placement
        (&self,
         input: &StopGameSessionPlacementInput)
         -> Result<StopGameSessionPlacementOutput, StopGameSessionPlacementError>;


    #[doc="<p>Cancels a matchmaking ticket that is currently being processed. To stop the matchmaking operation, specify the ticket ID. If successful, work on the ticket is stopped, and the ticket status is changed to <code>CANCELLED</code>.</p> <p>Matchmaking-related operations include:</p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> </ul>"]
    fn stop_matchmaking(&self,
                        input: &StopMatchmakingInput)
                        -> Result<StopMatchmakingOutput, StopMatchmakingError>;


    #[doc="<p>Updates properties for an alias. To update properties, specify the alias ID to be updated and provide the information to be changed. To reassign an alias to another fleet, provide an updated routing strategy. If successful, the updated alias record is returned.</p> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn update_alias(&self,
                    input: &UpdateAliasInput)
                    -> Result<UpdateAliasOutput, UpdateAliasError>;


    #[doc="<p>Updates metadata in a build record, including the build name and version. To update the metadata, specify the build ID to update and provide the new values. If successful, a build object containing the updated metadata is returned.</p> <p>Build-related operations include:</p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul>"]
    fn update_build(&self,
                    input: &UpdateBuildInput)
                    -> Result<UpdateBuildOutput, UpdateBuildError>;


    #[doc="<p>Updates fleet properties, including name and description, for a fleet. To update metadata, specify the fleet ID and the property values that you want to change. If successful, the fleet ID for the updated fleet is returned.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn update_fleet_attributes
        (&self,
         input: &UpdateFleetAttributesInput)
         -> Result<UpdateFleetAttributesOutput, UpdateFleetAttributesError>;


    #[doc="<p>Updates capacity settings for a fleet. Use this action to specify the number of EC2 instances (hosts) that you want this fleet to contain. Before calling this action, you may want to call <a>DescribeEC2InstanceLimits</a> to get the maximum capacity based on the fleet's EC2 instance type.</p> <p>If you're using autoscaling (see <a>PutScalingPolicy</a>), you may want to specify a minimum and/or maximum capacity. If you don't provide these, autoscaling can set capacity anywhere between zero and the <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_gamelift\">service limits</a>.</p> <p>To update fleet capacity, specify the fleet ID and the number of instances you want the fleet to host. If successful, Amazon GameLift starts or terminates instances so that the fleet's active instance count matches the desired instance count. You can view a fleet's current capacity information by calling <a>DescribeFleetCapacity</a>. If the desired instance count is higher than the instance type's limit, the \"Limit Exceeded\" exception occurs.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn update_fleet_capacity(&self,
                             input: &UpdateFleetCapacityInput)
                             -> Result<UpdateFleetCapacityOutput, UpdateFleetCapacityError>;


    #[doc="<p>Updates port settings for a fleet. To update settings, specify the fleet ID to be updated and list the permissions you want to update. List the permissions you want to add in <code>InboundPermissionAuthorizations</code>, and permissions you want to remove in <code>InboundPermissionRevocations</code>. Permissions to be removed must match existing fleet permissions. If successful, the fleet ID for the updated fleet is returned.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn update_fleet_port_settings
        (&self,
         input: &UpdateFleetPortSettingsInput)
         -> Result<UpdateFleetPortSettingsOutput, UpdateFleetPortSettingsError>;


    #[doc="<p>Updates game session properties. This includes the session name, maximum player count, protection policy, which controls whether or not an active game session can be terminated during a scale-down event, and the player session creation policy, which controls whether or not new players can join the session. To update a game session, specify the game session ID and the values you want to change. If successful, an updated <a>GameSession</a> object is returned. </p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn update_game_session(&self,
                           input: &UpdateGameSessionInput)
                           -> Result<UpdateGameSessionOutput, UpdateGameSessionError>;


    #[doc="<p>Updates settings for a game session queue, which determines how new game session requests in the queue are processed. To update settings, specify the queue name to be updated and provide the new settings. When updating destinations, provide a complete list of destinations. </p> <p>Queue-related operations include:</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul>"]
    fn update_game_session_queue
        (&self,
         input: &UpdateGameSessionQueueInput)
         -> Result<UpdateGameSessionQueueOutput, UpdateGameSessionQueueError>;


    #[doc="<p>Updates settings for a FlexMatch matchmaking configuration. To update settings, specify the configuration name to be updated and provide the new settings. </p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn update_matchmaking_configuration
        (&self,
         input: &UpdateMatchmakingConfigurationInput)
         -> Result<UpdateMatchmakingConfigurationOutput, UpdateMatchmakingConfigurationError>;


    #[doc="<p>Updates the current run-time configuration for the specified fleet, which tells Amazon GameLift how to launch server processes on instances in the fleet. You can update a fleet's run-time configuration at any time after the fleet is created; it does not need to be in an <code>ACTIVE</code> status.</p> <p>To update run-time configuration, specify the fleet ID and provide a <code>RuntimeConfiguration</code> object with the updated collection of server process configurations.</p> <p>Each instance in a Amazon GameLift fleet checks regularly for an updated run-time configuration and changes how it launches server processes to comply with the latest version. Existing server processes are not affected by the update; they continue to run until they end, while Amazon GameLift simply adds new server processes to fit the current run-time configuration. As a result, the run-time configuration changes are applied gradually as existing processes shut down and new processes are launched in Amazon GameLift's normal process recycling activity.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn update_runtime_configuration
        (&self,
         input: &UpdateRuntimeConfigurationInput)
         -> Result<UpdateRuntimeConfigurationOutput, UpdateRuntimeConfigurationError>;


    #[doc="<p>Validates the syntax of a matchmaking rule or rule set. This operation checks that the rule set uses syntactically correct JSON and that it conforms to allowed property expressions. To validate syntax, provide a rule set string.</p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn validate_matchmaking_rule_set
        (&self,
         input: &ValidateMatchmakingRuleSetInput)
         -> Result<ValidateMatchmakingRuleSetOutput, ValidateMatchmakingRuleSetError>;
}
/// A client for the Amazon GameLift API.
pub struct GameLiftClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> GameLiftClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        GameLiftClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> GameLift for GameLiftClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Registers a player's acceptance or rejection of a proposed FlexMatch match. A matchmaking configuration may require player acceptance; if so, then matches built with that configuration cannot be completed unless all players accept the proposed match within a specified time limit. </p> <p>When FlexMatch builds a match, all the matchmaking tickets involved in the proposed match are placed into status <code>REQUIRES_ACCEPTANCE</code>. This is a trigger for your game to get acceptance from all players in the ticket. Acceptances are only valid for tickets when they are in this status; all other acceptances result in an error.</p> <p>To register acceptance, specify the ticket ID, a response, and one or more players. Once all players have registered acceptance, the matchmaking tickets advance to status <code>PLACING</code>, where a new game session is created for the match. </p> <p>If any player rejects the match, or if acceptances are not received before a specified timeout, the proposed match is dropped. The matchmaking tickets are then handled in one of two ways: For tickets where all players accepted the match, the ticket status is returned to <code>SEARCHING</code> to find a new match. For tickets where one or more players failed to accept the match, the ticket status is set to <code>FAILED</code>, and processing is terminated. A new matchmaking request for these players can be submitted as needed. </p> <p>Matchmaking-related operations include:</p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> </ul>"]
    fn accept_match(&self,
                    input: &AcceptMatchInput)
                    -> Result<AcceptMatchOutput, AcceptMatchError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.AcceptMatch");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AcceptMatchOutput>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AcceptMatchError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates an alias for a fleet. In most situations, you can use an alias ID in place of a fleet ID. By using a fleet alias instead of a specific fleet ID, you can switch gameplay and players to a new fleet without changing your game client or other game components. For example, for games in production, using an alias allows you to seamlessly redirect your player base to a new game server update. </p> <p>Amazon GameLift supports two types of routing strategies for aliases: simple and terminal. A simple alias points to an active fleet. A terminal alias is used to display messaging or link to a URL instead of routing players to an active fleet. For example, you might use a terminal alias when a game version is no longer supported and you want to direct players to an upgrade site. </p> <p>To create a fleet alias, specify an alias name, routing strategy, and optional description. Each simple alias can point to only one fleet, but a fleet can have multiple aliases. If successful, a new alias record is returned, including an alias ID, which you can reference when creating a game session. You can reassign an alias to another fleet by calling <code>UpdateAlias</code>.</p> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn create_alias(&self,
                    input: &CreateAliasInput)
                    -> Result<CreateAliasOutput, CreateAliasError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.CreateAlias");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateAliasOutput>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateAliasError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new Amazon GameLift build from a set of game server binary files stored in an Amazon Simple Storage Service (Amazon S3) location. To use this API call, create a <code>.zip</code> file containing all of the files for the build and store it in an Amazon S3 bucket under your AWS account. For help on packaging your build files and creating a build, see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html\">Uploading Your Game to Amazon GameLift</a>.</p> <important> <p>Use this API action ONLY if you are storing your game build files in an Amazon S3 bucket. To create a build using files stored locally, use the CLI command <a href=\"http://docs.aws.amazon.com/cli/latest/reference/gamelift/upload-build.html\"> <code>upload-build</code> </a>, which uploads the build files from a file location you specify.</p> </important> <p>To create a new build using <code>CreateBuild</code>, identify the storage location and operating system of your game build. You also have the option of specifying a build name and version. If successful, this action creates a new build record with an unique build ID and in <code>INITIALIZED</code> status. Use the API call <a>DescribeBuild</a> to check the status of your build. A build must be in <code>READY</code> status before it can be used to create fleets to host your game.</p> <p>Build-related operations include:</p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul>"]
    fn create_build(&self,
                    input: &CreateBuildInput)
                    -> Result<CreateBuildOutput, CreateBuildError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.CreateBuild");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateBuildOutput>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateBuildError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new fleet to run your game servers. A fleet is a set of Amazon Elastic Compute Cloud (Amazon EC2) instances, each of which can run multiple server processes to host game sessions. You configure a fleet to create instances with certain hardware specifications (see <a href=\"http://aws.amazon.com/ec2/instance-types/\">Amazon EC2 Instance Types</a> for more information), and deploy a specified game build to each instance. A newly created fleet passes through several statuses; once it reaches the <code>ACTIVE</code> status, it can begin hosting game sessions.</p> <p>To create a new fleet, you must specify the following: (1) fleet name, (2) build ID of an uploaded game build, (3) an EC2 instance type, and (4) a run-time configuration that describes which server processes to run on each instance in the fleet. (Although the run-time configuration is not a required parameter, the fleet cannot be successfully activated without it.)</p> <p>You can also configure the new fleet with the following settings:</p> <ul> <li> <p>Fleet description</p> </li> <li> <p>Access permissions for inbound traffic</p> </li> <li> <p>Fleet-wide game session protection</p> </li> <li> <p>Resource creation limit</p> </li> </ul> <p>If you use Amazon CloudWatch for metrics, you can add the new fleet to a metric group. This allows you to view aggregated metrics for a set of fleets. Once you specify a metric group, the new fleet's metrics are included in the metric group's data.</p> <p>If the CreateFleet call is successful, Amazon GameLift performs the following tasks:</p> <ul> <li> <p>Creates a fleet record and sets the status to <code>NEW</code> (followed by other statuses as the fleet is activated).</p> </li> <li> <p>Sets the fleet's target capacity to 1 (desired instances), which causes Amazon GameLift to start one new EC2 instance.</p> </li> <li> <p>Starts launching server processes on the instance. If the fleet is configured to run multiple server processes per instance, Amazon GameLift staggers each launch by a few seconds.</p> </li> <li> <p>Begins writing events to the fleet event log, which can be accessed in the Amazon GameLift console.</p> </li> <li> <p>Sets the fleet's status to <code>ACTIVE</code> as soon as one server process in the fleet is ready to host a game session.</p> </li> </ul> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn create_fleet(&self,
                    input: &CreateFleetInput)
                    -> Result<CreateFleetOutput, CreateFleetError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.CreateFleet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateFleetOutput>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateFleetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a multiplayer game session for players. This action creates a game session record and assigns an available server process in the specified fleet to host the game session. A fleet must have an <code>ACTIVE</code> status before a game session can be created in it.</p> <p>To create a game session, specify either fleet ID or alias ID and indicate a maximum number of players to allow in the game session. You can also provide a name and game-specific properties for this game session. If successful, a <a>GameSession</a> object is returned containing the game session properties and other settings you specified.</p> <p> <b>Idempotency tokens.</b> You can add a token that uniquely identifies game session requests. This is useful for ensuring that game session requests are idempotent. Multiple requests with the same idempotency token are processed only once; subsequent requests return the original result. All response values are the same with the exception of game session status, which may change.</p> <p> <b>Resource creation limits.</b> If you are creating a game session on a fleet with a resource creation limit policy in force, then you must specify a creator ID. Without this ID, Amazon GameLift has no way to evaluate the policy for this new game session request.</p> <p> <b>Player acceptance policy.</b> By default, newly created game sessions are open to new players. You can restrict new player access by using <a>UpdateGameSession</a> to change the game session's player session creation policy.</p> <p> <b>Game session logs.</b> Logs are retained for all active game sessions for 14 days. To access the logs, call <a>GetGameSessionLogUrl</a> to download the log files.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn create_game_session(&self,
                           input: &CreateGameSessionInput)
                           -> Result<CreateGameSessionOutput, CreateGameSessionError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.CreateGameSession");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateGameSessionOutput>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateGameSessionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Establishes a new queue for processing requests to place new game sessions. A queue identifies where new game sessions can be hosted -- by specifying a list of destinations (fleets or aliases) -- and how long requests can wait in the queue before timing out. You can set up a queue to try to place game sessions on fleets in multiple regions. To add placement requests to a queue, call <a>StartGameSessionPlacement</a> and reference the queue name.</p> <p> <b>Destination order.</b> When processing a request for a game session, Amazon GameLift tries each destination in order until it finds one with available resources to host the new game session. A queue's default order is determined by how destinations are listed. The default order is overridden when a game session placement request provides player latency information. Player latency information enables Amazon GameLift to prioritize destinations where players report the lowest average latency, as a result placing the new game session where the majority of players will have the best possible gameplay experience.</p> <p> <b>Player latency policies.</b> For placement requests containing player latency information, use player latency policies to protect individual players from very high latencies. With a latency cap, even when a destination can deliver a low latency for most players, the game is not placed where any individual player is reporting latency higher than a policy's maximum. A queue can have multiple latency policies, which are enforced consecutively starting with the policy with the lowest latency cap. Use multiple policies to gradually relax latency controls; for example, you might set a policy with a low latency cap for the first 60 seconds, a second policy with a higher cap for the next 60 seconds, etc. </p> <p>To create a new queue, provide a name, timeout value, a list of destinations and, if desired, a set of latency policies. If successful, a new queue object is returned.</p> <p>Queue-related operations include:</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul>"]
    fn create_game_session_queue
        (&self,
         input: &CreateGameSessionQueueInput)
         -> Result<CreateGameSessionQueueOutput, CreateGameSessionQueueError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.CreateGameSessionQueue");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateGameSessionQueueOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateGameSessionQueueError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Defines a new matchmaking configuration for use with FlexMatch. A matchmaking configuration sets out guidelines for matching players and getting the matches into games. You can set up multiple matchmaking configurations to handle the scenarios needed for your game. Each matchmaking request (<a>StartMatchmaking</a>) specifies a configuration for the match and provides player attributes to support the configuration being used. </p> <p>To create a matchmaking configuration, at a minimum you must specify the following: configuration name; a rule set that governs how to evaluate players and find acceptable matches; a game session queue to use when placing a new game session for the match; and the maximum time allowed for a matchmaking attempt.</p> <p> <b>Player acceptance</b> -- In each configuration, you have the option to require that all players accept participation in a proposed match. To enable this feature, set <i>AcceptanceRequired</i> to true and specify a time limit for player acceptance. Players have the option to accept or reject a proposed match, and a match does not move ahead to game session placement unless all matched players accept. </p> <p> <b>Matchmaking status notification</b> -- There are two ways to track the progress of matchmaking tickets: (1) polling ticket status with <a>DescribeMatchmaking</a>; or (2) receiving notifications with Amazon Simple Notification Service (SNS). To use notifications, you first need to set up an SNS topic to receive the notifications, and provide the topic ARN in the matchmaking configuration (see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/match-notification.html\"> Setting up Notifications for Matchmaking</a>). Since notifications promise only \"best effort\" delivery, we recommend calling <code>DescribeMatchmaking</code> if no notifications are received within 30 seconds.</p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn create_matchmaking_configuration
        (&self,
         input: &CreateMatchmakingConfigurationInput)
         -> Result<CreateMatchmakingConfigurationOutput, CreateMatchmakingConfigurationError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.CreateMatchmakingConfiguration");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateMatchmakingConfigurationOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateMatchmakingConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new rule set for FlexMatch matchmaking. A rule set describes the type of match to create, such as the number and size of teams, and sets the parameters for acceptable player matches, such as minimum skill level or character type. Rule sets are used in matchmaking configurations, which define how matchmaking requests are handled. Each <a>MatchmakingConfiguration</a> uses one rule set; you can set up multiple rule sets to handle the scenarios that suit your game (such as for different game modes), and create a separate matchmaking configuration for each rule set. See additional information on rule set content in the <a>MatchmakingRuleSet</a> structure. For help creating rule sets, including useful examples, see the topic <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/match-intro.html\"> Adding FlexMatch to Your Game</a>.</p> <p>Once created, matchmaking rule sets cannot be changed or deleted, so we recommend checking the rule set syntax using <a>ValidateMatchmakingRuleSet</a>before creating the rule set.</p> <p>To create a matchmaking rule set, provide the set of rules and a unique name. Rule sets must be defined in the same region as the matchmaking configuration they will be used with. Rule sets cannot be edited or deleted. If you need to change a rule set, create a new one with the necessary edits and then update matchmaking configurations to use the new rule set.</p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn create_matchmaking_rule_set
        (&self,
         input: &CreateMatchmakingRuleSetInput)
         -> Result<CreateMatchmakingRuleSetOutput, CreateMatchmakingRuleSetError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.CreateMatchmakingRuleSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateMatchmakingRuleSetOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateMatchmakingRuleSetError::from_body(String::from_utf8_lossy(&body)
                                                                 .as_ref()))
            }
        }
    }


    #[doc="<p>Adds a player to a game session and creates a player session record. Before a player can be added, a game session must have an <code>ACTIVE</code> status, have a creation policy of <code>ALLOW_ALL</code>, and have an open player slot. To add a group of players to a game session, use <a>CreatePlayerSessions</a>.</p> <p>To create a player session, specify a game session ID, player ID, and optionally a string of player data. If successful, the player is added to the game session and a new <a>PlayerSession</a> object is returned. Player sessions cannot be updated. </p> <p> <i>Available in Amazon GameLift Local.</i> </p> <p>Player-session-related operations include:</p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn create_player_session(&self,
                             input: &CreatePlayerSessionInput)
                             -> Result<CreatePlayerSessionOutput, CreatePlayerSessionError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.CreatePlayerSession");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreatePlayerSessionOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreatePlayerSessionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Adds a group of players to a game session. This action is useful with a team matching feature. Before players can be added, a game session must have an <code>ACTIVE</code> status, have a creation policy of <code>ALLOW_ALL</code>, and have an open player slot. To add a single player to a game session, use <a>CreatePlayerSession</a>.</p> <p>To create player sessions, specify a game session ID, a list of player IDs, and optionally a set of player data strings. If successful, the players are added to the game session and a set of new <a>PlayerSession</a> objects is returned. Player sessions cannot be updated.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <p>Player-session-related operations include:</p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn create_player_sessions(&self,
                              input: &CreatePlayerSessionsInput)
                              -> Result<CreatePlayerSessionsOutput, CreatePlayerSessionsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.CreatePlayerSessions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreatePlayerSessionsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreatePlayerSessionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes an alias. This action removes all record of the alias. Game clients attempting to access a server process using the deleted alias receive an error. To delete an alias, specify the alias ID to be deleted.</p> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn delete_alias(&self, input: &DeleteAliasInput) -> Result<(), DeleteAliasError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DeleteAlias");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteAliasError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a build. This action permanently deletes the build record and any uploaded build files.</p> <p>To delete a build, specify its ID. Deleting a build does not affect the status of any active fleets using the build, but you can no longer create new fleets with the deleted build.</p> <p>Build-related operations include:</p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul>"]
    fn delete_build(&self, input: &DeleteBuildInput) -> Result<(), DeleteBuildError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DeleteBuild");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBuildError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes everything related to a fleet. Before deleting a fleet, you must set the fleet's desired capacity to zero. See <a>UpdateFleetCapacity</a>.</p> <p>This action removes the fleet's resources and the fleet record. Once a fleet is deleted, you can no longer use that fleet.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn delete_fleet(&self, input: &DeleteFleetInput) -> Result<(), DeleteFleetError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DeleteFleet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteFleetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a game session queue. This action means that any <a>StartGameSessionPlacement</a> requests that reference this queue will fail. To delete a queue, specify the queue name.</p> <p>Queue-related operations include:</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul>"]
    fn delete_game_session_queue
        (&self,
         input: &DeleteGameSessionQueueInput)
         -> Result<DeleteGameSessionQueueOutput, DeleteGameSessionQueueError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DeleteGameSessionQueue");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteGameSessionQueueOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteGameSessionQueueError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Permanently removes a FlexMatch matchmaking configuration. To delete, specify the configuration name. A matchmaking configuration cannot be deleted if it is being used in any active matchmaking tickets.</p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn delete_matchmaking_configuration
        (&self,
         input: &DeleteMatchmakingConfigurationInput)
         -> Result<DeleteMatchmakingConfigurationOutput, DeleteMatchmakingConfigurationError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DeleteMatchmakingConfiguration");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteMatchmakingConfigurationOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteMatchmakingConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a fleet scaling policy. This action means that the policy is no longer in force and removes all record of it. To delete a scaling policy, specify both the scaling policy name and the fleet ID it is associated with.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn delete_scaling_policy(&self,
                             input: &DeleteScalingPolicyInput)
                             -> Result<(), DeleteScalingPolicyError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DeleteScalingPolicy");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteScalingPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves properties for an alias. This operation returns all alias metadata and settings. To get an alias's target fleet ID only, use <code>ResolveAlias</code>. </p> <p>To get alias properties, specify the alias ID. If successful, the requested alias record is returned.</p> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn describe_alias(&self,
                      input: &DescribeAliasInput)
                      -> Result<DescribeAliasOutput, DescribeAliasError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeAlias");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeAliasOutput>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeAliasError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves properties for a build. To get a build record, specify a build ID. If successful, an object containing the build properties is returned.</p> <p>Build-related operations include:</p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul>"]
    fn describe_build(&self,
                      input: &DescribeBuildInput)
                      -> Result<DescribeBuildOutput, DescribeBuildError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeBuild");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeBuildOutput>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeBuildError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the following information for the specified EC2 instance type:</p> <ul> <li> <p>maximum number of instances allowed per AWS account (service limit)</p> </li> <li> <p>current usage level for the AWS account</p> </li> </ul> <p>Service limits vary depending on region. Available regions for Amazon GameLift can be found in the AWS Management Console for Amazon GameLift (see the drop-down list in the upper right corner).</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_ec2_instance_limits
        (&self,
         input: &DescribeEC2InstanceLimitsInput)
         -> Result<DescribeEC2InstanceLimitsOutput, DescribeEC2InstanceLimitsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeEC2InstanceLimits");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeEC2InstanceLimitsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeEC2InstanceLimitsError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves fleet properties, including metadata, status, and configuration, for one or more fleets. You can request attributes for all fleets, or specify a list of one or more fleet IDs. When requesting multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetAttributes</a> object is returned for each requested fleet ID. When specifying a list of fleet IDs, attribute objects are returned only for fleets that currently exist. </p> <note> <p>Some API actions may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed.</p> </note> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_fleet_attributes
        (&self,
         input: &DescribeFleetAttributesInput)
         -> Result<DescribeFleetAttributesOutput, DescribeFleetAttributesError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeFleetAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeFleetAttributesOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeFleetAttributesError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the current status of fleet capacity for one or more fleets. This information includes the number of instances that have been requested for the fleet and the number currently active. You can request capacity for all fleets, or specify a list of one or more fleet IDs. When requesting multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetCapacity</a> object is returned for each requested fleet ID. When specifying a list of fleet IDs, attribute objects are returned only for fleets that currently exist. </p> <note> <p>Some API actions may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed.</p> </note> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_fleet_capacity
        (&self,
         input: &DescribeFleetCapacityInput)
         -> Result<DescribeFleetCapacityOutput, DescribeFleetCapacityError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeFleetCapacity");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeFleetCapacityOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeFleetCapacityError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves entries from the specified fleet's event log. You can specify a time range to limit the result set. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a collection of event log entries matching the request are returned.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_fleet_events(&self,
                             input: &DescribeFleetEventsInput)
                             -> Result<DescribeFleetEventsOutput, DescribeFleetEventsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeFleetEvents");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeFleetEventsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeFleetEventsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the inbound connection permissions for a fleet. Connection permissions include a range of IP addresses and port settings that incoming traffic can use to access server processes in the fleet. To get a fleet's inbound connection permissions, specify a fleet ID. If successful, a collection of <a>IpPermission</a> objects is returned for the requested fleet ID. If the requested fleet has been deleted, the result set is empty.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_fleet_port_settings
        (&self,
         input: &DescribeFleetPortSettingsInput)
         -> Result<DescribeFleetPortSettingsOutput, DescribeFleetPortSettingsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeFleetPortSettings");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeFleetPortSettingsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeFleetPortSettingsError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves utilization statistics for one or more fleets. You can request utilization data for all fleets, or specify a list of one or more fleet IDs. When requesting multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetUtilization</a> object is returned for each requested fleet ID. When specifying a list of fleet IDs, utilization objects are returned only for fleets that currently exist. </p> <note> <p>Some API actions may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed.</p> </note> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_fleet_utilization
        (&self,
         input: &DescribeFleetUtilizationInput)
         -> Result<DescribeFleetUtilizationOutput, DescribeFleetUtilizationError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeFleetUtilization");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeFleetUtilizationOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeFleetUtilizationError::from_body(String::from_utf8_lossy(&body)
                                                                 .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves properties, including the protection policy in force, for one or more game sessions. This action can be used in several ways: (1) provide a <code>GameSessionId</code> or <code>GameSessionArn</code> to request details for a specific game session; (2) provide either a <code>FleetId</code> or an <code>AliasId</code> to request properties for all game sessions running on a fleet. </p> <p>To get game session record(s), specify just one of the following: game session ID, fleet ID, or alias ID. You can filter this request by game session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSessionDetail</a> object is returned for each session matching the request.</p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn describe_game_session_details
        (&self,
         input: &DescribeGameSessionDetailsInput)
         -> Result<DescribeGameSessionDetailsOutput, DescribeGameSessionDetailsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeGameSessionDetails");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeGameSessionDetailsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeGameSessionDetailsError::from_body(String::from_utf8_lossy(&body)
                                                                   .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves properties and current status of a game session placement request. To get game session placement details, specify the placement ID. If successful, a <a>GameSessionPlacement</a> object is returned.</p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn describe_game_session_placement
        (&self,
         input: &DescribeGameSessionPlacementInput)
         -> Result<DescribeGameSessionPlacementOutput, DescribeGameSessionPlacementError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeGameSessionPlacement");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeGameSessionPlacementOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeGameSessionPlacementError::from_body(String::from_utf8_lossy(&body)
                                                                     .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the properties for one or more game session queues. When requesting multiple queues, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSessionQueue</a> object is returned for each requested queue. When specifying a list of queues, objects are returned only for queues that currently exist in the region.</p> <p>Queue-related operations include:</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul>"]
    fn describe_game_session_queues
        (&self,
         input: &DescribeGameSessionQueuesInput)
         -> Result<DescribeGameSessionQueuesOutput, DescribeGameSessionQueuesError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeGameSessionQueues");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeGameSessionQueuesOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeGameSessionQueuesError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves a set of one or more game sessions. Request a specific game session or request all game sessions on a fleet. Alternatively, use <a>SearchGameSessions</a> to request a set of active game sessions that are filtered by certain criteria. To retrieve protection policy settings for game sessions, use <a>DescribeGameSessionDetails</a>.</p> <p>To get game sessions, specify one of the following: game session ID, fleet ID, or alias ID. You can filter this request by game session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSession</a> object is returned for each game session matching the request.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn describe_game_sessions(&self,
                              input: &DescribeGameSessionsInput)
                              -> Result<DescribeGameSessionsOutput, DescribeGameSessionsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeGameSessions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeGameSessionsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeGameSessionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves information about a fleet's instances, including instance IDs. Use this action to get details on all instances in the fleet or get details on one specific instance.</p> <p>To get a specific instance, specify fleet ID and instance ID. To get all instances in a fleet, specify a fleet ID only. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, an <a>Instance</a> object is returned for each result.</p>"]
    fn describe_instances(&self,
                          input: &DescribeInstancesInput)
                          -> Result<DescribeInstancesOutput, DescribeInstancesError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeInstances");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeInstancesOutput>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeInstancesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves a set of one or more matchmaking tickets. Use this operation to retrieve ticket information, including status and--once a successful match is made--acquire connection information for the resulting new game session. </p> <p>You can use this operation to track the progress of matchmaking requests (through polling) as an alternative to using event notifications. See more details on tracking matchmaking requests through polling or notifications in <a>StartMatchmaking</a>. </p> <p>You can request data for a one or a list of ticket IDs. If the request is successful, a ticket object is returned for each requested ID. When specifying a list of ticket IDs, objects are returned only for tickets that currently exist. </p> <p>Matchmaking-related operations include:</p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> </ul>"]
    fn describe_matchmaking(&self,
                            input: &DescribeMatchmakingInput)
                            -> Result<DescribeMatchmakingOutput, DescribeMatchmakingError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeMatchmaking");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeMatchmakingOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeMatchmakingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the details of FlexMatch matchmaking configurations. with this operation, you have the following options: (1) retrieve all existing configurations, (2) provide the names of one or more configurations to retrieve, or (3) retrieve all configurations that use a specified rule set name. When requesting multiple items, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a configuration is returned for each requested name. When specifying a list of names, only configurations that currently exist are returned. </p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn describe_matchmaking_configurations
        (&self,
         input: &DescribeMatchmakingConfigurationsInput)
         -> Result<DescribeMatchmakingConfigurationsOutput, DescribeMatchmakingConfigurationsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeMatchmakingConfigurations");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeMatchmakingConfigurationsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeMatchmakingConfigurationsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the details for FlexMatch matchmaking rule sets. You can request all existing rule sets for the region, or provide a list of one or more rule set names. When requesting multiple items, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a rule set is returned for each requested name. </p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn describe_matchmaking_rule_sets
        (&self,
         input: &DescribeMatchmakingRuleSetsInput)
         -> Result<DescribeMatchmakingRuleSetsOutput, DescribeMatchmakingRuleSetsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeMatchmakingRuleSets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeMatchmakingRuleSetsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeMatchmakingRuleSetsError::from_body(String::from_utf8_lossy(&body)
                                                                    .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves properties for one or more player sessions. This action can be used in several ways: (1) provide a <code>PlayerSessionId</code> to request properties for a specific player session; (2) provide a <code>GameSessionId</code> to request properties for all player sessions in the specified game session; (3) provide a <code>PlayerId</code> to request properties for all player sessions of a specified player. </p> <p>To get game session record(s), specify only one of the following: a player session ID, a game session ID, or a player ID. You can filter this request by player session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>PlayerSession</a> object is returned for each session matching the request.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <p>Player-session-related operations include:</p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn describe_player_sessions
        (&self,
         input: &DescribePlayerSessionsInput)
         -> Result<DescribePlayerSessionsOutput, DescribePlayerSessionsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribePlayerSessions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribePlayerSessionsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribePlayerSessionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the current run-time configuration for the specified fleet. The run-time configuration tells Amazon GameLift how to launch server processes on instances in the fleet.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_runtime_configuration
        (&self,
         input: &DescribeRuntimeConfigurationInput)
         -> Result<DescribeRuntimeConfigurationOutput, DescribeRuntimeConfigurationError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeRuntimeConfiguration");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeRuntimeConfigurationOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeRuntimeConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                     .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves all scaling policies applied to a fleet.</p> <p>To get a fleet's scaling policies, specify the fleet ID. You can filter this request by policy status, such as to retrieve only active scaling policies. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, set of <a>ScalingPolicy</a> objects is returned for the fleet.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn describe_scaling_policies
        (&self,
         input: &DescribeScalingPoliciesInput)
         -> Result<DescribeScalingPoliciesOutput, DescribeScalingPoliciesError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.DescribeScalingPolicies");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeScalingPoliciesOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeScalingPoliciesError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the location of stored game session logs for a specified game session. When a game session is terminated, Amazon GameLift automatically stores the logs in Amazon S3 and retains them for 14 days. Use this URL to download the logs.</p> <note> <p>See the <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_gamelift\">AWS Service Limits</a> page for maximum log file sizes. Log files that exceed this limit are not saved.</p> </note> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn get_game_session_log_url
        (&self,
         input: &GetGameSessionLogUrlInput)
         -> Result<GetGameSessionLogUrlOutput, GetGameSessionLogUrlError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.GetGameSessionLogUrl");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetGameSessionLogUrlOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetGameSessionLogUrlError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Requests remote access to a fleet instance. Remote access is useful for debugging, gathering benchmarking data, or watching activity in real time. </p> <p>Access requires credentials that match the operating system of the instance. For a Windows instance, Amazon GameLift returns a user name and password as strings for use with a Windows Remote Desktop client. For a Linux instance, Amazon GameLift returns a user name and RSA private key, also as strings, for use with an SSH client. The private key must be saved in the proper format to a <code>.pem</code> file before using. If you're making this request using the AWS CLI, saving the secret can be handled as part of the GetInstanceAccess request. (See the example later in this topic). For more information on remote access, see <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-remote-access.html\">Remotely Accessing an Instance</a>.</p> <p>To request access to a specific instance, specify the IDs of the instance and the fleet it belongs to. If successful, an <a>InstanceAccess</a> object is returned containing the instance's IP address and a set of credentials.</p>"]
    fn get_instance_access(&self,
                           input: &GetInstanceAccessInput)
                           -> Result<GetInstanceAccessOutput, GetInstanceAccessError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.GetInstanceAccess");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetInstanceAccessOutput>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetInstanceAccessError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves all aliases for this AWS account. You can filter the result set by alias name and/or routing strategy type. Use the pagination parameters to retrieve results in sequential pages.</p> <note> <p>Returned aliases are not listed in any particular order.</p> </note> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn list_aliases(&self,
                    input: &ListAliasesInput)
                    -> Result<ListAliasesOutput, ListAliasesError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.ListAliases");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListAliasesOutput>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListAliasesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves build records for all builds associated with the AWS account in use. You can limit results to builds that are in a specific status by using the <code>Status</code> parameter. Use the pagination parameters to retrieve results in a set of sequential pages. </p> <note> <p>Build records are not listed in any particular order.</p> </note> <p>Build-related operations include:</p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul>"]
    fn list_builds(&self, input: &ListBuildsInput) -> Result<ListBuildsOutput, ListBuildsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.ListBuilds");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListBuildsOutput>(String::from_utf8_lossy(&body)
                                                                .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListBuildsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves a collection of fleet records for this AWS account. You can filter the result set by build ID. Use the pagination parameters to retrieve results in sequential pages.</p> <note> <p>Fleet records are not listed in any particular order.</p> </note> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn list_fleets(&self, input: &ListFleetsInput) -> Result<ListFleetsOutput, ListFleetsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.ListFleets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListFleetsOutput>(String::from_utf8_lossy(&body)
                                                                .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListFleetsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates or updates a scaling policy for a fleet. An active scaling policy prompts Amazon GameLift to track a certain metric for a fleet and automatically change the fleet's capacity in specific circumstances. Each scaling policy contains one rule statement. Fleets can have multiple scaling policies in force simultaneously.</p> <p>A scaling policy rule statement has the following structure:</p> <p>If <code>[MetricName]</code> is <code>[ComparisonOperator]</code> <code>[Threshold]</code> for <code>[EvaluationPeriods]</code> minutes, then <code>[ScalingAdjustmentType]</code> to/by <code>[ScalingAdjustment]</code>.</p> <p>For example, this policy: \"If the number of idle instances exceeds 20 for more than 15 minutes, then reduce the fleet capacity by 10 instances\" could be implemented as the following rule statement:</p> <p>If [IdleInstances] is [GreaterThanOrEqualToThreshold] [20] for [15] minutes, then [ChangeInCapacity] by [-10].</p> <p>To create or update a scaling policy, specify a unique combination of name and fleet ID, and set the rule values. All parameters for this action are required. If successful, the policy name is returned. Scaling policies cannot be suspended or made inactive. To stop enforcing a scaling policy, call <a>DeleteScalingPolicy</a>.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn put_scaling_policy(&self,
                          input: &PutScalingPolicyInput)
                          -> Result<PutScalingPolicyOutput, PutScalingPolicyError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.PutScalingPolicy");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<PutScalingPolicyOutput>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutScalingPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p> <i>This API call is not currently in use. </i> Retrieves a fresh set of upload credentials and the assigned Amazon S3 storage location for a specific build. Valid credentials are required to upload your game build files to Amazon S3.</p>"]
    fn request_upload_credentials
        (&self,
         input: &RequestUploadCredentialsInput)
         -> Result<RequestUploadCredentialsOutput, RequestUploadCredentialsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.RequestUploadCredentials");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RequestUploadCredentialsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RequestUploadCredentialsError::from_body(String::from_utf8_lossy(&body)
                                                                 .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the fleet ID that a specified alias is currently pointing to.</p> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn resolve_alias(&self,
                     input: &ResolveAliasInput)
                     -> Result<ResolveAliasOutput, ResolveAliasError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.ResolveAlias");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ResolveAliasOutput>(String::from_utf8_lossy(&body)
                                                                  .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ResolveAliasError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves a set of game sessions that match a set of search criteria and sorts them in a specified order. A game session search is limited to a single fleet. Search results include only game sessions that are in <code>ACTIVE</code> status. If you need to retrieve game sessions with a status other than active, use <a>DescribeGameSessions</a>. If you need to retrieve the protection policy for each game session, use <a>DescribeGameSessionDetails</a>.</p> <p>You can search or sort by the following game session attributes:</p> <ul> <li> <p> <b>gameSessionId</b> -- Unique identifier for the game session. You can use either a <code>GameSessionId</code> or <code>GameSessionArn</code> value. </p> </li> <li> <p> <b>gameSessionName</b> -- Name assigned to a game session. This value is set when requesting a new game session with <a>CreateGameSession</a> or updating with <a>UpdateGameSession</a>. Game session names do not need to be unique to a game session.</p> </li> <li> <p> <b>creationTimeMillis</b> -- Value indicating when a game session was created. It is expressed in Unix time as milliseconds.</p> </li> <li> <p> <b>playerSessionCount</b> -- Number of players currently connected to a game session. This value changes rapidly as players join the session or drop out.</p> </li> <li> <p> <b>maximumSessions</b> -- Maximum number of player sessions allowed for a game session. This value is set when requesting a new game session with <a>CreateGameSession</a> or updating with <a>UpdateGameSession</a>.</p> </li> <li> <p> <b>hasAvailablePlayerSessions</b> -- Boolean value indicating whether a game session has reached its maximum number of players. When searching with this attribute, the search value must be <code>true</code> or <code>false</code>. It is highly recommended that all search requests include this filter attribute to optimize search performance and return only sessions that players can join. </p> </li> </ul> <p>To search or sort, specify either a fleet ID or an alias ID, and provide a search filter expression, a sort expression, or both. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a collection of <a>GameSession</a> objects matching the request is returned.</p> <note> <p>Returned values for <code>playerSessionCount</code> and <code>hasAvailablePlayerSessions</code> change quickly as players join sessions and others drop out. Results should be considered a snapshot in time. Be sure to refresh search results often, and handle sessions that fill up before a player can join. </p> </note> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn search_game_sessions(&self,
                            input: &SearchGameSessionsInput)
                            -> Result<SearchGameSessionsOutput, SearchGameSessionsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.SearchGameSessions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<SearchGameSessionsOutput>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SearchGameSessionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Places a request for a new game session in a queue (see <a>CreateGameSessionQueue</a>). When processing a placement request, Amazon GameLift searches for available resources on the queue's destinations, scanning each until it finds resources or the placement request times out.</p> <p>A game session placement request can also request player sessions. When a new game session is successfully created, Amazon GameLift creates a player session for each player included in the request.</p> <p>When placing a game session, by default Amazon GameLift tries each fleet in the order they are listed in the queue configuration. Ideally, a queue's destinations are listed in preference order.</p> <p>Alternatively, when requesting a game session with players, you can also provide latency data for each player in relevant regions. Latency data indicates the performance lag a player experiences when connected to a fleet in the region. Amazon GameLift uses latency data to reorder the list of destinations to place the game session in a region with minimal lag. If latency data is provided for multiple players, Amazon GameLift calculates each region's average lag for all players and reorders to get the best game play across all players. </p> <p>To place a new game session request, specify the following:</p> <ul> <li> <p>The queue name and a set of game session properties and settings</p> </li> <li> <p>A unique ID (such as a UUID) for the placement. You use this ID to track the status of the placement request</p> </li> <li> <p>(Optional) A set of IDs and player data for each player you want to join to the new game session</p> </li> <li> <p>Latency data for all players (if you want to optimize game play for the players)</p> </li> </ul> <p>If successful, a new game session placement is created.</p> <p>To track the status of a placement request, call <a>DescribeGameSessionPlacement</a> and check the request's status. If the status is <code>FULFILLED</code>, a new game session has been created and a game session ARN and region are referenced. If the placement request times out, you can resubmit the request or retry it with a different queue. </p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn start_game_session_placement
        (&self,
         input: &StartGameSessionPlacementInput)
         -> Result<StartGameSessionPlacementOutput, StartGameSessionPlacementError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.StartGameSessionPlacement");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartGameSessionPlacementOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartGameSessionPlacementError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="<p>Uses FlexMatch to create a game match for a group of players based on custom matchmaking rules, and starts a new game for the matched players. Each matchmaking request specifies the type of match to build (team configuration, rules for an acceptable match, etc.). The request also specifies the players to find a match for and where to host the new game session for optimal performance. A matchmaking request might start with a single player or a group of players who want to play together. FlexMatch finds additional players as needed to fill the match. Match type, rules, and the queue used to place a new game session are defined in a <code>MatchmakingConfiguration</code>. For complete information on setting up and using FlexMatch, see the topic <a href=\"http://docs.aws.amazon.com/gamelift/latest/developerguide/match-intro.html\"> Adding FlexMatch to Your Game</a>.</p> <p>To start matchmaking, provide a unique ticket ID, specify a matchmaking configuration, and include the players to be matched. You must also include a set of player attributes relevant for the matchmaking configuration. If successful, a matchmaking ticket is returned with status set to <code>QUEUED</code>. Track the status of the ticket to respond as needed and acquire game session connection information for sucessfully completed matches.</p> <p> <b>Tracking ticket status</b> -- A couple of options are available for tracking the status of matchmaking requests: </p> <ul> <li> <p>Polling -- Call <code>DescribeMatchmaking</code>. This operation returns the full ticket object, including current status and (for completed tickets) game session connection info. We recommend polling no more than once every 10 seconds.</p> </li> <li> <p>Notifications -- Get event notifications for changes in ticket status using Amazon Simple Notification Service (SNS). Notifications are easy to set up (see <a>CreateMatchmakingConfiguration</a>) and typically deliver match status changes faster and more efficiently than polling. We recommend that you use polling to back up to notifications (since delivery is not guaranteed) and call <code>DescribeMatchmaking</code> only when notifications are not received within 30 seconds.</p> </li> </ul> <p> <b>Processing a matchmaking request</b> -- FlexMatch handles a matchmaking request as follows: </p> <ol> <li> <p>Your client code submits a <code>StartMatchmaking</code> request for one or more players and tracks the status of the request ticket. </p> </li> <li> <p>FlexMatch uses this ticket and others in process to build an acceptable match. When a potential match is identified, all tickets in the proposed match are advanced to the next status. </p> </li> <li> <p>If the match requires player acceptance (set in the matchmaking configuration), the tickets move into status <code>REQUIRES_ACCEPTANCE</code>. This status triggers your client code to solicit acceptance from all players in every ticket involved in the match, and then call <a>AcceptMatch</a> for each player. If any player rejects or fails to accept the match before a specified timeout, the proposed match is dropped (see <code>AcceptMatch</code> for more details).</p> </li> <li> <p>Once a match is proposed and accepted, the matchmaking tickets move into status <code>PLACING</code>. FlexMatch locates resources for a new game session using the game session queue (set in the matchmaking configuration) and creates the game session based on the match data. </p> </li> <li> <p>When the match is successfully placed, the matchmaking tickets move into <code>COMPLETED</code> status. Connection information (including game session endpoint and player session) is added to the matchmaking tickets. Matched players can use the connection information to join the game. </p> </li> </ol> <p>Matchmaking-related operations include:</p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> </ul>"]
    fn start_matchmaking(&self,
                         input: &StartMatchmakingInput)
                         -> Result<StartMatchmakingOutput, StartMatchmakingError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.StartMatchmaking");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartMatchmakingOutput>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartMatchmakingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Cancels a game session placement that is in <code>PENDING</code> status. To stop a placement, provide the placement ID values. If successful, the placement is moved to <code>CANCELLED</code> status.</p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn stop_game_session_placement
        (&self,
         input: &StopGameSessionPlacementInput)
         -> Result<StopGameSessionPlacementOutput, StopGameSessionPlacementError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.StopGameSessionPlacement");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StopGameSessionPlacementOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StopGameSessionPlacementError::from_body(String::from_utf8_lossy(&body)
                                                                 .as_ref()))
            }
        }
    }


    #[doc="<p>Cancels a matchmaking ticket that is currently being processed. To stop the matchmaking operation, specify the ticket ID. If successful, work on the ticket is stopped, and the ticket status is changed to <code>CANCELLED</code>.</p> <p>Matchmaking-related operations include:</p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> </ul>"]
    fn stop_matchmaking(&self,
                        input: &StopMatchmakingInput)
                        -> Result<StopMatchmakingOutput, StopMatchmakingError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.StopMatchmaking");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StopMatchmakingOutput>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StopMatchmakingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates properties for an alias. To update properties, specify the alias ID to be updated and provide the information to be changed. To reassign an alias to another fleet, provide an updated routing strategy. If successful, the updated alias record is returned.</p> <p>Alias-related operations include:</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul>"]
    fn update_alias(&self,
                    input: &UpdateAliasInput)
                    -> Result<UpdateAliasOutput, UpdateAliasError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.UpdateAlias");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateAliasOutput>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateAliasError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates metadata in a build record, including the build name and version. To update the metadata, specify the build ID to update and provide the new values. If successful, a build object containing the updated metadata is returned.</p> <p>Build-related operations include:</p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul>"]
    fn update_build(&self,
                    input: &UpdateBuildInput)
                    -> Result<UpdateBuildOutput, UpdateBuildError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.UpdateBuild");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateBuildOutput>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateBuildError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates fleet properties, including name and description, for a fleet. To update metadata, specify the fleet ID and the property values that you want to change. If successful, the fleet ID for the updated fleet is returned.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn update_fleet_attributes
        (&self,
         input: &UpdateFleetAttributesInput)
         -> Result<UpdateFleetAttributesOutput, UpdateFleetAttributesError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.UpdateFleetAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateFleetAttributesOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateFleetAttributesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates capacity settings for a fleet. Use this action to specify the number of EC2 instances (hosts) that you want this fleet to contain. Before calling this action, you may want to call <a>DescribeEC2InstanceLimits</a> to get the maximum capacity based on the fleet's EC2 instance type.</p> <p>If you're using autoscaling (see <a>PutScalingPolicy</a>), you may want to specify a minimum and/or maximum capacity. If you don't provide these, autoscaling can set capacity anywhere between zero and the <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_gamelift\">service limits</a>.</p> <p>To update fleet capacity, specify the fleet ID and the number of instances you want the fleet to host. If successful, Amazon GameLift starts or terminates instances so that the fleet's active instance count matches the desired instance count. You can view a fleet's current capacity information by calling <a>DescribeFleetCapacity</a>. If the desired instance count is higher than the instance type's limit, the \"Limit Exceeded\" exception occurs.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn update_fleet_capacity(&self,
                             input: &UpdateFleetCapacityInput)
                             -> Result<UpdateFleetCapacityOutput, UpdateFleetCapacityError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.UpdateFleetCapacity");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateFleetCapacityOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateFleetCapacityError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates port settings for a fleet. To update settings, specify the fleet ID to be updated and list the permissions you want to update. List the permissions you want to add in <code>InboundPermissionAuthorizations</code>, and permissions you want to remove in <code>InboundPermissionRevocations</code>. Permissions to be removed must match existing fleet permissions. If successful, the fleet ID for the updated fleet is returned.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn update_fleet_port_settings
        (&self,
         input: &UpdateFleetPortSettingsInput)
         -> Result<UpdateFleetPortSettingsOutput, UpdateFleetPortSettingsError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.UpdateFleetPortSettings");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateFleetPortSettingsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateFleetPortSettingsError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }


    #[doc="<p>Updates game session properties. This includes the session name, maximum player count, protection policy, which controls whether or not an active game session can be terminated during a scale-down event, and the player session creation policy, which controls whether or not new players can join the session. To update a game session, specify the game session ID and the values you want to change. If successful, an updated <a>GameSession</a> object is returned. </p> <p>Game-session-related operations include:</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul>"]
    fn update_game_session(&self,
                           input: &UpdateGameSessionInput)
                           -> Result<UpdateGameSessionOutput, UpdateGameSessionError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.UpdateGameSession");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateGameSessionOutput>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateGameSessionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates settings for a game session queue, which determines how new game session requests in the queue are processed. To update settings, specify the queue name to be updated and provide the new settings. When updating destinations, provide a complete list of destinations. </p> <p>Queue-related operations include:</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul>"]
    fn update_game_session_queue
        (&self,
         input: &UpdateGameSessionQueueInput)
         -> Result<UpdateGameSessionQueueOutput, UpdateGameSessionQueueError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.UpdateGameSessionQueue");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateGameSessionQueueOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateGameSessionQueueError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates settings for a FlexMatch matchmaking configuration. To update settings, specify the configuration name to be updated and provide the new settings. </p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn update_matchmaking_configuration
        (&self,
         input: &UpdateMatchmakingConfigurationInput)
         -> Result<UpdateMatchmakingConfigurationOutput, UpdateMatchmakingConfigurationError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.UpdateMatchmakingConfiguration");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateMatchmakingConfigurationOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateMatchmakingConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Updates the current run-time configuration for the specified fleet, which tells Amazon GameLift how to launch server processes on instances in the fleet. You can update a fleet's run-time configuration at any time after the fleet is created; it does not need to be in an <code>ACTIVE</code> status.</p> <p>To update run-time configuration, specify the fleet ID and provide a <code>RuntimeConfiguration</code> object with the updated collection of server process configurations.</p> <p>Each instance in a Amazon GameLift fleet checks regularly for an updated run-time configuration and changes how it launches server processes to comply with the latest version. Existing server processes are not affected by the update; they continue to run until they end, while Amazon GameLift simply adds new server processes to fit the current run-time configuration. As a result, the run-time configuration changes are applied gradually as existing processes shut down and new processes are launched in Amazon GameLift's normal process recycling activity.</p> <p>Fleet-related operations include:</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p>Manage fleet capacity:</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>PutScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (automatic scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (automatic scaling)</p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> </ul> </li> <li> <p> <a>DeleteFleet</a> </p> </li> </ul>"]
    fn update_runtime_configuration
        (&self,
         input: &UpdateRuntimeConfigurationInput)
         -> Result<UpdateRuntimeConfigurationOutput, UpdateRuntimeConfigurationError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.UpdateRuntimeConfiguration");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateRuntimeConfigurationOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateRuntimeConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                   .as_ref()))
            }
        }
    }


    #[doc="<p>Validates the syntax of a matchmaking rule or rule set. This operation checks that the rule set uses syntactically correct JSON and that it conforms to allowed property expressions. To validate syntax, provide a rule set string.</p> <p>Operations related to match configurations and rule sets include:</p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> </ul>"]
    fn validate_matchmaking_rule_set
        (&self,
         input: &ValidateMatchmakingRuleSetInput)
         -> Result<ValidateMatchmakingRuleSetOutput, ValidateMatchmakingRuleSetError> {
        let mut request = SignedRequest::new("POST", "gamelift", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "GameLift.ValidateMatchmakingRuleSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ValidateMatchmakingRuleSetOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ValidateMatchmakingRuleSetError::from_body(String::from_utf8_lossy(&body)
                                                                   .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
