
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
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddFacetToObjectRequest {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>Attributes on the facet that you are adding to the object.</p>"]
    #[serde(rename="ObjectAttributeList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_attribute_list: Option<Vec<AttributeKeyAndValue>>,
    #[doc="<p>A reference to the object you are adding the specified facet to.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
    #[doc="<p>Identifiers for the facet that you are adding to the object.</p>"]
    #[serde(rename="SchemaFacet")]
    pub schema_facet: SchemaFacet,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddFacetToObjectResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct ApplySchemaRequest {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> into which the schema is copied. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>Published schema Amazon Resource Name (ARN) that needs to be copied. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="PublishedSchemaArn")]
    pub published_schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ApplySchemaResponse {
    #[doc="<p>The applied schema ARN that is associated with the copied schema in the <a>Directory</a>. You can use this ARN to describe the schema information applied on this directory. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="AppliedSchemaArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub applied_schema_arn: Option<String>,
    #[doc="<p>The ARN that is associated with the <a>Directory</a>. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_arn: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct AttachObjectRequest {
    #[doc="<p>The child object reference to be attached to the object.</p>"]
    #[serde(rename="ChildReference")]
    pub child_reference: ObjectReference,
    #[doc="<p>Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where both objects reside. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The link name with which the child object is attached to the parent.</p>"]
    #[serde(rename="LinkName")]
    pub link_name: String,
    #[doc="<p>The parent object reference.</p>"]
    #[serde(rename="ParentReference")]
    pub parent_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AttachObjectResponse {
    #[doc="<p>The attached <code>ObjectIdentifier</code>, which is the child <code>ObjectIdentifier</code>.</p>"]
    #[serde(rename="AttachedObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct AttachPolicyRequest {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where both objects reside. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_arn: Option<String>,
    #[doc="<p>The reference that identifies the object to which the policy will be attached.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
    #[doc="<p>The reference that is associated with the policy object.</p>"]
    #[serde(rename="PolicyReference")]
    pub policy_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AttachPolicyResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct AttachToIndexRequest {
    #[doc="<p>The Amazon Resource Name (ARN) of the directory where the object and index exist.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>A reference to the index that you are attaching the object to.</p>"]
    #[serde(rename="IndexReference")]
    pub index_reference: ObjectReference,
    #[doc="<p>A reference to the object that you are attaching to the index.</p>"]
    #[serde(rename="TargetReference")]
    pub target_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AttachToIndexResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> of the object that was attached to the index.</p>"]
    #[serde(rename="AttachedObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct AttachTypedLinkRequest {
    #[doc="<p>A set of attributes that are associated with the typed link.</p>"]
    #[serde(rename="Attributes")]
    pub attributes: Vec<AttributeNameAndValue>,
    #[doc="<p>The Amazon Resource Name (ARN) of the directory where you want to attach the typed link.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>Identifies the source object that the typed link will attach to.</p>"]
    #[serde(rename="SourceObjectReference")]
    pub source_object_reference: ObjectReference,
    #[doc="<p>Identifies the target object that the typed link will attach to.</p>"]
    #[serde(rename="TargetObjectReference")]
    pub target_object_reference: ObjectReference,
    #[doc="<p>Identifies the typed link facet that is associated with the typed link.</p>"]
    #[serde(rename="TypedLinkFacet")]
    pub typed_link_facet: TypedLinkSchemaAndFacetName,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AttachTypedLinkResponse {
    #[doc="<p>Returns a typed link specifier as output.</p>"]
    #[serde(rename="TypedLinkSpecifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub typed_link_specifier: Option<TypedLinkSpecifier>,
}

#[doc="<p>A unique identifier for an attribute.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct AttributeKey {
    #[doc="<p>The name of the facet that the attribute exists within.</p>"]
    #[serde(rename="FacetName")]
    pub facet_name: String,
    #[doc="<p>The name of the attribute.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The Amazon Resource Name (ARN) of the schema that contains the facet and attribute.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[doc="<p>The combination of an attribute key and an attribute value.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct AttributeKeyAndValue {
    #[doc="<p>The key of the attribute.</p>"]
    #[serde(rename="Key")]
    pub key: AttributeKey,
    #[doc="<p>The value of the attribute.</p>"]
    #[serde(rename="Value")]
    pub value: TypedAttributeValue,
}

#[doc="<p>Identifies the attribute name and value for a typed link.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct AttributeNameAndValue {
    #[doc="<p>The attribute name of the typed link.</p>"]
    #[serde(rename="AttributeName")]
    pub attribute_name: String,
    #[doc="<p>The value for the typed link.</p>"]
    #[serde(rename="Value")]
    pub value: TypedAttributeValue,
}

#[doc="<p>Represents the output of a batch add facet to object operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchAddFacetToObject {
    #[doc="<p>The attributes to set on the object.</p>"]
    #[serde(rename="ObjectAttributeList")]
    pub object_attribute_list: Vec<AttributeKeyAndValue>,
    #[doc="<p>A reference to the object being mutated.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
    #[doc="<p>Represents the facet being added to the object.</p>"]
    #[serde(rename="SchemaFacet")]
    pub schema_facet: SchemaFacet,
}

#[doc="<p>The result of a batch add facet to object operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchAddFacetToObjectResponse;

#[doc="<p>Represents the output of an <a>AttachObject</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchAttachObject {
    #[doc="<p>The child object reference that is to be attached to the object.</p>"]
    #[serde(rename="ChildReference")]
    pub child_reference: ObjectReference,
    #[doc="<p>The name of the link.</p>"]
    #[serde(rename="LinkName")]
    pub link_name: String,
    #[doc="<p>The parent object reference.</p>"]
    #[serde(rename="ParentReference")]
    pub parent_reference: ObjectReference,
}

#[doc="<p>Represents the output batch <a>AttachObject</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchAttachObjectResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> of the object that has been attached.</p>"]
    #[serde(rename="attachedObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[doc="<p>Attaches a policy object to a regular object inside a <a>BatchRead</a> operation. For more information, see <a>AttachPolicy</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchAttachPolicy {
    #[doc="<p>The reference that identifies the object to which the policy will be attached.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
    #[doc="<p>The reference that is associated with the policy object.</p>"]
    #[serde(rename="PolicyReference")]
    pub policy_reference: ObjectReference,
}

#[doc="<p>Represents the output of an <a>AttachPolicy</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchAttachPolicyResponse;

#[doc="<p>Attaches the specified object to the specified index inside a <a>BatchRead</a> operation. For more information, see <a>AttachToIndex</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchAttachToIndex {
    #[doc="<p>A reference to the index that you are attaching the object to.</p>"]
    #[serde(rename="IndexReference")]
    pub index_reference: ObjectReference,
    #[doc="<p>A reference to the object that you are attaching to the index.</p>"]
    #[serde(rename="TargetReference")]
    pub target_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>AttachToIndex</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchAttachToIndexResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> of the object that was attached to the index.</p>"]
    #[serde(rename="AttachedObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[doc="<p>Attaches a typed link to a specified source and target object inside a <a>BatchRead</a> operation. For more information, see <a>AttachTypedLink</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchAttachTypedLink {
    #[doc="<p>A set of attributes that are associated with the typed link.</p>"]
    #[serde(rename="Attributes")]
    pub attributes: Vec<AttributeNameAndValue>,
    #[doc="<p>Identifies the source object that the typed link will attach to.</p>"]
    #[serde(rename="SourceObjectReference")]
    pub source_object_reference: ObjectReference,
    #[doc="<p>Identifies the target object that the typed link will attach to.</p>"]
    #[serde(rename="TargetObjectReference")]
    pub target_object_reference: ObjectReference,
    #[doc="<p>Identifies the typed link facet that is associated with the typed link.</p>"]
    #[serde(rename="TypedLinkFacet")]
    pub typed_link_facet: TypedLinkSchemaAndFacetName,
}

#[doc="<p>Represents the output of a <a>AttachTypedLink</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchAttachTypedLinkResponse {
    #[doc="<p>Returns a typed link specifier as output.</p>"]
    #[serde(rename="TypedLinkSpecifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub typed_link_specifier: Option<TypedLinkSpecifier>,
}

#[doc="<p>Creates an index object inside of a <a>BatchRead</a> operation. For more information, see <a>CreateIndex</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchCreateIndex {
    #[doc="<p>The batch reference name. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#batches\">Batches</a> for more information.</p>"]
    #[serde(rename="BatchReferenceName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub batch_reference_name: Option<String>,
    #[doc="<p>Indicates whether the attribute that is being indexed has unique values or not.</p>"]
    #[serde(rename="IsUnique")]
    pub is_unique: bool,
    #[doc="<p>The name of the link between the parent object and the index object.</p>"]
    #[serde(rename="LinkName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub link_name: Option<String>,
    #[doc="<p>Specifies the attributes that should be indexed on. Currently only a single attribute is supported.</p>"]
    #[serde(rename="OrderedIndexedAttributeList")]
    pub ordered_indexed_attribute_list: Vec<AttributeKey>,
    #[doc="<p>A reference to the parent object that contains the index object.</p>"]
    #[serde(rename="ParentReference")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_reference: Option<ObjectReference>,
}

#[doc="<p>Represents the output of a <a>CreateIndex</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchCreateIndexResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> of the index created by this operation.</p>"]
    #[serde(rename="ObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifier: Option<String>,
}

#[doc="<p>Represents the output of a <a>CreateObject</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchCreateObject {
    #[doc="<p>The batch reference name. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#batches\">Batches</a> for more information.</p>"]
    #[serde(rename="BatchReferenceName")]
    pub batch_reference_name: String,
    #[doc="<p>The name of the link.</p>"]
    #[serde(rename="LinkName")]
    pub link_name: String,
    #[doc="<p>An attribute map, which contains an attribute ARN as the key and attribute value as the map value.</p>"]
    #[serde(rename="ObjectAttributeList")]
    pub object_attribute_list: Vec<AttributeKeyAndValue>,
    #[doc="<p>If specified, the parent reference to which this object will be attached.</p>"]
    #[serde(rename="ParentReference")]
    pub parent_reference: ObjectReference,
    #[doc="<p>A list of <code>FacetArns</code> that will be associated with the object. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaFacet")]
    pub schema_facet: Vec<SchemaFacet>,
}

#[doc="<p>Represents the output of a <a>CreateObject</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchCreateObjectResponse {
    #[doc="<p>The ID that is associated with the object.</p>"]
    #[serde(rename="ObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifier: Option<String>,
}

#[doc="<p>Represents the output of a <a>DeleteObject</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchDeleteObject {
    #[doc="<p>The reference that identifies the object.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>DeleteObject</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchDeleteObjectResponse;

#[doc="<p>Detaches the specified object from the specified index inside a <a>BatchRead</a> operation. For more information, see <a>DetachFromIndex</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchDetachFromIndex {
    #[doc="<p>A reference to the index object.</p>"]
    #[serde(rename="IndexReference")]
    pub index_reference: ObjectReference,
    #[doc="<p>A reference to the object being detached from the index.</p>"]
    #[serde(rename="TargetReference")]
    pub target_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>DetachFromIndex</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchDetachFromIndexResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> of the object that was detached from the index.</p>"]
    #[serde(rename="DetachedObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[doc="<p>Represents the output of a <a>DetachObject</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchDetachObject {
    #[doc="<p>The batch reference name. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#batches\">Batches</a> for more information.</p>"]
    #[serde(rename="BatchReferenceName")]
    pub batch_reference_name: String,
    #[doc="<p>The name of the link.</p>"]
    #[serde(rename="LinkName")]
    pub link_name: String,
    #[doc="<p>Parent reference from which the object with the specified link name is detached.</p>"]
    #[serde(rename="ParentReference")]
    pub parent_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>DetachObject</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchDetachObjectResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> of the detached object.</p>"]
    #[serde(rename="detachedObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[doc="<p>Detaches the specified policy from the specified directory inside a <a>BatchRead</a> operation. For more information, see <a>DetachPolicy</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchDetachPolicy {
    #[doc="<p>Reference that identifies the object whose policy object will be detached.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
    #[doc="<p>Reference that identifies the policy object.</p>"]
    #[serde(rename="PolicyReference")]
    pub policy_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>DetachPolicy</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchDetachPolicyResponse;

#[doc="<p>Detaches a typed link from a specified source and target object inside a <a>BatchRead</a> operation. For more information, see <a>DetachTypedLink</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchDetachTypedLink {
    #[doc="<p>Used to accept a typed link specifier as input.</p>"]
    #[serde(rename="TypedLinkSpecifier")]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[doc="<p>Represents the output of a <a>DetachTypedLink</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchDetachTypedLinkResponse;

#[doc="<p>Retrieves metadata about an object inside a <a>BatchRead</a> operation. For more information, see <a>GetObjectInformation</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchGetObjectInformation {
    #[doc="<p>A reference to the object.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>GetObjectInformation</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchGetObjectInformationResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> of the specified object.</p>"]
    #[serde(rename="ObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifier: Option<String>,
    #[doc="<p>The facets attached to the specified object.</p>"]
    #[serde(rename="SchemaFacets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_facets: Option<Vec<SchemaFacet>>,
}

#[doc="<p>Lists indices attached to an object inside a <a>BatchRead</a> operation. For more information, see <a>ListAttachedIndices</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchListAttachedIndices {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A reference to the object that has indices attached.</p>"]
    #[serde(rename="TargetReference")]
    pub target_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>ListAttachedIndices</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchListAttachedIndicesResponse {
    #[doc="<p>The indices attached to the specified object.</p>"]
    #[serde(rename="IndexAttachments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object inside a <a>BatchRead</a> operation. For more information, see <a>ListIncomingTypedLinks</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchListIncomingTypedLinks {
    #[doc="<p>Provides range filters for multiple attributes. When providing ranges to typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range.</p>"]
    #[serde(rename="FilterAttributeRanges")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    #[doc="<p>Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls.</p>"]
    #[serde(rename="FilterTypedLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The reference that identifies the object whose attributes will be listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>ListIncomingTypedLinks</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchListIncomingTypedLinksResponse {
    #[doc="<p>Returns one or more typed link specifiers as output.</p>"]
    #[serde(rename="LinkSpecifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub link_specifiers: Option<Vec<TypedLinkSpecifier>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Lists objects attached to the specified index inside a <a>BatchRead</a> operation. For more information, see <a>ListIndex</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchListIndex {
    #[doc="<p>The reference to the index to list.</p>"]
    #[serde(rename="IndexReference")]
    pub index_reference: ObjectReference,
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Specifies the ranges of indexed values that you want to query.</p>"]
    #[serde(rename="RangesOnIndexedValues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ranges_on_indexed_values: Option<Vec<ObjectAttributeRange>>,
}

#[doc="<p>Represents the output of a <a>ListIndex</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchListIndexResponse {
    #[doc="<p>The objects and indexed values attached to the index.</p>"]
    #[serde(rename="IndexAttachments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the output of a <a>ListObjectAttributes</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchListObjectAttributes {
    #[doc="<p>Used to filter the list of object attributes that are associated with a certain facet.</p>"]
    #[serde(rename="FacetFilter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub facet_filter: Option<SchemaFacet>,
    #[doc="<p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Reference of the object whose attributes need to be listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>ListObjectAttributes</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchListObjectAttributesResponse {
    #[doc="<p>The attributes map that is associated with the object. <code>AttributeArn</code> is the key; attribute value is the value.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the output of a <a>ListObjectChildren</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchListObjectChildren {
    #[doc="<p>Maximum number of items to be retrieved in a single call. This is an approximate number.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Reference of the object for which child objects are being listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>ListObjectChildren</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchListObjectChildrenResponse {
    #[doc="<p>The children structure, which is a map with the key as the <code>LinkName</code> and <code>ObjectIdentifier</code> as the value.</p>"]
    #[serde(rename="Children")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub children: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects inside a <a>BatchRead</a> operation. For more information, see <a>ListObjectParentPaths</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchListObjectParentPaths {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The reference that identifies the object whose attributes will be listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>ListObjectParentPaths</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchListObjectParentPathsResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Returns the path to the <code>ObjectIdentifiers</code> that are associated with the directory.</p>"]
    #[serde(rename="PathToObjectIdentifiersList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path_to_object_identifiers_list: Option<Vec<PathToObjectIdentifiers>>,
}

#[doc="<p>Returns policies attached to an object in pagination fashion inside a <a>BatchRead</a> operation. For more information, see <a>ListObjectPolicies</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchListObjectPolicies {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The reference that identifies the object whose attributes will be listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>ListObjectPolicies</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchListObjectPoliciesResponse {
    #[doc="<p>A list of policy <code>ObjectIdentifiers</code>, that are attached to the object.</p>"]
    #[serde(rename="AttachedPolicyIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attached_policy_ids: Option<Vec<String>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object inside a <a>BatchRead</a> operation. For more information, see <a>ListOutgoingTypedLinks</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchListOutgoingTypedLinks {
    #[doc="<p>Provides range filters for multiple attributes. When providing ranges to typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range.</p>"]
    #[serde(rename="FilterAttributeRanges")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    #[doc="<p>Filters are interpreted in the order of the attributes defined on the typed link facet, not the order they are supplied to any API calls.</p>"]
    #[serde(rename="FilterTypedLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The reference that identifies the object whose attributes will be listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>ListOutgoingTypedLinks</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchListOutgoingTypedLinksResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Returns a typed link specifier as output.</p>"]
    #[serde(rename="TypedLinkSpecifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub typed_link_specifiers: Option<Vec<TypedLinkSpecifier>>,
}

#[doc="<p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached inside a <a>BatchRead</a> operation. For more information, see <a>ListPolicyAttachments</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchListPolicyAttachments {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The reference that identifies the policy object.</p>"]
    #[serde(rename="PolicyReference")]
    pub policy_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>ListPolicyAttachments</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchListPolicyAttachmentsResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of <code>ObjectIdentifiers</code> to which the policy is attached.</p>"]
    #[serde(rename="ObjectIdentifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifiers: Option<Vec<String>>,
}

#[doc="<p>Lists all policies from the root of the Directory to the object specified inside a <a>BatchRead</a> operation. For more information, see <a>LookupPolicy</a> and <a>BatchReadRequest$Operations</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchLookupPolicy {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Reference that identifies the object whose policies will be looked up.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <a>LookupPolicy</a> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchLookupPolicyResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Provides list of path to policies. Policies contain <code>PolicyId</code>, <code>ObjectIdentifier</code>, and <code>PolicyType</code>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies\">Policies</a>.</p>"]
    #[serde(rename="PolicyToPathList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_to_path_list: Option<Vec<PolicyToPath>>,
}

#[doc="<p>The batch read exception structure, which contains the exception type and message.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchReadException {
    #[doc="<p>An exception message that is associated with the failure.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[doc="<p>A type of exception, such as <code>InvalidArnException</code>.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[doc="<p>Represents the output of a <code>BatchRead</code> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchReadOperation {
    #[doc="<p>Retrieves metadata about an object.</p>"]
    #[serde(rename="GetObjectInformation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub get_object_information: Option<BatchGetObjectInformation>,
    #[doc="<p>Lists indices attached to an object.</p>"]
    #[serde(rename="ListAttachedIndices")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_attached_indices: Option<BatchListAttachedIndices>,
    #[doc="<p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    #[serde(rename="ListIncomingTypedLinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_incoming_typed_links: Option<BatchListIncomingTypedLinks>,
    #[doc="<p>Lists objects attached to the specified index.</p>"]
    #[serde(rename="ListIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_index: Option<BatchListIndex>,
    #[doc="<p>Lists all attributes that are associated with an object.</p>"]
    #[serde(rename="ListObjectAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_object_attributes: Option<BatchListObjectAttributes>,
    #[doc="<p>Returns a paginated list of child objects that are associated with a given object.</p>"]
    #[serde(rename="ListObjectChildren")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_object_children: Option<BatchListObjectChildren>,
    #[doc="<p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#dirstructure\">Directory Structure</a>.</p>"]
    #[serde(rename="ListObjectParentPaths")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_object_parent_paths: Option<BatchListObjectParentPaths>,
    #[doc="<p>Returns policies attached to an object in pagination fashion.</p>"]
    #[serde(rename="ListObjectPolicies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_object_policies: Option<BatchListObjectPolicies>,
    #[doc="<p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    #[serde(rename="ListOutgoingTypedLinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_outgoing_typed_links: Option<BatchListOutgoingTypedLinks>,
    #[doc="<p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>"]
    #[serde(rename="ListPolicyAttachments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_policy_attachments: Option<BatchListPolicyAttachments>,
    #[doc="<p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies\">Policies</a>.</p>"]
    #[serde(rename="LookupPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lookup_policy: Option<BatchLookupPolicy>,
}

#[doc="<p>Represents the output of a <code>BatchRead</code> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchReadOperationResponse {
    #[doc="<p>Identifies which operation in a batch has failed.</p>"]
    #[serde(rename="ExceptionResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exception_response: Option<BatchReadException>,
    #[doc="<p>Identifies which operation in a batch has succeeded.</p>"]
    #[serde(rename="SuccessfulResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub successful_response: Option<BatchReadSuccessfulResponse>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchReadRequest {
    #[doc="<p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>"]
    #[serde(rename="ConsistencyLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consistency_level: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a>. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>A list of operations that are part of the batch.</p>"]
    #[serde(rename="Operations")]
    pub operations: Vec<BatchReadOperation>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchReadResponse {
    #[doc="<p>A list of all the responses for each batch read.</p>"]
    #[serde(rename="Responses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub responses: Option<Vec<BatchReadOperationResponse>>,
}

#[doc="<p>Represents the output of a <code>BatchRead</code> success response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchReadSuccessfulResponse {
    #[doc="<p>Retrieves metadata about an object.</p>"]
    #[serde(rename="GetObjectInformation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub get_object_information: Option<BatchGetObjectInformationResponse>,
    #[doc="<p>Lists indices attached to an object.</p>"]
    #[serde(rename="ListAttachedIndices")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_attached_indices: Option<BatchListAttachedIndicesResponse>,
    #[doc="<p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    #[serde(rename="ListIncomingTypedLinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_incoming_typed_links: Option<BatchListIncomingTypedLinksResponse>,
    #[doc="<p>Lists objects attached to the specified index.</p>"]
    #[serde(rename="ListIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_index: Option<BatchListIndexResponse>,
    #[doc="<p>Lists all attributes that are associated with an object.</p>"]
    #[serde(rename="ListObjectAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_object_attributes: Option<BatchListObjectAttributesResponse>,
    #[doc="<p>Returns a paginated list of child objects that are associated with a given object.</p>"]
    #[serde(rename="ListObjectChildren")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_object_children: Option<BatchListObjectChildrenResponse>,
    #[doc="<p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#dirstructure\">Directory Structure</a>.</p>"]
    #[serde(rename="ListObjectParentPaths")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_object_parent_paths: Option<BatchListObjectParentPathsResponse>,
    #[doc="<p>Returns policies attached to an object in pagination fashion.</p>"]
    #[serde(rename="ListObjectPolicies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_object_policies: Option<BatchListObjectPoliciesResponse>,
    #[doc="<p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    #[serde(rename="ListOutgoingTypedLinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_outgoing_typed_links: Option<BatchListOutgoingTypedLinksResponse>,
    #[doc="<p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>"]
    #[serde(rename="ListPolicyAttachments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list_policy_attachments: Option<BatchListPolicyAttachmentsResponse>,
    #[doc="<p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies\">Policies</a>.</p>"]
    #[serde(rename="LookupPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lookup_policy: Option<BatchLookupPolicyResponse>,
}

#[doc="<p>A batch operation to remove a facet from an object.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchRemoveFacetFromObject {
    #[doc="<p>A reference to the object whose facet will be removed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
    #[doc="<p>The facet to remove from the object.</p>"]
    #[serde(rename="SchemaFacet")]
    pub schema_facet: SchemaFacet,
}

#[doc="<p>An empty result that represents success.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchRemoveFacetFromObjectResponse;

#[doc="<p>Represents the output of a <code>BatchUpdate</code> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchUpdateObjectAttributes {
    #[doc="<p>Attributes update structure.</p>"]
    #[serde(rename="AttributeUpdates")]
    pub attribute_updates: Vec<ObjectAttributeUpdate>,
    #[doc="<p>Reference that identifies the object.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[doc="<p>Represents the output of a <code>BatchUpdate</code> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchUpdateObjectAttributesResponse {
    #[doc="<p>ID that is associated with the object.</p>"]
    #[serde(rename="ObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifier: Option<String>,
}

#[doc="<p>Represents the output of a <code>BatchWrite</code> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchWriteOperation {
    #[doc="<p>A batch operation that adds a facet to an object.</p>"]
    #[serde(rename="AddFacetToObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub add_facet_to_object: Option<BatchAddFacetToObject>,
    #[doc="<p>Attaches an object to a <a>Directory</a>.</p>"]
    #[serde(rename="AttachObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attach_object: Option<BatchAttachObject>,
    #[doc="<p>Attaches a policy object to a regular object. An object can have a limited number of attached policies.</p>"]
    #[serde(rename="AttachPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attach_policy: Option<BatchAttachPolicy>,
    #[doc="<p>Attaches the specified object to the specified index.</p>"]
    #[serde(rename="AttachToIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attach_to_index: Option<BatchAttachToIndex>,
    #[doc="<p>Attaches a typed link to a specified source and target object. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    #[serde(rename="AttachTypedLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attach_typed_link: Option<BatchAttachTypedLink>,
    #[doc="<p>Creates an index object. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_indexing.html\">Indexing</a> for more information.</p>"]
    #[serde(rename="CreateIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create_index: Option<BatchCreateIndex>,
    #[doc="<p>Creates an object.</p>"]
    #[serde(rename="CreateObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create_object: Option<BatchCreateObject>,
    #[doc="<p>Deletes an object in a <a>Directory</a>.</p>"]
    #[serde(rename="DeleteObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_object: Option<BatchDeleteObject>,
    #[doc="<p>Detaches the specified object from the specified index.</p>"]
    #[serde(rename="DetachFromIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detach_from_index: Option<BatchDetachFromIndex>,
    #[doc="<p>Detaches an object from a <a>Directory</a>.</p>"]
    #[serde(rename="DetachObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detach_object: Option<BatchDetachObject>,
    #[doc="<p>Detaches a policy from a <a>Directory</a>.</p>"]
    #[serde(rename="DetachPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detach_policy: Option<BatchDetachPolicy>,
    #[doc="<p>Detaches a typed link from a specified source and target object. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    #[serde(rename="DetachTypedLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detach_typed_link: Option<BatchDetachTypedLink>,
    #[doc="<p>A batch operation that removes a facet from an object.</p>"]
    #[serde(rename="RemoveFacetFromObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remove_facet_from_object: Option<BatchRemoveFacetFromObject>,
    #[doc="<p>Updates a given object's attributes.</p>"]
    #[serde(rename="UpdateObjectAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub update_object_attributes: Option<BatchUpdateObjectAttributes>,
}

#[doc="<p>Represents the output of a <code>BatchWrite</code> response operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchWriteOperationResponse {
    #[doc="<p>The result of an add facet to object batch operation.</p>"]
    #[serde(rename="AddFacetToObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub add_facet_to_object: Option<BatchAddFacetToObjectResponse>,
    #[doc="<p>Attaches an object to a <a>Directory</a>.</p>"]
    #[serde(rename="AttachObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attach_object: Option<BatchAttachObjectResponse>,
    #[doc="<p>Attaches a policy object to a regular object. An object can have a limited number of attached policies.</p>"]
    #[serde(rename="AttachPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attach_policy: Option<BatchAttachPolicyResponse>,
    #[doc="<p>Attaches the specified object to the specified index.</p>"]
    #[serde(rename="AttachToIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attach_to_index: Option<BatchAttachToIndexResponse>,
    #[doc="<p>Attaches a typed link to a specified source and target object. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    #[serde(rename="AttachTypedLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attach_typed_link: Option<BatchAttachTypedLinkResponse>,
    #[doc="<p>Creates an index object. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_indexing.html\">Indexing</a> for more information.</p>"]
    #[serde(rename="CreateIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create_index: Option<BatchCreateIndexResponse>,
    #[doc="<p>Creates an object in a <a>Directory</a>.</p>"]
    #[serde(rename="CreateObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create_object: Option<BatchCreateObjectResponse>,
    #[doc="<p>Deletes an object in a <a>Directory</a>.</p>"]
    #[serde(rename="DeleteObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_object: Option<BatchDeleteObjectResponse>,
    #[doc="<p>Detaches the specified object from the specified index.</p>"]
    #[serde(rename="DetachFromIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detach_from_index: Option<BatchDetachFromIndexResponse>,
    #[doc="<p>Detaches an object from a <a>Directory</a>.</p>"]
    #[serde(rename="DetachObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detach_object: Option<BatchDetachObjectResponse>,
    #[doc="<p>Detaches a policy from a <a>Directory</a>.</p>"]
    #[serde(rename="DetachPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detach_policy: Option<BatchDetachPolicyResponse>,
    #[doc="<p>Detaches a typed link from a specified source and target object. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    #[serde(rename="DetachTypedLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detach_typed_link: Option<BatchDetachTypedLinkResponse>,
    #[doc="<p>The result of a batch remove facet from object operation.</p>"]
    #[serde(rename="RemoveFacetFromObject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remove_facet_from_object: Option<BatchRemoveFacetFromObjectResponse>,
    #[doc="<p>Updates a given object’s attributes.</p>"]
    #[serde(rename="UpdateObjectAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub update_object_attributes: Option<BatchUpdateObjectAttributesResponse>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchWriteRequest {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a>. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>A list of operations that are part of the batch.</p>"]
    #[serde(rename="Operations")]
    pub operations: Vec<BatchWriteOperation>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchWriteResponse {
    #[doc="<p>A list of all the responses for each batch write.</p>"]
    #[serde(rename="Responses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub responses: Option<Vec<BatchWriteOperationResponse>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateDirectoryRequest {
    #[doc="<p>The name of the <a>Directory</a>. Should be unique per account, per region.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The Amazon Resource Name (ARN) of the published schema that will be copied into the data <a>Directory</a>. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateDirectoryResponse {
    #[doc="<p>The ARN of the published schema in the <a>Directory</a>. Once a published schema is copied into the directory, it has its own ARN, which is referred to applied schema ARN. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="AppliedSchemaArn")]
    pub applied_schema_arn: String,
    #[doc="<p>The ARN that is associated with the <a>Directory</a>. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The name of the <a>Directory</a>.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The root object node of the created directory.</p>"]
    #[serde(rename="ObjectIdentifier")]
    pub object_identifier: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateFacetRequest {
    #[doc="<p>The attributes that are associated with the <a>Facet</a>.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<FacetAttribute>>,
    #[doc="<p>The name of the <a>Facet</a>, which is unique for a given schema.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>Specifies whether a given object created from this facet is of type node, leaf node, policy or index.</p> <ul> <li> <p>Node: Can have multiple children but one parent.</p> </li> </ul> <ul> <li> <p>Leaf node: Cannot have children but can have multiple parents.</p> </li> </ul> <ul> <li> <p>Policy: Allows you to store a policy document and policy type. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies\">Policies</a>.</p> </li> </ul> <ul> <li> <p>Index: Can be created with the Index API.</p> </li> </ul>"]
    #[serde(rename="ObjectType")]
    pub object_type: String,
    #[doc="<p>The schema ARN in which the new <a>Facet</a> will be created. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateFacetResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateIndexRequest {
    #[doc="<p>The ARN of the directory where the index should be created.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>Indicates whether the attribute that is being indexed has unique values or not.</p>"]
    #[serde(rename="IsUnique")]
    pub is_unique: bool,
    #[doc="<p>The name of the link between the parent object and the index object.</p>"]
    #[serde(rename="LinkName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub link_name: Option<String>,
    #[doc="<p>Specifies the attributes that should be indexed on. Currently only a single attribute is supported.</p>"]
    #[serde(rename="OrderedIndexedAttributeList")]
    pub ordered_indexed_attribute_list: Vec<AttributeKey>,
    #[doc="<p>A reference to the parent object that contains the index object.</p>"]
    #[serde(rename="ParentReference")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_reference: Option<ObjectReference>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateIndexResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> of the index created by this operation.</p>"]
    #[serde(rename="ObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateObjectRequest {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> in which the object will be created. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The name of link that is used to attach this object to a parent.</p>"]
    #[serde(rename="LinkName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub link_name: Option<String>,
    #[doc="<p>The attribute map whose attribute ARN contains the key and attribute value as the map value.</p>"]
    #[serde(rename="ObjectAttributeList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_attribute_list: Option<Vec<AttributeKeyAndValue>>,
    #[doc="<p>If specified, the parent reference to which this object will be attached.</p>"]
    #[serde(rename="ParentReference")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_reference: Option<ObjectReference>,
    #[doc="<p>A list of schema facets to be associated with the object that contains <code>SchemaArn</code> and facet name. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaFacets")]
    pub schema_facets: Vec<SchemaFacet>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateObjectResponse {
    #[doc="<p>The identifier that is associated with the object.</p>"]
    #[serde(rename="ObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateSchemaRequest {
    #[doc="<p>The name that is associated with the schema. This is unique to each account and in each region.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateSchemaResponse {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateTypedLinkFacetRequest {
    #[doc="<p> <a>Facet</a> structure that is associated with the typed link facet.</p>"]
    #[serde(rename="Facet")]
    pub facet: TypedLinkFacet,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateTypedLinkFacetResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteDirectoryRequest {
    #[doc="<p>The ARN of the directory to delete.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteDirectoryResponse {
    #[doc="<p>The ARN of the deleted directory.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteFacetRequest {
    #[doc="<p>The name of the facet to delete.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Facet</a>. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteFacetResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteObjectRequest {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>A reference that identifies the object.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteObjectResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteSchemaRequest {
    #[doc="<p>The Amazon Resource Name (ARN) of the development schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteSchemaResponse {
    #[doc="<p>The input ARN that is returned as part of the response. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteTypedLinkFacetRequest {
    #[doc="<p>The unique name of the typed link facet.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteTypedLinkFacetResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DetachFromIndexRequest {
    #[doc="<p>The Amazon Resource Name (ARN) of the directory the index and object exist in.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>A reference to the index object.</p>"]
    #[serde(rename="IndexReference")]
    pub index_reference: ObjectReference,
    #[doc="<p>A reference to the object being detached from the index.</p>"]
    #[serde(rename="TargetReference")]
    pub target_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DetachFromIndexResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> of the object that was detached from the index.</p>"]
    #[serde(rename="DetachedObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DetachObjectRequest {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where objects reside. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The link name associated with the object that needs to be detached.</p>"]
    #[serde(rename="LinkName")]
    pub link_name: String,
    #[doc="<p>The parent reference from which the object with the specified link name is detached.</p>"]
    #[serde(rename="ParentReference")]
    pub parent_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DetachObjectResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> that was detached from the object.</p>"]
    #[serde(rename="DetachedObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DetachPolicyRequest {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where both objects reside. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>Reference that identifies the object whose policy object will be detached.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
    #[doc="<p>Reference that identifies the policy object.</p>"]
    #[serde(rename="PolicyReference")]
    pub policy_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DetachPolicyResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DetachTypedLinkRequest {
    #[doc="<p>The Amazon Resource Name (ARN) of the directory where you want to detach the typed link.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>Used to accept a typed link specifier as input.</p>"]
    #[serde(rename="TypedLinkSpecifier")]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[doc="<p>Directory structure that includes the directory name and directory ARN.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Directory {
    #[doc="<p>The date and time when the directory was created.</p>"]
    #[serde(rename="CreationDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the directory. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_arn: Option<String>,
    #[doc="<p>The name of the directory.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The state of the directory. Can be either <code>Enabled</code>, <code>Disabled</code>, or <code>Deleted</code>.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DisableDirectoryRequest {
    #[doc="<p>The ARN of the directory to disable.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DisableDirectoryResponse {
    #[doc="<p>The ARN of the directory that has been disabled.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct EnableDirectoryRequest {
    #[doc="<p>The ARN of the directory to enable.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct EnableDirectoryResponse {
    #[doc="<p>The ARN of the enabled directory.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
}

#[doc="<p>A structure that contains <code>Name</code>, <code>ARN</code>, <code>Attributes</code>, <a>Rule</a>s, and <code>ObjectTypes</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Facet {
    #[doc="<p>The name of the <a>Facet</a>.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The object type that is associated with the facet. See <a>CreateFacetRequest$ObjectType</a> for more details.</p>"]
    #[serde(rename="ObjectType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_type: Option<String>,
}

#[doc="<p>An attribute that is associated with the <a>Facet</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct FacetAttribute {
    #[doc="<p>A facet attribute consists of either a definition or a reference. This structure contains the attribute definition. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#attributereferences\">Attribute References</a> for more information.</p>"]
    #[serde(rename="AttributeDefinition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_definition: Option<FacetAttributeDefinition>,
    #[doc="<p>An attribute reference that is associated with the attribute. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#attributereferences\">Attribute References</a> for more information.</p>"]
    #[serde(rename="AttributeReference")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_reference: Option<FacetAttributeReference>,
    #[doc="<p>The name of the facet attribute.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The required behavior of the <code>FacetAttribute</code>.</p>"]
    #[serde(rename="RequiredBehavior")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_behavior: Option<String>,
}

#[doc="<p>A facet attribute definition. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#attributereferences\">Attribute References</a> for more information.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct FacetAttributeDefinition {
    #[doc="<p>The default value of the attribute (if configured).</p>"]
    #[serde(rename="DefaultValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_value: Option<TypedAttributeValue>,
    #[doc="<p>Whether the attribute is mutable or not.</p>"]
    #[serde(rename="IsImmutable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_immutable: Option<bool>,
    #[doc="<p>Validation rules attached to the attribute definition.</p>"]
    #[serde(rename="Rules")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rules: Option<::std::collections::HashMap<String, Rule>>,
    #[doc="<p>The type of the attribute.</p>"]
    #[serde(rename="Type")]
    pub type_: String,
}

#[doc="<p>The facet attribute reference that specifies the attribute definition that contains the attribute facet name and attribute name.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct FacetAttributeReference {
    #[doc="<p>The target attribute name that is associated with the facet reference. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#attributereferences\">Attribute References</a> for more information.</p>"]
    #[serde(rename="TargetAttributeName")]
    pub target_attribute_name: String,
    #[doc="<p>The target facet name that is associated with the facet reference. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_advanced.html#attributereferences\">Attribute References</a> for more information.</p>"]
    #[serde(rename="TargetFacetName")]
    pub target_facet_name: String,
}

#[doc="<p>A structure that contains information used to update an attribute.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct FacetAttributeUpdate {
    #[doc="<p>The action to perform when updating the attribute.</p>"]
    #[serde(rename="Action")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action: Option<String>,
    #[doc="<p>The attribute to update.</p>"]
    #[serde(rename="Attribute")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute: Option<FacetAttribute>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetDirectoryRequest {
    #[doc="<p>The ARN of the directory.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetDirectoryResponse {
    #[doc="<p>Metadata about the directory.</p>"]
    #[serde(rename="Directory")]
    pub directory: Directory,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetFacetRequest {
    #[doc="<p>The name of the facet to retrieve.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Facet</a>. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetFacetResponse {
    #[doc="<p>The <a>Facet</a> structure that is associated with the facet.</p>"]
    #[serde(rename="Facet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub facet: Option<Facet>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetObjectInformationRequest {
    #[doc="<p>The consistency level at which to retrieve the object information.</p>"]
    #[serde(rename="ConsistencyLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consistency_level: Option<String>,
    #[doc="<p>The ARN of the directory being retrieved.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>A reference to the object.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetObjectInformationResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> of the specified object.</p>"]
    #[serde(rename="ObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifier: Option<String>,
    #[doc="<p>The facets attached to the specified object.</p>"]
    #[serde(rename="SchemaFacets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_facets: Option<Vec<SchemaFacet>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetSchemaAsJsonRequest {
    #[doc="<p>The ARN of the schema to retrieve.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetSchemaAsJsonResponse {
    #[doc="<p>The JSON representation of the schema document.</p>"]
    #[serde(rename="Document")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub document: Option<String>,
    #[doc="<p>The name of the retrieved schema.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetTypedLinkFacetInformationRequest {
    #[doc="<p>The unique name of the typed link facet.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetTypedLinkFacetInformationResponse {
    #[doc="<p>The order of identity attributes for the facet, from most significant to least significant. The ability to filter typed links considers the order that the attributes are defined on the typed link facet. When providing ranges to typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range. Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls. For more information about identity attributes, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    #[serde(rename="IdentityAttributeOrder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_attribute_order: Option<Vec<String>>,
}

#[doc="<p>Represents an index and an attached object.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct IndexAttachment {
    #[doc="<p>The indexed attribute values.</p>"]
    #[serde(rename="IndexedAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub indexed_attributes: Option<Vec<AttributeKeyAndValue>>,
    #[doc="<p>The <code>ObjectIdentifier</code> of the object attached to the index.</p>"]
    #[serde(rename="ObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListAppliedSchemaArnsRequest {
    #[doc="<p>The ARN of the directory you are listing.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListAppliedSchemaArnsResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The ARNs of schemas that are applied to the directory.</p>"]
    #[serde(rename="SchemaArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_arns: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListAttachedIndicesRequest {
    #[doc="<p>The consistency level to use for this operation.</p>"]
    #[serde(rename="ConsistencyLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consistency_level: Option<String>,
    #[doc="<p>The ARN of the directory.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A reference to the object that has indices attached.</p>"]
    #[serde(rename="TargetReference")]
    pub target_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListAttachedIndicesResponse {
    #[doc="<p>The indices attached to the specified object.</p>"]
    #[serde(rename="IndexAttachments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListDevelopmentSchemaArnsRequest {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListDevelopmentSchemaArnsResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The ARNs of retrieved development schemas.</p>"]
    #[serde(rename="SchemaArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_arns: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListDirectoriesRequest {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The state of the directories in the list. Can be either Enabled, Disabled, or Deleted.</p>"]
    #[serde(rename="state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListDirectoriesResponse {
    #[doc="<p>Lists all directories that are associated with your account in pagination fashion.</p>"]
    #[serde(rename="Directories")]
    pub directories: Vec<Directory>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListFacetAttributesRequest {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The name of the facet whose attributes will be retrieved.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The ARN of the schema where the facet resides.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListFacetAttributesResponse {
    #[doc="<p>The attributes attached to the facet.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<FacetAttribute>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListFacetNamesRequest {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) to retrieve facet names from.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListFacetNamesResponse {
    #[doc="<p>The names of facets that exist within the schema.</p>"]
    #[serde(rename="FacetNames")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub facet_names: Option<Vec<String>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListIncomingTypedLinksRequest {
    #[doc="<p>The consistency level to execute the request at.</p>"]
    #[serde(rename="ConsistencyLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consistency_level: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the directory where you want to list the typed links.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>Provides range filters for multiple attributes. When providing ranges to typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range.</p>"]
    #[serde(rename="FilterAttributeRanges")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    #[doc="<p>Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls.</p>"]
    #[serde(rename="FilterTypedLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Reference that identifies the object whose attributes will be listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListIncomingTypedLinksResponse {
    #[doc="<p>Returns one or more typed link specifiers as output.</p>"]
    #[serde(rename="LinkSpecifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub link_specifiers: Option<Vec<TypedLinkSpecifier>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListIndexRequest {
    #[doc="<p>The consistency level to execute the request at.</p>"]
    #[serde(rename="ConsistencyLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consistency_level: Option<String>,
    #[doc="<p>The ARN of the directory that the index exists in.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The reference to the index to list.</p>"]
    #[serde(rename="IndexReference")]
    pub index_reference: ObjectReference,
    #[doc="<p>The maximum number of results to retrieve from the index.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Specifies the ranges of indexed values that you want to query.</p>"]
    #[serde(rename="RangesOnIndexedValues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ranges_on_indexed_values: Option<Vec<ObjectAttributeRange>>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListIndexResponse {
    #[doc="<p>The objects and indexed values attached to the index.</p>"]
    #[serde(rename="IndexAttachments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListObjectAttributesRequest {
    #[doc="<p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>"]
    #[serde(rename="ConsistencyLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consistency_level: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>Used to filter the list of object attributes that are associated with a certain facet.</p>"]
    #[serde(rename="FacetFilter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub facet_filter: Option<SchemaFacet>,
    #[doc="<p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The reference that identifies the object whose attributes will be listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListObjectAttributesResponse {
    #[doc="<p>Attributes map that is associated with the object. <code>AttributeArn</code> is the key, and attribute value is the value.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListObjectChildrenRequest {
    #[doc="<p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>"]
    #[serde(rename="ConsistencyLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consistency_level: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The reference that identifies the object for which child objects are being listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListObjectChildrenResponse {
    #[doc="<p>Children structure, which is a map with key as the <code>LinkName</code> and <code>ObjectIdentifier</code> as the value.</p>"]
    #[serde(rename="Children")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub children: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListObjectParentPathsRequest {
    #[doc="<p>The ARN of the directory to which the parent path applies.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The reference that identifies the object whose parent paths are listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListObjectParentPathsResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Returns the path to the <code>ObjectIdentifiers</code> that are associated with the directory.</p>"]
    #[serde(rename="PathToObjectIdentifiersList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path_to_object_identifiers_list: Option<Vec<PathToObjectIdentifiers>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListObjectParentsRequest {
    #[doc="<p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>"]
    #[serde(rename="ConsistencyLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consistency_level: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The reference that identifies the object for which parent objects are being listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListObjectParentsResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The parent structure, which is a map with key as the <code>ObjectIdentifier</code> and LinkName as the value.</p>"]
    #[serde(rename="Parents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parents: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListObjectPoliciesRequest {
    #[doc="<p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>"]
    #[serde(rename="ConsistencyLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consistency_level: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where objects reside. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Reference that identifies the object for which policies will be listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListObjectPoliciesResponse {
    #[doc="<p>A list of policy <code>ObjectIdentifiers</code>, that are attached to the object.</p>"]
    #[serde(rename="AttachedPolicyIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attached_policy_ids: Option<Vec<String>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListOutgoingTypedLinksRequest {
    #[doc="<p>The consistency level to execute the request at.</p>"]
    #[serde(rename="ConsistencyLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consistency_level: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the directory where you want to list the typed links.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>Provides range filters for multiple attributes. When providing ranges to typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range.</p>"]
    #[serde(rename="FilterAttributeRanges")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    #[doc="<p>Filters are interpreted in the order of the attributes defined on the typed link facet, not the order they are supplied to any API calls.</p>"]
    #[serde(rename="FilterTypedLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A reference that identifies the object whose attributes will be listed.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListOutgoingTypedLinksResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Returns a typed link specifier as output.</p>"]
    #[serde(rename="TypedLinkSpecifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub typed_link_specifiers: Option<Vec<TypedLinkSpecifier>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListPolicyAttachmentsRequest {
    #[doc="<p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>"]
    #[serde(rename="ConsistencyLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consistency_level: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where objects reside. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The reference that identifies the policy object.</p>"]
    #[serde(rename="PolicyReference")]
    pub policy_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListPolicyAttachmentsResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of <code>ObjectIdentifiers</code> to which the policy is attached.</p>"]
    #[serde(rename="ObjectIdentifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifiers: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListPublishedSchemaArnsRequest {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListPublishedSchemaArnsResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The ARNs of published schemas.</p>"]
    #[serde(rename="SchemaArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_arns: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTagsForResourceRequest {
    #[doc="<p>The <code>MaxResults</code> parameter sets the maximum number of results returned in a single page. This is for future use and is not supported currently.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token. This is for future use. Currently pagination is not supported for tagging.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the resource. Tagging is only supported for directories.</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTagsForResourceResponse {
    #[doc="<p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of tag key value pairs that are associated with the response.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTypedLinkFacetAttributesRequest {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The unique name of the typed link facet.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTypedLinkFacetAttributesResponse {
    #[doc="<p>An ordered set of attributes associate with the typed link.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<TypedLinkAttributeDefinition>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTypedLinkFacetNamesRequest {
    #[doc="<p>The maximum number of results to retrieve.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTypedLinkFacetNamesResponse {
    #[doc="<p>The names of typed link facets that exist within the schema.</p>"]
    #[serde(rename="FacetNames")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub facet_names: Option<Vec<String>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct LookupPolicyRequest {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a>. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The token to request the next page of results.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Reference that identifies the object whose policies will be looked up.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct LookupPolicyResponse {
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Provides list of path to policies. Policies contain <code>PolicyId</code>, <code>ObjectIdentifier</code>, and <code>PolicyType</code>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies\">Policies</a>.</p>"]
    #[serde(rename="PolicyToPathList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_to_path_list: Option<Vec<PolicyToPath>>,
}

#[doc="<p>The action to take on the object attribute.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ObjectAttributeAction {
    #[doc="<p>A type that can be either <code>Update</code> or <code>Delete</code>.</p>"]
    #[serde(rename="ObjectAttributeActionType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_attribute_action_type: Option<String>,
    #[doc="<p>The value that you want to update to.</p>"]
    #[serde(rename="ObjectAttributeUpdateValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_attribute_update_value: Option<TypedAttributeValue>,
}

#[doc="<p>A range of attributes.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ObjectAttributeRange {
    #[doc="<p>The key of the attribute that the attribute range covers.</p>"]
    #[serde(rename="AttributeKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_key: Option<AttributeKey>,
    #[doc="<p>The range of attribute values being selected.</p>"]
    #[serde(rename="Range")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub range: Option<TypedAttributeValueRange>,
}

#[doc="<p>Structure that contains attribute update information.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ObjectAttributeUpdate {
    #[doc="<p>The action to perform as part of the attribute update.</p>"]
    #[serde(rename="ObjectAttributeAction")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_attribute_action: Option<ObjectAttributeAction>,
    #[doc="<p>The key of the attribute being updated.</p>"]
    #[serde(rename="ObjectAttributeKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_attribute_key: Option<AttributeKey>,
}

#[doc="<p>The reference that identifies an object.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ObjectReference {
    #[doc="<p>A path selector supports easy selection of an object by the parent/child links leading to it from the directory root. Use the link names from each parent/child link to construct the path. Path selectors start with a slash (/) and link names are separated by slashes. For more information about paths, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#accessingobjects\">Accessing Objects</a>. You can identify an object in one of the following ways:</p> <ul> <li> <p> <i>$ObjectIdentifier</i> - An object identifier is an opaque string provided by Amazon Cloud Directory. When creating objects, the system will provide you with the identifier of the created object. An object’s identifier is immutable and no two objects will ever share the same object identifier</p> </li> <li> <p> <i>/some/path</i> - Identifies the object based on path</p> </li> <li> <p> <i>#SomeBatchReference</i> - Identifies the object in a batch call</p> </li> </ul>"]
    #[serde(rename="Selector")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selector: Option<String>,
}

#[doc="<p>Returns the path to the <code>ObjectIdentifiers</code> that is associated with the directory.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PathToObjectIdentifiers {
    #[doc="<p>Lists <code>ObjectIdentifiers</code> starting from directory root to the object in the request.</p>"]
    #[serde(rename="ObjectIdentifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifiers: Option<Vec<String>>,
    #[doc="<p>The path that is used to identify the object starting from directory root.</p>"]
    #[serde(rename="Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
}

#[doc="<p>Contains the <code>PolicyType</code>, <code>PolicyId</code>, and the <code>ObjectIdentifier</code> to which it is attached. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies\">Policies</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PolicyAttachment {
    #[doc="<p>The <code>ObjectIdentifier</code> that is associated with <code>PolicyAttachment</code>.</p>"]
    #[serde(rename="ObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifier: Option<String>,
    #[doc="<p>The ID of <code>PolicyAttachment</code>.</p>"]
    #[serde(rename="PolicyId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_id: Option<String>,
    #[doc="<p>The type of policy that can be associated with <code>PolicyAttachment</code>.</p>"]
    #[serde(rename="PolicyType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_type: Option<String>,
}

#[doc="<p>Used when a regular object exists in a <a>Directory</a> and you want to find all of the policies that are associated with that object and the parent to that object.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PolicyToPath {
    #[doc="<p>The path that is referenced from the root.</p>"]
    #[serde(rename="Path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
    #[doc="<p>List of policy objects.</p>"]
    #[serde(rename="Policies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policies: Option<Vec<PolicyAttachment>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PublishSchemaRequest {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the development schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DevelopmentSchemaArn")]
    pub development_schema_arn: String,
    #[doc="<p>The new name under which the schema will be published. If this is not provided, the development schema is considered.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The version under which the schema will be published.</p>"]
    #[serde(rename="Version")]
    pub version: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PublishSchemaResponse {
    #[doc="<p>The ARN that is associated with the published schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="PublishedSchemaArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub published_schema_arn: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PutSchemaFromJsonRequest {
    #[doc="<p>The replacement JSON schema.</p>"]
    #[serde(rename="Document")]
    pub document: String,
    #[doc="<p>The ARN of the schema to update.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutSchemaFromJsonResponse {
    #[doc="<p>The ARN of the schema to update.</p>"]
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RemoveFacetFromObjectRequest {
    #[doc="<p>The ARN of the directory in which the object resides.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>A reference to the object to remove the facet from.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
    #[doc="<p>The facet to remove.</p>"]
    #[serde(rename="SchemaFacet")]
    pub schema_facet: SchemaFacet,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RemoveFacetFromObjectResponse;

#[doc="<p>Contains an Amazon Resource Name (ARN) and parameters that are associated with the rule.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Rule {
    #[doc="<p>The minimum and maximum parameters that are associated with the rule.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The type of attribute validation rule.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[doc="<p>A facet.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SchemaFacet {
    #[doc="<p>The name of the facet.</p>"]
    #[serde(rename="FacetName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub facet_name: Option<String>,
    #[doc="<p>The ARN of the schema that contains the facet.</p>"]
    #[serde(rename="SchemaArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_arn: Option<String>,
}

#[doc="<p>The tag structure that contains a tag key and value.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Tag {
    #[doc="<p>The key that is associated with the tag.</p>"]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,
    #[doc="<p>The value that is associated with the tag.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct TagResourceRequest {
    #[doc="<p>The Amazon Resource Name (ARN) of the resource. Tagging is only supported for directories.</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: String,
    #[doc="<p>A list of tag key-value pairs.</p>"]
    #[serde(rename="Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct TagResourceResponse;

#[doc="<p>Represents the data for a typed attribute. You can set one, and only one, of the elements. Each attribute in an item is a name-value pair. Attributes have a single value.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct TypedAttributeValue {
    #[doc="<p>A binary data value.</p>"]
    #[serde(rename="BinaryValue")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub binary_value: Option<Vec<u8>>,
    #[doc="<p>A Boolean data value.</p>"]
    #[serde(rename="BooleanValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub boolean_value: Option<bool>,
    #[doc="<p>A date and time value.</p>"]
    #[serde(rename="DatetimeValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub datetime_value: Option<f64>,
    #[doc="<p>A number data value.</p>"]
    #[serde(rename="NumberValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number_value: Option<String>,
    #[doc="<p>A string data value.</p>"]
    #[serde(rename="StringValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub string_value: Option<String>,
}

#[doc="<p>A range of attribute values.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TypedAttributeValueRange {
    #[doc="<p>The inclusive or exclusive range end.</p>"]
    #[serde(rename="EndMode")]
    pub end_mode: String,
    #[doc="<p>The attribute value to terminate the range at.</p>"]
    #[serde(rename="EndValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_value: Option<TypedAttributeValue>,
    #[doc="<p>The inclusive or exclusive range start.</p>"]
    #[serde(rename="StartMode")]
    pub start_mode: String,
    #[doc="<p>The value to start the range at.</p>"]
    #[serde(rename="StartValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_value: Option<TypedAttributeValue>,
}

#[doc="<p>A typed link attribute definition.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct TypedLinkAttributeDefinition {
    #[doc="<p>The default value of the attribute (if configured).</p>"]
    #[serde(rename="DefaultValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_value: Option<TypedAttributeValue>,
    #[doc="<p>Whether the attribute is mutable or not.</p>"]
    #[serde(rename="IsImmutable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_immutable: Option<bool>,
    #[doc="<p>The unique name of the typed link attribute.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The required behavior of the <code>TypedLinkAttributeDefinition</code>.</p>"]
    #[serde(rename="RequiredBehavior")]
    pub required_behavior: String,
    #[doc="<p>Validation rules that are attached to the attribute definition.</p>"]
    #[serde(rename="Rules")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rules: Option<::std::collections::HashMap<String, Rule>>,
    #[doc="<p>The type of the attribute.</p>"]
    #[serde(rename="Type")]
    pub type_: String,
}

#[doc="<p>Identifies the range of attributes that are used by a specified filter.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TypedLinkAttributeRange {
    #[doc="<p>The unique name of the typed link attribute.</p>"]
    #[serde(rename="AttributeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_name: Option<String>,
    #[doc="<p>The range of attribute values that are being selected.</p>"]
    #[serde(rename="Range")]
    pub range: TypedAttributeValueRange,
}

#[doc="<p>Defines the typed links structure and its attributes. To create a typed link facet, use the <a>CreateTypedLinkFacet</a> API.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TypedLinkFacet {
    #[doc="<p>A set of key-value pairs associated with the typed link. Typed link attributes are used when you have data values that are related to the link itself, and not to one of the two objects being linked. Identity attributes also serve to distinguish the link from others of the same type between the same objects.</p>"]
    #[serde(rename="Attributes")]
    pub attributes: Vec<TypedLinkAttributeDefinition>,
    #[doc="<p>The set of attributes that distinguish links made from this facet from each other, in the order of significance. Listing typed links can filter on the values of these attributes. See <a>ListOutgoingTypedLinks</a> and <a>ListIncomingTypedLinks</a> for details.</p>"]
    #[serde(rename="IdentityAttributeOrder")]
    pub identity_attribute_order: Vec<String>,
    #[doc="<p>The unique name of the typed link facet.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[doc="<p>A typed link facet attribute update.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TypedLinkFacetAttributeUpdate {
    #[doc="<p>The action to perform when updating the attribute.</p>"]
    #[serde(rename="Action")]
    pub action: String,
    #[doc="<p>The attribute to update.</p>"]
    #[serde(rename="Attribute")]
    pub attribute: TypedLinkAttributeDefinition,
}

#[doc="<p>Identifies the schema Amazon Resource Name (ARN) and facet name for the typed link.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct TypedLinkSchemaAndFacetName {
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
    #[doc="<p>The unique name of the typed link facet.</p>"]
    #[serde(rename="TypedLinkName")]
    pub typed_link_name: String,
}

#[doc="<p>Contains all the information that is used to uniquely identify a typed link. The parameters discussed in this topic are used to uniquely specify the typed link being operated on. The <a>AttachTypedLink</a> API returns a typed link specifier while the <a>DetachTypedLink</a> API accepts one as input. Similarly, the <a>ListIncomingTypedLinks</a> and <a>ListOutgoingTypedLinks</a> API operations provide typed link specifiers as output. You can also construct a typed link specifier from scratch.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct TypedLinkSpecifier {
    #[doc="<p>Identifies the attribute value to update.</p>"]
    #[serde(rename="IdentityAttributeValues")]
    pub identity_attribute_values: Vec<AttributeNameAndValue>,
    #[doc="<p>Identifies the source object that the typed link will attach to.</p>"]
    #[serde(rename="SourceObjectReference")]
    pub source_object_reference: ObjectReference,
    #[doc="<p>Identifies the target object that the typed link will attach to.</p>"]
    #[serde(rename="TargetObjectReference")]
    pub target_object_reference: ObjectReference,
    #[doc="<p>Identifies the typed link facet that is associated with the typed link.</p>"]
    #[serde(rename="TypedLinkFacet")]
    pub typed_link_facet: TypedLinkSchemaAndFacetName,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UntagResourceRequest {
    #[doc="<p>The Amazon Resource Name (ARN) of the resource. Tagging is only supported for directories.</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: String,
    #[doc="<p>Keys of the tag that need to be removed from the resource.</p>"]
    #[serde(rename="TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UntagResourceResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateFacetRequest {
    #[doc="<p>List of attributes that need to be updated in a given schema <a>Facet</a>. Each attribute is followed by <code>AttributeAction</code>, which specifies the type of update operation to perform. </p>"]
    #[serde(rename="AttributeUpdates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_updates: Option<Vec<FacetAttributeUpdate>>,
    #[doc="<p>The name of the facet.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The object type that is associated with the facet. See <a>CreateFacetRequest$ObjectType</a> for more details.</p>"]
    #[serde(rename="ObjectType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_type: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Facet</a>. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateFacetResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateObjectAttributesRequest {
    #[doc="<p>The attributes update structure.</p>"]
    #[serde(rename="AttributeUpdates")]
    pub attribute_updates: Vec<ObjectAttributeUpdate>,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a> where the object resides. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="DirectoryArn")]
    pub directory_arn: String,
    #[doc="<p>The reference that identifies the object.</p>"]
    #[serde(rename="ObjectReference")]
    pub object_reference: ObjectReference,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateObjectAttributesResponse {
    #[doc="<p>The <code>ObjectIdentifier</code> of the updated object.</p>"]
    #[serde(rename="ObjectIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateSchemaRequest {
    #[doc="<p>The name of the schema.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The Amazon Resource Name (ARN) of the development schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateSchemaResponse {
    #[doc="<p>The ARN that is associated with the updated schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateTypedLinkFacetRequest {
    #[doc="<p>Attributes update structure.</p>"]
    #[serde(rename="AttributeUpdates")]
    pub attribute_updates: Vec<TypedLinkFacetAttributeUpdate>,
    #[doc="<p>The order of identity attributes for the facet, from most significant to least significant. The ability to filter typed links considers the order that the attributes are defined on the typed link facet. When providing ranges to a typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range. Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls. For more information about identity attributes, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    #[serde(rename="IdentityAttributeOrder")]
    pub identity_attribute_order: Vec<String>,
    #[doc="<p>The unique name of the typed link facet.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <a>arns</a>.</p>"]
    #[serde(rename="SchemaArn")]
    pub schema_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateTypedLinkFacetResponse;

/// Errors returned by AddFacetToObject
#[derive(Debug, PartialEq)]
pub enum AddFacetToObjectError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AddFacetToObjectError {
    pub fn from_body(body: &str) -> AddFacetToObjectError {
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
                        AddFacetToObjectError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        AddFacetToObjectError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "FacetValidationException" => {
                        AddFacetToObjectError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        AddFacetToObjectError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        AddFacetToObjectError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AddFacetToObjectError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AddFacetToObjectError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        AddFacetToObjectError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddFacetToObjectError::Validation(error_message.to_string())
                    }
                    _ => AddFacetToObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddFacetToObjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddFacetToObjectError {
    fn from(err: serde_json::error::Error) -> AddFacetToObjectError {
        AddFacetToObjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddFacetToObjectError {
    fn from(err: CredentialsError) -> AddFacetToObjectError {
        AddFacetToObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddFacetToObjectError {
    fn from(err: HttpDispatchError) -> AddFacetToObjectError {
        AddFacetToObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddFacetToObjectError {
    fn from(err: io::Error) -> AddFacetToObjectError {
        AddFacetToObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddFacetToObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddFacetToObjectError {
    fn description(&self) -> &str {
        match *self {
            AddFacetToObjectError::AccessDenied(ref cause) => cause,
            AddFacetToObjectError::DirectoryNotEnabled(ref cause) => cause,
            AddFacetToObjectError::FacetValidation(ref cause) => cause,
            AddFacetToObjectError::InternalService(ref cause) => cause,
            AddFacetToObjectError::InvalidArn(ref cause) => cause,
            AddFacetToObjectError::LimitExceeded(ref cause) => cause,
            AddFacetToObjectError::ResourceNotFound(ref cause) => cause,
            AddFacetToObjectError::RetryableConflict(ref cause) => cause,
            AddFacetToObjectError::Validation(ref cause) => cause,
            AddFacetToObjectError::Credentials(ref err) => err.description(),
            AddFacetToObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddFacetToObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ApplySchema
#[derive(Debug, PartialEq)]
pub enum ApplySchemaError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that an attempt to attach an object with the same link name or to apply a schema with the same name has occurred. Rename the link or the schema and then try again.</p>
    InvalidAttachment(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ApplySchemaError {
    pub fn from_body(body: &str) -> ApplySchemaError {
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
                        ApplySchemaError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ApplySchemaError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ApplySchemaError::InvalidArn(String::from(error_message))
                    }
                    "InvalidAttachmentException" => {
                        ApplySchemaError::InvalidAttachment(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ApplySchemaError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ApplySchemaError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ApplySchemaError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ApplySchemaError::Validation(error_message.to_string())
                    }
                    _ => ApplySchemaError::Unknown(String::from(body)),
                }
            }
            Err(_) => ApplySchemaError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ApplySchemaError {
    fn from(err: serde_json::error::Error) -> ApplySchemaError {
        ApplySchemaError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ApplySchemaError {
    fn from(err: CredentialsError) -> ApplySchemaError {
        ApplySchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ApplySchemaError {
    fn from(err: HttpDispatchError) -> ApplySchemaError {
        ApplySchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for ApplySchemaError {
    fn from(err: io::Error) -> ApplySchemaError {
        ApplySchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ApplySchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ApplySchemaError {
    fn description(&self) -> &str {
        match *self {
            ApplySchemaError::AccessDenied(ref cause) => cause,
            ApplySchemaError::InternalService(ref cause) => cause,
            ApplySchemaError::InvalidArn(ref cause) => cause,
            ApplySchemaError::InvalidAttachment(ref cause) => cause,
            ApplySchemaError::LimitExceeded(ref cause) => cause,
            ApplySchemaError::ResourceNotFound(ref cause) => cause,
            ApplySchemaError::RetryableConflict(ref cause) => cause,
            ApplySchemaError::Validation(ref cause) => cause,
            ApplySchemaError::Credentials(ref err) => err.description(),
            ApplySchemaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ApplySchemaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachObject
#[derive(Debug, PartialEq)]
pub enum AttachObjectError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that an attempt to attach an object with the same link name or to apply a schema with the same name has occurred. Rename the link or the schema and then try again.</p>
    InvalidAttachment(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AttachObjectError {
    pub fn from_body(body: &str) -> AttachObjectError {
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
                        AttachObjectError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        AttachObjectError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "FacetValidationException" => {
                        AttachObjectError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        AttachObjectError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        AttachObjectError::InvalidArn(String::from(error_message))
                    }
                    "InvalidAttachmentException" => {
                        AttachObjectError::InvalidAttachment(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AttachObjectError::LimitExceeded(String::from(error_message))
                    }
                    "LinkNameAlreadyInUseException" => {
                        AttachObjectError::LinkNameAlreadyInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AttachObjectError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        AttachObjectError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        AttachObjectError::Validation(error_message.to_string())
                    }
                    _ => AttachObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachObjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachObjectError {
    fn from(err: serde_json::error::Error) -> AttachObjectError {
        AttachObjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachObjectError {
    fn from(err: CredentialsError) -> AttachObjectError {
        AttachObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachObjectError {
    fn from(err: HttpDispatchError) -> AttachObjectError {
        AttachObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachObjectError {
    fn from(err: io::Error) -> AttachObjectError {
        AttachObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachObjectError {
    fn description(&self) -> &str {
        match *self {
            AttachObjectError::AccessDenied(ref cause) => cause,
            AttachObjectError::DirectoryNotEnabled(ref cause) => cause,
            AttachObjectError::FacetValidation(ref cause) => cause,
            AttachObjectError::InternalService(ref cause) => cause,
            AttachObjectError::InvalidArn(ref cause) => cause,
            AttachObjectError::InvalidAttachment(ref cause) => cause,
            AttachObjectError::LimitExceeded(ref cause) => cause,
            AttachObjectError::LinkNameAlreadyInUse(ref cause) => cause,
            AttachObjectError::ResourceNotFound(ref cause) => cause,
            AttachObjectError::RetryableConflict(ref cause) => cause,
            AttachObjectError::Validation(ref cause) => cause,
            AttachObjectError::Credentials(ref err) => err.description(),
            AttachObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachPolicy
#[derive(Debug, PartialEq)]
pub enum AttachPolicyError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Indicates that the requested operation can only operate on policy objects.</p>
    NotPolicy(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AttachPolicyError {
    pub fn from_body(body: &str) -> AttachPolicyError {
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
                        AttachPolicyError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        AttachPolicyError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        AttachPolicyError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        AttachPolicyError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AttachPolicyError::LimitExceeded(String::from(error_message))
                    }
                    "NotPolicyException" => {
                        AttachPolicyError::NotPolicy(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AttachPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        AttachPolicyError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        AttachPolicyError::Validation(error_message.to_string())
                    }
                    _ => AttachPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachPolicyError {
    fn from(err: serde_json::error::Error) -> AttachPolicyError {
        AttachPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachPolicyError {
    fn from(err: CredentialsError) -> AttachPolicyError {
        AttachPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachPolicyError {
    fn from(err: HttpDispatchError) -> AttachPolicyError {
        AttachPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachPolicyError {
    fn from(err: io::Error) -> AttachPolicyError {
        AttachPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachPolicyError {
    fn description(&self) -> &str {
        match *self {
            AttachPolicyError::AccessDenied(ref cause) => cause,
            AttachPolicyError::DirectoryNotEnabled(ref cause) => cause,
            AttachPolicyError::InternalService(ref cause) => cause,
            AttachPolicyError::InvalidArn(ref cause) => cause,
            AttachPolicyError::LimitExceeded(ref cause) => cause,
            AttachPolicyError::NotPolicy(ref cause) => cause,
            AttachPolicyError::ResourceNotFound(ref cause) => cause,
            AttachPolicyError::RetryableConflict(ref cause) => cause,
            AttachPolicyError::Validation(ref cause) => cause,
            AttachPolicyError::Credentials(ref err) => err.description(),
            AttachPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachToIndex
#[derive(Debug, PartialEq)]
pub enum AttachToIndexError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>An object has been attempted to be attached to an object that does not have the appropriate attribute value.</p>
    IndexedAttributeMissing(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    ///<p>Indicates that the requested operation can only operate on index objects.</p>
    NotIndex(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AttachToIndexError {
    pub fn from_body(body: &str) -> AttachToIndexError {
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
                        AttachToIndexError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        AttachToIndexError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "IndexedAttributeMissingException" => {
                        AttachToIndexError::IndexedAttributeMissing(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        AttachToIndexError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        AttachToIndexError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AttachToIndexError::LimitExceeded(String::from(error_message))
                    }
                    "LinkNameAlreadyInUseException" => {
                        AttachToIndexError::LinkNameAlreadyInUse(String::from(error_message))
                    }
                    "NotIndexException" => {
                        AttachToIndexError::NotIndex(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AttachToIndexError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        AttachToIndexError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        AttachToIndexError::Validation(error_message.to_string())
                    }
                    _ => AttachToIndexError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachToIndexError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachToIndexError {
    fn from(err: serde_json::error::Error) -> AttachToIndexError {
        AttachToIndexError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachToIndexError {
    fn from(err: CredentialsError) -> AttachToIndexError {
        AttachToIndexError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachToIndexError {
    fn from(err: HttpDispatchError) -> AttachToIndexError {
        AttachToIndexError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachToIndexError {
    fn from(err: io::Error) -> AttachToIndexError {
        AttachToIndexError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachToIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachToIndexError {
    fn description(&self) -> &str {
        match *self {
            AttachToIndexError::AccessDenied(ref cause) => cause,
            AttachToIndexError::DirectoryNotEnabled(ref cause) => cause,
            AttachToIndexError::IndexedAttributeMissing(ref cause) => cause,
            AttachToIndexError::InternalService(ref cause) => cause,
            AttachToIndexError::InvalidArn(ref cause) => cause,
            AttachToIndexError::LimitExceeded(ref cause) => cause,
            AttachToIndexError::LinkNameAlreadyInUse(ref cause) => cause,
            AttachToIndexError::NotIndex(ref cause) => cause,
            AttachToIndexError::ResourceNotFound(ref cause) => cause,
            AttachToIndexError::RetryableConflict(ref cause) => cause,
            AttachToIndexError::Validation(ref cause) => cause,
            AttachToIndexError::Credentials(ref err) => err.description(),
            AttachToIndexError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachToIndexError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachTypedLink
#[derive(Debug, PartialEq)]
pub enum AttachTypedLinkError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that an attempt to attach an object with the same link name or to apply a schema with the same name has occurred. Rename the link or the schema and then try again.</p>
    InvalidAttachment(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AttachTypedLinkError {
    pub fn from_body(body: &str) -> AttachTypedLinkError {
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
                        AttachTypedLinkError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        AttachTypedLinkError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "FacetValidationException" => {
                        AttachTypedLinkError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        AttachTypedLinkError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        AttachTypedLinkError::InvalidArn(String::from(error_message))
                    }
                    "InvalidAttachmentException" => {
                        AttachTypedLinkError::InvalidAttachment(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AttachTypedLinkError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AttachTypedLinkError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        AttachTypedLinkError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        AttachTypedLinkError::Validation(error_message.to_string())
                    }
                    _ => AttachTypedLinkError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachTypedLinkError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachTypedLinkError {
    fn from(err: serde_json::error::Error) -> AttachTypedLinkError {
        AttachTypedLinkError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachTypedLinkError {
    fn from(err: CredentialsError) -> AttachTypedLinkError {
        AttachTypedLinkError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachTypedLinkError {
    fn from(err: HttpDispatchError) -> AttachTypedLinkError {
        AttachTypedLinkError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachTypedLinkError {
    fn from(err: io::Error) -> AttachTypedLinkError {
        AttachTypedLinkError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachTypedLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachTypedLinkError {
    fn description(&self) -> &str {
        match *self {
            AttachTypedLinkError::AccessDenied(ref cause) => cause,
            AttachTypedLinkError::DirectoryNotEnabled(ref cause) => cause,
            AttachTypedLinkError::FacetValidation(ref cause) => cause,
            AttachTypedLinkError::InternalService(ref cause) => cause,
            AttachTypedLinkError::InvalidArn(ref cause) => cause,
            AttachTypedLinkError::InvalidAttachment(ref cause) => cause,
            AttachTypedLinkError::LimitExceeded(ref cause) => cause,
            AttachTypedLinkError::ResourceNotFound(ref cause) => cause,
            AttachTypedLinkError::RetryableConflict(ref cause) => cause,
            AttachTypedLinkError::Validation(ref cause) => cause,
            AttachTypedLinkError::Credentials(ref err) => err.description(),
            AttachTypedLinkError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachTypedLinkError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchRead
#[derive(Debug, PartialEq)]
pub enum BatchReadError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl BatchReadError {
    pub fn from_body(body: &str) -> BatchReadError {
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
                        BatchReadError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        BatchReadError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        BatchReadError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        BatchReadError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        BatchReadError::LimitExceeded(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        BatchReadError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => BatchReadError::Validation(error_message.to_string()),
                    _ => BatchReadError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchReadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchReadError {
    fn from(err: serde_json::error::Error) -> BatchReadError {
        BatchReadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchReadError {
    fn from(err: CredentialsError) -> BatchReadError {
        BatchReadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchReadError {
    fn from(err: HttpDispatchError) -> BatchReadError {
        BatchReadError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchReadError {
    fn from(err: io::Error) -> BatchReadError {
        BatchReadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchReadError {
    fn description(&self) -> &str {
        match *self {
            BatchReadError::AccessDenied(ref cause) => cause,
            BatchReadError::DirectoryNotEnabled(ref cause) => cause,
            BatchReadError::InternalService(ref cause) => cause,
            BatchReadError::InvalidArn(ref cause) => cause,
            BatchReadError::LimitExceeded(ref cause) => cause,
            BatchReadError::RetryableConflict(ref cause) => cause,
            BatchReadError::Validation(ref cause) => cause,
            BatchReadError::Credentials(ref err) => err.description(),
            BatchReadError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchReadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchWrite
#[derive(Debug, PartialEq)]
pub enum BatchWriteError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>A <code>BatchWrite</code> exception has occurred.</p>
    BatchWrite(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl BatchWriteError {
    pub fn from_body(body: &str) -> BatchWriteError {
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
                        BatchWriteError::AccessDenied(String::from(error_message))
                    }
                    "BatchWriteException" => {
                        BatchWriteError::BatchWrite(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        BatchWriteError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        BatchWriteError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        BatchWriteError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        BatchWriteError::LimitExceeded(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        BatchWriteError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => BatchWriteError::Validation(error_message.to_string()),
                    _ => BatchWriteError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchWriteError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchWriteError {
    fn from(err: serde_json::error::Error) -> BatchWriteError {
        BatchWriteError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchWriteError {
    fn from(err: CredentialsError) -> BatchWriteError {
        BatchWriteError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchWriteError {
    fn from(err: HttpDispatchError) -> BatchWriteError {
        BatchWriteError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchWriteError {
    fn from(err: io::Error) -> BatchWriteError {
        BatchWriteError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchWriteError {
    fn description(&self) -> &str {
        match *self {
            BatchWriteError::AccessDenied(ref cause) => cause,
            BatchWriteError::BatchWrite(ref cause) => cause,
            BatchWriteError::DirectoryNotEnabled(ref cause) => cause,
            BatchWriteError::InternalService(ref cause) => cause,
            BatchWriteError::InvalidArn(ref cause) => cause,
            BatchWriteError::LimitExceeded(ref cause) => cause,
            BatchWriteError::RetryableConflict(ref cause) => cause,
            BatchWriteError::Validation(ref cause) => cause,
            BatchWriteError::Credentials(ref err) => err.description(),
            BatchWriteError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchWriteError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDirectory
#[derive(Debug, PartialEq)]
pub enum CreateDirectoryError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates that a <a>Directory</a> could not be created due to a naming conflict. Choose a different name and try again.</p>
    DirectoryAlreadyExists(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateDirectoryError {
    pub fn from_body(body: &str) -> CreateDirectoryError {
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
                        CreateDirectoryError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryAlreadyExistsException" => {
                        CreateDirectoryError::DirectoryAlreadyExists(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreateDirectoryError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        CreateDirectoryError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateDirectoryError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateDirectoryError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        CreateDirectoryError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDirectoryError::Validation(error_message.to_string())
                    }
                    _ => CreateDirectoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDirectoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDirectoryError {
    fn from(err: serde_json::error::Error) -> CreateDirectoryError {
        CreateDirectoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDirectoryError {
    fn from(err: CredentialsError) -> CreateDirectoryError {
        CreateDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDirectoryError {
    fn from(err: HttpDispatchError) -> CreateDirectoryError {
        CreateDirectoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDirectoryError {
    fn from(err: io::Error) -> CreateDirectoryError {
        CreateDirectoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDirectoryError {
    fn description(&self) -> &str {
        match *self {
            CreateDirectoryError::AccessDenied(ref cause) => cause,
            CreateDirectoryError::DirectoryAlreadyExists(ref cause) => cause,
            CreateDirectoryError::InternalService(ref cause) => cause,
            CreateDirectoryError::InvalidArn(ref cause) => cause,
            CreateDirectoryError::LimitExceeded(ref cause) => cause,
            CreateDirectoryError::ResourceNotFound(ref cause) => cause,
            CreateDirectoryError::RetryableConflict(ref cause) => cause,
            CreateDirectoryError::Validation(ref cause) => cause,
            CreateDirectoryError::Credentials(ref err) => err.description(),
            CreateDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDirectoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFacet
#[derive(Debug, PartialEq)]
pub enum CreateFacetError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>A facet with the same name already exists.</p>
    FacetAlreadyExists(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Occurs when any of the rule parameter keys or values are invalid.</p>
    InvalidRule(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateFacetError {
    pub fn from_body(body: &str) -> CreateFacetError {
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
                        CreateFacetError::AccessDenied(String::from(error_message))
                    }
                    "FacetAlreadyExistsException" => {
                        CreateFacetError::FacetAlreadyExists(String::from(error_message))
                    }
                    "FacetValidationException" => {
                        CreateFacetError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreateFacetError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        CreateFacetError::InvalidArn(String::from(error_message))
                    }
                    "InvalidRuleException" => {
                        CreateFacetError::InvalidRule(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateFacetError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateFacetError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        CreateFacetError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateFacetError::Validation(error_message.to_string())
                    }
                    _ => CreateFacetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateFacetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateFacetError {
    fn from(err: serde_json::error::Error) -> CreateFacetError {
        CreateFacetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateFacetError {
    fn from(err: CredentialsError) -> CreateFacetError {
        CreateFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateFacetError {
    fn from(err: HttpDispatchError) -> CreateFacetError {
        CreateFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateFacetError {
    fn from(err: io::Error) -> CreateFacetError {
        CreateFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFacetError {
    fn description(&self) -> &str {
        match *self {
            CreateFacetError::AccessDenied(ref cause) => cause,
            CreateFacetError::FacetAlreadyExists(ref cause) => cause,
            CreateFacetError::FacetValidation(ref cause) => cause,
            CreateFacetError::InternalService(ref cause) => cause,
            CreateFacetError::InvalidArn(ref cause) => cause,
            CreateFacetError::InvalidRule(ref cause) => cause,
            CreateFacetError::LimitExceeded(ref cause) => cause,
            CreateFacetError::ResourceNotFound(ref cause) => cause,
            CreateFacetError::RetryableConflict(ref cause) => cause,
            CreateFacetError::Validation(ref cause) => cause,
            CreateFacetError::Credentials(ref err) => err.description(),
            CreateFacetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateFacetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateIndex
#[derive(Debug, PartialEq)]
pub enum CreateIndexError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    ///<p>Indicates that the requested index type is not supported.</p>
    UnsupportedIndexType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateIndexError {
    pub fn from_body(body: &str) -> CreateIndexError {
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
                        CreateIndexError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        CreateIndexError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "FacetValidationException" => {
                        CreateIndexError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreateIndexError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        CreateIndexError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateIndexError::LimitExceeded(String::from(error_message))
                    }
                    "LinkNameAlreadyInUseException" => {
                        CreateIndexError::LinkNameAlreadyInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateIndexError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        CreateIndexError::RetryableConflict(String::from(error_message))
                    }
                    "UnsupportedIndexTypeException" => {
                        CreateIndexError::UnsupportedIndexType(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateIndexError::Validation(error_message.to_string())
                    }
                    _ => CreateIndexError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateIndexError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateIndexError {
    fn from(err: serde_json::error::Error) -> CreateIndexError {
        CreateIndexError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateIndexError {
    fn from(err: CredentialsError) -> CreateIndexError {
        CreateIndexError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateIndexError {
    fn from(err: HttpDispatchError) -> CreateIndexError {
        CreateIndexError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateIndexError {
    fn from(err: io::Error) -> CreateIndexError {
        CreateIndexError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateIndexError {
    fn description(&self) -> &str {
        match *self {
            CreateIndexError::AccessDenied(ref cause) => cause,
            CreateIndexError::DirectoryNotEnabled(ref cause) => cause,
            CreateIndexError::FacetValidation(ref cause) => cause,
            CreateIndexError::InternalService(ref cause) => cause,
            CreateIndexError::InvalidArn(ref cause) => cause,
            CreateIndexError::LimitExceeded(ref cause) => cause,
            CreateIndexError::LinkNameAlreadyInUse(ref cause) => cause,
            CreateIndexError::ResourceNotFound(ref cause) => cause,
            CreateIndexError::RetryableConflict(ref cause) => cause,
            CreateIndexError::UnsupportedIndexType(ref cause) => cause,
            CreateIndexError::Validation(ref cause) => cause,
            CreateIndexError::Credentials(ref err) => err.description(),
            CreateIndexError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateIndexError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateObject
#[derive(Debug, PartialEq)]
pub enum CreateObjectError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again.</p>
    LinkNameAlreadyInUse(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    ///<p>Indicates that the requested index type is not supported.</p>
    UnsupportedIndexType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateObjectError {
    pub fn from_body(body: &str) -> CreateObjectError {
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
                        CreateObjectError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        CreateObjectError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "FacetValidationException" => {
                        CreateObjectError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreateObjectError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        CreateObjectError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateObjectError::LimitExceeded(String::from(error_message))
                    }
                    "LinkNameAlreadyInUseException" => {
                        CreateObjectError::LinkNameAlreadyInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateObjectError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        CreateObjectError::RetryableConflict(String::from(error_message))
                    }
                    "UnsupportedIndexTypeException" => {
                        CreateObjectError::UnsupportedIndexType(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateObjectError::Validation(error_message.to_string())
                    }
                    _ => CreateObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateObjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateObjectError {
    fn from(err: serde_json::error::Error) -> CreateObjectError {
        CreateObjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateObjectError {
    fn from(err: CredentialsError) -> CreateObjectError {
        CreateObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateObjectError {
    fn from(err: HttpDispatchError) -> CreateObjectError {
        CreateObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateObjectError {
    fn from(err: io::Error) -> CreateObjectError {
        CreateObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateObjectError {
    fn description(&self) -> &str {
        match *self {
            CreateObjectError::AccessDenied(ref cause) => cause,
            CreateObjectError::DirectoryNotEnabled(ref cause) => cause,
            CreateObjectError::FacetValidation(ref cause) => cause,
            CreateObjectError::InternalService(ref cause) => cause,
            CreateObjectError::InvalidArn(ref cause) => cause,
            CreateObjectError::LimitExceeded(ref cause) => cause,
            CreateObjectError::LinkNameAlreadyInUse(ref cause) => cause,
            CreateObjectError::ResourceNotFound(ref cause) => cause,
            CreateObjectError::RetryableConflict(ref cause) => cause,
            CreateObjectError::UnsupportedIndexType(ref cause) => cause,
            CreateObjectError::Validation(ref cause) => cause,
            CreateObjectError::Credentials(ref err) => err.description(),
            CreateObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSchema
#[derive(Debug, PartialEq)]
pub enum CreateSchemaError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    ///<p>Indicates that a schema could not be created due to a naming conflict. Please select a different name and then try again.</p>
    SchemaAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateSchemaError {
    pub fn from_body(body: &str) -> CreateSchemaError {
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
                        CreateSchemaError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreateSchemaError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        CreateSchemaError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateSchemaError::LimitExceeded(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        CreateSchemaError::RetryableConflict(String::from(error_message))
                    }
                    "SchemaAlreadyExistsException" => {
                        CreateSchemaError::SchemaAlreadyExists(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateSchemaError::Validation(error_message.to_string())
                    }
                    _ => CreateSchemaError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSchemaError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSchemaError {
    fn from(err: serde_json::error::Error) -> CreateSchemaError {
        CreateSchemaError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSchemaError {
    fn from(err: CredentialsError) -> CreateSchemaError {
        CreateSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSchemaError {
    fn from(err: HttpDispatchError) -> CreateSchemaError {
        CreateSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSchemaError {
    fn from(err: io::Error) -> CreateSchemaError {
        CreateSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSchemaError {
    fn description(&self) -> &str {
        match *self {
            CreateSchemaError::AccessDenied(ref cause) => cause,
            CreateSchemaError::InternalService(ref cause) => cause,
            CreateSchemaError::InvalidArn(ref cause) => cause,
            CreateSchemaError::LimitExceeded(ref cause) => cause,
            CreateSchemaError::RetryableConflict(ref cause) => cause,
            CreateSchemaError::SchemaAlreadyExists(ref cause) => cause,
            CreateSchemaError::Validation(ref cause) => cause,
            CreateSchemaError::Credentials(ref err) => err.description(),
            CreateSchemaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSchemaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTypedLinkFacet
#[derive(Debug, PartialEq)]
pub enum CreateTypedLinkFacetError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>A facet with the same name already exists.</p>
    FacetAlreadyExists(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Occurs when any of the rule parameter keys or values are invalid.</p>
    InvalidRule(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateTypedLinkFacetError {
    pub fn from_body(body: &str) -> CreateTypedLinkFacetError {
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
                        CreateTypedLinkFacetError::AccessDenied(String::from(error_message))
                    }
                    "FacetAlreadyExistsException" => {
                        CreateTypedLinkFacetError::FacetAlreadyExists(String::from(error_message))
                    }
                    "FacetValidationException" => {
                        CreateTypedLinkFacetError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        CreateTypedLinkFacetError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        CreateTypedLinkFacetError::InvalidArn(String::from(error_message))
                    }
                    "InvalidRuleException" => {
                        CreateTypedLinkFacetError::InvalidRule(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateTypedLinkFacetError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateTypedLinkFacetError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        CreateTypedLinkFacetError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateTypedLinkFacetError::Validation(error_message.to_string())
                    }
                    _ => CreateTypedLinkFacetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTypedLinkFacetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTypedLinkFacetError {
    fn from(err: serde_json::error::Error) -> CreateTypedLinkFacetError {
        CreateTypedLinkFacetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTypedLinkFacetError {
    fn from(err: CredentialsError) -> CreateTypedLinkFacetError {
        CreateTypedLinkFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTypedLinkFacetError {
    fn from(err: HttpDispatchError) -> CreateTypedLinkFacetError {
        CreateTypedLinkFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTypedLinkFacetError {
    fn from(err: io::Error) -> CreateTypedLinkFacetError {
        CreateTypedLinkFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTypedLinkFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTypedLinkFacetError {
    fn description(&self) -> &str {
        match *self {
            CreateTypedLinkFacetError::AccessDenied(ref cause) => cause,
            CreateTypedLinkFacetError::FacetAlreadyExists(ref cause) => cause,
            CreateTypedLinkFacetError::FacetValidation(ref cause) => cause,
            CreateTypedLinkFacetError::InternalService(ref cause) => cause,
            CreateTypedLinkFacetError::InvalidArn(ref cause) => cause,
            CreateTypedLinkFacetError::InvalidRule(ref cause) => cause,
            CreateTypedLinkFacetError::LimitExceeded(ref cause) => cause,
            CreateTypedLinkFacetError::ResourceNotFound(ref cause) => cause,
            CreateTypedLinkFacetError::RetryableConflict(ref cause) => cause,
            CreateTypedLinkFacetError::Validation(ref cause) => cause,
            CreateTypedLinkFacetError::Credentials(ref err) => err.description(),
            CreateTypedLinkFacetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateTypedLinkFacetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDirectory
#[derive(Debug, PartialEq)]
pub enum DeleteDirectoryError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>A directory that has been deleted and to which access has been attempted. Note: The requested resource will eventually cease to exist.</p>
    DirectoryDeleted(String),
    ///<p>An operation can only operate on a disabled directory.</p>
    DirectoryNotDisabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteDirectoryError {
    pub fn from_body(body: &str) -> DeleteDirectoryError {
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
                        DeleteDirectoryError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryDeletedException" => {
                        DeleteDirectoryError::DirectoryDeleted(String::from(error_message))
                    }
                    "DirectoryNotDisabledException" => {
                        DeleteDirectoryError::DirectoryNotDisabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeleteDirectoryError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DeleteDirectoryError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteDirectoryError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteDirectoryError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        DeleteDirectoryError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDirectoryError::Validation(error_message.to_string())
                    }
                    _ => DeleteDirectoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDirectoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDirectoryError {
    fn from(err: serde_json::error::Error) -> DeleteDirectoryError {
        DeleteDirectoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDirectoryError {
    fn from(err: CredentialsError) -> DeleteDirectoryError {
        DeleteDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDirectoryError {
    fn from(err: HttpDispatchError) -> DeleteDirectoryError {
        DeleteDirectoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDirectoryError {
    fn from(err: io::Error) -> DeleteDirectoryError {
        DeleteDirectoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDirectoryError {
    fn description(&self) -> &str {
        match *self {
            DeleteDirectoryError::AccessDenied(ref cause) => cause,
            DeleteDirectoryError::DirectoryDeleted(ref cause) => cause,
            DeleteDirectoryError::DirectoryNotDisabled(ref cause) => cause,
            DeleteDirectoryError::InternalService(ref cause) => cause,
            DeleteDirectoryError::InvalidArn(ref cause) => cause,
            DeleteDirectoryError::LimitExceeded(ref cause) => cause,
            DeleteDirectoryError::ResourceNotFound(ref cause) => cause,
            DeleteDirectoryError::RetryableConflict(ref cause) => cause,
            DeleteDirectoryError::Validation(ref cause) => cause,
            DeleteDirectoryError::Credentials(ref err) => err.description(),
            DeleteDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDirectoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFacet
#[derive(Debug, PartialEq)]
pub enum DeleteFacetError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Occurs when deleting a facet that contains an attribute that is a target to an attribute reference in a different facet.</p>
    FacetInUse(String),
    ///<p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteFacetError {
    pub fn from_body(body: &str) -> DeleteFacetError {
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
                        DeleteFacetError::AccessDenied(String::from(error_message))
                    }
                    "FacetInUseException" => {
                        DeleteFacetError::FacetInUse(String::from(error_message))
                    }
                    "FacetNotFoundException" => {
                        DeleteFacetError::FacetNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeleteFacetError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DeleteFacetError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteFacetError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteFacetError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        DeleteFacetError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteFacetError::Validation(error_message.to_string())
                    }
                    _ => DeleteFacetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteFacetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteFacetError {
    fn from(err: serde_json::error::Error) -> DeleteFacetError {
        DeleteFacetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFacetError {
    fn from(err: CredentialsError) -> DeleteFacetError {
        DeleteFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFacetError {
    fn from(err: HttpDispatchError) -> DeleteFacetError {
        DeleteFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFacetError {
    fn from(err: io::Error) -> DeleteFacetError {
        DeleteFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFacetError {
    fn description(&self) -> &str {
        match *self {
            DeleteFacetError::AccessDenied(ref cause) => cause,
            DeleteFacetError::FacetInUse(ref cause) => cause,
            DeleteFacetError::FacetNotFound(ref cause) => cause,
            DeleteFacetError::InternalService(ref cause) => cause,
            DeleteFacetError::InvalidArn(ref cause) => cause,
            DeleteFacetError::LimitExceeded(ref cause) => cause,
            DeleteFacetError::ResourceNotFound(ref cause) => cause,
            DeleteFacetError::RetryableConflict(ref cause) => cause,
            DeleteFacetError::Validation(ref cause) => cause,
            DeleteFacetError::Credentials(ref err) => err.description(),
            DeleteFacetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteFacetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteObject
#[derive(Debug, PartialEq)]
pub enum DeleteObjectError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Indicates that the requested operation cannot be completed because the object has not been detached from the tree.</p>
    ObjectNotDetached(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteObjectError {
    pub fn from_body(body: &str) -> DeleteObjectError {
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
                        DeleteObjectError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        DeleteObjectError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeleteObjectError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DeleteObjectError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteObjectError::LimitExceeded(String::from(error_message))
                    }
                    "ObjectNotDetachedException" => {
                        DeleteObjectError::ObjectNotDetached(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteObjectError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        DeleteObjectError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteObjectError::Validation(error_message.to_string())
                    }
                    _ => DeleteObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteObjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteObjectError {
    fn from(err: serde_json::error::Error) -> DeleteObjectError {
        DeleteObjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteObjectError {
    fn from(err: CredentialsError) -> DeleteObjectError {
        DeleteObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteObjectError {
    fn from(err: HttpDispatchError) -> DeleteObjectError {
        DeleteObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteObjectError {
    fn from(err: io::Error) -> DeleteObjectError {
        DeleteObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectError {
    fn description(&self) -> &str {
        match *self {
            DeleteObjectError::AccessDenied(ref cause) => cause,
            DeleteObjectError::DirectoryNotEnabled(ref cause) => cause,
            DeleteObjectError::InternalService(ref cause) => cause,
            DeleteObjectError::InvalidArn(ref cause) => cause,
            DeleteObjectError::LimitExceeded(ref cause) => cause,
            DeleteObjectError::ObjectNotDetached(ref cause) => cause,
            DeleteObjectError::ResourceNotFound(ref cause) => cause,
            DeleteObjectError::RetryableConflict(ref cause) => cause,
            DeleteObjectError::Validation(ref cause) => cause,
            DeleteObjectError::Credentials(ref err) => err.description(),
            DeleteObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSchema
#[derive(Debug, PartialEq)]
pub enum DeleteSchemaError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    ///<p>The object could not be deleted because links still exist. Remove the links and then try the operation again.</p>
    StillContainsLinks(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteSchemaError {
    pub fn from_body(body: &str) -> DeleteSchemaError {
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
                        DeleteSchemaError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeleteSchemaError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DeleteSchemaError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteSchemaError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteSchemaError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        DeleteSchemaError::RetryableConflict(String::from(error_message))
                    }
                    "StillContainsLinksException" => {
                        DeleteSchemaError::StillContainsLinks(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteSchemaError::Validation(error_message.to_string())
                    }
                    _ => DeleteSchemaError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSchemaError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSchemaError {
    fn from(err: serde_json::error::Error) -> DeleteSchemaError {
        DeleteSchemaError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSchemaError {
    fn from(err: CredentialsError) -> DeleteSchemaError {
        DeleteSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSchemaError {
    fn from(err: HttpDispatchError) -> DeleteSchemaError {
        DeleteSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSchemaError {
    fn from(err: io::Error) -> DeleteSchemaError {
        DeleteSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSchemaError {
    fn description(&self) -> &str {
        match *self {
            DeleteSchemaError::AccessDenied(ref cause) => cause,
            DeleteSchemaError::InternalService(ref cause) => cause,
            DeleteSchemaError::InvalidArn(ref cause) => cause,
            DeleteSchemaError::LimitExceeded(ref cause) => cause,
            DeleteSchemaError::ResourceNotFound(ref cause) => cause,
            DeleteSchemaError::RetryableConflict(ref cause) => cause,
            DeleteSchemaError::StillContainsLinks(ref cause) => cause,
            DeleteSchemaError::Validation(ref cause) => cause,
            DeleteSchemaError::Credentials(ref err) => err.description(),
            DeleteSchemaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSchemaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTypedLinkFacet
#[derive(Debug, PartialEq)]
pub enum DeleteTypedLinkFacetError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteTypedLinkFacetError {
    pub fn from_body(body: &str) -> DeleteTypedLinkFacetError {
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
                        DeleteTypedLinkFacetError::AccessDenied(String::from(error_message))
                    }
                    "FacetNotFoundException" => {
                        DeleteTypedLinkFacetError::FacetNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeleteTypedLinkFacetError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DeleteTypedLinkFacetError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteTypedLinkFacetError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteTypedLinkFacetError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        DeleteTypedLinkFacetError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteTypedLinkFacetError::Validation(error_message.to_string())
                    }
                    _ => DeleteTypedLinkFacetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTypedLinkFacetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTypedLinkFacetError {
    fn from(err: serde_json::error::Error) -> DeleteTypedLinkFacetError {
        DeleteTypedLinkFacetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTypedLinkFacetError {
    fn from(err: CredentialsError) -> DeleteTypedLinkFacetError {
        DeleteTypedLinkFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTypedLinkFacetError {
    fn from(err: HttpDispatchError) -> DeleteTypedLinkFacetError {
        DeleteTypedLinkFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTypedLinkFacetError {
    fn from(err: io::Error) -> DeleteTypedLinkFacetError {
        DeleteTypedLinkFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTypedLinkFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTypedLinkFacetError {
    fn description(&self) -> &str {
        match *self {
            DeleteTypedLinkFacetError::AccessDenied(ref cause) => cause,
            DeleteTypedLinkFacetError::FacetNotFound(ref cause) => cause,
            DeleteTypedLinkFacetError::InternalService(ref cause) => cause,
            DeleteTypedLinkFacetError::InvalidArn(ref cause) => cause,
            DeleteTypedLinkFacetError::LimitExceeded(ref cause) => cause,
            DeleteTypedLinkFacetError::ResourceNotFound(ref cause) => cause,
            DeleteTypedLinkFacetError::RetryableConflict(ref cause) => cause,
            DeleteTypedLinkFacetError::Validation(ref cause) => cause,
            DeleteTypedLinkFacetError::Credentials(ref err) => err.description(),
            DeleteTypedLinkFacetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteTypedLinkFacetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachFromIndex
#[derive(Debug, PartialEq)]
pub enum DetachFromIndexError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Indicates that the requested operation can only operate on index objects.</p>
    NotIndex(String),
    ///<p>Indicates that the object is not attached to the index.</p>
    ObjectAlreadyDetached(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DetachFromIndexError {
    pub fn from_body(body: &str) -> DetachFromIndexError {
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
                        DetachFromIndexError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        DetachFromIndexError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DetachFromIndexError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DetachFromIndexError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DetachFromIndexError::LimitExceeded(String::from(error_message))
                    }
                    "NotIndexException" => {
                        DetachFromIndexError::NotIndex(String::from(error_message))
                    }
                    "ObjectAlreadyDetachedException" => {
                        DetachFromIndexError::ObjectAlreadyDetached(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DetachFromIndexError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        DetachFromIndexError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetachFromIndexError::Validation(error_message.to_string())
                    }
                    _ => DetachFromIndexError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachFromIndexError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachFromIndexError {
    fn from(err: serde_json::error::Error) -> DetachFromIndexError {
        DetachFromIndexError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachFromIndexError {
    fn from(err: CredentialsError) -> DetachFromIndexError {
        DetachFromIndexError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachFromIndexError {
    fn from(err: HttpDispatchError) -> DetachFromIndexError {
        DetachFromIndexError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachFromIndexError {
    fn from(err: io::Error) -> DetachFromIndexError {
        DetachFromIndexError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachFromIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachFromIndexError {
    fn description(&self) -> &str {
        match *self {
            DetachFromIndexError::AccessDenied(ref cause) => cause,
            DetachFromIndexError::DirectoryNotEnabled(ref cause) => cause,
            DetachFromIndexError::InternalService(ref cause) => cause,
            DetachFromIndexError::InvalidArn(ref cause) => cause,
            DetachFromIndexError::LimitExceeded(ref cause) => cause,
            DetachFromIndexError::NotIndex(ref cause) => cause,
            DetachFromIndexError::ObjectAlreadyDetached(ref cause) => cause,
            DetachFromIndexError::ResourceNotFound(ref cause) => cause,
            DetachFromIndexError::RetryableConflict(ref cause) => cause,
            DetachFromIndexError::Validation(ref cause) => cause,
            DetachFromIndexError::Credentials(ref err) => err.description(),
            DetachFromIndexError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachFromIndexError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachObject
#[derive(Debug, PartialEq)]
pub enum DetachObjectError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DetachObjectError {
    pub fn from_body(body: &str) -> DetachObjectError {
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
                        DetachObjectError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        DetachObjectError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DetachObjectError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DetachObjectError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DetachObjectError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DetachObjectError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        DetachObjectError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetachObjectError::Validation(error_message.to_string())
                    }
                    _ => DetachObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachObjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachObjectError {
    fn from(err: serde_json::error::Error) -> DetachObjectError {
        DetachObjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachObjectError {
    fn from(err: CredentialsError) -> DetachObjectError {
        DetachObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachObjectError {
    fn from(err: HttpDispatchError) -> DetachObjectError {
        DetachObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachObjectError {
    fn from(err: io::Error) -> DetachObjectError {
        DetachObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachObjectError {
    fn description(&self) -> &str {
        match *self {
            DetachObjectError::AccessDenied(ref cause) => cause,
            DetachObjectError::DirectoryNotEnabled(ref cause) => cause,
            DetachObjectError::InternalService(ref cause) => cause,
            DetachObjectError::InvalidArn(ref cause) => cause,
            DetachObjectError::LimitExceeded(ref cause) => cause,
            DetachObjectError::ResourceNotFound(ref cause) => cause,
            DetachObjectError::RetryableConflict(ref cause) => cause,
            DetachObjectError::Validation(ref cause) => cause,
            DetachObjectError::Credentials(ref err) => err.description(),
            DetachObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachPolicy
#[derive(Debug, PartialEq)]
pub enum DetachPolicyError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Indicates that the requested operation can only operate on policy objects.</p>
    NotPolicy(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DetachPolicyError {
    pub fn from_body(body: &str) -> DetachPolicyError {
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
                        DetachPolicyError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        DetachPolicyError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DetachPolicyError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DetachPolicyError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DetachPolicyError::LimitExceeded(String::from(error_message))
                    }
                    "NotPolicyException" => {
                        DetachPolicyError::NotPolicy(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DetachPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        DetachPolicyError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetachPolicyError::Validation(error_message.to_string())
                    }
                    _ => DetachPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachPolicyError {
    fn from(err: serde_json::error::Error) -> DetachPolicyError {
        DetachPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachPolicyError {
    fn from(err: CredentialsError) -> DetachPolicyError {
        DetachPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachPolicyError {
    fn from(err: HttpDispatchError) -> DetachPolicyError {
        DetachPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachPolicyError {
    fn from(err: io::Error) -> DetachPolicyError {
        DetachPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachPolicyError {
    fn description(&self) -> &str {
        match *self {
            DetachPolicyError::AccessDenied(ref cause) => cause,
            DetachPolicyError::DirectoryNotEnabled(ref cause) => cause,
            DetachPolicyError::InternalService(ref cause) => cause,
            DetachPolicyError::InvalidArn(ref cause) => cause,
            DetachPolicyError::LimitExceeded(ref cause) => cause,
            DetachPolicyError::NotPolicy(ref cause) => cause,
            DetachPolicyError::ResourceNotFound(ref cause) => cause,
            DetachPolicyError::RetryableConflict(ref cause) => cause,
            DetachPolicyError::Validation(ref cause) => cause,
            DetachPolicyError::Credentials(ref err) => err.description(),
            DetachPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachTypedLink
#[derive(Debug, PartialEq)]
pub enum DetachTypedLinkError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DetachTypedLinkError {
    pub fn from_body(body: &str) -> DetachTypedLinkError {
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
                        DetachTypedLinkError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        DetachTypedLinkError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "FacetValidationException" => {
                        DetachTypedLinkError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DetachTypedLinkError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DetachTypedLinkError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DetachTypedLinkError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DetachTypedLinkError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        DetachTypedLinkError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetachTypedLinkError::Validation(error_message.to_string())
                    }
                    _ => DetachTypedLinkError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachTypedLinkError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachTypedLinkError {
    fn from(err: serde_json::error::Error) -> DetachTypedLinkError {
        DetachTypedLinkError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachTypedLinkError {
    fn from(err: CredentialsError) -> DetachTypedLinkError {
        DetachTypedLinkError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachTypedLinkError {
    fn from(err: HttpDispatchError) -> DetachTypedLinkError {
        DetachTypedLinkError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachTypedLinkError {
    fn from(err: io::Error) -> DetachTypedLinkError {
        DetachTypedLinkError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachTypedLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachTypedLinkError {
    fn description(&self) -> &str {
        match *self {
            DetachTypedLinkError::AccessDenied(ref cause) => cause,
            DetachTypedLinkError::DirectoryNotEnabled(ref cause) => cause,
            DetachTypedLinkError::FacetValidation(ref cause) => cause,
            DetachTypedLinkError::InternalService(ref cause) => cause,
            DetachTypedLinkError::InvalidArn(ref cause) => cause,
            DetachTypedLinkError::LimitExceeded(ref cause) => cause,
            DetachTypedLinkError::ResourceNotFound(ref cause) => cause,
            DetachTypedLinkError::RetryableConflict(ref cause) => cause,
            DetachTypedLinkError::Validation(ref cause) => cause,
            DetachTypedLinkError::Credentials(ref err) => err.description(),
            DetachTypedLinkError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachTypedLinkError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableDirectory
#[derive(Debug, PartialEq)]
pub enum DisableDirectoryError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>A directory that has been deleted and to which access has been attempted. Note: The requested resource will eventually cease to exist.</p>
    DirectoryDeleted(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisableDirectoryError {
    pub fn from_body(body: &str) -> DisableDirectoryError {
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
                        DisableDirectoryError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryDeletedException" => {
                        DisableDirectoryError::DirectoryDeleted(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DisableDirectoryError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DisableDirectoryError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DisableDirectoryError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DisableDirectoryError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        DisableDirectoryError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisableDirectoryError::Validation(error_message.to_string())
                    }
                    _ => DisableDirectoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableDirectoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableDirectoryError {
    fn from(err: serde_json::error::Error) -> DisableDirectoryError {
        DisableDirectoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableDirectoryError {
    fn from(err: CredentialsError) -> DisableDirectoryError {
        DisableDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableDirectoryError {
    fn from(err: HttpDispatchError) -> DisableDirectoryError {
        DisableDirectoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableDirectoryError {
    fn from(err: io::Error) -> DisableDirectoryError {
        DisableDirectoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableDirectoryError {
    fn description(&self) -> &str {
        match *self {
            DisableDirectoryError::AccessDenied(ref cause) => cause,
            DisableDirectoryError::DirectoryDeleted(ref cause) => cause,
            DisableDirectoryError::InternalService(ref cause) => cause,
            DisableDirectoryError::InvalidArn(ref cause) => cause,
            DisableDirectoryError::LimitExceeded(ref cause) => cause,
            DisableDirectoryError::ResourceNotFound(ref cause) => cause,
            DisableDirectoryError::RetryableConflict(ref cause) => cause,
            DisableDirectoryError::Validation(ref cause) => cause,
            DisableDirectoryError::Credentials(ref err) => err.description(),
            DisableDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisableDirectoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableDirectory
#[derive(Debug, PartialEq)]
pub enum EnableDirectoryError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>A directory that has been deleted and to which access has been attempted. Note: The requested resource will eventually cease to exist.</p>
    DirectoryDeleted(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl EnableDirectoryError {
    pub fn from_body(body: &str) -> EnableDirectoryError {
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
                        EnableDirectoryError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryDeletedException" => {
                        EnableDirectoryError::DirectoryDeleted(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        EnableDirectoryError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        EnableDirectoryError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        EnableDirectoryError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        EnableDirectoryError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        EnableDirectoryError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        EnableDirectoryError::Validation(error_message.to_string())
                    }
                    _ => EnableDirectoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableDirectoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableDirectoryError {
    fn from(err: serde_json::error::Error) -> EnableDirectoryError {
        EnableDirectoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableDirectoryError {
    fn from(err: CredentialsError) -> EnableDirectoryError {
        EnableDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableDirectoryError {
    fn from(err: HttpDispatchError) -> EnableDirectoryError {
        EnableDirectoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableDirectoryError {
    fn from(err: io::Error) -> EnableDirectoryError {
        EnableDirectoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableDirectoryError {
    fn description(&self) -> &str {
        match *self {
            EnableDirectoryError::AccessDenied(ref cause) => cause,
            EnableDirectoryError::DirectoryDeleted(ref cause) => cause,
            EnableDirectoryError::InternalService(ref cause) => cause,
            EnableDirectoryError::InvalidArn(ref cause) => cause,
            EnableDirectoryError::LimitExceeded(ref cause) => cause,
            EnableDirectoryError::ResourceNotFound(ref cause) => cause,
            EnableDirectoryError::RetryableConflict(ref cause) => cause,
            EnableDirectoryError::Validation(ref cause) => cause,
            EnableDirectoryError::Credentials(ref err) => err.description(),
            EnableDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnableDirectoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDirectory
#[derive(Debug, PartialEq)]
pub enum GetDirectoryError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetDirectoryError {
    pub fn from_body(body: &str) -> GetDirectoryError {
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
                        GetDirectoryError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetDirectoryError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        GetDirectoryError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetDirectoryError::LimitExceeded(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        GetDirectoryError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDirectoryError::Validation(error_message.to_string())
                    }
                    _ => GetDirectoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDirectoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDirectoryError {
    fn from(err: serde_json::error::Error) -> GetDirectoryError {
        GetDirectoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDirectoryError {
    fn from(err: CredentialsError) -> GetDirectoryError {
        GetDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDirectoryError {
    fn from(err: HttpDispatchError) -> GetDirectoryError {
        GetDirectoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDirectoryError {
    fn from(err: io::Error) -> GetDirectoryError {
        GetDirectoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDirectoryError {
    fn description(&self) -> &str {
        match *self {
            GetDirectoryError::AccessDenied(ref cause) => cause,
            GetDirectoryError::InternalService(ref cause) => cause,
            GetDirectoryError::InvalidArn(ref cause) => cause,
            GetDirectoryError::LimitExceeded(ref cause) => cause,
            GetDirectoryError::RetryableConflict(ref cause) => cause,
            GetDirectoryError::Validation(ref cause) => cause,
            GetDirectoryError::Credentials(ref err) => err.description(),
            GetDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDirectoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFacet
#[derive(Debug, PartialEq)]
pub enum GetFacetError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetFacetError {
    pub fn from_body(body: &str) -> GetFacetError {
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
                        GetFacetError::AccessDenied(String::from(error_message))
                    }
                    "FacetNotFoundException" => {
                        GetFacetError::FacetNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetFacetError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => GetFacetError::InvalidArn(String::from(error_message)),
                    "LimitExceededException" => {
                        GetFacetError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetFacetError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        GetFacetError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => GetFacetError::Validation(error_message.to_string()),
                    _ => GetFacetError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetFacetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetFacetError {
    fn from(err: serde_json::error::Error) -> GetFacetError {
        GetFacetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFacetError {
    fn from(err: CredentialsError) -> GetFacetError {
        GetFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFacetError {
    fn from(err: HttpDispatchError) -> GetFacetError {
        GetFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFacetError {
    fn from(err: io::Error) -> GetFacetError {
        GetFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFacetError {
    fn description(&self) -> &str {
        match *self {
            GetFacetError::AccessDenied(ref cause) => cause,
            GetFacetError::FacetNotFound(ref cause) => cause,
            GetFacetError::InternalService(ref cause) => cause,
            GetFacetError::InvalidArn(ref cause) => cause,
            GetFacetError::LimitExceeded(ref cause) => cause,
            GetFacetError::ResourceNotFound(ref cause) => cause,
            GetFacetError::RetryableConflict(ref cause) => cause,
            GetFacetError::Validation(ref cause) => cause,
            GetFacetError::Credentials(ref err) => err.description(),
            GetFacetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetFacetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetObjectInformation
#[derive(Debug, PartialEq)]
pub enum GetObjectInformationError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetObjectInformationError {
    pub fn from_body(body: &str) -> GetObjectInformationError {
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
                        GetObjectInformationError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        GetObjectInformationError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetObjectInformationError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        GetObjectInformationError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetObjectInformationError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetObjectInformationError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        GetObjectInformationError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetObjectInformationError::Validation(error_message.to_string())
                    }
                    _ => GetObjectInformationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetObjectInformationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetObjectInformationError {
    fn from(err: serde_json::error::Error) -> GetObjectInformationError {
        GetObjectInformationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetObjectInformationError {
    fn from(err: CredentialsError) -> GetObjectInformationError {
        GetObjectInformationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectInformationError {
    fn from(err: HttpDispatchError) -> GetObjectInformationError {
        GetObjectInformationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectInformationError {
    fn from(err: io::Error) -> GetObjectInformationError {
        GetObjectInformationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectInformationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectInformationError {
    fn description(&self) -> &str {
        match *self {
            GetObjectInformationError::AccessDenied(ref cause) => cause,
            GetObjectInformationError::DirectoryNotEnabled(ref cause) => cause,
            GetObjectInformationError::InternalService(ref cause) => cause,
            GetObjectInformationError::InvalidArn(ref cause) => cause,
            GetObjectInformationError::LimitExceeded(ref cause) => cause,
            GetObjectInformationError::ResourceNotFound(ref cause) => cause,
            GetObjectInformationError::RetryableConflict(ref cause) => cause,
            GetObjectInformationError::Validation(ref cause) => cause,
            GetObjectInformationError::Credentials(ref err) => err.description(),
            GetObjectInformationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetObjectInformationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSchemaAsJson
#[derive(Debug, PartialEq)]
pub enum GetSchemaAsJsonError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetSchemaAsJsonError {
    pub fn from_body(body: &str) -> GetSchemaAsJsonError {
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
                        GetSchemaAsJsonError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        GetSchemaAsJsonError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        GetSchemaAsJsonError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetSchemaAsJsonError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetSchemaAsJsonError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        GetSchemaAsJsonError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSchemaAsJsonError::Validation(error_message.to_string())
                    }
                    _ => GetSchemaAsJsonError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSchemaAsJsonError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSchemaAsJsonError {
    fn from(err: serde_json::error::Error) -> GetSchemaAsJsonError {
        GetSchemaAsJsonError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSchemaAsJsonError {
    fn from(err: CredentialsError) -> GetSchemaAsJsonError {
        GetSchemaAsJsonError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSchemaAsJsonError {
    fn from(err: HttpDispatchError) -> GetSchemaAsJsonError {
        GetSchemaAsJsonError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSchemaAsJsonError {
    fn from(err: io::Error) -> GetSchemaAsJsonError {
        GetSchemaAsJsonError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSchemaAsJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSchemaAsJsonError {
    fn description(&self) -> &str {
        match *self {
            GetSchemaAsJsonError::AccessDenied(ref cause) => cause,
            GetSchemaAsJsonError::InternalService(ref cause) => cause,
            GetSchemaAsJsonError::InvalidArn(ref cause) => cause,
            GetSchemaAsJsonError::LimitExceeded(ref cause) => cause,
            GetSchemaAsJsonError::ResourceNotFound(ref cause) => cause,
            GetSchemaAsJsonError::RetryableConflict(ref cause) => cause,
            GetSchemaAsJsonError::Validation(ref cause) => cause,
            GetSchemaAsJsonError::Credentials(ref err) => err.description(),
            GetSchemaAsJsonError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSchemaAsJsonError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTypedLinkFacetInformation
#[derive(Debug, PartialEq)]
pub enum GetTypedLinkFacetInformationError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetTypedLinkFacetInformationError {
    pub fn from_body(body: &str) -> GetTypedLinkFacetInformationError {
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
                        GetTypedLinkFacetInformationError::AccessDenied(String::from(error_message))
                    }
                    "FacetNotFoundException" => GetTypedLinkFacetInformationError::FacetNotFound(String::from(error_message)),
                    "InternalServiceException" => GetTypedLinkFacetInformationError::InternalService(String::from(error_message)),
                    "InvalidArnException" => {
                        GetTypedLinkFacetInformationError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => GetTypedLinkFacetInformationError::InvalidNextToken(String::from(error_message)),
                    "LimitExceededException" => GetTypedLinkFacetInformationError::LimitExceeded(String::from(error_message)),
                    "ResourceNotFoundException" => GetTypedLinkFacetInformationError::ResourceNotFound(String::from(error_message)),
                    "RetryableConflictException" => GetTypedLinkFacetInformationError::RetryableConflict(String::from(error_message)),
                    "ValidationException" => {
                        GetTypedLinkFacetInformationError::Validation(error_message.to_string())
                    }
                    _ => GetTypedLinkFacetInformationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTypedLinkFacetInformationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTypedLinkFacetInformationError {
    fn from(err: serde_json::error::Error) -> GetTypedLinkFacetInformationError {
        GetTypedLinkFacetInformationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTypedLinkFacetInformationError {
    fn from(err: CredentialsError) -> GetTypedLinkFacetInformationError {
        GetTypedLinkFacetInformationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTypedLinkFacetInformationError {
    fn from(err: HttpDispatchError) -> GetTypedLinkFacetInformationError {
        GetTypedLinkFacetInformationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTypedLinkFacetInformationError {
    fn from(err: io::Error) -> GetTypedLinkFacetInformationError {
        GetTypedLinkFacetInformationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTypedLinkFacetInformationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTypedLinkFacetInformationError {
    fn description(&self) -> &str {
        match *self {
            GetTypedLinkFacetInformationError::AccessDenied(ref cause) => cause,
            GetTypedLinkFacetInformationError::FacetNotFound(ref cause) => cause,
            GetTypedLinkFacetInformationError::InternalService(ref cause) => cause,
            GetTypedLinkFacetInformationError::InvalidArn(ref cause) => cause,
            GetTypedLinkFacetInformationError::InvalidNextToken(ref cause) => cause,
            GetTypedLinkFacetInformationError::LimitExceeded(ref cause) => cause,
            GetTypedLinkFacetInformationError::ResourceNotFound(ref cause) => cause,
            GetTypedLinkFacetInformationError::RetryableConflict(ref cause) => cause,
            GetTypedLinkFacetInformationError::Validation(ref cause) => cause,
            GetTypedLinkFacetInformationError::Credentials(ref err) => err.description(),
            GetTypedLinkFacetInformationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetTypedLinkFacetInformationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAppliedSchemaArns
#[derive(Debug, PartialEq)]
pub enum ListAppliedSchemaArnsError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListAppliedSchemaArnsError {
    pub fn from_body(body: &str) -> ListAppliedSchemaArnsError {
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
                        ListAppliedSchemaArnsError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListAppliedSchemaArnsError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListAppliedSchemaArnsError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListAppliedSchemaArnsError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListAppliedSchemaArnsError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListAppliedSchemaArnsError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListAppliedSchemaArnsError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAppliedSchemaArnsError::Validation(error_message.to_string())
                    }
                    _ => ListAppliedSchemaArnsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAppliedSchemaArnsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAppliedSchemaArnsError {
    fn from(err: serde_json::error::Error) -> ListAppliedSchemaArnsError {
        ListAppliedSchemaArnsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAppliedSchemaArnsError {
    fn from(err: CredentialsError) -> ListAppliedSchemaArnsError {
        ListAppliedSchemaArnsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAppliedSchemaArnsError {
    fn from(err: HttpDispatchError) -> ListAppliedSchemaArnsError {
        ListAppliedSchemaArnsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAppliedSchemaArnsError {
    fn from(err: io::Error) -> ListAppliedSchemaArnsError {
        ListAppliedSchemaArnsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAppliedSchemaArnsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAppliedSchemaArnsError {
    fn description(&self) -> &str {
        match *self {
            ListAppliedSchemaArnsError::AccessDenied(ref cause) => cause,
            ListAppliedSchemaArnsError::InternalService(ref cause) => cause,
            ListAppliedSchemaArnsError::InvalidArn(ref cause) => cause,
            ListAppliedSchemaArnsError::InvalidNextToken(ref cause) => cause,
            ListAppliedSchemaArnsError::LimitExceeded(ref cause) => cause,
            ListAppliedSchemaArnsError::ResourceNotFound(ref cause) => cause,
            ListAppliedSchemaArnsError::RetryableConflict(ref cause) => cause,
            ListAppliedSchemaArnsError::Validation(ref cause) => cause,
            ListAppliedSchemaArnsError::Credentials(ref err) => err.description(),
            ListAppliedSchemaArnsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAppliedSchemaArnsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAttachedIndices
#[derive(Debug, PartialEq)]
pub enum ListAttachedIndicesError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListAttachedIndicesError {
    pub fn from_body(body: &str) -> ListAttachedIndicesError {
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
                        ListAttachedIndicesError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        ListAttachedIndicesError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListAttachedIndicesError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListAttachedIndicesError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListAttachedIndicesError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListAttachedIndicesError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListAttachedIndicesError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAttachedIndicesError::Validation(error_message.to_string())
                    }
                    _ => ListAttachedIndicesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAttachedIndicesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAttachedIndicesError {
    fn from(err: serde_json::error::Error) -> ListAttachedIndicesError {
        ListAttachedIndicesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAttachedIndicesError {
    fn from(err: CredentialsError) -> ListAttachedIndicesError {
        ListAttachedIndicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAttachedIndicesError {
    fn from(err: HttpDispatchError) -> ListAttachedIndicesError {
        ListAttachedIndicesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAttachedIndicesError {
    fn from(err: io::Error) -> ListAttachedIndicesError {
        ListAttachedIndicesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAttachedIndicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAttachedIndicesError {
    fn description(&self) -> &str {
        match *self {
            ListAttachedIndicesError::AccessDenied(ref cause) => cause,
            ListAttachedIndicesError::DirectoryNotEnabled(ref cause) => cause,
            ListAttachedIndicesError::InternalService(ref cause) => cause,
            ListAttachedIndicesError::InvalidArn(ref cause) => cause,
            ListAttachedIndicesError::LimitExceeded(ref cause) => cause,
            ListAttachedIndicesError::ResourceNotFound(ref cause) => cause,
            ListAttachedIndicesError::RetryableConflict(ref cause) => cause,
            ListAttachedIndicesError::Validation(ref cause) => cause,
            ListAttachedIndicesError::Credentials(ref err) => err.description(),
            ListAttachedIndicesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAttachedIndicesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDevelopmentSchemaArns
#[derive(Debug, PartialEq)]
pub enum ListDevelopmentSchemaArnsError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListDevelopmentSchemaArnsError {
    pub fn from_body(body: &str) -> ListDevelopmentSchemaArnsError {
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
                        ListDevelopmentSchemaArnsError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListDevelopmentSchemaArnsError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListDevelopmentSchemaArnsError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => ListDevelopmentSchemaArnsError::InvalidNextToken(String::from(error_message)),
                    "LimitExceededException" => {
                        ListDevelopmentSchemaArnsError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => ListDevelopmentSchemaArnsError::ResourceNotFound(String::from(error_message)),
                    "RetryableConflictException" => ListDevelopmentSchemaArnsError::RetryableConflict(String::from(error_message)),
                    "ValidationException" => {
                        ListDevelopmentSchemaArnsError::Validation(error_message.to_string())
                    }
                    _ => ListDevelopmentSchemaArnsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDevelopmentSchemaArnsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDevelopmentSchemaArnsError {
    fn from(err: serde_json::error::Error) -> ListDevelopmentSchemaArnsError {
        ListDevelopmentSchemaArnsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDevelopmentSchemaArnsError {
    fn from(err: CredentialsError) -> ListDevelopmentSchemaArnsError {
        ListDevelopmentSchemaArnsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDevelopmentSchemaArnsError {
    fn from(err: HttpDispatchError) -> ListDevelopmentSchemaArnsError {
        ListDevelopmentSchemaArnsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDevelopmentSchemaArnsError {
    fn from(err: io::Error) -> ListDevelopmentSchemaArnsError {
        ListDevelopmentSchemaArnsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDevelopmentSchemaArnsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDevelopmentSchemaArnsError {
    fn description(&self) -> &str {
        match *self {
            ListDevelopmentSchemaArnsError::AccessDenied(ref cause) => cause,
            ListDevelopmentSchemaArnsError::InternalService(ref cause) => cause,
            ListDevelopmentSchemaArnsError::InvalidArn(ref cause) => cause,
            ListDevelopmentSchemaArnsError::InvalidNextToken(ref cause) => cause,
            ListDevelopmentSchemaArnsError::LimitExceeded(ref cause) => cause,
            ListDevelopmentSchemaArnsError::ResourceNotFound(ref cause) => cause,
            ListDevelopmentSchemaArnsError::RetryableConflict(ref cause) => cause,
            ListDevelopmentSchemaArnsError::Validation(ref cause) => cause,
            ListDevelopmentSchemaArnsError::Credentials(ref err) => err.description(),
            ListDevelopmentSchemaArnsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDevelopmentSchemaArnsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDirectories
#[derive(Debug, PartialEq)]
pub enum ListDirectoriesError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListDirectoriesError {
    pub fn from_body(body: &str) -> ListDirectoriesError {
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
                        ListDirectoriesError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListDirectoriesError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListDirectoriesError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListDirectoriesError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListDirectoriesError::LimitExceeded(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListDirectoriesError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListDirectoriesError::Validation(error_message.to_string())
                    }
                    _ => ListDirectoriesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDirectoriesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDirectoriesError {
    fn from(err: serde_json::error::Error) -> ListDirectoriesError {
        ListDirectoriesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDirectoriesError {
    fn from(err: CredentialsError) -> ListDirectoriesError {
        ListDirectoriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDirectoriesError {
    fn from(err: HttpDispatchError) -> ListDirectoriesError {
        ListDirectoriesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDirectoriesError {
    fn from(err: io::Error) -> ListDirectoriesError {
        ListDirectoriesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDirectoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDirectoriesError {
    fn description(&self) -> &str {
        match *self {
            ListDirectoriesError::AccessDenied(ref cause) => cause,
            ListDirectoriesError::InternalService(ref cause) => cause,
            ListDirectoriesError::InvalidArn(ref cause) => cause,
            ListDirectoriesError::InvalidNextToken(ref cause) => cause,
            ListDirectoriesError::LimitExceeded(ref cause) => cause,
            ListDirectoriesError::RetryableConflict(ref cause) => cause,
            ListDirectoriesError::Validation(ref cause) => cause,
            ListDirectoriesError::Credentials(ref err) => err.description(),
            ListDirectoriesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDirectoriesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFacetAttributes
#[derive(Debug, PartialEq)]
pub enum ListFacetAttributesError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListFacetAttributesError {
    pub fn from_body(body: &str) -> ListFacetAttributesError {
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
                        ListFacetAttributesError::AccessDenied(String::from(error_message))
                    }
                    "FacetNotFoundException" => {
                        ListFacetAttributesError::FacetNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListFacetAttributesError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListFacetAttributesError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListFacetAttributesError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListFacetAttributesError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListFacetAttributesError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListFacetAttributesError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListFacetAttributesError::Validation(error_message.to_string())
                    }
                    _ => ListFacetAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListFacetAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListFacetAttributesError {
    fn from(err: serde_json::error::Error) -> ListFacetAttributesError {
        ListFacetAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFacetAttributesError {
    fn from(err: CredentialsError) -> ListFacetAttributesError {
        ListFacetAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFacetAttributesError {
    fn from(err: HttpDispatchError) -> ListFacetAttributesError {
        ListFacetAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFacetAttributesError {
    fn from(err: io::Error) -> ListFacetAttributesError {
        ListFacetAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFacetAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFacetAttributesError {
    fn description(&self) -> &str {
        match *self {
            ListFacetAttributesError::AccessDenied(ref cause) => cause,
            ListFacetAttributesError::FacetNotFound(ref cause) => cause,
            ListFacetAttributesError::InternalService(ref cause) => cause,
            ListFacetAttributesError::InvalidArn(ref cause) => cause,
            ListFacetAttributesError::InvalidNextToken(ref cause) => cause,
            ListFacetAttributesError::LimitExceeded(ref cause) => cause,
            ListFacetAttributesError::ResourceNotFound(ref cause) => cause,
            ListFacetAttributesError::RetryableConflict(ref cause) => cause,
            ListFacetAttributesError::Validation(ref cause) => cause,
            ListFacetAttributesError::Credentials(ref err) => err.description(),
            ListFacetAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListFacetAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFacetNames
#[derive(Debug, PartialEq)]
pub enum ListFacetNamesError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListFacetNamesError {
    pub fn from_body(body: &str) -> ListFacetNamesError {
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
                        ListFacetNamesError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListFacetNamesError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListFacetNamesError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListFacetNamesError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListFacetNamesError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListFacetNamesError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListFacetNamesError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListFacetNamesError::Validation(error_message.to_string())
                    }
                    _ => ListFacetNamesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListFacetNamesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListFacetNamesError {
    fn from(err: serde_json::error::Error) -> ListFacetNamesError {
        ListFacetNamesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFacetNamesError {
    fn from(err: CredentialsError) -> ListFacetNamesError {
        ListFacetNamesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFacetNamesError {
    fn from(err: HttpDispatchError) -> ListFacetNamesError {
        ListFacetNamesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFacetNamesError {
    fn from(err: io::Error) -> ListFacetNamesError {
        ListFacetNamesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFacetNamesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFacetNamesError {
    fn description(&self) -> &str {
        match *self {
            ListFacetNamesError::AccessDenied(ref cause) => cause,
            ListFacetNamesError::InternalService(ref cause) => cause,
            ListFacetNamesError::InvalidArn(ref cause) => cause,
            ListFacetNamesError::InvalidNextToken(ref cause) => cause,
            ListFacetNamesError::LimitExceeded(ref cause) => cause,
            ListFacetNamesError::ResourceNotFound(ref cause) => cause,
            ListFacetNamesError::RetryableConflict(ref cause) => cause,
            ListFacetNamesError::Validation(ref cause) => cause,
            ListFacetNamesError::Credentials(ref err) => err.description(),
            ListFacetNamesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListFacetNamesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIncomingTypedLinks
#[derive(Debug, PartialEq)]
pub enum ListIncomingTypedLinksError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListIncomingTypedLinksError {
    pub fn from_body(body: &str) -> ListIncomingTypedLinksError {
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
                        ListIncomingTypedLinksError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => ListIncomingTypedLinksError::DirectoryNotEnabled(String::from(error_message)),
                    "FacetValidationException" => {
                        ListIncomingTypedLinksError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListIncomingTypedLinksError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListIncomingTypedLinksError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListIncomingTypedLinksError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListIncomingTypedLinksError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListIncomingTypedLinksError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListIncomingTypedLinksError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListIncomingTypedLinksError::Validation(error_message.to_string())
                    }
                    _ => ListIncomingTypedLinksError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListIncomingTypedLinksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListIncomingTypedLinksError {
    fn from(err: serde_json::error::Error) -> ListIncomingTypedLinksError {
        ListIncomingTypedLinksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListIncomingTypedLinksError {
    fn from(err: CredentialsError) -> ListIncomingTypedLinksError {
        ListIncomingTypedLinksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIncomingTypedLinksError {
    fn from(err: HttpDispatchError) -> ListIncomingTypedLinksError {
        ListIncomingTypedLinksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListIncomingTypedLinksError {
    fn from(err: io::Error) -> ListIncomingTypedLinksError {
        ListIncomingTypedLinksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListIncomingTypedLinksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIncomingTypedLinksError {
    fn description(&self) -> &str {
        match *self {
            ListIncomingTypedLinksError::AccessDenied(ref cause) => cause,
            ListIncomingTypedLinksError::DirectoryNotEnabled(ref cause) => cause,
            ListIncomingTypedLinksError::FacetValidation(ref cause) => cause,
            ListIncomingTypedLinksError::InternalService(ref cause) => cause,
            ListIncomingTypedLinksError::InvalidArn(ref cause) => cause,
            ListIncomingTypedLinksError::InvalidNextToken(ref cause) => cause,
            ListIncomingTypedLinksError::LimitExceeded(ref cause) => cause,
            ListIncomingTypedLinksError::ResourceNotFound(ref cause) => cause,
            ListIncomingTypedLinksError::RetryableConflict(ref cause) => cause,
            ListIncomingTypedLinksError::Validation(ref cause) => cause,
            ListIncomingTypedLinksError::Credentials(ref err) => err.description(),
            ListIncomingTypedLinksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListIncomingTypedLinksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIndex
#[derive(Debug, PartialEq)]
pub enum ListIndexError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Indicates that the requested operation can only operate on index objects.</p>
    NotIndex(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListIndexError {
    pub fn from_body(body: &str) -> ListIndexError {
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
                        ListIndexError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        ListIndexError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListIndexError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListIndexError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListIndexError::LimitExceeded(String::from(error_message))
                    }
                    "NotIndexException" => ListIndexError::NotIndex(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        ListIndexError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListIndexError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => ListIndexError::Validation(error_message.to_string()),
                    _ => ListIndexError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListIndexError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListIndexError {
    fn from(err: serde_json::error::Error) -> ListIndexError {
        ListIndexError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListIndexError {
    fn from(err: CredentialsError) -> ListIndexError {
        ListIndexError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIndexError {
    fn from(err: HttpDispatchError) -> ListIndexError {
        ListIndexError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListIndexError {
    fn from(err: io::Error) -> ListIndexError {
        ListIndexError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIndexError {
    fn description(&self) -> &str {
        match *self {
            ListIndexError::AccessDenied(ref cause) => cause,
            ListIndexError::DirectoryNotEnabled(ref cause) => cause,
            ListIndexError::InternalService(ref cause) => cause,
            ListIndexError::InvalidArn(ref cause) => cause,
            ListIndexError::LimitExceeded(ref cause) => cause,
            ListIndexError::NotIndex(ref cause) => cause,
            ListIndexError::ResourceNotFound(ref cause) => cause,
            ListIndexError::RetryableConflict(ref cause) => cause,
            ListIndexError::Validation(ref cause) => cause,
            ListIndexError::Credentials(ref err) => err.description(),
            ListIndexError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListIndexError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListObjectAttributes
#[derive(Debug, PartialEq)]
pub enum ListObjectAttributesError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListObjectAttributesError {
    pub fn from_body(body: &str) -> ListObjectAttributesError {
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
                        ListObjectAttributesError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        ListObjectAttributesError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "FacetValidationException" => {
                        ListObjectAttributesError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListObjectAttributesError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListObjectAttributesError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListObjectAttributesError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListObjectAttributesError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListObjectAttributesError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListObjectAttributesError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListObjectAttributesError::Validation(error_message.to_string())
                    }
                    _ => ListObjectAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListObjectAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListObjectAttributesError {
    fn from(err: serde_json::error::Error) -> ListObjectAttributesError {
        ListObjectAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListObjectAttributesError {
    fn from(err: CredentialsError) -> ListObjectAttributesError {
        ListObjectAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectAttributesError {
    fn from(err: HttpDispatchError) -> ListObjectAttributesError {
        ListObjectAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectAttributesError {
    fn from(err: io::Error) -> ListObjectAttributesError {
        ListObjectAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectAttributesError {
    fn description(&self) -> &str {
        match *self {
            ListObjectAttributesError::AccessDenied(ref cause) => cause,
            ListObjectAttributesError::DirectoryNotEnabled(ref cause) => cause,
            ListObjectAttributesError::FacetValidation(ref cause) => cause,
            ListObjectAttributesError::InternalService(ref cause) => cause,
            ListObjectAttributesError::InvalidArn(ref cause) => cause,
            ListObjectAttributesError::InvalidNextToken(ref cause) => cause,
            ListObjectAttributesError::LimitExceeded(ref cause) => cause,
            ListObjectAttributesError::ResourceNotFound(ref cause) => cause,
            ListObjectAttributesError::RetryableConflict(ref cause) => cause,
            ListObjectAttributesError::Validation(ref cause) => cause,
            ListObjectAttributesError::Credentials(ref err) => err.description(),
            ListObjectAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListObjectChildren
#[derive(Debug, PartialEq)]
pub enum ListObjectChildrenError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Occurs when any invalid operations are performed on an object that is not a node, such as calling <code>ListObjectChildren</code> for a leaf node object.</p>
    NotNode(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListObjectChildrenError {
    pub fn from_body(body: &str) -> ListObjectChildrenError {
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
                        ListObjectChildrenError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        ListObjectChildrenError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListObjectChildrenError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListObjectChildrenError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListObjectChildrenError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListObjectChildrenError::LimitExceeded(String::from(error_message))
                    }
                    "NotNodeException" => {
                        ListObjectChildrenError::NotNode(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListObjectChildrenError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListObjectChildrenError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListObjectChildrenError::Validation(error_message.to_string())
                    }
                    _ => ListObjectChildrenError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListObjectChildrenError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListObjectChildrenError {
    fn from(err: serde_json::error::Error) -> ListObjectChildrenError {
        ListObjectChildrenError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListObjectChildrenError {
    fn from(err: CredentialsError) -> ListObjectChildrenError {
        ListObjectChildrenError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectChildrenError {
    fn from(err: HttpDispatchError) -> ListObjectChildrenError {
        ListObjectChildrenError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectChildrenError {
    fn from(err: io::Error) -> ListObjectChildrenError {
        ListObjectChildrenError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectChildrenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectChildrenError {
    fn description(&self) -> &str {
        match *self {
            ListObjectChildrenError::AccessDenied(ref cause) => cause,
            ListObjectChildrenError::DirectoryNotEnabled(ref cause) => cause,
            ListObjectChildrenError::InternalService(ref cause) => cause,
            ListObjectChildrenError::InvalidArn(ref cause) => cause,
            ListObjectChildrenError::InvalidNextToken(ref cause) => cause,
            ListObjectChildrenError::LimitExceeded(ref cause) => cause,
            ListObjectChildrenError::NotNode(ref cause) => cause,
            ListObjectChildrenError::ResourceNotFound(ref cause) => cause,
            ListObjectChildrenError::RetryableConflict(ref cause) => cause,
            ListObjectChildrenError::Validation(ref cause) => cause,
            ListObjectChildrenError::Credentials(ref err) => err.description(),
            ListObjectChildrenError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectChildrenError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListObjectParentPaths
#[derive(Debug, PartialEq)]
pub enum ListObjectParentPathsError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListObjectParentPathsError {
    pub fn from_body(body: &str) -> ListObjectParentPathsError {
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
                        ListObjectParentPathsError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        ListObjectParentPathsError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListObjectParentPathsError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListObjectParentPathsError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListObjectParentPathsError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListObjectParentPathsError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListObjectParentPathsError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListObjectParentPathsError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListObjectParentPathsError::Validation(error_message.to_string())
                    }
                    _ => ListObjectParentPathsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListObjectParentPathsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListObjectParentPathsError {
    fn from(err: serde_json::error::Error) -> ListObjectParentPathsError {
        ListObjectParentPathsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListObjectParentPathsError {
    fn from(err: CredentialsError) -> ListObjectParentPathsError {
        ListObjectParentPathsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectParentPathsError {
    fn from(err: HttpDispatchError) -> ListObjectParentPathsError {
        ListObjectParentPathsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectParentPathsError {
    fn from(err: io::Error) -> ListObjectParentPathsError {
        ListObjectParentPathsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectParentPathsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectParentPathsError {
    fn description(&self) -> &str {
        match *self {
            ListObjectParentPathsError::AccessDenied(ref cause) => cause,
            ListObjectParentPathsError::DirectoryNotEnabled(ref cause) => cause,
            ListObjectParentPathsError::InternalService(ref cause) => cause,
            ListObjectParentPathsError::InvalidArn(ref cause) => cause,
            ListObjectParentPathsError::InvalidNextToken(ref cause) => cause,
            ListObjectParentPathsError::LimitExceeded(ref cause) => cause,
            ListObjectParentPathsError::ResourceNotFound(ref cause) => cause,
            ListObjectParentPathsError::RetryableConflict(ref cause) => cause,
            ListObjectParentPathsError::Validation(ref cause) => cause,
            ListObjectParentPathsError::Credentials(ref err) => err.description(),
            ListObjectParentPathsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectParentPathsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListObjectParents
#[derive(Debug, PartialEq)]
pub enum ListObjectParentsError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Cannot list the parents of a <a>Directory</a> root.</p>
    CannotListParentOfRoot(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListObjectParentsError {
    pub fn from_body(body: &str) -> ListObjectParentsError {
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
                        ListObjectParentsError::AccessDenied(String::from(error_message))
                    }
                    "CannotListParentOfRootException" => {
                        ListObjectParentsError::CannotListParentOfRoot(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        ListObjectParentsError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListObjectParentsError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListObjectParentsError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListObjectParentsError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListObjectParentsError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListObjectParentsError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListObjectParentsError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListObjectParentsError::Validation(error_message.to_string())
                    }
                    _ => ListObjectParentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListObjectParentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListObjectParentsError {
    fn from(err: serde_json::error::Error) -> ListObjectParentsError {
        ListObjectParentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListObjectParentsError {
    fn from(err: CredentialsError) -> ListObjectParentsError {
        ListObjectParentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectParentsError {
    fn from(err: HttpDispatchError) -> ListObjectParentsError {
        ListObjectParentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectParentsError {
    fn from(err: io::Error) -> ListObjectParentsError {
        ListObjectParentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectParentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectParentsError {
    fn description(&self) -> &str {
        match *self {
            ListObjectParentsError::AccessDenied(ref cause) => cause,
            ListObjectParentsError::CannotListParentOfRoot(ref cause) => cause,
            ListObjectParentsError::DirectoryNotEnabled(ref cause) => cause,
            ListObjectParentsError::InternalService(ref cause) => cause,
            ListObjectParentsError::InvalidArn(ref cause) => cause,
            ListObjectParentsError::InvalidNextToken(ref cause) => cause,
            ListObjectParentsError::LimitExceeded(ref cause) => cause,
            ListObjectParentsError::ResourceNotFound(ref cause) => cause,
            ListObjectParentsError::RetryableConflict(ref cause) => cause,
            ListObjectParentsError::Validation(ref cause) => cause,
            ListObjectParentsError::Credentials(ref err) => err.description(),
            ListObjectParentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectParentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListObjectPolicies
#[derive(Debug, PartialEq)]
pub enum ListObjectPoliciesError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListObjectPoliciesError {
    pub fn from_body(body: &str) -> ListObjectPoliciesError {
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
                        ListObjectPoliciesError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        ListObjectPoliciesError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListObjectPoliciesError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListObjectPoliciesError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListObjectPoliciesError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListObjectPoliciesError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListObjectPoliciesError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListObjectPoliciesError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListObjectPoliciesError::Validation(error_message.to_string())
                    }
                    _ => ListObjectPoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListObjectPoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListObjectPoliciesError {
    fn from(err: serde_json::error::Error) -> ListObjectPoliciesError {
        ListObjectPoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListObjectPoliciesError {
    fn from(err: CredentialsError) -> ListObjectPoliciesError {
        ListObjectPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectPoliciesError {
    fn from(err: HttpDispatchError) -> ListObjectPoliciesError {
        ListObjectPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectPoliciesError {
    fn from(err: io::Error) -> ListObjectPoliciesError {
        ListObjectPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListObjectPoliciesError::AccessDenied(ref cause) => cause,
            ListObjectPoliciesError::DirectoryNotEnabled(ref cause) => cause,
            ListObjectPoliciesError::InternalService(ref cause) => cause,
            ListObjectPoliciesError::InvalidArn(ref cause) => cause,
            ListObjectPoliciesError::InvalidNextToken(ref cause) => cause,
            ListObjectPoliciesError::LimitExceeded(ref cause) => cause,
            ListObjectPoliciesError::ResourceNotFound(ref cause) => cause,
            ListObjectPoliciesError::RetryableConflict(ref cause) => cause,
            ListObjectPoliciesError::Validation(ref cause) => cause,
            ListObjectPoliciesError::Credentials(ref err) => err.description(),
            ListObjectPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOutgoingTypedLinks
#[derive(Debug, PartialEq)]
pub enum ListOutgoingTypedLinksError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListOutgoingTypedLinksError {
    pub fn from_body(body: &str) -> ListOutgoingTypedLinksError {
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
                        ListOutgoingTypedLinksError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => ListOutgoingTypedLinksError::DirectoryNotEnabled(String::from(error_message)),
                    "FacetValidationException" => {
                        ListOutgoingTypedLinksError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListOutgoingTypedLinksError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListOutgoingTypedLinksError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListOutgoingTypedLinksError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListOutgoingTypedLinksError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListOutgoingTypedLinksError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListOutgoingTypedLinksError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListOutgoingTypedLinksError::Validation(error_message.to_string())
                    }
                    _ => ListOutgoingTypedLinksError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListOutgoingTypedLinksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListOutgoingTypedLinksError {
    fn from(err: serde_json::error::Error) -> ListOutgoingTypedLinksError {
        ListOutgoingTypedLinksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOutgoingTypedLinksError {
    fn from(err: CredentialsError) -> ListOutgoingTypedLinksError {
        ListOutgoingTypedLinksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOutgoingTypedLinksError {
    fn from(err: HttpDispatchError) -> ListOutgoingTypedLinksError {
        ListOutgoingTypedLinksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOutgoingTypedLinksError {
    fn from(err: io::Error) -> ListOutgoingTypedLinksError {
        ListOutgoingTypedLinksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOutgoingTypedLinksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOutgoingTypedLinksError {
    fn description(&self) -> &str {
        match *self {
            ListOutgoingTypedLinksError::AccessDenied(ref cause) => cause,
            ListOutgoingTypedLinksError::DirectoryNotEnabled(ref cause) => cause,
            ListOutgoingTypedLinksError::FacetValidation(ref cause) => cause,
            ListOutgoingTypedLinksError::InternalService(ref cause) => cause,
            ListOutgoingTypedLinksError::InvalidArn(ref cause) => cause,
            ListOutgoingTypedLinksError::InvalidNextToken(ref cause) => cause,
            ListOutgoingTypedLinksError::LimitExceeded(ref cause) => cause,
            ListOutgoingTypedLinksError::ResourceNotFound(ref cause) => cause,
            ListOutgoingTypedLinksError::RetryableConflict(ref cause) => cause,
            ListOutgoingTypedLinksError::Validation(ref cause) => cause,
            ListOutgoingTypedLinksError::Credentials(ref err) => err.description(),
            ListOutgoingTypedLinksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListOutgoingTypedLinksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPolicyAttachments
#[derive(Debug, PartialEq)]
pub enum ListPolicyAttachmentsError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Indicates that the requested operation can only operate on policy objects.</p>
    NotPolicy(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPolicyAttachmentsError {
    pub fn from_body(body: &str) -> ListPolicyAttachmentsError {
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
                        ListPolicyAttachmentsError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        ListPolicyAttachmentsError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListPolicyAttachmentsError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListPolicyAttachmentsError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListPolicyAttachmentsError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListPolicyAttachmentsError::LimitExceeded(String::from(error_message))
                    }
                    "NotPolicyException" => {
                        ListPolicyAttachmentsError::NotPolicy(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPolicyAttachmentsError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListPolicyAttachmentsError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPolicyAttachmentsError::Validation(error_message.to_string())
                    }
                    _ => ListPolicyAttachmentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPolicyAttachmentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPolicyAttachmentsError {
    fn from(err: serde_json::error::Error) -> ListPolicyAttachmentsError {
        ListPolicyAttachmentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPolicyAttachmentsError {
    fn from(err: CredentialsError) -> ListPolicyAttachmentsError {
        ListPolicyAttachmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPolicyAttachmentsError {
    fn from(err: HttpDispatchError) -> ListPolicyAttachmentsError {
        ListPolicyAttachmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPolicyAttachmentsError {
    fn from(err: io::Error) -> ListPolicyAttachmentsError {
        ListPolicyAttachmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPolicyAttachmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPolicyAttachmentsError {
    fn description(&self) -> &str {
        match *self {
            ListPolicyAttachmentsError::AccessDenied(ref cause) => cause,
            ListPolicyAttachmentsError::DirectoryNotEnabled(ref cause) => cause,
            ListPolicyAttachmentsError::InternalService(ref cause) => cause,
            ListPolicyAttachmentsError::InvalidArn(ref cause) => cause,
            ListPolicyAttachmentsError::InvalidNextToken(ref cause) => cause,
            ListPolicyAttachmentsError::LimitExceeded(ref cause) => cause,
            ListPolicyAttachmentsError::NotPolicy(ref cause) => cause,
            ListPolicyAttachmentsError::ResourceNotFound(ref cause) => cause,
            ListPolicyAttachmentsError::RetryableConflict(ref cause) => cause,
            ListPolicyAttachmentsError::Validation(ref cause) => cause,
            ListPolicyAttachmentsError::Credentials(ref err) => err.description(),
            ListPolicyAttachmentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPolicyAttachmentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPublishedSchemaArns
#[derive(Debug, PartialEq)]
pub enum ListPublishedSchemaArnsError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPublishedSchemaArnsError {
    pub fn from_body(body: &str) -> ListPublishedSchemaArnsError {
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
                        ListPublishedSchemaArnsError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListPublishedSchemaArnsError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListPublishedSchemaArnsError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListPublishedSchemaArnsError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListPublishedSchemaArnsError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPublishedSchemaArnsError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListPublishedSchemaArnsError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPublishedSchemaArnsError::Validation(error_message.to_string())
                    }
                    _ => ListPublishedSchemaArnsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPublishedSchemaArnsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPublishedSchemaArnsError {
    fn from(err: serde_json::error::Error) -> ListPublishedSchemaArnsError {
        ListPublishedSchemaArnsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPublishedSchemaArnsError {
    fn from(err: CredentialsError) -> ListPublishedSchemaArnsError {
        ListPublishedSchemaArnsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPublishedSchemaArnsError {
    fn from(err: HttpDispatchError) -> ListPublishedSchemaArnsError {
        ListPublishedSchemaArnsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPublishedSchemaArnsError {
    fn from(err: io::Error) -> ListPublishedSchemaArnsError {
        ListPublishedSchemaArnsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPublishedSchemaArnsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPublishedSchemaArnsError {
    fn description(&self) -> &str {
        match *self {
            ListPublishedSchemaArnsError::AccessDenied(ref cause) => cause,
            ListPublishedSchemaArnsError::InternalService(ref cause) => cause,
            ListPublishedSchemaArnsError::InvalidArn(ref cause) => cause,
            ListPublishedSchemaArnsError::InvalidNextToken(ref cause) => cause,
            ListPublishedSchemaArnsError::LimitExceeded(ref cause) => cause,
            ListPublishedSchemaArnsError::ResourceNotFound(ref cause) => cause,
            ListPublishedSchemaArnsError::RetryableConflict(ref cause) => cause,
            ListPublishedSchemaArnsError::Validation(ref cause) => cause,
            ListPublishedSchemaArnsError::Credentials(ref err) => err.description(),
            ListPublishedSchemaArnsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPublishedSchemaArnsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Can occur for multiple reasons such as when you tag a resource that doesn’t exist or if you specify a higher number of tags for a resource than the allowed limit. Allowed limit is 50 tags per resource.</p>
    InvalidTaggingRequest(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListTagsForResourceError {
    pub fn from_body(body: &str) -> ListTagsForResourceError {
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
                        ListTagsForResourceError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListTagsForResourceError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListTagsForResourceError::InvalidArn(String::from(error_message))
                    }
                    "InvalidTaggingRequestException" => {
                        ListTagsForResourceError::InvalidTaggingRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListTagsForResourceError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListTagsForResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListTagsForResourceError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsForResourceError::Validation(error_message.to_string())
                    }
                    _ => ListTagsForResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsForResourceError {
    fn from(err: serde_json::error::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourceError {
    fn from(err: CredentialsError) -> ListTagsForResourceError {
        ListTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourceError {
    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForResourceError {
    fn from(err: io::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::AccessDenied(ref cause) => cause,
            ListTagsForResourceError::InternalService(ref cause) => cause,
            ListTagsForResourceError::InvalidArn(ref cause) => cause,
            ListTagsForResourceError::InvalidTaggingRequest(ref cause) => cause,
            ListTagsForResourceError::LimitExceeded(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
            ListTagsForResourceError::RetryableConflict(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTypedLinkFacetAttributes
#[derive(Debug, PartialEq)]
pub enum ListTypedLinkFacetAttributesError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListTypedLinkFacetAttributesError {
    pub fn from_body(body: &str) -> ListTypedLinkFacetAttributesError {
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
                        ListTypedLinkFacetAttributesError::AccessDenied(String::from(error_message))
                    }
                    "FacetNotFoundException" => ListTypedLinkFacetAttributesError::FacetNotFound(String::from(error_message)),
                    "InternalServiceException" => ListTypedLinkFacetAttributesError::InternalService(String::from(error_message)),
                    "InvalidArnException" => {
                        ListTypedLinkFacetAttributesError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => ListTypedLinkFacetAttributesError::InvalidNextToken(String::from(error_message)),
                    "LimitExceededException" => ListTypedLinkFacetAttributesError::LimitExceeded(String::from(error_message)),
                    "ResourceNotFoundException" => ListTypedLinkFacetAttributesError::ResourceNotFound(String::from(error_message)),
                    "RetryableConflictException" => ListTypedLinkFacetAttributesError::RetryableConflict(String::from(error_message)),
                    "ValidationException" => {
                        ListTypedLinkFacetAttributesError::Validation(error_message.to_string())
                    }
                    _ => ListTypedLinkFacetAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTypedLinkFacetAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTypedLinkFacetAttributesError {
    fn from(err: serde_json::error::Error) -> ListTypedLinkFacetAttributesError {
        ListTypedLinkFacetAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTypedLinkFacetAttributesError {
    fn from(err: CredentialsError) -> ListTypedLinkFacetAttributesError {
        ListTypedLinkFacetAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTypedLinkFacetAttributesError {
    fn from(err: HttpDispatchError) -> ListTypedLinkFacetAttributesError {
        ListTypedLinkFacetAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTypedLinkFacetAttributesError {
    fn from(err: io::Error) -> ListTypedLinkFacetAttributesError {
        ListTypedLinkFacetAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTypedLinkFacetAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTypedLinkFacetAttributesError {
    fn description(&self) -> &str {
        match *self {
            ListTypedLinkFacetAttributesError::AccessDenied(ref cause) => cause,
            ListTypedLinkFacetAttributesError::FacetNotFound(ref cause) => cause,
            ListTypedLinkFacetAttributesError::InternalService(ref cause) => cause,
            ListTypedLinkFacetAttributesError::InvalidArn(ref cause) => cause,
            ListTypedLinkFacetAttributesError::InvalidNextToken(ref cause) => cause,
            ListTypedLinkFacetAttributesError::LimitExceeded(ref cause) => cause,
            ListTypedLinkFacetAttributesError::ResourceNotFound(ref cause) => cause,
            ListTypedLinkFacetAttributesError::RetryableConflict(ref cause) => cause,
            ListTypedLinkFacetAttributesError::Validation(ref cause) => cause,
            ListTypedLinkFacetAttributesError::Credentials(ref err) => err.description(),
            ListTypedLinkFacetAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTypedLinkFacetAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTypedLinkFacetNames
#[derive(Debug, PartialEq)]
pub enum ListTypedLinkFacetNamesError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListTypedLinkFacetNamesError {
    pub fn from_body(body: &str) -> ListTypedLinkFacetNamesError {
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
                        ListTypedLinkFacetNamesError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        ListTypedLinkFacetNamesError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListTypedLinkFacetNamesError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListTypedLinkFacetNamesError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListTypedLinkFacetNamesError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListTypedLinkFacetNamesError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        ListTypedLinkFacetNamesError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTypedLinkFacetNamesError::Validation(error_message.to_string())
                    }
                    _ => ListTypedLinkFacetNamesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTypedLinkFacetNamesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTypedLinkFacetNamesError {
    fn from(err: serde_json::error::Error) -> ListTypedLinkFacetNamesError {
        ListTypedLinkFacetNamesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTypedLinkFacetNamesError {
    fn from(err: CredentialsError) -> ListTypedLinkFacetNamesError {
        ListTypedLinkFacetNamesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTypedLinkFacetNamesError {
    fn from(err: HttpDispatchError) -> ListTypedLinkFacetNamesError {
        ListTypedLinkFacetNamesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTypedLinkFacetNamesError {
    fn from(err: io::Error) -> ListTypedLinkFacetNamesError {
        ListTypedLinkFacetNamesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTypedLinkFacetNamesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTypedLinkFacetNamesError {
    fn description(&self) -> &str {
        match *self {
            ListTypedLinkFacetNamesError::AccessDenied(ref cause) => cause,
            ListTypedLinkFacetNamesError::InternalService(ref cause) => cause,
            ListTypedLinkFacetNamesError::InvalidArn(ref cause) => cause,
            ListTypedLinkFacetNamesError::InvalidNextToken(ref cause) => cause,
            ListTypedLinkFacetNamesError::LimitExceeded(ref cause) => cause,
            ListTypedLinkFacetNamesError::ResourceNotFound(ref cause) => cause,
            ListTypedLinkFacetNamesError::RetryableConflict(ref cause) => cause,
            ListTypedLinkFacetNamesError::Validation(ref cause) => cause,
            ListTypedLinkFacetNamesError::Credentials(ref err) => err.description(),
            ListTypedLinkFacetNamesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTypedLinkFacetNamesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by LookupPolicy
#[derive(Debug, PartialEq)]
pub enum LookupPolicyError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that the <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl LookupPolicyError {
    pub fn from_body(body: &str) -> LookupPolicyError {
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
                        LookupPolicyError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        LookupPolicyError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        LookupPolicyError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        LookupPolicyError::InvalidArn(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        LookupPolicyError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        LookupPolicyError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        LookupPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        LookupPolicyError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        LookupPolicyError::Validation(error_message.to_string())
                    }
                    _ => LookupPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => LookupPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for LookupPolicyError {
    fn from(err: serde_json::error::Error) -> LookupPolicyError {
        LookupPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for LookupPolicyError {
    fn from(err: CredentialsError) -> LookupPolicyError {
        LookupPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for LookupPolicyError {
    fn from(err: HttpDispatchError) -> LookupPolicyError {
        LookupPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for LookupPolicyError {
    fn from(err: io::Error) -> LookupPolicyError {
        LookupPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for LookupPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for LookupPolicyError {
    fn description(&self) -> &str {
        match *self {
            LookupPolicyError::AccessDenied(ref cause) => cause,
            LookupPolicyError::DirectoryNotEnabled(ref cause) => cause,
            LookupPolicyError::InternalService(ref cause) => cause,
            LookupPolicyError::InvalidArn(ref cause) => cause,
            LookupPolicyError::InvalidNextToken(ref cause) => cause,
            LookupPolicyError::LimitExceeded(ref cause) => cause,
            LookupPolicyError::ResourceNotFound(ref cause) => cause,
            LookupPolicyError::RetryableConflict(ref cause) => cause,
            LookupPolicyError::Validation(ref cause) => cause,
            LookupPolicyError::Credentials(ref err) => err.description(),
            LookupPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            LookupPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PublishSchema
#[derive(Debug, PartialEq)]
pub enum PublishSchemaError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    ///<p>Indicates that a schema is already published.</p>
    SchemaAlreadyPublished(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PublishSchemaError {
    pub fn from_body(body: &str) -> PublishSchemaError {
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
                        PublishSchemaError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        PublishSchemaError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        PublishSchemaError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PublishSchemaError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PublishSchemaError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        PublishSchemaError::RetryableConflict(String::from(error_message))
                    }
                    "SchemaAlreadyPublishedException" => {
                        PublishSchemaError::SchemaAlreadyPublished(String::from(error_message))
                    }
                    "ValidationException" => {
                        PublishSchemaError::Validation(error_message.to_string())
                    }
                    _ => PublishSchemaError::Unknown(String::from(body)),
                }
            }
            Err(_) => PublishSchemaError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PublishSchemaError {
    fn from(err: serde_json::error::Error) -> PublishSchemaError {
        PublishSchemaError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PublishSchemaError {
    fn from(err: CredentialsError) -> PublishSchemaError {
        PublishSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PublishSchemaError {
    fn from(err: HttpDispatchError) -> PublishSchemaError {
        PublishSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for PublishSchemaError {
    fn from(err: io::Error) -> PublishSchemaError {
        PublishSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PublishSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PublishSchemaError {
    fn description(&self) -> &str {
        match *self {
            PublishSchemaError::AccessDenied(ref cause) => cause,
            PublishSchemaError::InternalService(ref cause) => cause,
            PublishSchemaError::InvalidArn(ref cause) => cause,
            PublishSchemaError::LimitExceeded(ref cause) => cause,
            PublishSchemaError::ResourceNotFound(ref cause) => cause,
            PublishSchemaError::RetryableConflict(ref cause) => cause,
            PublishSchemaError::SchemaAlreadyPublished(ref cause) => cause,
            PublishSchemaError::Validation(ref cause) => cause,
            PublishSchemaError::Credentials(ref err) => err.description(),
            PublishSchemaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PublishSchemaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutSchemaFromJson
#[derive(Debug, PartialEq)]
pub enum PutSchemaFromJsonError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Occurs when any of the rule parameter keys or values are invalid.</p>
    InvalidRule(String),
    ///<p>Indicates that the provided <code>SchemaDoc</code> value is not valid.</p>
    InvalidSchemaDoc(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutSchemaFromJsonError {
    pub fn from_body(body: &str) -> PutSchemaFromJsonError {
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
                        PutSchemaFromJsonError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        PutSchemaFromJsonError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        PutSchemaFromJsonError::InvalidArn(String::from(error_message))
                    }
                    "InvalidRuleException" => {
                        PutSchemaFromJsonError::InvalidRule(String::from(error_message))
                    }
                    "InvalidSchemaDocException" => {
                        PutSchemaFromJsonError::InvalidSchemaDoc(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutSchemaFromJsonError::LimitExceeded(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        PutSchemaFromJsonError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutSchemaFromJsonError::Validation(error_message.to_string())
                    }
                    _ => PutSchemaFromJsonError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutSchemaFromJsonError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutSchemaFromJsonError {
    fn from(err: serde_json::error::Error) -> PutSchemaFromJsonError {
        PutSchemaFromJsonError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutSchemaFromJsonError {
    fn from(err: CredentialsError) -> PutSchemaFromJsonError {
        PutSchemaFromJsonError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutSchemaFromJsonError {
    fn from(err: HttpDispatchError) -> PutSchemaFromJsonError {
        PutSchemaFromJsonError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutSchemaFromJsonError {
    fn from(err: io::Error) -> PutSchemaFromJsonError {
        PutSchemaFromJsonError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutSchemaFromJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutSchemaFromJsonError {
    fn description(&self) -> &str {
        match *self {
            PutSchemaFromJsonError::AccessDenied(ref cause) => cause,
            PutSchemaFromJsonError::InternalService(ref cause) => cause,
            PutSchemaFromJsonError::InvalidArn(ref cause) => cause,
            PutSchemaFromJsonError::InvalidRule(ref cause) => cause,
            PutSchemaFromJsonError::InvalidSchemaDoc(ref cause) => cause,
            PutSchemaFromJsonError::LimitExceeded(ref cause) => cause,
            PutSchemaFromJsonError::RetryableConflict(ref cause) => cause,
            PutSchemaFromJsonError::Validation(ref cause) => cause,
            PutSchemaFromJsonError::Credentials(ref err) => err.description(),
            PutSchemaFromJsonError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutSchemaFromJsonError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveFacetFromObject
#[derive(Debug, PartialEq)]
pub enum RemoveFacetFromObjectError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RemoveFacetFromObjectError {
    pub fn from_body(body: &str) -> RemoveFacetFromObjectError {
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
                        RemoveFacetFromObjectError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => {
                        RemoveFacetFromObjectError::DirectoryNotEnabled(String::from(error_message))
                    }
                    "FacetValidationException" => {
                        RemoveFacetFromObjectError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        RemoveFacetFromObjectError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        RemoveFacetFromObjectError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        RemoveFacetFromObjectError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RemoveFacetFromObjectError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        RemoveFacetFromObjectError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        RemoveFacetFromObjectError::Validation(error_message.to_string())
                    }
                    _ => RemoveFacetFromObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveFacetFromObjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveFacetFromObjectError {
    fn from(err: serde_json::error::Error) -> RemoveFacetFromObjectError {
        RemoveFacetFromObjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveFacetFromObjectError {
    fn from(err: CredentialsError) -> RemoveFacetFromObjectError {
        RemoveFacetFromObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveFacetFromObjectError {
    fn from(err: HttpDispatchError) -> RemoveFacetFromObjectError {
        RemoveFacetFromObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveFacetFromObjectError {
    fn from(err: io::Error) -> RemoveFacetFromObjectError {
        RemoveFacetFromObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveFacetFromObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveFacetFromObjectError {
    fn description(&self) -> &str {
        match *self {
            RemoveFacetFromObjectError::AccessDenied(ref cause) => cause,
            RemoveFacetFromObjectError::DirectoryNotEnabled(ref cause) => cause,
            RemoveFacetFromObjectError::FacetValidation(ref cause) => cause,
            RemoveFacetFromObjectError::InternalService(ref cause) => cause,
            RemoveFacetFromObjectError::InvalidArn(ref cause) => cause,
            RemoveFacetFromObjectError::LimitExceeded(ref cause) => cause,
            RemoveFacetFromObjectError::ResourceNotFound(ref cause) => cause,
            RemoveFacetFromObjectError::RetryableConflict(ref cause) => cause,
            RemoveFacetFromObjectError::Validation(ref cause) => cause,
            RemoveFacetFromObjectError::Credentials(ref err) => err.description(),
            RemoveFacetFromObjectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveFacetFromObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Can occur for multiple reasons such as when you tag a resource that doesn’t exist or if you specify a higher number of tags for a resource than the allowed limit. Allowed limit is 50 tags per resource.</p>
    InvalidTaggingRequest(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl TagResourceError {
    pub fn from_body(body: &str) -> TagResourceError {
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
                        TagResourceError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        TagResourceError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        TagResourceError::InvalidArn(String::from(error_message))
                    }
                    "InvalidTaggingRequestException" => {
                        TagResourceError::InvalidTaggingRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        TagResourceError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        TagResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        TagResourceError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        TagResourceError::Validation(error_message.to_string())
                    }
                    _ => TagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::AccessDenied(ref cause) => cause,
            TagResourceError::InternalService(ref cause) => cause,
            TagResourceError::InvalidArn(ref cause) => cause,
            TagResourceError::InvalidTaggingRequest(ref cause) => cause,
            TagResourceError::LimitExceeded(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::RetryableConflict(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Can occur for multiple reasons such as when you tag a resource that doesn’t exist or if you specify a higher number of tags for a resource than the allowed limit. Allowed limit is 50 tags per resource.</p>
    InvalidTaggingRequest(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UntagResourceError {
    pub fn from_body(body: &str) -> UntagResourceError {
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
                        UntagResourceError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UntagResourceError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        UntagResourceError::InvalidArn(String::from(error_message))
                    }
                    "InvalidTaggingRequestException" => {
                        UntagResourceError::InvalidTaggingRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UntagResourceError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UntagResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        UntagResourceError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagResourceError::Validation(error_message.to_string())
                    }
                    _ => UntagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::AccessDenied(ref cause) => cause,
            UntagResourceError::InternalService(ref cause) => cause,
            UntagResourceError::InvalidArn(ref cause) => cause,
            UntagResourceError::InvalidTaggingRequest(ref cause) => cause,
            UntagResourceError::LimitExceeded(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
            UntagResourceError::RetryableConflict(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFacet
#[derive(Debug, PartialEq)]
pub enum UpdateFacetError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>An attempt to modify a <a>Facet</a> resulted in an invalid schema exception.</p>
    InvalidFacetUpdate(String),
    ///<p>Occurs when any of the rule parameter keys or values are invalid.</p>
    InvalidRule(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateFacetError {
    pub fn from_body(body: &str) -> UpdateFacetError {
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
                        UpdateFacetError::AccessDenied(String::from(error_message))
                    }
                    "FacetNotFoundException" => {
                        UpdateFacetError::FacetNotFound(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateFacetError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        UpdateFacetError::InvalidArn(String::from(error_message))
                    }
                    "InvalidFacetUpdateException" => {
                        UpdateFacetError::InvalidFacetUpdate(String::from(error_message))
                    }
                    "InvalidRuleException" => {
                        UpdateFacetError::InvalidRule(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateFacetError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateFacetError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        UpdateFacetError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateFacetError::Validation(error_message.to_string())
                    }
                    _ => UpdateFacetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateFacetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateFacetError {
    fn from(err: serde_json::error::Error) -> UpdateFacetError {
        UpdateFacetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFacetError {
    fn from(err: CredentialsError) -> UpdateFacetError {
        UpdateFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFacetError {
    fn from(err: HttpDispatchError) -> UpdateFacetError {
        UpdateFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFacetError {
    fn from(err: io::Error) -> UpdateFacetError {
        UpdateFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFacetError {
    fn description(&self) -> &str {
        match *self {
            UpdateFacetError::AccessDenied(ref cause) => cause,
            UpdateFacetError::FacetNotFound(ref cause) => cause,
            UpdateFacetError::InternalService(ref cause) => cause,
            UpdateFacetError::InvalidArn(ref cause) => cause,
            UpdateFacetError::InvalidFacetUpdate(ref cause) => cause,
            UpdateFacetError::InvalidRule(ref cause) => cause,
            UpdateFacetError::LimitExceeded(ref cause) => cause,
            UpdateFacetError::ResourceNotFound(ref cause) => cause,
            UpdateFacetError::RetryableConflict(ref cause) => cause,
            UpdateFacetError::Validation(ref cause) => cause,
            UpdateFacetError::Credentials(ref err) => err.description(),
            UpdateFacetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateFacetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateObjectAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateObjectAttributesError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>An operation can only operate on a directory that is not enabled.</p>
    DirectoryNotEnabled(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateObjectAttributesError {
    pub fn from_body(body: &str) -> UpdateObjectAttributesError {
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
                        UpdateObjectAttributesError::AccessDenied(String::from(error_message))
                    }
                    "DirectoryNotEnabledException" => UpdateObjectAttributesError::DirectoryNotEnabled(String::from(error_message)),
                    "FacetValidationException" => {
                        UpdateObjectAttributesError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateObjectAttributesError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        UpdateObjectAttributesError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateObjectAttributesError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateObjectAttributesError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        UpdateObjectAttributesError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateObjectAttributesError::Validation(error_message.to_string())
                    }
                    _ => UpdateObjectAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateObjectAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateObjectAttributesError {
    fn from(err: serde_json::error::Error) -> UpdateObjectAttributesError {
        UpdateObjectAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateObjectAttributesError {
    fn from(err: CredentialsError) -> UpdateObjectAttributesError {
        UpdateObjectAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateObjectAttributesError {
    fn from(err: HttpDispatchError) -> UpdateObjectAttributesError {
        UpdateObjectAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateObjectAttributesError {
    fn from(err: io::Error) -> UpdateObjectAttributesError {
        UpdateObjectAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateObjectAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateObjectAttributesError {
    fn description(&self) -> &str {
        match *self {
            UpdateObjectAttributesError::AccessDenied(ref cause) => cause,
            UpdateObjectAttributesError::DirectoryNotEnabled(ref cause) => cause,
            UpdateObjectAttributesError::FacetValidation(ref cause) => cause,
            UpdateObjectAttributesError::InternalService(ref cause) => cause,
            UpdateObjectAttributesError::InvalidArn(ref cause) => cause,
            UpdateObjectAttributesError::LimitExceeded(ref cause) => cause,
            UpdateObjectAttributesError::ResourceNotFound(ref cause) => cause,
            UpdateObjectAttributesError::RetryableConflict(ref cause) => cause,
            UpdateObjectAttributesError::Validation(ref cause) => cause,
            UpdateObjectAttributesError::Credentials(ref err) => err.description(),
            UpdateObjectAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateObjectAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSchema
#[derive(Debug, PartialEq)]
pub enum UpdateSchemaError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateSchemaError {
    pub fn from_body(body: &str) -> UpdateSchemaError {
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
                        UpdateSchemaError::AccessDenied(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateSchemaError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        UpdateSchemaError::InvalidArn(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateSchemaError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateSchemaError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        UpdateSchemaError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateSchemaError::Validation(error_message.to_string())
                    }
                    _ => UpdateSchemaError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSchemaError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSchemaError {
    fn from(err: serde_json::error::Error) -> UpdateSchemaError {
        UpdateSchemaError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSchemaError {
    fn from(err: CredentialsError) -> UpdateSchemaError {
        UpdateSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSchemaError {
    fn from(err: HttpDispatchError) -> UpdateSchemaError {
        UpdateSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSchemaError {
    fn from(err: io::Error) -> UpdateSchemaError {
        UpdateSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSchemaError {
    fn description(&self) -> &str {
        match *self {
            UpdateSchemaError::AccessDenied(ref cause) => cause,
            UpdateSchemaError::InternalService(ref cause) => cause,
            UpdateSchemaError::InvalidArn(ref cause) => cause,
            UpdateSchemaError::LimitExceeded(ref cause) => cause,
            UpdateSchemaError::ResourceNotFound(ref cause) => cause,
            UpdateSchemaError::RetryableConflict(ref cause) => cause,
            UpdateSchemaError::Validation(ref cause) => cause,
            UpdateSchemaError::Credentials(ref err) => err.description(),
            UpdateSchemaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateSchemaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTypedLinkFacet
#[derive(Debug, PartialEq)]
pub enum UpdateTypedLinkFacetError {
    ///<p>Access denied. Check your permissions.</p>
    AccessDenied(String),
    ///<p>The specified <a>Facet</a> could not be found.</p>
    FacetNotFound(String),
    ///<p>The <a>Facet</a> that you provided was not well formed or could not be validated with the schema.</p>
    FacetValidation(String),
    ///<p>Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds. Otherwise, go to the <a href="http://status.aws.amazon.com/">AWS Service Health Dashboard</a> site to see if there are any operational issues with the service.</p>
    InternalService(String),
    ///<p>Indicates that the provided ARN value is not valid.</p>
    InvalidArn(String),
    ///<p>An attempt to modify a <a>Facet</a> resulted in an invalid schema exception.</p>
    InvalidFacetUpdate(String),
    ///<p>Occurs when any of the rule parameter keys or values are invalid.</p>
    InvalidRule(String),
    ///<p>Indicates that limits are exceeded. See <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/limits.html">Limits</a> for more information.</p>
    LimitExceeded(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to read the object using “SERIALIZABLE” consistency, this exception may result. This generally occurs when the previous write did not have time to propagate to the host serving the current request. A retry (with appropriate backoff logic) is the recommended response to this exception.</p>
    RetryableConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateTypedLinkFacetError {
    pub fn from_body(body: &str) -> UpdateTypedLinkFacetError {
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
                        UpdateTypedLinkFacetError::AccessDenied(String::from(error_message))
                    }
                    "FacetNotFoundException" => {
                        UpdateTypedLinkFacetError::FacetNotFound(String::from(error_message))
                    }
                    "FacetValidationException" => {
                        UpdateTypedLinkFacetError::FacetValidation(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        UpdateTypedLinkFacetError::InternalService(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        UpdateTypedLinkFacetError::InvalidArn(String::from(error_message))
                    }
                    "InvalidFacetUpdateException" => {
                        UpdateTypedLinkFacetError::InvalidFacetUpdate(String::from(error_message))
                    }
                    "InvalidRuleException" => {
                        UpdateTypedLinkFacetError::InvalidRule(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateTypedLinkFacetError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateTypedLinkFacetError::ResourceNotFound(String::from(error_message))
                    }
                    "RetryableConflictException" => {
                        UpdateTypedLinkFacetError::RetryableConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateTypedLinkFacetError::Validation(error_message.to_string())
                    }
                    _ => UpdateTypedLinkFacetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateTypedLinkFacetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateTypedLinkFacetError {
    fn from(err: serde_json::error::Error) -> UpdateTypedLinkFacetError {
        UpdateTypedLinkFacetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateTypedLinkFacetError {
    fn from(err: CredentialsError) -> UpdateTypedLinkFacetError {
        UpdateTypedLinkFacetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTypedLinkFacetError {
    fn from(err: HttpDispatchError) -> UpdateTypedLinkFacetError {
        UpdateTypedLinkFacetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTypedLinkFacetError {
    fn from(err: io::Error) -> UpdateTypedLinkFacetError {
        UpdateTypedLinkFacetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTypedLinkFacetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTypedLinkFacetError {
    fn description(&self) -> &str {
        match *self {
            UpdateTypedLinkFacetError::AccessDenied(ref cause) => cause,
            UpdateTypedLinkFacetError::FacetNotFound(ref cause) => cause,
            UpdateTypedLinkFacetError::FacetValidation(ref cause) => cause,
            UpdateTypedLinkFacetError::InternalService(ref cause) => cause,
            UpdateTypedLinkFacetError::InvalidArn(ref cause) => cause,
            UpdateTypedLinkFacetError::InvalidFacetUpdate(ref cause) => cause,
            UpdateTypedLinkFacetError::InvalidRule(ref cause) => cause,
            UpdateTypedLinkFacetError::LimitExceeded(ref cause) => cause,
            UpdateTypedLinkFacetError::ResourceNotFound(ref cause) => cause,
            UpdateTypedLinkFacetError::RetryableConflict(ref cause) => cause,
            UpdateTypedLinkFacetError::Validation(ref cause) => cause,
            UpdateTypedLinkFacetError::Credentials(ref err) => err.description(),
            UpdateTypedLinkFacetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateTypedLinkFacetError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon CloudDirectory API. Amazon CloudDirectory clients implement this trait.
pub trait CloudDirectory {
    #[doc="<p>Adds a new <a>Facet</a> to an object.</p>"]
    fn add_facet_to_object(&self,
                           input: &AddFacetToObjectRequest)
                           -> Result<AddFacetToObjectResponse, AddFacetToObjectError>;


    #[doc="<p>Copies the input published schema into the <a>Directory</a> with the same name and version as that of the published schema .</p>"]
    fn apply_schema(&self,
                    input: &ApplySchemaRequest)
                    -> Result<ApplySchemaResponse, ApplySchemaError>;


    #[doc="<p>Attaches an existing object to another object. An object can be accessed in two ways:</p> <ol> <li> <p>Using the path</p> </li> <li> <p>Using <code>ObjectIdentifier</code> </p> </li> </ol>"]
    fn attach_object(&self,
                     input: &AttachObjectRequest)
                     -> Result<AttachObjectResponse, AttachObjectError>;


    #[doc="<p>Attaches a policy object to a regular object. An object can have a limited number of attached policies.</p>"]
    fn attach_policy(&self,
                     input: &AttachPolicyRequest)
                     -> Result<AttachPolicyResponse, AttachPolicyError>;


    #[doc="<p>Attaches the specified object to the specified index.</p>"]
    fn attach_to_index(&self,
                       input: &AttachToIndexRequest)
                       -> Result<AttachToIndexResponse, AttachToIndexError>;


    #[doc="<p>Attaches a typed link to a specified source and target object. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn attach_typed_link(&self,
                         input: &AttachTypedLinkRequest)
                         -> Result<AttachTypedLinkResponse, AttachTypedLinkError>;


    #[doc="<p>Performs all the read operations in a batch. </p>"]
    fn batch_read(&self, input: &BatchReadRequest) -> Result<BatchReadResponse, BatchReadError>;


    #[doc="<p>Performs all the write operations in a batch. Either all the operations succeed or none. Batch writes supports only object-related operations.</p>"]
    fn batch_write(&self,
                   input: &BatchWriteRequest)
                   -> Result<BatchWriteResponse, BatchWriteError>;


    #[doc="<p>Creates a <a>Directory</a> by copying the published schema into the directory. A directory cannot be created without a schema.</p>"]
    fn create_directory(&self,
                        input: &CreateDirectoryRequest)
                        -> Result<CreateDirectoryResponse, CreateDirectoryError>;


    #[doc="<p>Creates a new <a>Facet</a> in a schema. Facet creation is allowed only in development or applied schemas.</p>"]
    fn create_facet(&self,
                    input: &CreateFacetRequest)
                    -> Result<CreateFacetResponse, CreateFacetError>;


    #[doc="<p>Creates an index object. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_indexing.html\">Indexing</a> for more information.</p>"]
    fn create_index(&self,
                    input: &CreateIndexRequest)
                    -> Result<CreateIndexResponse, CreateIndexError>;


    #[doc="<p>Creates an object in a <a>Directory</a>. Additionally attaches the object to a parent, if a parent reference and <code>LinkName</code> is specified. An object is simply a collection of <a>Facet</a> attributes. You can also use this API call to create a policy object, if the facet from which you create the object is a policy facet. </p>"]
    fn create_object(&self,
                     input: &CreateObjectRequest)
                     -> Result<CreateObjectResponse, CreateObjectError>;


    #[doc="<p>Creates a new schema in a development state. A schema can exist in three phases:</p> <ul> <li> <p> <i>Development:</i> This is a mutable phase of the schema. All new schemas are in the development phase. Once the schema is finalized, it can be published.</p> </li> <li> <p> <i>Published:</i> Published schemas are immutable and have a version associated with them.</p> </li> <li> <p> <i>Applied:</i> Applied schemas are mutable in a way that allows you to add new schema facets. You can also add new, nonrequired attributes to existing schema facets. You can apply only published schemas to directories. </p> </li> </ul>"]
    fn create_schema(&self,
                     input: &CreateSchemaRequest)
                     -> Result<CreateSchemaResponse, CreateSchemaError>;


    #[doc="<p>Creates a <a>TypedLinkFacet</a>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn create_typed_link_facet
        (&self,
         input: &CreateTypedLinkFacetRequest)
         -> Result<CreateTypedLinkFacetResponse, CreateTypedLinkFacetError>;


    #[doc="<p>Deletes a directory. Only disabled directories can be deleted. A deleted directory cannot be undone. Exercise extreme caution when deleting directories.</p>"]
    fn delete_directory(&self,
                        input: &DeleteDirectoryRequest)
                        -> Result<DeleteDirectoryResponse, DeleteDirectoryError>;


    #[doc="<p>Deletes a given <a>Facet</a>. All attributes and <a>Rule</a>s that are associated with the facet will be deleted. Only development schema facets are allowed deletion.</p>"]
    fn delete_facet(&self,
                    input: &DeleteFacetRequest)
                    -> Result<DeleteFacetResponse, DeleteFacetError>;


    #[doc="<p>Deletes an object and its associated attributes. Only objects with no children and no parents can be deleted.</p>"]
    fn delete_object(&self,
                     input: &DeleteObjectRequest)
                     -> Result<DeleteObjectResponse, DeleteObjectError>;


    #[doc="<p>Deletes a given schema. Schemas in a development and published state can only be deleted. </p>"]
    fn delete_schema(&self,
                     input: &DeleteSchemaRequest)
                     -> Result<DeleteSchemaResponse, DeleteSchemaError>;


    #[doc="<p>Deletes a <a>TypedLinkFacet</a>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn delete_typed_link_facet
        (&self,
         input: &DeleteTypedLinkFacetRequest)
         -> Result<DeleteTypedLinkFacetResponse, DeleteTypedLinkFacetError>;


    #[doc="<p>Detaches the specified object from the specified index.</p>"]
    fn detach_from_index(&self,
                         input: &DetachFromIndexRequest)
                         -> Result<DetachFromIndexResponse, DetachFromIndexError>;


    #[doc="<p>Detaches a given object from the parent object. The object that is to be detached from the parent is specified by the link name.</p>"]
    fn detach_object(&self,
                     input: &DetachObjectRequest)
                     -> Result<DetachObjectResponse, DetachObjectError>;


    #[doc="<p>Detaches a policy from an object.</p>"]
    fn detach_policy(&self,
                     input: &DetachPolicyRequest)
                     -> Result<DetachPolicyResponse, DetachPolicyError>;


    #[doc="<p>Detaches a typed link from a specified source and target object. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn detach_typed_link(&self,
                         input: &DetachTypedLinkRequest)
                         -> Result<(), DetachTypedLinkError>;


    #[doc="<p>Disables the specified directory. Disabled directories cannot be read or written to. Only enabled directories can be disabled. Disabled directories may be reenabled.</p>"]
    fn disable_directory(&self,
                         input: &DisableDirectoryRequest)
                         -> Result<DisableDirectoryResponse, DisableDirectoryError>;


    #[doc="<p>Enables the specified directory. Only disabled directories can be enabled. Once enabled, the directory can then be read and written to.</p>"]
    fn enable_directory(&self,
                        input: &EnableDirectoryRequest)
                        -> Result<EnableDirectoryResponse, EnableDirectoryError>;


    #[doc="<p>Retrieves metadata about a directory.</p>"]
    fn get_directory(&self,
                     input: &GetDirectoryRequest)
                     -> Result<GetDirectoryResponse, GetDirectoryError>;


    #[doc="<p>Gets details of the <a>Facet</a>, such as facet name, attributes, <a>Rule</a>s, or <code>ObjectType</code>. You can call this on all kinds of schema facets -- published, development, or applied.</p>"]
    fn get_facet(&self, input: &GetFacetRequest) -> Result<GetFacetResponse, GetFacetError>;


    #[doc="<p>Retrieves metadata about an object.</p>"]
    fn get_object_information
        (&self,
         input: &GetObjectInformationRequest)
         -> Result<GetObjectInformationResponse, GetObjectInformationError>;


    #[doc="<p>Retrieves a JSON representation of the schema. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_schemas.html#jsonformat\">JSON Schema Format</a> for more information.</p>"]
    fn get_schema_as_json(&self,
                          input: &GetSchemaAsJsonRequest)
                          -> Result<GetSchemaAsJsonResponse, GetSchemaAsJsonError>;


    #[doc="<p>Returns the identity attribute order for a specific <a>TypedLinkFacet</a>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn get_typed_link_facet_information
        (&self,
         input: &GetTypedLinkFacetInformationRequest)
         -> Result<GetTypedLinkFacetInformationResponse, GetTypedLinkFacetInformationError>;


    #[doc="<p>Lists schemas applied to a directory.</p>"]
    fn list_applied_schema_arns
        (&self,
         input: &ListAppliedSchemaArnsRequest)
         -> Result<ListAppliedSchemaArnsResponse, ListAppliedSchemaArnsError>;


    #[doc="<p>Lists indices attached to an object.</p>"]
    fn list_attached_indices(&self,
                             input: &ListAttachedIndicesRequest)
                             -> Result<ListAttachedIndicesResponse, ListAttachedIndicesError>;


    #[doc="<p>Retrieves each Amazon Resource Name (ARN) of schemas in the development state.</p>"]
    fn list_development_schema_arns
        (&self,
         input: &ListDevelopmentSchemaArnsRequest)
         -> Result<ListDevelopmentSchemaArnsResponse, ListDevelopmentSchemaArnsError>;


    #[doc="<p>Lists directories created within an account.</p>"]
    fn list_directories(&self,
                        input: &ListDirectoriesRequest)
                        -> Result<ListDirectoriesResponse, ListDirectoriesError>;


    #[doc="<p>Retrieves attributes attached to the facet.</p>"]
    fn list_facet_attributes(&self,
                             input: &ListFacetAttributesRequest)
                             -> Result<ListFacetAttributesResponse, ListFacetAttributesError>;


    #[doc="<p>Retrieves the names of facets that exist in a schema.</p>"]
    fn list_facet_names(&self,
                        input: &ListFacetNamesRequest)
                        -> Result<ListFacetNamesResponse, ListFacetNamesError>;


    #[doc="<p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn list_incoming_typed_links
        (&self,
         input: &ListIncomingTypedLinksRequest)
         -> Result<ListIncomingTypedLinksResponse, ListIncomingTypedLinksError>;


    #[doc="<p>Lists objects attached to the specified index.</p>"]
    fn list_index(&self, input: &ListIndexRequest) -> Result<ListIndexResponse, ListIndexError>;


    #[doc="<p>Lists all attributes that are associated with an object. </p>"]
    fn list_object_attributes
        (&self,
         input: &ListObjectAttributesRequest)
         -> Result<ListObjectAttributesResponse, ListObjectAttributesError>;


    #[doc="<p>Returns a paginated list of child objects that are associated with a given object.</p>"]
    fn list_object_children(&self,
                            input: &ListObjectChildrenRequest)
                            -> Result<ListObjectChildrenResponse, ListObjectChildrenError>;


    #[doc="<p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#dirstructure\">Directory Structure</a>.</p> <p>Use this API to evaluate all parents for an object. The call returns all objects from the root of the directory up to the requested object. The API returns the number of paths based on user-defined <code>MaxResults</code>, in case there are multiple paths to the parent. The order of the paths and nodes returned is consistent among multiple API calls unless the objects are deleted or moved. Paths not leading to the directory root are ignored from the target object.</p>"]
    fn list_object_parent_paths
        (&self,
         input: &ListObjectParentPathsRequest)
         -> Result<ListObjectParentPathsResponse, ListObjectParentPathsError>;


    #[doc="<p>Lists parent objects that are associated with a given object in pagination fashion.</p>"]
    fn list_object_parents(&self,
                           input: &ListObjectParentsRequest)
                           -> Result<ListObjectParentsResponse, ListObjectParentsError>;


    #[doc="<p>Returns policies attached to an object in pagination fashion.</p>"]
    fn list_object_policies(&self,
                            input: &ListObjectPoliciesRequest)
                            -> Result<ListObjectPoliciesResponse, ListObjectPoliciesError>;


    #[doc="<p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn list_outgoing_typed_links
        (&self,
         input: &ListOutgoingTypedLinksRequest)
         -> Result<ListOutgoingTypedLinksResponse, ListOutgoingTypedLinksError>;


    #[doc="<p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>"]
    fn list_policy_attachments
        (&self,
         input: &ListPolicyAttachmentsRequest)
         -> Result<ListPolicyAttachmentsResponse, ListPolicyAttachmentsError>;


    #[doc="<p>Retrieves each published schema Amazon Resource Name (ARN).</p>"]
    fn list_published_schema_arns
        (&self,
         input: &ListPublishedSchemaArnsRequest)
         -> Result<ListPublishedSchemaArnsResponse, ListPublishedSchemaArnsError>;


    #[doc="<p>Returns tags for a resource. Tagging is currently supported only for directories with a limit of 50 tags per directory. All 50 tags are returned for a given directory with this API call.</p>"]
    fn list_tags_for_resource(&self,
                              input: &ListTagsForResourceRequest)
                              -> Result<ListTagsForResourceResponse, ListTagsForResourceError>;


    #[doc="<p>Returns a paginated list of all attribute definitions for a particular <a>TypedLinkFacet</a>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn list_typed_link_facet_attributes
        (&self,
         input: &ListTypedLinkFacetAttributesRequest)
         -> Result<ListTypedLinkFacetAttributesResponse, ListTypedLinkFacetAttributesError>;


    #[doc="<p>Returns a paginated list of <code>TypedLink</code> facet names for a particular schema. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn list_typed_link_facet_names
        (&self,
         input: &ListTypedLinkFacetNamesRequest)
         -> Result<ListTypedLinkFacetNamesResponse, ListTypedLinkFacetNamesError>;


    #[doc="<p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies\">Policies</a>.</p>"]
    fn lookup_policy(&self,
                     input: &LookupPolicyRequest)
                     -> Result<LookupPolicyResponse, LookupPolicyError>;


    #[doc="<p>Publishes a development schema with a version. If description and attributes are specified, <code>PublishSchema</code> overrides the development schema description and attributes. If not, the development schema description and attributes are used.</p>"]
    fn publish_schema(&self,
                      input: &PublishSchemaRequest)
                      -> Result<PublishSchemaResponse, PublishSchemaError>;


    #[doc="<p>Allows a schema to be updated using JSON upload. Only available for development schemas. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_schemas.html#jsonformat\">JSON Schema Format</a> for more information.</p>"]
    fn put_schema_from_json(&self,
                            input: &PutSchemaFromJsonRequest)
                            -> Result<PutSchemaFromJsonResponse, PutSchemaFromJsonError>;


    #[doc="<p>Removes the specified facet from the specified object.</p>"]
    fn remove_facet_from_object
        (&self,
         input: &RemoveFacetFromObjectRequest)
         -> Result<RemoveFacetFromObjectResponse, RemoveFacetFromObjectError>;


    #[doc="<p>An API operation for adding tags to a resource.</p>"]
    fn tag_resource(&self,
                    input: &TagResourceRequest)
                    -> Result<TagResourceResponse, TagResourceError>;


    #[doc="<p>An API operation for removing tags from a resource.</p>"]
    fn untag_resource(&self,
                      input: &UntagResourceRequest)
                      -> Result<UntagResourceResponse, UntagResourceError>;


    #[doc="<p>Does the following:</p> <ol> <li> <p>Adds new <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Updates existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Deletes existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> </ol>"]
    fn update_facet(&self,
                    input: &UpdateFacetRequest)
                    -> Result<UpdateFacetResponse, UpdateFacetError>;


    #[doc="<p>Updates a given object's attributes.</p>"]
    fn update_object_attributes
        (&self,
         input: &UpdateObjectAttributesRequest)
         -> Result<UpdateObjectAttributesResponse, UpdateObjectAttributesError>;


    #[doc="<p>Updates the schema name with a new name. Only development schema names can be updated.</p>"]
    fn update_schema(&self,
                     input: &UpdateSchemaRequest)
                     -> Result<UpdateSchemaResponse, UpdateSchemaError>;


    #[doc="<p>Updates a <a>TypedLinkFacet</a>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn update_typed_link_facet
        (&self,
         input: &UpdateTypedLinkFacetRequest)
         -> Result<UpdateTypedLinkFacetResponse, UpdateTypedLinkFacetError>;
}
/// A client for the Amazon CloudDirectory API.
pub struct CloudDirectoryClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> CloudDirectoryClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudDirectoryClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> CloudDirectory for CloudDirectoryClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Adds a new <a>Facet</a> to an object.</p>"]
    fn add_facet_to_object(&self,
                           input: &AddFacetToObjectRequest)
                           -> Result<AddFacetToObjectResponse, AddFacetToObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/facets";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<AddFacetToObjectResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AddFacetToObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Copies the input published schema into the <a>Directory</a> with the same name and version as that of the published schema .</p>"]
    fn apply_schema(&self,
                    input: &ApplySchemaRequest)
                    -> Result<ApplySchemaResponse, ApplySchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/apply";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ApplySchemaResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ApplySchemaError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Attaches an existing object to another object. An object can be accessed in two ways:</p> <ol> <li> <p>Using the path</p> </li> <li> <p>Using <code>ObjectIdentifier</code> </p> </li> </ol>"]
    fn attach_object(&self,
                     input: &AttachObjectRequest)
                     -> Result<AttachObjectResponse, AttachObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<AttachObjectResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AttachObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Attaches a policy object to a regular object. An object can have a limited number of attached policies.</p>"]
    fn attach_policy(&self,
                     input: &AttachPolicyRequest)
                     -> Result<AttachPolicyResponse, AttachPolicyError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref directory_arn) = input.directory_arn {
            request.add_header("x-amz-data-partition", &directory_arn.to_string());
        }


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<AttachPolicyResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AttachPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Attaches the specified object to the specified index.</p>"]
    fn attach_to_index(&self,
                       input: &AttachToIndexRequest)
                       -> Result<AttachToIndexResponse, AttachToIndexError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<AttachToIndexResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AttachToIndexError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Attaches a typed link to a specified source and target object. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn attach_typed_link(&self,
                         input: &AttachTypedLinkRequest)
                         -> Result<AttachTypedLinkResponse, AttachTypedLinkError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/attach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<AttachTypedLinkResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AttachTypedLinkError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Performs all the read operations in a batch. </p>"]
    fn batch_read(&self, input: &BatchReadRequest) -> Result<BatchReadResponse, BatchReadError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/batchread";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<BatchReadResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(BatchReadError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Performs all the write operations in a batch. Either all the operations succeed or none. Batch writes supports only object-related operations.</p>"]
    fn batch_write(&self,
                   input: &BatchWriteRequest)
                   -> Result<BatchWriteResponse, BatchWriteError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/batchwrite";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<BatchWriteResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(BatchWriteError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a <a>Directory</a> by copying the published schema into the directory. A directory cannot be created without a schema.</p>"]
    fn create_directory(&self,
                        input: &CreateDirectoryRequest)
                        -> Result<CreateDirectoryResponse, CreateDirectoryError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateDirectoryResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateDirectoryError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new <a>Facet</a> in a schema. Facet creation is allowed only in development or applied schemas.</p>"]
    fn create_facet(&self,
                    input: &CreateFacetRequest)
                    -> Result<CreateFacetResponse, CreateFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateFacetResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateFacetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates an index object. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_indexing.html\">Indexing</a> for more information.</p>"]
    fn create_index(&self,
                    input: &CreateIndexRequest)
                    -> Result<CreateIndexResponse, CreateIndexError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateIndexResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateIndexError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates an object in a <a>Directory</a>. Additionally attaches the object to a parent, if a parent reference and <code>LinkName</code> is specified. An object is simply a collection of <a>Facet</a> attributes. You can also use this API call to create a policy object, if the facet from which you create the object is a policy facet. </p>"]
    fn create_object(&self,
                     input: &CreateObjectRequest)
                     -> Result<CreateObjectResponse, CreateObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateObjectResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new schema in a development state. A schema can exist in three phases:</p> <ul> <li> <p> <i>Development:</i> This is a mutable phase of the schema. All new schemas are in the development phase. Once the schema is finalized, it can be published.</p> </li> <li> <p> <i>Published:</i> Published schemas are immutable and have a version associated with them.</p> </li> <li> <p> <i>Applied:</i> Applied schemas are mutable in a way that allows you to add new schema facets. You can also add new, nonrequired attributes to existing schema facets. You can apply only published schemas to directories. </p> </li> </ul>"]
    fn create_schema(&self,
                     input: &CreateSchemaRequest)
                     -> Result<CreateSchemaResponse, CreateSchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateSchemaResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateSchemaError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a <a>TypedLinkFacet</a>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn create_typed_link_facet
        (&self,
         input: &CreateTypedLinkFacetRequest)
         -> Result<CreateTypedLinkFacetResponse, CreateTypedLinkFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/create";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateTypedLinkFacetResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateTypedLinkFacetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a directory. Only disabled directories can be deleted. A deleted directory cannot be undone. Exercise extreme caution when deleting directories.</p>"]
    fn delete_directory(&self,
                        input: &DeleteDirectoryRequest)
                        -> Result<DeleteDirectoryResponse, DeleteDirectoryError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());



        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteDirectoryResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteDirectoryError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a given <a>Facet</a>. All attributes and <a>Rule</a>s that are associated with the facet will be deleted. Only development schema facets are allowed deletion.</p>"]
    fn delete_facet(&self,
                    input: &DeleteFacetRequest)
                    -> Result<DeleteFacetResponse, DeleteFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteFacetResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteFacetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes an object and its associated attributes. Only objects with no children and no parents can be deleted.</p>"]
    fn delete_object(&self,
                     input: &DeleteObjectRequest)
                     -> Result<DeleteObjectResponse, DeleteObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteObjectResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a given schema. Schemas in a development and published state can only be deleted. </p>"]
    fn delete_schema(&self,
                     input: &DeleteSchemaRequest)
                     -> Result<DeleteSchemaResponse, DeleteSchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());



        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteSchemaResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteSchemaError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a <a>TypedLinkFacet</a>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn delete_typed_link_facet
        (&self,
         input: &DeleteTypedLinkFacetRequest)
         -> Result<DeleteTypedLinkFacetResponse, DeleteTypedLinkFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteTypedLinkFacetResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteTypedLinkFacetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Detaches the specified object from the specified index.</p>"]
    fn detach_from_index(&self,
                         input: &DetachFromIndexRequest)
                         -> Result<DetachFromIndexResponse, DetachFromIndexError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DetachFromIndexResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DetachFromIndexError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Detaches a given object from the parent object. The object that is to be detached from the parent is specified by the link name.</p>"]
    fn detach_object(&self,
                     input: &DetachObjectRequest)
                     -> Result<DetachObjectResponse, DetachObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DetachObjectResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DetachObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Detaches a policy from an object.</p>"]
    fn detach_policy(&self,
                     input: &DetachPolicyRequest)
                     -> Result<DetachPolicyResponse, DetachPolicyError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DetachPolicyResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DetachPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Detaches a typed link from a specified source and target object. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn detach_typed_link(&self,
                         input: &DetachTypedLinkRequest)
                         -> Result<(), DetachTypedLinkError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/detach";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DetachTypedLinkError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Disables the specified directory. Disabled directories cannot be read or written to. Only enabled directories can be disabled. Disabled directories may be reenabled.</p>"]
    fn disable_directory(&self,
                         input: &DisableDirectoryRequest)
                         -> Result<DisableDirectoryResponse, DisableDirectoryError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/disable";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());



        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DisableDirectoryResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DisableDirectoryError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Enables the specified directory. Only disabled directories can be enabled. Once enabled, the directory can then be read and written to.</p>"]
    fn enable_directory(&self,
                        input: &EnableDirectoryRequest)
                        -> Result<EnableDirectoryResponse, EnableDirectoryError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/enable";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());



        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<EnableDirectoryResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(EnableDirectoryError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves metadata about a directory.</p>"]
    fn get_directory(&self,
                     input: &GetDirectoryRequest)
                     -> Result<GetDirectoryResponse, GetDirectoryError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/get";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());



        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetDirectoryResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDirectoryError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets details of the <a>Facet</a>, such as facet name, attributes, <a>Rule</a>s, or <code>ObjectType</code>. You can call this on all kinds of schema facets -- published, development, or applied.</p>"]
    fn get_facet(&self, input: &GetFacetRequest) -> Result<GetFacetResponse, GetFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetFacetResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetFacetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves metadata about an object.</p>"]
    fn get_object_information
        (&self,
         input: &GetObjectInformationRequest)
         -> Result<GetObjectInformationResponse, GetObjectInformationError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/information";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetObjectInformationResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetObjectInformationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves a JSON representation of the schema. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_schemas.html#jsonformat\">JSON Schema Format</a> for more information.</p>"]
    fn get_schema_as_json(&self,
                          input: &GetSchemaAsJsonRequest)
                          -> Result<GetSchemaAsJsonResponse, GetSchemaAsJsonError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/json";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());



        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetSchemaAsJsonResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetSchemaAsJsonError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns the identity attribute order for a specific <a>TypedLinkFacet</a>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn get_typed_link_facet_information
        (&self,
         input: &GetTypedLinkFacetInformationRequest)
         -> Result<GetTypedLinkFacetInformationResponse, GetTypedLinkFacetInformationError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/get";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetTypedLinkFacetInformationResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetTypedLinkFacetInformationError::from_body(String::from_utf8_lossy(&body)
                                                                     .as_ref()))
            }
        }
    }


    #[doc="<p>Lists schemas applied to a directory.</p>"]
    fn list_applied_schema_arns
        (&self,
         input: &ListAppliedSchemaArnsRequest)
         -> Result<ListAppliedSchemaArnsResponse, ListAppliedSchemaArnsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/applied";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListAppliedSchemaArnsResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListAppliedSchemaArnsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists indices attached to an object.</p>"]
    fn list_attached_indices(&self,
                             input: &ListAttachedIndicesRequest)
                             -> Result<ListAttachedIndicesResponse, ListAttachedIndicesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/indices";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListAttachedIndicesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListAttachedIndicesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves each Amazon Resource Name (ARN) of schemas in the development state.</p>"]
    fn list_development_schema_arns
        (&self,
         input: &ListDevelopmentSchemaArnsRequest)
         -> Result<ListDevelopmentSchemaArnsResponse, ListDevelopmentSchemaArnsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/development";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListDevelopmentSchemaArnsResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListDevelopmentSchemaArnsError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="<p>Lists directories created within an account.</p>"]
    fn list_directories(&self,
                        input: &ListDirectoriesRequest)
                        -> Result<ListDirectoriesResponse, ListDirectoriesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/directory/list";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListDirectoriesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListDirectoriesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves attributes attached to the facet.</p>"]
    fn list_facet_attributes(&self,
                             input: &ListFacetAttributesRequest)
                             -> Result<ListFacetAttributesResponse, ListFacetAttributesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/attributes";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListFacetAttributesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListFacetAttributesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the names of facets that exist in a schema.</p>"]
    fn list_facet_names(&self,
                        input: &ListFacetNamesRequest)
                        -> Result<ListFacetNamesResponse, ListFacetNamesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet/list";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListFacetNamesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListFacetNamesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a paginated list of all the incoming <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn list_incoming_typed_links
        (&self,
         input: &ListIncomingTypedLinksRequest)
         -> Result<ListIncomingTypedLinksResponse, ListIncomingTypedLinksError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/incoming";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListIncomingTypedLinksResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListIncomingTypedLinksError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists objects attached to the specified index.</p>"]
    fn list_index(&self, input: &ListIndexRequest) -> Result<ListIndexResponse, ListIndexError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/index/targets";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListIndexResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListIndexError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists all attributes that are associated with an object. </p>"]
    fn list_object_attributes
        (&self,
         input: &ListObjectAttributesRequest)
         -> Result<ListObjectAttributesResponse, ListObjectAttributesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/attributes";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListObjectAttributesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListObjectAttributesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a paginated list of child objects that are associated with a given object.</p>"]
    fn list_object_children(&self,
                            input: &ListObjectChildrenRequest)
                            -> Result<ListObjectChildrenResponse, ListObjectChildrenError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/children";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListObjectChildrenResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListObjectChildrenError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#dirstructure\">Directory Structure</a>.</p> <p>Use this API to evaluate all parents for an object. The call returns all objects from the root of the directory up to the requested object. The API returns the number of paths based on user-defined <code>MaxResults</code>, in case there are multiple paths to the parent. The order of the paths and nodes returned is consistent among multiple API calls unless the objects are deleted or moved. Paths not leading to the directory root are ignored from the target object.</p>"]
    fn list_object_parent_paths
        (&self,
         input: &ListObjectParentPathsRequest)
         -> Result<ListObjectParentPathsResponse, ListObjectParentPathsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/parentpaths";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListObjectParentPathsResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListObjectParentPathsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists parent objects that are associated with a given object in pagination fashion.</p>"]
    fn list_object_parents(&self,
                           input: &ListObjectParentsRequest)
                           -> Result<ListObjectParentsResponse, ListObjectParentsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/parent";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListObjectParentsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListObjectParentsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns policies attached to an object in pagination fashion.</p>"]
    fn list_object_policies(&self,
                            input: &ListObjectPoliciesRequest)
                            -> Result<ListObjectPoliciesResponse, ListObjectPoliciesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/policy";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListObjectPoliciesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListObjectPoliciesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a paginated list of all the outgoing <a>TypedLinkSpecifier</a> information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn list_outgoing_typed_links
        (&self,
         input: &ListOutgoingTypedLinksRequest)
         -> Result<ListOutgoingTypedLinksResponse, ListOutgoingTypedLinksError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/outgoing";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListOutgoingTypedLinksResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListOutgoingTypedLinksError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns all of the <code>ObjectIdentifiers</code> to which a given policy is attached.</p>"]
    fn list_policy_attachments
        (&self,
         input: &ListPolicyAttachmentsRequest)
         -> Result<ListPolicyAttachmentsResponse, ListPolicyAttachmentsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/attachment";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref consistency_level) = input.consistency_level {
            request.add_header("x-amz-consistency-level", &consistency_level.to_string());
        }
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListPolicyAttachmentsResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListPolicyAttachmentsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves each published schema Amazon Resource Name (ARN).</p>"]
    fn list_published_schema_arns
        (&self,
         input: &ListPublishedSchemaArnsRequest)
         -> Result<ListPublishedSchemaArnsResponse, ListPublishedSchemaArnsError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/published";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListPublishedSchemaArnsResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListPublishedSchemaArnsError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }


    #[doc="<p>Returns tags for a resource. Tagging is currently supported only for directories with a limit of 50 tags per directory. All 50 tags are returned for a given directory with this API call.</p>"]
    fn list_tags_for_resource(&self,
                              input: &ListTagsForResourceRequest)
                              -> Result<ListTagsForResourceResponse, ListTagsForResourceError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/tags";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListTagsForResourceResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListTagsForResourceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a paginated list of all attribute definitions for a particular <a>TypedLinkFacet</a>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn list_typed_link_facet_attributes
        (&self,
         input: &ListTypedLinkFacetAttributesRequest)
         -> Result<ListTypedLinkFacetAttributesResponse, ListTypedLinkFacetAttributesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/attributes";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListTypedLinkFacetAttributesResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListTypedLinkFacetAttributesError::from_body(String::from_utf8_lossy(&body)
                                                                     .as_ref()))
            }
        }
    }


    #[doc="<p>Returns a paginated list of <code>TypedLink</code> facet names for a particular schema. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn list_typed_link_facet_names
        (&self,
         input: &ListTypedLinkFacetNamesRequest)
         -> Result<ListTypedLinkFacetNamesResponse, ListTypedLinkFacetNamesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet/list";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListTypedLinkFacetNamesResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListTypedLinkFacetNamesError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }


    #[doc="<p>Lists all policies from the root of the <a>Directory</a> to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, it returns the <code>ObjectIdentifier</code> for such objects. If policies are present, it returns <code>ObjectIdentifier</code>, <code>policyId</code>, and <code>policyType</code>. Paths that don't lead to the root from the target object are ignored. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_key_concepts.html#policies\">Policies</a>.</p>"]
    fn lookup_policy(&self,
                     input: &LookupPolicyRequest)
                     -> Result<LookupPolicyResponse, LookupPolicyError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/policy/lookup";

        let mut request = SignedRequest::new("POST", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<LookupPolicyResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(LookupPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Publishes a development schema with a version. If description and attributes are specified, <code>PublishSchema</code> overrides the development schema description and attributes. If not, the development schema description and attributes are used.</p>"]
    fn publish_schema(&self,
                      input: &PublishSchemaRequest)
                      -> Result<PublishSchemaResponse, PublishSchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/publish";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.development_schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<PublishSchemaResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PublishSchemaError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Allows a schema to be updated using JSON upload. Only available for development schemas. See <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/cd_schemas.html#jsonformat\">JSON Schema Format</a> for more information.</p>"]
    fn put_schema_from_json(&self,
                            input: &PutSchemaFromJsonRequest)
                            -> Result<PutSchemaFromJsonResponse, PutSchemaFromJsonError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/json";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<PutSchemaFromJsonResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutSchemaFromJsonError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Removes the specified facet from the specified object.</p>"]
    fn remove_facet_from_object
        (&self,
         input: &RemoveFacetFromObjectRequest)
         -> Result<RemoveFacetFromObjectResponse, RemoveFacetFromObjectError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/facets/delete";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<RemoveFacetFromObjectResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RemoveFacetFromObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>An API operation for adding tags to a resource.</p>"]
    fn tag_resource(&self,
                    input: &TagResourceRequest)
                    -> Result<TagResourceResponse, TagResourceError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/tags/add";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<TagResourceResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(TagResourceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>An API operation for removing tags from a resource.</p>"]
    fn untag_resource(&self,
                      input: &UntagResourceRequest)
                      -> Result<UntagResourceResponse, UntagResourceError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/tags/remove";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UntagResourceResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UntagResourceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Does the following:</p> <ol> <li> <p>Adds new <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Updates existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> <li> <p>Deletes existing <code>Attributes</code>, <code>Rules</code>, or <code>ObjectTypes</code>.</p> </li> </ol>"]
    fn update_facet(&self,
                    input: &UpdateFacetRequest)
                    -> Result<UpdateFacetResponse, UpdateFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/facet";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateFacetResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateFacetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a given object's attributes.</p>"]
    fn update_object_attributes
        (&self,
         input: &UpdateObjectAttributesRequest)
         -> Result<UpdateObjectAttributesResponse, UpdateObjectAttributesError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/object/update";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.directory_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateObjectAttributesResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateObjectAttributesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates the schema name with a new name. Only development schema names can be updated.</p>"]
    fn update_schema(&self,
                     input: &UpdateSchemaRequest)
                     -> Result<UpdateSchemaResponse, UpdateSchemaError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/schema/update";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateSchemaResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateSchemaError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a <a>TypedLinkFacet</a>. For more information, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/objectsandlinks.html#typedlink\">Typed link</a>.</p>"]
    fn update_typed_link_facet
        (&self,
         input: &UpdateTypedLinkFacetRequest)
         -> Result<UpdateTypedLinkFacetResponse, UpdateTypedLinkFacetError> {
        let request_uri = "/amazonclouddirectory/2017-01-11/typedlink/facet";

        let mut request = SignedRequest::new("PUT", "clouddirectory", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);
        request.add_header("x-amz-data-partition", &input.schema_arn);


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateTypedLinkFacetResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateTypedLinkFacetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
