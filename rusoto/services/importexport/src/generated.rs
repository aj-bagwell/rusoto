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

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
#[allow(unused_imports)]
use rusoto_core::pagination::{aws_stream, Paged, PagedOutput, PagedRequest, RusotoStream};
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};
#[allow(unused_imports)]
use std::borrow::Cow;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto::xml::error::*;
use rusoto_core::proto::xml::util::{
    self as xml_util, deserialize_elements, find_start_element, skip_tree,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[cfg(feature = "deserialize_structs")]
use serde::Deserialize;
#[cfg(feature = "serialize_structs")]
use serde::Serialize;
use serde_urlencoded;
use std::str::FromStr;
use xml::EventReader;

impl ImportExportClient {
    fn new_params(&self, operation_name: &str) -> Params {
        let mut params = Params::new();

        params.put("Action", operation_name);
        params.put("Version", "2010-06-01");

        params
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

/// <p>A discrete item that contains the description and URL of an artifact (such as a PDF).</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Artifact {
    pub description: Option<String>,
    pub url: Option<String>,
}

#[allow(dead_code)]
struct ArtifactDeserializer;
impl ArtifactDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Artifact, XmlParseError> {
        deserialize_elements::<_, Artifact, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "URL" => {
                    obj.url = Some(URLDeserializer::deserialize("URL", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct ArtifactListDeserializer;
impl ArtifactListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Artifact>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ArtifactDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Input structure for the CancelJob operation.</p>
/// see [ImportExport::cancel_job]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelJobInput {
    pub api_version: Option<String>,
    pub job_id: String,
}

/// Serialize `CancelJobInput` contents to a `SignedRequest`.
struct CancelJobInputSerializer;
impl CancelJobInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CancelJobInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(&format!("{}{}", prefix, "APIVersion"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "JobId"), &obj.job_id);
    }
}

/// <p>Output structure for the CancelJob operation.</p>
/// see [ImportExport::cancel_job]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CancelJobOutput {
    pub success: Option<bool>,
}

#[allow(dead_code)]
struct CancelJobOutputDeserializer;
impl CancelJobOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CancelJobOutput, XmlParseError> {
        deserialize_elements::<_, CancelJobOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Success" => {
                    obj.success = Some(SuccessDeserializer::deserialize("Success", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct CarrierDeserializer;
impl CarrierDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Input structure for the CreateJob operation.</p>
/// see [ImportExport::create_job]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateJobInput {
    pub api_version: Option<String>,
    pub job_type: String,
    pub manifest: String,
    pub manifest_addendum: Option<String>,
    pub validate_only: bool,
}

/// Serialize `CreateJobInput` contents to a `SignedRequest`.
struct CreateJobInputSerializer;
impl CreateJobInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateJobInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(&format!("{}{}", prefix, "APIVersion"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "JobType"), &obj.job_type);
        params.put(&format!("{}{}", prefix, "Manifest"), &obj.manifest);
        if let Some(ref field_value) = obj.manifest_addendum {
            params.put(&format!("{}{}", prefix, "ManifestAddendum"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "ValidateOnly"), &obj.validate_only);
    }
}

/// <p>Output structure for the CreateJob operation.</p>
/// see [ImportExport::create_job]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateJobOutput {
    pub artifact_list: Option<Vec<Artifact>>,
    pub job_id: Option<String>,
    pub job_type: Option<String>,
    pub signature: Option<String>,
    pub signature_file_contents: Option<String>,
    pub warning_message: Option<String>,
}

#[allow(dead_code)]
struct CreateJobOutputDeserializer;
impl CreateJobOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateJobOutput, XmlParseError> {
        deserialize_elements::<_, CreateJobOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ArtifactList" => {
                    obj.artifact_list.get_or_insert(vec![]).extend(
                        ArtifactListDeserializer::deserialize("ArtifactList", stack)?,
                    );
                }
                "JobId" => {
                    obj.job_id = Some(JobIdDeserializer::deserialize("JobId", stack)?);
                }
                "JobType" => {
                    obj.job_type = Some(JobTypeDeserializer::deserialize("JobType", stack)?);
                }
                "Signature" => {
                    obj.signature = Some(SignatureDeserializer::deserialize("Signature", stack)?);
                }
                "SignatureFileContents" => {
                    obj.signature_file_contents =
                        Some(SignatureFileContentsDeserializer::deserialize(
                            "SignatureFileContents",
                            stack,
                        )?);
                }
                "WarningMessage" => {
                    obj.warning_message = Some(WarningMessageDeserializer::deserialize(
                        "WarningMessage",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct CreationDateDeserializer;
impl CreationDateDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct CurrentManifestDeserializer;
impl CurrentManifestDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct DescriptionDeserializer;
impl DescriptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct ErrorCountDeserializer;
impl ErrorCountDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct GenericStringDeserializer;
impl GenericStringDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// see [ImportExport::get_shipping_label]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetShippingLabelInput {
    pub api_version: Option<String>,
    pub city: Option<String>,
    pub company: Option<String>,
    pub country: Option<String>,
    pub job_ids: Vec<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub postal_code: Option<String>,
    pub state_or_province: Option<String>,
    pub street_1: Option<String>,
    pub street_2: Option<String>,
    pub street_3: Option<String>,
}

/// Serialize `GetShippingLabelInput` contents to a `SignedRequest`.
struct GetShippingLabelInputSerializer;
impl GetShippingLabelInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetShippingLabelInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(&format!("{}{}", prefix, "APIVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.city {
            params.put(&format!("{}{}", prefix, "city"), &field_value);
        }
        if let Some(ref field_value) = obj.company {
            params.put(&format!("{}{}", prefix, "company"), &field_value);
        }
        if let Some(ref field_value) = obj.country {
            params.put(&format!("{}{}", prefix, "country"), &field_value);
        }
        JobIdListSerializer::serialize(params, &format!("{}{}", prefix, "jobIds"), &obj.job_ids);
        if let Some(ref field_value) = obj.name {
            params.put(&format!("{}{}", prefix, "name"), &field_value);
        }
        if let Some(ref field_value) = obj.phone_number {
            params.put(&format!("{}{}", prefix, "phoneNumber"), &field_value);
        }
        if let Some(ref field_value) = obj.postal_code {
            params.put(&format!("{}{}", prefix, "postalCode"), &field_value);
        }
        if let Some(ref field_value) = obj.state_or_province {
            params.put(&format!("{}{}", prefix, "stateOrProvince"), &field_value);
        }
        if let Some(ref field_value) = obj.street_1 {
            params.put(&format!("{}{}", prefix, "street1"), &field_value);
        }
        if let Some(ref field_value) = obj.street_2 {
            params.put(&format!("{}{}", prefix, "street2"), &field_value);
        }
        if let Some(ref field_value) = obj.street_3 {
            params.put(&format!("{}{}", prefix, "street3"), &field_value);
        }
    }
}

/// see [ImportExport::get_shipping_label]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetShippingLabelOutput {
    pub shipping_label_url: Option<String>,
    pub warning: Option<String>,
}

#[allow(dead_code)]
struct GetShippingLabelOutputDeserializer;
impl GetShippingLabelOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetShippingLabelOutput, XmlParseError> {
        deserialize_elements::<_, GetShippingLabelOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ShippingLabelURL" => {
                    obj.shipping_label_url = Some(GenericStringDeserializer::deserialize(
                        "ShippingLabelURL",
                        stack,
                    )?);
                }
                "Warning" => {
                    obj.warning = Some(GenericStringDeserializer::deserialize("Warning", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Input structure for the GetStatus operation.</p>
/// see [ImportExport::get_status]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetStatusInput {
    pub api_version: Option<String>,
    pub job_id: String,
}

/// Serialize `GetStatusInput` contents to a `SignedRequest`.
struct GetStatusInputSerializer;
impl GetStatusInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetStatusInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(&format!("{}{}", prefix, "APIVersion"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "JobId"), &obj.job_id);
    }
}

/// <p>Output structure for the GetStatus operation.</p>
/// see [ImportExport::get_status]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetStatusOutput {
    pub artifact_list: Option<Vec<Artifact>>,
    pub carrier: Option<String>,
    pub creation_date: Option<String>,
    pub current_manifest: Option<String>,
    pub error_count: Option<i64>,
    pub job_id: Option<String>,
    pub job_type: Option<String>,
    pub location_code: Option<String>,
    pub location_message: Option<String>,
    pub log_bucket: Option<String>,
    pub log_key: Option<String>,
    pub progress_code: Option<String>,
    pub progress_message: Option<String>,
    pub signature: Option<String>,
    pub signature_file_contents: Option<String>,
    pub tracking_number: Option<String>,
}

#[allow(dead_code)]
struct GetStatusOutputDeserializer;
impl GetStatusOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetStatusOutput, XmlParseError> {
        deserialize_elements::<_, GetStatusOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ArtifactList" => {
                    obj.artifact_list.get_or_insert(vec![]).extend(
                        ArtifactListDeserializer::deserialize("ArtifactList", stack)?,
                    );
                }
                "Carrier" => {
                    obj.carrier = Some(CarrierDeserializer::deserialize("Carrier", stack)?);
                }
                "CreationDate" => {
                    obj.creation_date = Some(CreationDateDeserializer::deserialize(
                        "CreationDate",
                        stack,
                    )?);
                }
                "CurrentManifest" => {
                    obj.current_manifest = Some(CurrentManifestDeserializer::deserialize(
                        "CurrentManifest",
                        stack,
                    )?);
                }
                "ErrorCount" => {
                    obj.error_count =
                        Some(ErrorCountDeserializer::deserialize("ErrorCount", stack)?);
                }
                "JobId" => {
                    obj.job_id = Some(JobIdDeserializer::deserialize("JobId", stack)?);
                }
                "JobType" => {
                    obj.job_type = Some(JobTypeDeserializer::deserialize("JobType", stack)?);
                }
                "LocationCode" => {
                    obj.location_code = Some(LocationCodeDeserializer::deserialize(
                        "LocationCode",
                        stack,
                    )?);
                }
                "LocationMessage" => {
                    obj.location_message = Some(LocationMessageDeserializer::deserialize(
                        "LocationMessage",
                        stack,
                    )?);
                }
                "LogBucket" => {
                    obj.log_bucket = Some(LogBucketDeserializer::deserialize("LogBucket", stack)?);
                }
                "LogKey" => {
                    obj.log_key = Some(LogKeyDeserializer::deserialize("LogKey", stack)?);
                }
                "ProgressCode" => {
                    obj.progress_code = Some(ProgressCodeDeserializer::deserialize(
                        "ProgressCode",
                        stack,
                    )?);
                }
                "ProgressMessage" => {
                    obj.progress_message = Some(ProgressMessageDeserializer::deserialize(
                        "ProgressMessage",
                        stack,
                    )?);
                }
                "Signature" => {
                    obj.signature = Some(SignatureDeserializer::deserialize("Signature", stack)?);
                }
                "SignatureFileContents" => {
                    obj.signature_file_contents = Some(SignatureDeserializer::deserialize(
                        "SignatureFileContents",
                        stack,
                    )?);
                }
                "TrackingNumber" => {
                    obj.tracking_number = Some(TrackingNumberDeserializer::deserialize(
                        "TrackingNumber",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct IsCanceledDeserializer;
impl IsCanceledDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(bool::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct IsTruncatedDeserializer;
impl IsTruncatedDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(bool::from_str(&s).unwrap()))
    }
}
/// <p>Representation of a job returned by the ListJobs operation.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Job {
    pub creation_date: Option<String>,
    pub is_canceled: Option<bool>,
    pub job_id: Option<String>,
    pub job_type: Option<String>,
}

#[allow(dead_code)]
struct JobDeserializer;
impl JobDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Job, XmlParseError> {
        deserialize_elements::<_, Job, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CreationDate" => {
                    obj.creation_date = Some(CreationDateDeserializer::deserialize(
                        "CreationDate",
                        stack,
                    )?);
                }
                "IsCanceled" => {
                    obj.is_canceled =
                        Some(IsCanceledDeserializer::deserialize("IsCanceled", stack)?);
                }
                "JobId" => {
                    obj.job_id = Some(JobIdDeserializer::deserialize("JobId", stack)?);
                }
                "JobType" => {
                    obj.job_type = Some(JobTypeDeserializer::deserialize("JobType", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct JobIdDeserializer;
impl JobIdDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

/// Serialize `JobIdList` contents to a `SignedRequest`.
struct JobIdListSerializer;
impl JobIdListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct JobTypeDeserializer;
impl JobTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct JobsListDeserializer;
impl JobsListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Job>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(JobDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Input structure for the ListJobs operation.</p>
/// see [ImportExport::list_jobs]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobsInput {
    pub api_version: Option<String>,
    pub marker: Option<String>,
    pub max_jobs: Option<i64>,
}

impl Paged for ListJobsInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.marker)
    }
}

impl PagedRequest for ListJobsInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.marker = key;
    }
}

/// Serialize `ListJobsInput` contents to a `SignedRequest`.
struct ListJobsInputSerializer;
impl ListJobsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListJobsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(&format!("{}{}", prefix, "APIVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_jobs {
            params.put(&format!("{}{}", prefix, "MaxJobs"), &field_value);
        }
    }
}

/// <p>Output structure for the ListJobs operation.</p>
/// see [ImportExport::list_jobs]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListJobsOutput {
    pub is_truncated: Option<bool>,
    pub jobs: Option<Vec<Job>>,
}

impl Paged for ListJobsOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.jobs.as_ref()?.last()?.job_id.clone()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Owned((|| self.jobs.as_ref()?.last()?.job_id.clone())())
    }
}

impl PagedOutput for ListJobsOutput {
    type Item = Job;

    fn into_pagination_page(self) -> Vec<Job> {
        self.jobs.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.is_truncated.unwrap_or_default()
    }
}

#[allow(dead_code)]
struct ListJobsOutputDeserializer;
impl ListJobsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListJobsOutput, XmlParseError> {
        deserialize_elements::<_, ListJobsOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IsTruncated" => {
                    obj.is_truncated =
                        Some(IsTruncatedDeserializer::deserialize("IsTruncated", stack)?);
                }
                "Jobs" => {
                    obj.jobs
                        .get_or_insert(vec![])
                        .extend(JobsListDeserializer::deserialize("Jobs", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct LocationCodeDeserializer;
impl LocationCodeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct LocationMessageDeserializer;
impl LocationMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct LogBucketDeserializer;
impl LogBucketDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct LogKeyDeserializer;
impl LogKeyDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct ProgressCodeDeserializer;
impl ProgressCodeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct ProgressMessageDeserializer;
impl ProgressMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct SignatureDeserializer;
impl SignatureDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct SignatureFileContentsDeserializer;
impl SignatureFileContentsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct SuccessDeserializer;
impl SuccessDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(bool::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct TrackingNumberDeserializer;
impl TrackingNumberDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct URLDeserializer;
impl URLDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Input structure for the UpateJob operation.</p>
/// see [ImportExport::update_job]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateJobInput {
    pub api_version: Option<String>,
    pub job_id: String,
    pub job_type: String,
    pub manifest: String,
    pub validate_only: bool,
}

/// Serialize `UpdateJobInput` contents to a `SignedRequest`.
struct UpdateJobInputSerializer;
impl UpdateJobInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateJobInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(&format!("{}{}", prefix, "APIVersion"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "JobId"), &obj.job_id);
        params.put(&format!("{}{}", prefix, "JobType"), &obj.job_type);
        params.put(&format!("{}{}", prefix, "Manifest"), &obj.manifest);
        params.put(&format!("{}{}", prefix, "ValidateOnly"), &obj.validate_only);
    }
}

/// <p>Output structure for the UpateJob operation.</p>
/// see [ImportExport::update_job]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateJobOutput {
    pub artifact_list: Option<Vec<Artifact>>,
    pub success: Option<bool>,
    pub warning_message: Option<String>,
}

#[allow(dead_code)]
struct UpdateJobOutputDeserializer;
impl UpdateJobOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateJobOutput, XmlParseError> {
        deserialize_elements::<_, UpdateJobOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ArtifactList" => {
                    obj.artifact_list.get_or_insert(vec![]).extend(
                        ArtifactListDeserializer::deserialize("ArtifactList", stack)?,
                    );
                }
                "Success" => {
                    obj.success = Some(SuccessDeserializer::deserialize("Success", stack)?);
                }
                "WarningMessage" => {
                    obj.warning_message = Some(WarningMessageDeserializer::deserialize(
                        "WarningMessage",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct WarningMessageDeserializer;
impl WarningMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    /// <p>The specified job ID has been canceled and is no longer valid.</p>
    CanceledJobId(String),
    /// <p>Indicates that the specified job has expired out of the system.</p>
    ExpiredJobId(String),
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>The JOBID was missing, not found, or not associated with the AWS account.</p>
    InvalidJobId(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
    /// <p>AWS Import/Export cannot cancel the job</p>
    UnableToCancelJobId(String),
}

impl CancelJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelJobError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CanceledJobIdException" => {
                        return RusotoError::Service(CancelJobError::CanceledJobId(
                            parsed_error.message,
                        ))
                    }
                    "ExpiredJobIdException" => {
                        return RusotoError::Service(CancelJobError::ExpiredJobId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidAccessKeyIdException" => {
                        return RusotoError::Service(CancelJobError::InvalidAccessKeyId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidJobIdException" => {
                        return RusotoError::Service(CancelJobError::InvalidJobId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVersionException" => {
                        return RusotoError::Service(CancelJobError::InvalidVersion(
                            parsed_error.message,
                        ))
                    }
                    "UnableToCancelJobIdException" => {
                        return RusotoError::Service(CancelJobError::UnableToCancelJobId(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CancelJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelJobError::CanceledJobId(ref cause) => write!(f, "{}", cause),
            CancelJobError::ExpiredJobId(ref cause) => write!(f, "{}", cause),
            CancelJobError::InvalidAccessKeyId(ref cause) => write!(f, "{}", cause),
            CancelJobError::InvalidJobId(ref cause) => write!(f, "{}", cause),
            CancelJobError::InvalidVersion(ref cause) => write!(f, "{}", cause),
            CancelJobError::UnableToCancelJobId(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelJobError {}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    /// <p>The account specified does not have the appropriate bucket permissions.</p>
    BucketPermission(String),
    /// <p>Each account can create only a certain number of jobs per day. If you need to create more than this, please contact awsimportexport@amazon.com to explain your particular use case.</p>
    CreateJobQuotaExceeded(String),
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>The address specified in the manifest is invalid.</p>
    InvalidAddress(String),
    /// <p>One or more customs parameters was invalid. Please correct and resubmit.</p>
    InvalidCustoms(String),
    /// <p>File system specified in export manifest is invalid.</p>
    InvalidFileSystem(String),
    /// <p>The JOBID was missing, not found, or not associated with the AWS account.</p>
    InvalidJobId(String),
    /// <p>One or more manifest fields was invalid. Please correct and resubmit.</p>
    InvalidManifestField(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
    /// <p>Your manifest is not well-formed.</p>
    MalformedManifest(String),
    /// <p>One or more required customs parameters was missing from the manifest.</p>
    MissingCustoms(String),
    /// <p>One or more required fields were missing from the manifest file. Please correct and resubmit.</p>
    MissingManifestField(String),
    /// <p>One or more required parameters was missing from the request.</p>
    MissingParameter(String),
    /// <p>Your manifest file contained buckets from multiple regions. A job is restricted to buckets from one region. Please correct and resubmit.</p>
    MultipleRegions(String),
    /// <p>The specified bucket does not exist. Create the specified bucket or change the manifest&#39;s bucket, exportBucket, or logBucket field to a bucket that the account, as specified by the manifest&#39;s Access Key ID, has write permissions to.</p>
    NoSuchBucket(String),
}

impl CreateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJobError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BucketPermissionException" => {
                        return RusotoError::Service(CreateJobError::BucketPermission(
                            parsed_error.message,
                        ))
                    }
                    "CreateJobQuotaExceededException" => {
                        return RusotoError::Service(CreateJobError::CreateJobQuotaExceeded(
                            parsed_error.message,
                        ))
                    }
                    "InvalidAccessKeyIdException" => {
                        return RusotoError::Service(CreateJobError::InvalidAccessKeyId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidAddressException" => {
                        return RusotoError::Service(CreateJobError::InvalidAddress(
                            parsed_error.message,
                        ))
                    }
                    "InvalidCustomsException" => {
                        return RusotoError::Service(CreateJobError::InvalidCustoms(
                            parsed_error.message,
                        ))
                    }
                    "InvalidFileSystemException" => {
                        return RusotoError::Service(CreateJobError::InvalidFileSystem(
                            parsed_error.message,
                        ))
                    }
                    "InvalidJobIdException" => {
                        return RusotoError::Service(CreateJobError::InvalidJobId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidManifestFieldException" => {
                        return RusotoError::Service(CreateJobError::InvalidManifestField(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterException" => {
                        return RusotoError::Service(CreateJobError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVersionException" => {
                        return RusotoError::Service(CreateJobError::InvalidVersion(
                            parsed_error.message,
                        ))
                    }
                    "MalformedManifestException" => {
                        return RusotoError::Service(CreateJobError::MalformedManifest(
                            parsed_error.message,
                        ))
                    }
                    "MissingCustomsException" => {
                        return RusotoError::Service(CreateJobError::MissingCustoms(
                            parsed_error.message,
                        ))
                    }
                    "MissingManifestFieldException" => {
                        return RusotoError::Service(CreateJobError::MissingManifestField(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameterException" => {
                        return RusotoError::Service(CreateJobError::MissingParameter(
                            parsed_error.message,
                        ))
                    }
                    "MultipleRegionsException" => {
                        return RusotoError::Service(CreateJobError::MultipleRegions(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchBucketException" => {
                        return RusotoError::Service(CreateJobError::NoSuchBucket(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateJobError::BucketPermission(ref cause) => write!(f, "{}", cause),
            CreateJobError::CreateJobQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateJobError::InvalidAccessKeyId(ref cause) => write!(f, "{}", cause),
            CreateJobError::InvalidAddress(ref cause) => write!(f, "{}", cause),
            CreateJobError::InvalidCustoms(ref cause) => write!(f, "{}", cause),
            CreateJobError::InvalidFileSystem(ref cause) => write!(f, "{}", cause),
            CreateJobError::InvalidJobId(ref cause) => write!(f, "{}", cause),
            CreateJobError::InvalidManifestField(ref cause) => write!(f, "{}", cause),
            CreateJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateJobError::InvalidVersion(ref cause) => write!(f, "{}", cause),
            CreateJobError::MalformedManifest(ref cause) => write!(f, "{}", cause),
            CreateJobError::MissingCustoms(ref cause) => write!(f, "{}", cause),
            CreateJobError::MissingManifestField(ref cause) => write!(f, "{}", cause),
            CreateJobError::MissingParameter(ref cause) => write!(f, "{}", cause),
            CreateJobError::MultipleRegions(ref cause) => write!(f, "{}", cause),
            CreateJobError::NoSuchBucket(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateJobError {}
/// Errors returned by GetShippingLabel
#[derive(Debug, PartialEq)]
pub enum GetShippingLabelError {
    /// <p>The specified job ID has been canceled and is no longer valid.</p>
    CanceledJobId(String),
    /// <p>Indicates that the specified job has expired out of the system.</p>
    ExpiredJobId(String),
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>The address specified in the manifest is invalid.</p>
    InvalidAddress(String),
    /// <p>The JOBID was missing, not found, or not associated with the AWS account.</p>
    InvalidJobId(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
}

impl GetShippingLabelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetShippingLabelError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CanceledJobIdException" => {
                        return RusotoError::Service(GetShippingLabelError::CanceledJobId(
                            parsed_error.message,
                        ))
                    }
                    "ExpiredJobIdException" => {
                        return RusotoError::Service(GetShippingLabelError::ExpiredJobId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidAccessKeyIdException" => {
                        return RusotoError::Service(GetShippingLabelError::InvalidAccessKeyId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidAddressException" => {
                        return RusotoError::Service(GetShippingLabelError::InvalidAddress(
                            parsed_error.message,
                        ))
                    }
                    "InvalidJobIdException" => {
                        return RusotoError::Service(GetShippingLabelError::InvalidJobId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterException" => {
                        return RusotoError::Service(GetShippingLabelError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVersionException" => {
                        return RusotoError::Service(GetShippingLabelError::InvalidVersion(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetShippingLabelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetShippingLabelError::CanceledJobId(ref cause) => write!(f, "{}", cause),
            GetShippingLabelError::ExpiredJobId(ref cause) => write!(f, "{}", cause),
            GetShippingLabelError::InvalidAccessKeyId(ref cause) => write!(f, "{}", cause),
            GetShippingLabelError::InvalidAddress(ref cause) => write!(f, "{}", cause),
            GetShippingLabelError::InvalidJobId(ref cause) => write!(f, "{}", cause),
            GetShippingLabelError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetShippingLabelError::InvalidVersion(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetShippingLabelError {}
/// Errors returned by GetStatus
#[derive(Debug, PartialEq)]
pub enum GetStatusError {
    /// <p>The specified job ID has been canceled and is no longer valid.</p>
    CanceledJobId(String),
    /// <p>Indicates that the specified job has expired out of the system.</p>
    ExpiredJobId(String),
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>The JOBID was missing, not found, or not associated with the AWS account.</p>
    InvalidJobId(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
}

impl GetStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetStatusError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CanceledJobIdException" => {
                        return RusotoError::Service(GetStatusError::CanceledJobId(
                            parsed_error.message,
                        ))
                    }
                    "ExpiredJobIdException" => {
                        return RusotoError::Service(GetStatusError::ExpiredJobId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidAccessKeyIdException" => {
                        return RusotoError::Service(GetStatusError::InvalidAccessKeyId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidJobIdException" => {
                        return RusotoError::Service(GetStatusError::InvalidJobId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVersionException" => {
                        return RusotoError::Service(GetStatusError::InvalidVersion(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetStatusError::CanceledJobId(ref cause) => write!(f, "{}", cause),
            GetStatusError::ExpiredJobId(ref cause) => write!(f, "{}", cause),
            GetStatusError::InvalidAccessKeyId(ref cause) => write!(f, "{}", cause),
            GetStatusError::InvalidJobId(ref cause) => write!(f, "{}", cause),
            GetStatusError::InvalidVersion(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetStatusError {}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidAccessKeyIdException" => {
                        return RusotoError::Service(ListJobsError::InvalidAccessKeyId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterException" => {
                        return RusotoError::Service(ListJobsError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVersionException" => {
                        return RusotoError::Service(ListJobsError::InvalidVersion(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListJobsError::InvalidAccessKeyId(ref cause) => write!(f, "{}", cause),
            ListJobsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListJobsError::InvalidVersion(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobsError {}
/// Errors returned by UpdateJob
#[derive(Debug, PartialEq)]
pub enum UpdateJobError {
    /// <p>The account specified does not have the appropriate bucket permissions.</p>
    BucketPermission(String),
    /// <p>The specified job ID has been canceled and is no longer valid.</p>
    CanceledJobId(String),
    /// <p>Indicates that the specified job has expired out of the system.</p>
    ExpiredJobId(String),
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>The address specified in the manifest is invalid.</p>
    InvalidAddress(String),
    /// <p>One or more customs parameters was invalid. Please correct and resubmit.</p>
    InvalidCustoms(String),
    /// <p>File system specified in export manifest is invalid.</p>
    InvalidFileSystem(String),
    /// <p>The JOBID was missing, not found, or not associated with the AWS account.</p>
    InvalidJobId(String),
    /// <p>One or more manifest fields was invalid. Please correct and resubmit.</p>
    InvalidManifestField(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
    /// <p>Your manifest is not well-formed.</p>
    MalformedManifest(String),
    /// <p>One or more required customs parameters was missing from the manifest.</p>
    MissingCustoms(String),
    /// <p>One or more required fields were missing from the manifest file. Please correct and resubmit.</p>
    MissingManifestField(String),
    /// <p>One or more required parameters was missing from the request.</p>
    MissingParameter(String),
    /// <p>Your manifest file contained buckets from multiple regions. A job is restricted to buckets from one region. Please correct and resubmit.</p>
    MultipleRegions(String),
    /// <p>The specified bucket does not exist. Create the specified bucket or change the manifest&#39;s bucket, exportBucket, or logBucket field to a bucket that the account, as specified by the manifest&#39;s Access Key ID, has write permissions to.</p>
    NoSuchBucket(String),
    /// <p>AWS Import/Export cannot update the job</p>
    UnableToUpdateJobId(String),
}

impl UpdateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateJobError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BucketPermissionException" => {
                        return RusotoError::Service(UpdateJobError::BucketPermission(
                            parsed_error.message,
                        ))
                    }
                    "CanceledJobIdException" => {
                        return RusotoError::Service(UpdateJobError::CanceledJobId(
                            parsed_error.message,
                        ))
                    }
                    "ExpiredJobIdException" => {
                        return RusotoError::Service(UpdateJobError::ExpiredJobId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidAccessKeyIdException" => {
                        return RusotoError::Service(UpdateJobError::InvalidAccessKeyId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidAddressException" => {
                        return RusotoError::Service(UpdateJobError::InvalidAddress(
                            parsed_error.message,
                        ))
                    }
                    "InvalidCustomsException" => {
                        return RusotoError::Service(UpdateJobError::InvalidCustoms(
                            parsed_error.message,
                        ))
                    }
                    "InvalidFileSystemException" => {
                        return RusotoError::Service(UpdateJobError::InvalidFileSystem(
                            parsed_error.message,
                        ))
                    }
                    "InvalidJobIdException" => {
                        return RusotoError::Service(UpdateJobError::InvalidJobId(
                            parsed_error.message,
                        ))
                    }
                    "InvalidManifestFieldException" => {
                        return RusotoError::Service(UpdateJobError::InvalidManifestField(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterException" => {
                        return RusotoError::Service(UpdateJobError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVersionException" => {
                        return RusotoError::Service(UpdateJobError::InvalidVersion(
                            parsed_error.message,
                        ))
                    }
                    "MalformedManifestException" => {
                        return RusotoError::Service(UpdateJobError::MalformedManifest(
                            parsed_error.message,
                        ))
                    }
                    "MissingCustomsException" => {
                        return RusotoError::Service(UpdateJobError::MissingCustoms(
                            parsed_error.message,
                        ))
                    }
                    "MissingManifestFieldException" => {
                        return RusotoError::Service(UpdateJobError::MissingManifestField(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameterException" => {
                        return RusotoError::Service(UpdateJobError::MissingParameter(
                            parsed_error.message,
                        ))
                    }
                    "MultipleRegionsException" => {
                        return RusotoError::Service(UpdateJobError::MultipleRegions(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchBucketException" => {
                        return RusotoError::Service(UpdateJobError::NoSuchBucket(
                            parsed_error.message,
                        ))
                    }
                    "UnableToUpdateJobIdException" => {
                        return RusotoError::Service(UpdateJobError::UnableToUpdateJobId(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateJobError::BucketPermission(ref cause) => write!(f, "{}", cause),
            UpdateJobError::CanceledJobId(ref cause) => write!(f, "{}", cause),
            UpdateJobError::ExpiredJobId(ref cause) => write!(f, "{}", cause),
            UpdateJobError::InvalidAccessKeyId(ref cause) => write!(f, "{}", cause),
            UpdateJobError::InvalidAddress(ref cause) => write!(f, "{}", cause),
            UpdateJobError::InvalidCustoms(ref cause) => write!(f, "{}", cause),
            UpdateJobError::InvalidFileSystem(ref cause) => write!(f, "{}", cause),
            UpdateJobError::InvalidJobId(ref cause) => write!(f, "{}", cause),
            UpdateJobError::InvalidManifestField(ref cause) => write!(f, "{}", cause),
            UpdateJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateJobError::InvalidVersion(ref cause) => write!(f, "{}", cause),
            UpdateJobError::MalformedManifest(ref cause) => write!(f, "{}", cause),
            UpdateJobError::MissingCustoms(ref cause) => write!(f, "{}", cause),
            UpdateJobError::MissingManifestField(ref cause) => write!(f, "{}", cause),
            UpdateJobError::MissingParameter(ref cause) => write!(f, "{}", cause),
            UpdateJobError::MultipleRegions(ref cause) => write!(f, "{}", cause),
            UpdateJobError::NoSuchBucket(ref cause) => write!(f, "{}", cause),
            UpdateJobError::UnableToUpdateJobId(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateJobError {}
/// Trait representing the capabilities of the AWS Import/Export API. AWS Import/Export clients implement this trait.
#[async_trait]
pub trait ImportExport: Clone + Sync + Send + 'static {
    /// <p>This operation cancels a specified job. Only the job owner can cancel it. The operation fails if the job has already started or is complete.</p>
    async fn cancel_job(
        &self,
        input: CancelJobInput,
    ) -> Result<CancelJobOutput, RusotoError<CancelJobError>>;

    /// <p>This operation initiates the process of scheduling an upload or download of your data. You include in the request a manifest that describes the data transfer specifics. The response to the request includes a job ID, which you can use in other operations, a signature that you use to identify your storage device, and the address where you should ship your storage device.</p>
    async fn create_job(
        &self,
        input: CreateJobInput,
    ) -> Result<CreateJobOutput, RusotoError<CreateJobError>>;

    /// <p>This operation generates a pre-paid UPS shipping label that you will use to ship your device to AWS for processing.</p>
    async fn get_shipping_label(
        &self,
        input: GetShippingLabelInput,
    ) -> Result<GetShippingLabelOutput, RusotoError<GetShippingLabelError>>;

    /// <p>This operation returns information about a job, including where the job is in the processing pipeline, the status of the results, and the signature value associated with the job. You can only return information about jobs you own.</p>
    async fn get_status(
        &self,
        input: GetStatusInput,
    ) -> Result<GetStatusOutput, RusotoError<GetStatusError>>;

    /// <p>This operation returns the jobs associated with the requester. AWS Import/Export lists the jobs in reverse chronological order based on the date of creation. For example if Job Test1 was created 2009Dec30 and Test2 was created 2010Feb05, the ListJobs operation would return Test2 followed by Test1.</p>
    async fn list_jobs(
        &self,
        input: ListJobsInput,
    ) -> Result<ListJobsOutput, RusotoError<ListJobsError>>;

    /// Auto-paginating version of `list_jobs`
    fn list_jobs_pages<'a>(
        &'a self,
        mut input: ListJobsInput,
    ) -> RusotoStream<'a, Job, ListJobsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_jobs(input.clone())
        }))
    }

    /// <p>You use this operation to change the parameters specified in the original manifest file by supplying a new manifest file. The manifest file attached to this request replaces the original manifest file. You can only use the operation after a CreateJob request but before the data transfer starts and you can only use it on jobs you own.</p>
    async fn update_job(
        &self,
        input: UpdateJobInput,
    ) -> Result<UpdateJobOutput, RusotoError<UpdateJobError>>;
}
/// A client for the AWS Import/Export API.
#[derive(Clone)]
pub struct ImportExportClient {
    client: Client,
    region: region::Region,
}

impl ImportExportClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ImportExportClient {
        ImportExportClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ImportExportClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ImportExportClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ImportExportClient {
        ImportExportClient { client, region }
    }
}

#[async_trait]
impl ImportExport for ImportExportClient {
    /// <p>This operation cancels a specified job. Only the job owner can cancel it. The operation fails if the job has already started or is complete.</p>
    async fn cancel_job(
        &self,
        input: CancelJobInput,
    ) -> Result<CancelJobOutput, RusotoError<CancelJobError>> {
        let mut request = SignedRequest::new(
            "POST",
            "importexport",
            &self.region,
            "/?Operation=CancelJob",
        );
        let params = self.new_params("CancelJob");
        let mut params = params;
        CancelJobInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CancelJobError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CancelJobOutputDeserializer::deserialize("CancelJobResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>This operation initiates the process of scheduling an upload or download of your data. You include in the request a manifest that describes the data transfer specifics. The response to the request includes a job ID, which you can use in other operations, a signature that you use to identify your storage device, and the address where you should ship your storage device.</p>
    async fn create_job(
        &self,
        input: CreateJobInput,
    ) -> Result<CreateJobOutput, RusotoError<CreateJobError>> {
        let mut request = SignedRequest::new(
            "POST",
            "importexport",
            &self.region,
            "/?Operation=CreateJob",
        );
        let params = self.new_params("CreateJob");
        let mut params = params;
        CreateJobInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateJobError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CreateJobOutputDeserializer::deserialize("CreateJobResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>This operation generates a pre-paid UPS shipping label that you will use to ship your device to AWS for processing.</p>
    async fn get_shipping_label(
        &self,
        input: GetShippingLabelInput,
    ) -> Result<GetShippingLabelOutput, RusotoError<GetShippingLabelError>> {
        let mut request = SignedRequest::new(
            "POST",
            "importexport",
            &self.region,
            "/?Operation=GetShippingLabel",
        );
        let params = self.new_params("GetShippingLabel");
        let mut params = params;
        GetShippingLabelInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, GetShippingLabelError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result =
                GetShippingLabelOutputDeserializer::deserialize("GetShippingLabelResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>This operation returns information about a job, including where the job is in the processing pipeline, the status of the results, and the signature value associated with the job. You can only return information about jobs you own.</p>
    async fn get_status(
        &self,
        input: GetStatusInput,
    ) -> Result<GetStatusOutput, RusotoError<GetStatusError>> {
        let mut request = SignedRequest::new(
            "POST",
            "importexport",
            &self.region,
            "/?Operation=GetStatus",
        );
        let params = self.new_params("GetStatus");
        let mut params = params;
        GetStatusInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, GetStatusError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = GetStatusOutputDeserializer::deserialize("GetStatusResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>This operation returns the jobs associated with the requester. AWS Import/Export lists the jobs in reverse chronological order based on the date of creation. For example if Job Test1 was created 2009Dec30 and Test2 was created 2010Feb05, the ListJobs operation would return Test2 followed by Test1.</p>
    async fn list_jobs(
        &self,
        input: ListJobsInput,
    ) -> Result<ListJobsOutput, RusotoError<ListJobsError>> {
        let mut request =
            SignedRequest::new("POST", "importexport", &self.region, "/?Operation=ListJobs");
        let params = self.new_params("ListJobs");
        let mut params = params;
        ListJobsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ListJobsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ListJobsOutputDeserializer::deserialize("ListJobsResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>You use this operation to change the parameters specified in the original manifest file by supplying a new manifest file. The manifest file attached to this request replaces the original manifest file. You can only use the operation after a CreateJob request but before the data transfer starts and you can only use it on jobs you own.</p>
    async fn update_job(
        &self,
        input: UpdateJobInput,
    ) -> Result<UpdateJobOutput, RusotoError<UpdateJobError>> {
        let mut request = SignedRequest::new(
            "POST",
            "importexport",
            &self.region,
            "/?Operation=UpdateJob",
        );
        let params = self.new_params("UpdateJob");
        let mut params = params;
        UpdateJobInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, UpdateJobError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = UpdateJobOutputDeserializer::deserialize("UpdateJobResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[tokio::test]
    async fn test_parse_error_importexport_get_status() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "importexport-get-status.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client =
            ImportExportClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetStatusInput::default();
        let result = client.get_status(request).await;
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_importexport_list_jobs() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "importexport-list-jobs.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ImportExportClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListJobsInput::default();
        let result = client.list_jobs(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
