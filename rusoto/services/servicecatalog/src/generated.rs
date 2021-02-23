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

use rusoto_core::proto;
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl ServiceCatalogClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "servicecatalog", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
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

use serde_json;
/// see [ServiceCatalog::accept_portfolio_share]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptPortfolioShareInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The type of shared portfolios to accept. The default is to accept imported portfolios.</p> <ul> <li> <p> <code>AWS_ORGANIZATIONS</code> - Accept portfolios shared by the management account of your organization.</p> </li> <li> <p> <code>IMPORTED</code> - Accept imported portfolios.</p> </li> <li> <p> <code>AWS_SERVICECATALOG</code> - Not supported. (Throws ResourceNotFoundException.)</p> </li> </ul> <p>For example, <code>aws servicecatalog accept-portfolio-share --portfolio-id "port-2qwzkwxt3y5fk" --portfolio-share-type AWS_ORGANIZATIONS</code> </p>
    #[serde(rename = "PortfolioShareType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_type: Option<String>,
}

/// see [ServiceCatalog::accept_portfolio_share]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptPortfolioShareOutput {}

/// <p>The access level to use to filter results.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AccessLevelFilter {
    /// <p><p>The access level.</p> <ul> <li> <p> <code>Account</code> - Filter results based on the account.</p> </li> <li> <p> <code>Role</code> - Filter results based on the federated role of the specified user.</p> </li> <li> <p> <code>User</code> - Filter results based on the specified user.</p> </li> </ul></p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The user to which the access level applies. The only supported value is <code>Self</code>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// see [ServiceCatalog::associate_budget_with_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateBudgetWithResourceInput {
    /// <p>The name of the budget you want to associate.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p> The resource identifier. Either a portfolio-id or a product-id.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

/// see [ServiceCatalog::associate_budget_with_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateBudgetWithResourceOutput {}

/// see [ServiceCatalog::associate_principal_with_portfolio]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociatePrincipalWithPortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The ARN of the principal (IAM user, role, or group).</p>
    #[serde(rename = "PrincipalARN")]
    pub principal_arn: String,
    /// <p>The principal type. The supported value is <code>IAM</code>.</p>
    #[serde(rename = "PrincipalType")]
    pub principal_type: String,
}

/// see [ServiceCatalog::associate_principal_with_portfolio]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociatePrincipalWithPortfolioOutput {}

/// see [ServiceCatalog::associate_product_with_portfolio]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateProductWithPortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the source portfolio.</p>
    #[serde(rename = "SourcePortfolioId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_portfolio_id: Option<String>,
}

/// see [ServiceCatalog::associate_product_with_portfolio]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateProductWithPortfolioOutput {}

/// see [ServiceCatalog::associate_service_action_with_provisioning_artifact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateServiceActionWithProvisioningArtifactInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
}

/// see [ServiceCatalog::associate_service_action_with_provisioning_artifact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateServiceActionWithProvisioningArtifactOutput {}

/// see [ServiceCatalog::associate_tag_option_with_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateTagOptionWithResourceInput {
    /// <p>The resource identifier.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "TagOptionId")]
    pub tag_option_id: String,
}

/// see [ServiceCatalog::associate_tag_option_with_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateTagOptionWithResourceOutput {}

/// see [ServiceCatalog::batch_associate_service_action_with_provisioning_artifact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchAssociateServiceActionWithProvisioningArtifactInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>One or more associations, each consisting of the Action ID, the Product ID, and the Provisioning Artifact ID.</p>
    #[serde(rename = "ServiceActionAssociations")]
    pub service_action_associations: Vec<ServiceActionAssociation>,
}

/// see [ServiceCatalog::batch_associate_service_action_with_provisioning_artifact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchAssociateServiceActionWithProvisioningArtifactOutput {
    /// <p>An object that contains a list of errors, along with information to help you identify the self-service action.</p>
    #[serde(rename = "FailedServiceActionAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_service_action_associations: Option<Vec<FailedServiceActionAssociation>>,
}

/// see [ServiceCatalog::batch_disassociate_service_action_from_provisioning_artifact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDisassociateServiceActionFromProvisioningArtifactInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>One or more associations, each consisting of the Action ID, the Product ID, and the Provisioning Artifact ID.</p>
    #[serde(rename = "ServiceActionAssociations")]
    pub service_action_associations: Vec<ServiceActionAssociation>,
}

/// see [ServiceCatalog::batch_disassociate_service_action_from_provisioning_artifact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDisassociateServiceActionFromProvisioningArtifactOutput {
    /// <p>An object that contains a list of errors, along with information to help you identify the self-service action.</p>
    #[serde(rename = "FailedServiceActionAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_service_action_associations: Option<Vec<FailedServiceActionAssociation>>,
}

/// <p>Information about a budget.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BudgetDetail {
    /// <p>Name of the associated budget.</p>
    #[serde(rename = "BudgetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
}

/// <p>Information about a CloudWatch dashboard.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CloudWatchDashboard {
    /// <p>The name of the CloudWatch dashboard.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a constraint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConstraintDetail {
    /// <p>The identifier of the constraint.</p>
    #[serde(rename = "ConstraintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_id: Option<String>,
    /// <p>The description of the constraint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The owner of the constraint.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The identifier of the portfolio the product resides in. The constraint applies only to the instance of the product that lives within this portfolio.</p>
    #[serde(rename = "PortfolioId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_id: Option<String>,
    /// <p>The identifier of the product the constraint applies to. Note that a constraint applies to a specific instance of a product within a certain portfolio.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p><p>The type of constraint.</p> <ul> <li> <p> <code>LAUNCH</code> </p> </li> <li> <p> <code>NOTIFICATION</code> </p> </li> <li> <p>STACKSET</p> </li> <li> <p> <code>TEMPLATE</code> </p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Summary information about a constraint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConstraintSummary {
    /// <p>The description of the constraint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The type of constraint.</p> <ul> <li> <p> <code>LAUNCH</code> </p> </li> <li> <p> <code>NOTIFICATION</code> </p> </li> <li> <p>STACKSET</p> </li> <li> <p> <code>TEMPLATE</code> </p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// see [ServiceCatalog::copy_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CopyProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The copy options. If the value is <code>CopyTags</code>, the tags from the source product are copied to the target product.</p>
    #[serde(rename = "CopyOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_options: Option<Vec<String>>,
    /// <p> A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request. </p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The Amazon Resource Name (ARN) of the source product.</p>
    #[serde(rename = "SourceProductArn")]
    pub source_product_arn: String,
    /// <p>The identifiers of the provisioning artifacts (also known as versions) of the product to copy. By default, all provisioning artifacts are copied.</p>
    #[serde(rename = "SourceProvisioningArtifactIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provisioning_artifact_identifiers:
        Option<Vec<::std::collections::HashMap<String, String>>>,
    /// <p>The identifier of the target product. By default, a new product is created.</p>
    #[serde(rename = "TargetProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_product_id: Option<String>,
    /// <p>A name for the target product. The default is the name of the source product.</p>
    #[serde(rename = "TargetProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_product_name: Option<String>,
}

/// see [ServiceCatalog::copy_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CopyProductOutput {
    /// <p>The token to use to track the progress of the operation.</p>
    #[serde(rename = "CopyProductToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_product_token: Option<String>,
}

/// see [ServiceCatalog::create_constraint]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConstraintInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The description of the constraint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p><p>The constraint parameters, in JSON format. The syntax depends on the constraint type as follows:</p> <dl> <dt>LAUNCH</dt> <dd> <p>You are required to specify either the <code>RoleArn</code> or the <code>LocalRoleName</code> but can&#39;t use both.</p> <p>Specify the <code>RoleArn</code> property as follows:</p> <p> <code>{&quot;RoleArn&quot; : &quot;arn:aws:iam::123456789012:role/LaunchRole&quot;}</code> </p> <p>Specify the <code>LocalRoleName</code> property as follows:</p> <p> <code>{&quot;LocalRoleName&quot;: &quot;SCBasicLaunchRole&quot;}</code> </p> <p>If you specify the <code>LocalRoleName</code> property, when an account uses the launch constraint, the IAM role with that name in the account will be used. This allows launch-role constraints to be account-agnostic so the administrator can create fewer resources per shared account.</p> <note> <p>The given role name must exist in the account used to create the launch constraint and the account of the user who launches a product with this launch constraint.</p> </note> <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p> <p>You also cannot have more than one <code>LAUNCH</code> constraint on a product and portfolio.</p> </dd> <dt>NOTIFICATION</dt> <dd> <p>Specify the <code>NotificationArns</code> property as follows:</p> <p> <code>{&quot;NotificationArns&quot; : [&quot;arn:aws:sns:us-east-1:123456789012:Topic&quot;]}</code> </p> </dd> <dt>RESOURCE<em>UPDATE</dt> <dd> <p>Specify the <code>TagUpdatesOnProvisionedProduct</code> property as follows:</p> <p> <code>{&quot;Version&quot;:&quot;2.0&quot;,&quot;Properties&quot;:{&quot;TagUpdateOnProvisionedProduct&quot;:&quot;String&quot;}}</code> </p> <p>The <code>TagUpdatesOnProvisionedProduct</code> property accepts a string value of <code>ALLOWED</code> or <code>NOT</em>ALLOWED</code>.</p> </dd> <dt>STACKSET</dt> <dd> <p>Specify the <code>Parameters</code> property as follows:</p> <p> <code>{&quot;Version&quot;: &quot;String&quot;, &quot;Properties&quot;: {&quot;AccountList&quot;: [ &quot;String&quot; ], &quot;RegionList&quot;: [ &quot;String&quot; ], &quot;AdminRole&quot;: &quot;String&quot;, &quot;ExecutionRole&quot;: &quot;String&quot;}}</code> </p> <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p> <p>You also cannot have more than one <code>STACKSET</code> constraint on a product and portfolio.</p> <p>Products with a <code>STACKSET</code> constraint will launch an AWS CloudFormation stack set.</p> </dd> <dt>TEMPLATE</dt> <dd> <p>Specify the <code>Rules</code> property. For more information, see <a href="http://docs.aws.amazon.com/servicecatalog/latest/adminguide/reference-template_constraint_rules.html">Template Constraint Rules</a>.</p> </dd> </dl></p>
    #[serde(rename = "Parameters")]
    pub parameters: String,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p><p>The type of constraint.</p> <ul> <li> <p> <code>LAUNCH</code> </p> </li> <li> <p> <code>NOTIFICATION</code> </p> </li> <li> <p> <code>RESOURCE_UPDATE</code> </p> </li> <li> <p> <code>STACKSET</code> </p> </li> <li> <p> <code>TEMPLATE</code> </p> </li> </ul></p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// see [ServiceCatalog::create_constraint]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConstraintOutput {
    /// <p>Information about the constraint.</p>
    #[serde(rename = "ConstraintDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    /// <p>The constraint parameters.</p>
    #[serde(rename = "ConstraintParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_parameters: Option<String>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ServiceCatalog::create_portfolio]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The description of the portfolio.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name to use for display purposes.</p>
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The name of the portfolio provider.</p>
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [ServiceCatalog::create_portfolio]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePortfolioOutput {
    /// <p>Information about the portfolio.</p>
    #[serde(rename = "PortfolioDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    /// <p>Information about the tags associated with the portfolio.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [ServiceCatalog::create_portfolio_share]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePortfolioShareInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The AWS account ID. For example, <code>123456789012</code>.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The organization node to whom you are going to share. If <code>OrganizationNode</code> is passed in, <code>PortfolioShare</code> will be created for the node an ListOrganizationPortfolioAccessd its children (when applies), and a <code>PortfolioShareToken</code> will be returned in the output in order for the administrator to monitor the status of the <code>PortfolioShare</code> creation process.</p>
    #[serde(rename = "OrganizationNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_node: Option<OrganizationNode>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>Enables or disables <code>TagOptions </code> sharing when creating the portfolio share. If this flag is not provided, TagOptions sharing is disabled.</p>
    #[serde(rename = "ShareTagOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_tag_options: Option<bool>,
}

/// see [ServiceCatalog::create_portfolio_share]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePortfolioShareOutput {
    /// <p>The portfolio shares a unique identifier that only returns if the portfolio is shared to an organization node.</p>
    #[serde(rename = "PortfolioShareToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_token: Option<String>,
}

/// see [ServiceCatalog::create_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The description of the product.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The distributor of the product.</p>
    #[serde(rename = "Distributor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The name of the product.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The owner of the product.</p>
    #[serde(rename = "Owner")]
    pub owner: String,
    /// <p>The type of product.</p>
    #[serde(rename = "ProductType")]
    pub product_type: String,
    /// <p>The configuration of the provisioning artifact. </p>
    #[serde(rename = "ProvisioningArtifactParameters")]
    pub provisioning_artifact_parameters: ProvisioningArtifactProperties,
    /// <p>The support information about the product.</p>
    #[serde(rename = "SupportDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_description: Option<String>,
    /// <p>The contact email for product support.</p>
    #[serde(rename = "SupportEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// <p>The contact URL for product support.</p> <p> <code>^https?:\/\// </code>/ is the pattern used to validate SupportUrl.</p>
    #[serde(rename = "SupportUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [ServiceCatalog::create_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProductOutput {
    /// <p>Information about the product view.</p>
    #[serde(rename = "ProductViewDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    /// <p>Information about the provisioning artifact. </p>
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    /// <p>Information about the tags associated with the product.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [ServiceCatalog::create_provisioned_product_plan]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProvisionedProductPlanInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.</p>
    #[serde(rename = "NotificationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    /// <p>The path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use <a>ListLaunchPaths</a>.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The name of the plan.</p>
    #[serde(rename = "PlanName")]
    pub plan_name: String,
    /// <p>The plan type.</p>
    #[serde(rename = "PlanType")]
    pub plan_type: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>A user-friendly name for the provisioned product. This value must be unique for the AWS account and cannot be updated after the product is provisioned.</p>
    #[serde(rename = "ProvisionedProductName")]
    pub provisioned_product_name: String,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
    /// <p>Parameters specified by the administrator that are required for provisioning the product.</p>
    #[serde(rename = "ProvisioningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<UpdateProvisioningParameter>>,
    /// <p>One or more tags.</p> <p>If the plan is for an existing provisioned product, the product must have a <code>RESOURCE_UPDATE</code> constraint with <code>TagUpdatesOnProvisionedProduct</code> set to <code>ALLOWED</code> to allow tag updates.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [ServiceCatalog::create_provisioned_product_plan]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProvisionedProductPlanOutput {
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// <p>The name of the plan.</p>
    #[serde(rename = "PlanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProvisionProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
}

/// see [ServiceCatalog::create_provisioning_artifact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProvisioningArtifactInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The configuration for the provisioning artifact.</p>
    #[serde(rename = "Parameters")]
    pub parameters: ProvisioningArtifactProperties,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

/// see [ServiceCatalog::create_provisioning_artifact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProvisioningArtifactOutput {
    /// <p>Specify the template source with one of the following options, but not both. Keys accepted: [ <code>LoadTemplateFromURL</code>, <code>ImportFromPhysicalId</code> ].</p> <p>The URL of the CloudFormation template in Amazon S3, in JSON format. </p> <p> <code>LoadTemplateFromURL</code> </p> <p>Use the URL of the CloudFormation template in Amazon S3 in JSON format.</p> <p> <code>ImportFromPhysicalId</code> </p> <p>Use the physical id of the resource that contains the template; currently supports CloudFormation stack ARN.</p>
    #[serde(rename = "Info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<::std::collections::HashMap<String, String>>,
    /// <p>Information about the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ServiceCatalog::create_service_action]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateServiceActionInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p><p>The self-service action definition. Can be one of the following:</p> <dl> <dt>Name</dt> <dd> <p>The name of the AWS Systems Manager document (SSM document). For example, <code>AWS-RestartEC2Instance</code>.</p> <p>If you are using a shared SSM document, you must provide the ARN instead of the name.</p> </dd> <dt>Version</dt> <dd> <p>The AWS Systems Manager automation document version. For example, <code>&quot;Version&quot;: &quot;1&quot;</code> </p> </dd> <dt>AssumeRole</dt> <dd> <p>The Amazon Resource Name (ARN) of the role that performs the self-service actions on your behalf. For example, <code>&quot;AssumeRole&quot;: &quot;arn:aws:iam::12345678910:role/ActionRole&quot;</code>.</p> <p>To reuse the provisioned product launch role, set to <code>&quot;AssumeRole&quot;: &quot;LAUNCH<em>ROLE&quot;</code>.</p> </dd> <dt>Parameters</dt> <dd> <p>The list of parameters in JSON format.</p> <p>For example: <code>[{&quot;Name&quot;:&quot;InstanceId&quot;,&quot;Type&quot;:&quot;TARGET&quot;}]</code> or <code>[{&quot;Name&quot;:&quot;InstanceId&quot;,&quot;Type&quot;:&quot;TEXT</em>VALUE&quot;}]</code>.</p> </dd> </dl></p>
    #[serde(rename = "Definition")]
    pub definition: ::std::collections::HashMap<String, String>,
    /// <p>The service action definition type. For example, <code>SSM_AUTOMATION</code>.</p>
    #[serde(rename = "DefinitionType")]
    pub definition_type: String,
    /// <p>The self-service action description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The self-service action name.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// see [ServiceCatalog::create_service_action]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateServiceActionOutput {
    /// <p>An object containing information about the self-service action.</p>
    #[serde(rename = "ServiceActionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_detail: Option<ServiceActionDetail>,
}

/// see [ServiceCatalog::create_tag_option]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTagOptionInput {
    /// <p>The TagOption key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The TagOption value.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// see [ServiceCatalog::create_tag_option]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTagOptionOutput {
    /// <p>Information about the TagOption.</p>
    #[serde(rename = "TagOptionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}

/// see [ServiceCatalog::delete_constraint]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConstraintInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The identifier of the constraint.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// see [ServiceCatalog::delete_constraint]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConstraintOutput {}

/// see [ServiceCatalog::delete_portfolio]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// see [ServiceCatalog::delete_portfolio]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePortfolioOutput {}

/// see [ServiceCatalog::delete_portfolio_share]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePortfolioShareInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The organization node to whom you are going to stop sharing.</p>
    #[serde(rename = "OrganizationNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_node: Option<OrganizationNode>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

/// see [ServiceCatalog::delete_portfolio_share]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePortfolioShareOutput {
    /// <p>The portfolio share unique identifier. This will only be returned if delete is made to an organization node.</p>
    #[serde(rename = "PortfolioShareToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_token: Option<String>,
}

/// see [ServiceCatalog::delete_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// see [ServiceCatalog::delete_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProductOutput {}

/// see [ServiceCatalog::delete_provisioned_product_plan]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProvisionedProductPlanInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>If set to true, AWS Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources.</p>
    #[serde(rename = "IgnoreErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_errors: Option<bool>,
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    pub plan_id: String,
}

/// see [ServiceCatalog::delete_provisioned_product_plan]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProvisionedProductPlanOutput {}

/// see [ServiceCatalog::delete_provisioning_artifact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProvisioningArtifactInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
}

/// see [ServiceCatalog::delete_provisioning_artifact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProvisioningArtifactOutput {}

/// see [ServiceCatalog::delete_service_action]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteServiceActionInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// see [ServiceCatalog::delete_service_action]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteServiceActionOutput {}

/// see [ServiceCatalog::delete_tag_option]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTagOptionInput {
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// see [ServiceCatalog::delete_tag_option]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTagOptionOutput {}

/// see [ServiceCatalog::describe_constraint]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConstraintInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The identifier of the constraint.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// see [ServiceCatalog::describe_constraint]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConstraintOutput {
    /// <p>Information about the constraint.</p>
    #[serde(rename = "ConstraintDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    /// <p>The constraint parameters.</p>
    #[serde(rename = "ConstraintParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_parameters: Option<String>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ServiceCatalog::describe_copy_product_status]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCopyProductStatusInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The token for the copy product operation. This token is returned by <a>CopyProduct</a>.</p>
    #[serde(rename = "CopyProductToken")]
    pub copy_product_token: String,
}

/// see [ServiceCatalog::describe_copy_product_status]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCopyProductStatusOutput {
    /// <p>The status of the copy product operation.</p>
    #[serde(rename = "CopyProductStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_product_status: Option<String>,
    /// <p>The status message.</p>
    #[serde(rename = "StatusDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    /// <p>The identifier of the copied product.</p>
    #[serde(rename = "TargetProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_product_id: Option<String>,
}

/// see [ServiceCatalog::describe_portfolio]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// see [ServiceCatalog::describe_portfolio]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePortfolioOutput {
    /// <p>Information about the associated budgets.</p>
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    /// <p>Information about the portfolio.</p>
    #[serde(rename = "PortfolioDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    /// <p>Information about the TagOptions associated with the portfolio.</p>
    #[serde(rename = "TagOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_options: Option<Vec<TagOptionDetail>>,
    /// <p>Information about the tags associated with the portfolio.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [ServiceCatalog::describe_portfolio_share_status]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePortfolioShareStatusInput {
    /// <p>The token for the portfolio share operation. This token is returned either by CreatePortfolioShare or by DeletePortfolioShare.</p>
    #[serde(rename = "PortfolioShareToken")]
    pub portfolio_share_token: String,
}

/// see [ServiceCatalog::describe_portfolio_share_status]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePortfolioShareStatusOutput {
    /// <p>Organization node identifier. It can be either account id, organizational unit id or organization id.</p>
    #[serde(rename = "OrganizationNodeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_node_value: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_id: Option<String>,
    /// <p>The token for the portfolio share operation. For example, <code>share-6v24abcdefghi</code>.</p>
    #[serde(rename = "PortfolioShareToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_token: Option<String>,
    /// <p>Information about the portfolio share operation.</p>
    #[serde(rename = "ShareDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_details: Option<ShareDetails>,
    /// <p>Status of the portfolio share operation.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ServiceCatalog::describe_portfolio_shares]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePortfolioSharesInput {
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The unique identifier of the portfolio for which shares will be retrieved.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The type of portfolio share to summarize. This field acts as a filter on the type of portfolio share, which can be one of the following:</p> <p>1. <code>ACCOUNT</code> - Represents an external account to account share.</p> <p>2. <code>ORGANIZATION</code> - Represents a share to an organization. This share is available to every account in the organization.</p> <p>3. <code>ORGANIZATIONAL_UNIT</code> - Represents a share to an organizational unit.</p> <p>4. <code>ORGANIZATION_MEMBER_ACCOUNT</code> - Represents a share to an account in the organization.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// see [ServiceCatalog::describe_portfolio_shares]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePortfolioSharesOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Summaries about each of the portfolio shares.</p>
    #[serde(rename = "PortfolioShareDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_details: Option<Vec<PortfolioShareDetail>>,
}

/// see [ServiceCatalog::describe_product_as_admin]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProductAsAdminInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The product name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier of the shared portfolio that the specified product is associated with.</p> <p>You can provide this parameter to retrieve the shared TagOptions associated with the product. If this parameter is provided and if TagOptions sharing is enabled in the portfolio share, the API returns both local and shared TagOptions associated with the product. Otherwise only local TagOptions will be returned. </p>
    #[serde(rename = "SourcePortfolioId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_portfolio_id: Option<String>,
}

/// see [ServiceCatalog::describe_product_as_admin]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProductAsAdminOutput {
    /// <p>Information about the associated budgets.</p>
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    /// <p>Information about the product view.</p>
    #[serde(rename = "ProductViewDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    /// <p>Information about the provisioning artifacts (also known as versions) for the specified product.</p>
    #[serde(rename = "ProvisioningArtifactSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_summaries: Option<Vec<ProvisioningArtifactSummary>>,
    /// <p>Information about the TagOptions associated with the product.</p>
    #[serde(rename = "TagOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_options: Option<Vec<TagOptionDetail>>,
    /// <p>Information about the tags associated with the product.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [ServiceCatalog::describe_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The product name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// see [ServiceCatalog::describe_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProductOutput {
    /// <p>Information about the associated budgets.</p>
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    /// <p>Information about the associated launch paths.</p>
    #[serde(rename = "LaunchPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_paths: Option<Vec<LaunchPath>>,
    /// <p>Summary information about the product view.</p>
    #[serde(rename = "ProductViewSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    /// <p>Information about the provisioning artifacts for the specified product.</p>
    #[serde(rename = "ProvisioningArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifacts: Option<Vec<ProvisioningArtifact>>,
}

/// see [ServiceCatalog::describe_product_view]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProductViewInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product view identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// see [ServiceCatalog::describe_product_view]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProductViewOutput {
    /// <p>Summary information about the product.</p>
    #[serde(rename = "ProductViewSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    /// <p>Information about the provisioning artifacts for the product.</p>
    #[serde(rename = "ProvisioningArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifacts: Option<Vec<ProvisioningArtifact>>,
}

/// <p>DescribeProvisionedProductAPI input structure. AcceptLanguage - [Optional] The language code for localization. Id - [Optional] The provisioned product identifier. Name - [Optional] Another provisioned product identifier. Customers must provide either Id or Name.</p>
/// see [ServiceCatalog::describe_provisioned_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProvisionedProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The provisioned product identifier. You must provide the name or ID, but not both.</p> <p>If you do not provide a name or ID, or you provide both name and ID, an <code>InvalidParametersException</code> will occur.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the provisioned product. You must provide the name or ID, but not both.</p> <p>If you do not provide a name or ID, or you provide both name and ID, an <code>InvalidParametersException</code> will occur.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// see [ServiceCatalog::describe_provisioned_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProvisionedProductOutput {
    /// <p>Any CloudWatch dashboards that were created when provisioning the product.</p>
    #[serde(rename = "CloudWatchDashboards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_dashboards: Option<Vec<CloudWatchDashboard>>,
    /// <p>Information about the provisioned product.</p>
    #[serde(rename = "ProvisionedProductDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_detail: Option<ProvisionedProductDetail>,
}

/// see [ServiceCatalog::describe_provisioned_product_plan]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProvisionedProductPlanInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    pub plan_id: String,
}

/// see [ServiceCatalog::describe_provisioned_product_plan]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProvisionedProductPlanOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the plan.</p>
    #[serde(rename = "ProvisionedProductPlanDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_plan_details: Option<ProvisionedProductPlanDetails>,
    /// <p>Information about the resource changes that will occur when the plan is executed.</p>
    #[serde(rename = "ResourceChanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_changes: Option<Vec<ResourceChange>>,
}

/// see [ServiceCatalog::describe_provisioning_artifact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProvisioningArtifactInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The product name.</p>
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>The provisioning artifact name.</p>
    #[serde(rename = "ProvisioningArtifactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_name: Option<String>,
    /// <p>Indicates whether a verbose level of detail is enabled.</p>
    #[serde(rename = "Verbose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,
}

/// see [ServiceCatalog::describe_provisioning_artifact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProvisioningArtifactOutput {
    /// <p>The URL of the CloudFormation template in Amazon S3.</p>
    #[serde(rename = "Info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<::std::collections::HashMap<String, String>>,
    /// <p>Information about the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ServiceCatalog::describe_provisioning_parameters]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProvisioningParametersInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use <a>ListLaunchPaths</a>. You must provide the name or ID, but not both.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The name of the path. You must provide the name or ID, but not both.</p>
    #[serde(rename = "PathName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_name: Option<String>,
    /// <p>The product identifier. You must provide the product name or ID, but not both.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The name of the product. You must provide the name or ID, but not both.</p>
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// <p>The identifier of the provisioning artifact. You must provide the name or ID, but not both.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>The name of the provisioning artifact. You must provide the name or ID, but not both.</p>
    #[serde(rename = "ProvisioningArtifactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_name: Option<String>,
}

/// see [ServiceCatalog::describe_provisioning_parameters]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProvisioningParametersOutput {
    /// <p>Information about the constraints used to provision the product.</p>
    #[serde(rename = "ConstraintSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_summaries: Option<Vec<ConstraintSummary>>,
    /// <p>The output of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_outputs: Option<Vec<ProvisioningArtifactOutput>>,
    /// <p>Information about the parameters used to provision the product.</p>
    #[serde(rename = "ProvisioningArtifactParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_parameters: Option<Vec<ProvisioningArtifactParameter>>,
    /// <p>An object that contains information about preferences, such as regions and accounts, for the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_preferences: Option<ProvisioningArtifactPreferences>,
    /// <p>Information about the TagOptions associated with the resource.</p>
    #[serde(rename = "TagOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_options: Option<Vec<TagOptionSummary>>,
    /// <p>Any additional metadata specifically related to the provisioning of the product. For example, see the <code>Version</code> field of the CloudFormation template.</p>
    #[serde(rename = "UsageInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_instructions: Option<Vec<UsageInstruction>>,
}

/// see [ServiceCatalog::describe_record]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRecordInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The record identifier of the provisioned product. This identifier is returned by the request operation.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// see [ServiceCatalog::describe_record]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRecordOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the product.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
    /// <p>Information about the product created as the result of a request. For example, the output for a CloudFormation-backed product that creates an S3 bucket would include the S3 bucket URL.</p>
    #[serde(rename = "RecordOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_outputs: Option<Vec<RecordOutput>>,
}

/// see [ServiceCatalog::describe_service_action_execution_parameters]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeServiceActionExecutionParametersInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductId")]
    pub provisioned_product_id: String,
    /// <p>The self-service action identifier.</p>
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
}

/// see [ServiceCatalog::describe_service_action_execution_parameters]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeServiceActionExecutionParametersOutput {
    /// <p>The parameters of the self-service action.</p>
    #[serde(rename = "ServiceActionParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_parameters: Option<Vec<ExecutionParameter>>,
}

/// see [ServiceCatalog::describe_service_action]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeServiceActionInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The self-service action identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// see [ServiceCatalog::describe_service_action]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeServiceActionOutput {
    /// <p>Detailed information about the self-service action.</p>
    #[serde(rename = "ServiceActionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_detail: Option<ServiceActionDetail>,
}

/// see [ServiceCatalog::describe_tag_option]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTagOptionInput {
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// see [ServiceCatalog::describe_tag_option]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTagOptionOutput {
    /// <p>Information about the TagOption.</p>
    #[serde(rename = "TagOptionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}

/// see [ServiceCatalog::disable_aws_organizations_access]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableAWSOrganizationsAccessInput {}

/// see [ServiceCatalog::disable_aws_organizations_access]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableAWSOrganizationsAccessOutput {}

/// see [ServiceCatalog::disassociate_budget_from_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateBudgetFromResourceInput {
    /// <p>The name of the budget you want to disassociate.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>The resource identifier you want to disassociate from. Either a portfolio-id or a product-id.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

/// see [ServiceCatalog::disassociate_budget_from_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateBudgetFromResourceOutput {}

/// see [ServiceCatalog::disassociate_principal_from_portfolio]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociatePrincipalFromPortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The ARN of the principal (IAM user, role, or group).</p>
    #[serde(rename = "PrincipalARN")]
    pub principal_arn: String,
}

/// see [ServiceCatalog::disassociate_principal_from_portfolio]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociatePrincipalFromPortfolioOutput {}

/// see [ServiceCatalog::disassociate_product_from_portfolio]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateProductFromPortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

/// see [ServiceCatalog::disassociate_product_from_portfolio]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateProductFromPortfolioOutput {}

/// see [ServiceCatalog::disassociate_service_action_from_provisioning_artifact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateServiceActionFromProvisioningArtifactInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
}

/// see [ServiceCatalog::disassociate_service_action_from_provisioning_artifact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateServiceActionFromProvisioningArtifactOutput {}

/// see [ServiceCatalog::disassociate_tag_option_from_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateTagOptionFromResourceInput {
    /// <p>The resource identifier.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "TagOptionId")]
    pub tag_option_id: String,
}

/// see [ServiceCatalog::disassociate_tag_option_from_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateTagOptionFromResourceOutput {}

/// see [ServiceCatalog::enable_aws_organizations_access]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableAWSOrganizationsAccessInput {}

/// see [ServiceCatalog::enable_aws_organizations_access]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableAWSOrganizationsAccessOutput {}

/// see [ServiceCatalog::execute_provisioned_product_plan]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExecuteProvisionedProductPlanInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    pub plan_id: String,
}

/// see [ServiceCatalog::execute_provisioned_product_plan]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecuteProvisionedProductPlanOutput {
    /// <p>Information about the result of provisioning the product.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

/// see [ServiceCatalog::execute_provisioned_product_service_action]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExecuteProvisionedProductServiceActionInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>An idempotency token that uniquely identifies the execute request.</p>
    #[serde(rename = "ExecuteToken")]
    pub execute_token: String,
    /// <p>A map of all self-service action parameters and their values. If a provided parameter is of a special type, such as <code>TARGET</code>, the provided value will override the default value generated by AWS Service Catalog. If the parameters field is not provided, no additional parameters are passed and default values will be used for any special parameters such as <code>TARGET</code>.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductId")]
    pub provisioned_product_id: String,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
}

/// see [ServiceCatalog::execute_provisioned_product_service_action]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecuteProvisionedProductServiceActionOutput {
    /// <p>An object containing detailed information about the result of provisioning the product.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

/// <p>Details of an execution parameter value that is passed to a self-service action when executed on a provisioned product.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecutionParameter {
    /// <p>The default values for the execution parameter.</p>
    #[serde(rename = "DefaultValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_values: Option<Vec<String>>,
    /// <p>The name of the execution parameter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The execution parameter type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>An object containing information about the error, along with identifying information about the self-service action and its associations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailedServiceActionAssociation {
    /// <p>The error code. Valid values are listed below.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A text description of the error.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_id: Option<String>,
}

/// see [ServiceCatalog::get_aws_organizations_access_status]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAWSOrganizationsAccessStatusInput {}

/// see [ServiceCatalog::get_aws_organizations_access_status]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAWSOrganizationsAccessStatusOutput {
    /// <p>The status of the portfolio share feature.</p>
    #[serde(rename = "AccessStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_status: Option<String>,
}

/// see [ServiceCatalog::get_provisioned_product_outputs]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetProvisionedProductOutputsInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The list of keys that the API should return with their values. If none are provided, the API will return all outputs of the provisioned product.</p>
    #[serde(rename = "OutputKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_keys: Option<Vec<String>>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The identifier of the provisioned product that you want the outputs from.</p>
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    /// <p>The name of the provisioned product that you want the outputs from.</p>
    #[serde(rename = "ProvisionedProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
}

/// see [ServiceCatalog::get_provisioned_product_outputs]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetProvisionedProductOutputsOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the product created as the result of a request. For example, the output for a CloudFormation-backed product that creates an S3 bucket would include the S3 bucket URL. </p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<RecordOutput>>,
}

/// see [ServiceCatalog::import_as_provisioned_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportAsProvisionedProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The unique identifier of the resource to be imported. It only currently supports CloudFormation stack IDs.</p>
    #[serde(rename = "PhysicalId")]
    pub physical_id: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The user-friendly name of the provisioned product. The value must be unique for the AWS account. The name cannot be updated after the product is provisioned. </p>
    #[serde(rename = "ProvisionedProductName")]
    pub provisioned_product_name: String,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
}

/// see [ServiceCatalog::import_as_provisioned_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportAsProvisionedProductOutput {
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

/// <p>A launch path object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LaunchPath {
    /// <p>The identifier of the launch path.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the launch path.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Summary information about a product path for a user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LaunchPathSummary {
    /// <p>The constraints on the portfolio-product relationship.</p>
    #[serde(rename = "ConstraintSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_summaries: Option<Vec<ConstraintSummary>>,
    /// <p>The identifier of the product path.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the portfolio to which the user was assigned.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The tags associated with this product path.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [ServiceCatalog::list_accepted_portfolio_shares]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAcceptedPortfolioSharesInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p><p>The type of shared portfolios to list. The default is to list imported portfolios.</p> <ul> <li> <p> <code>AWS<em>ORGANIZATIONS</code> - List portfolios shared by the management account of your organization</p> </li> <li> <p> <code>AWS</em>SERVICECATALOG</code> - List default portfolios</p> </li> <li> <p> <code>IMPORTED</code> - List imported portfolios</p> </li> </ul></p>
    #[serde(rename = "PortfolioShareType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_type: Option<String>,
}

impl Paged for ListAcceptedPortfolioSharesInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListAcceptedPortfolioSharesInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_accepted_portfolio_shares]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAcceptedPortfolioSharesOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the portfolios.</p>
    #[serde(rename = "PortfolioDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_details: Option<Vec<PortfolioDetail>>,
}

impl Paged for ListAcceptedPortfolioSharesOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListAcceptedPortfolioSharesOutput {
    type Item = PortfolioDetail;

    fn into_pagination_page(self) -> Vec<PortfolioDetail> {
        self.portfolio_details.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_budgets_for_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBudgetsForResourceInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The resource identifier.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

/// see [ServiceCatalog::list_budgets_for_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBudgetsForResourceOutput {
    /// <p>Information about the associated budgets.</p>
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// see [ServiceCatalog::list_constraints_for_portfolio]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConstraintsForPortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl Paged for ListConstraintsForPortfolioInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListConstraintsForPortfolioInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_constraints_for_portfolio]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConstraintsForPortfolioOutput {
    /// <p>Information about the constraints.</p>
    #[serde(rename = "ConstraintDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_details: Option<Vec<ConstraintDetail>>,
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl Paged for ListConstraintsForPortfolioOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListConstraintsForPortfolioOutput {
    type Item = ConstraintDetail;

    fn into_pagination_page(self) -> Vec<ConstraintDetail> {
        self.constraint_details.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_launch_paths]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLaunchPathsInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

impl Paged for ListLaunchPathsInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListLaunchPathsInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_launch_paths]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLaunchPathsOutput {
    /// <p>Information about the launch path.</p>
    #[serde(rename = "LaunchPathSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_path_summaries: Option<Vec<LaunchPathSummary>>,
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl Paged for ListLaunchPathsOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListLaunchPathsOutput {
    type Item = LaunchPathSummary;

    fn into_pagination_page(self) -> Vec<LaunchPathSummary> {
        self.launch_path_summaries.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_organization_portfolio_access]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOrganizationPortfolioAccessInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p><p>The organization node type that will be returned in the output.</p> <ul> <li> <p> <code>ORGANIZATION</code> - Organization that has access to the portfolio. </p> </li> <li> <p> <code>ORGANIZATIONAL_UNIT</code> - Organizational unit that has access to the portfolio within your organization.</p> </li> <li> <p> <code>ACCOUNT</code> - Account that has access to the portfolio within your organization.</p> </li> </ul></p>
    #[serde(rename = "OrganizationNodeType")]
    pub organization_node_type: String,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The portfolio identifier. For example, <code>port-2abcdext3y5fk</code>.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

impl Paged for ListOrganizationPortfolioAccessInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListOrganizationPortfolioAccessInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_organization_portfolio_access]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOrganizationPortfolioAccessOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Displays information about the organization nodes.</p>
    #[serde(rename = "OrganizationNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_nodes: Option<Vec<OrganizationNode>>,
}

impl Paged for ListOrganizationPortfolioAccessOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListOrganizationPortfolioAccessOutput {
    type Item = OrganizationNode;

    fn into_pagination_page(self) -> Vec<OrganizationNode> {
        self.organization_nodes.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_portfolio_access]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPortfolioAccessInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The ID of an organization node the portfolio is shared with. All children of this node with an inherited portfolio share will be returned.</p>
    #[serde(rename = "OrganizationParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_parent_id: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

/// see [ServiceCatalog::list_portfolio_access]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPortfolioAccessOutput {
    /// <p>Information about the AWS accounts with access to the portfolio.</p>
    #[serde(rename = "AccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// see [ServiceCatalog::list_portfolios_for_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPortfoliosForProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

impl Paged for ListPortfoliosForProductInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListPortfoliosForProductInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_portfolios_for_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPortfoliosForProductOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the portfolios.</p>
    #[serde(rename = "PortfolioDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_details: Option<Vec<PortfolioDetail>>,
}

impl Paged for ListPortfoliosForProductOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListPortfoliosForProductOutput {
    type Item = PortfolioDetail;

    fn into_pagination_page(self) -> Vec<PortfolioDetail> {
        self.portfolio_details.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_portfolios]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPortfoliosInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Paged for ListPortfoliosInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListPortfoliosInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_portfolios]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPortfoliosOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the portfolios.</p>
    #[serde(rename = "PortfolioDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_details: Option<Vec<PortfolioDetail>>,
}

impl Paged for ListPortfoliosOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListPortfoliosOutput {
    type Item = PortfolioDetail;

    fn into_pagination_page(self) -> Vec<PortfolioDetail> {
        self.portfolio_details.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_principals_for_portfolio]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPrincipalsForPortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

impl Paged for ListPrincipalsForPortfolioInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListPrincipalsForPortfolioInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_principals_for_portfolio]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPrincipalsForPortfolioOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The IAM principals (users or roles) associated with the portfolio.</p>
    #[serde(rename = "Principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<Principal>>,
}

impl Paged for ListPrincipalsForPortfolioOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListPrincipalsForPortfolioOutput {
    type Item = Principal;

    fn into_pagination_page(self) -> Vec<Principal> {
        self.principals.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_provisioned_product_plans]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProvisionedProductPlansInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    #[serde(rename = "AccessLevelFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProvisionProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
}

impl Paged for ListProvisionedProductPlansInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListProvisionedProductPlansInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_provisioned_product_plans]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProvisionedProductPlansOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the plans.</p>
    #[serde(rename = "ProvisionedProductPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_plans: Option<Vec<ProvisionedProductPlanSummary>>,
}

impl Paged for ListProvisionedProductPlansOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListProvisionedProductPlansOutput {
    type Item = ProvisionedProductPlanSummary;

    fn into_pagination_page(self) -> Vec<ProvisionedProductPlanSummary> {
        self.provisioned_product_plans.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_provisioning_artifacts_for_service_action]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProvisioningArtifactsForServiceActionInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
}

impl Paged for ListProvisioningArtifactsForServiceActionInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListProvisioningArtifactsForServiceActionInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_provisioning_artifacts_for_service_action]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProvisioningArtifactsForServiceActionOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An array of objects with information about product views and provisioning artifacts.</p>
    #[serde(rename = "ProvisioningArtifactViews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_views: Option<Vec<ProvisioningArtifactView>>,
}

impl Paged for ListProvisioningArtifactsForServiceActionOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListProvisioningArtifactsForServiceActionOutput {
    type Item = ProvisioningArtifactView;

    fn into_pagination_page(self) -> Vec<ProvisioningArtifactView> {
        self.provisioning_artifact_views.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_provisioning_artifacts]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProvisioningArtifactsInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

/// see [ServiceCatalog::list_provisioning_artifacts]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProvisioningArtifactsOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the provisioning artifacts.</p>
    #[serde(rename = "ProvisioningArtifactDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_details: Option<Vec<ProvisioningArtifactDetail>>,
}

/// see [ServiceCatalog::list_record_history]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRecordHistoryInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    #[serde(rename = "AccessLevelFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The search filter to scope the results.</p>
    #[serde(rename = "SearchFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<ListRecordHistorySearchFilter>,
}

impl Paged for ListRecordHistoryInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListRecordHistoryInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_record_history]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRecordHistoryOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The records, in reverse chronological order.</p>
    #[serde(rename = "RecordDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_details: Option<Vec<RecordDetail>>,
}

impl Paged for ListRecordHistoryOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListRecordHistoryOutput {
    type Item = RecordDetail;

    fn into_pagination_page(self) -> Vec<RecordDetail> {
        self.record_details.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// <p>The search filter to use when listing history records.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRecordHistorySearchFilter {
    /// <p><p>The filter key.</p> <ul> <li> <p> <code>product</code> - Filter results based on the specified product identifier.</p> </li> <li> <p> <code>provisionedproduct</code> - Filter results based on the provisioned product identifier.</p> </li> </ul></p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The filter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// see [ServiceCatalog::list_resources_for_tag_option]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourcesForTagOptionInput {
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p><p>The resource type.</p> <ul> <li> <p> <code>Portfolio</code> </p> </li> <li> <p> <code>Product</code> </p> </li> </ul></p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "TagOptionId")]
    pub tag_option_id: String,
}

impl Paged for ListResourcesForTagOptionInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListResourcesForTagOptionInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_resources_for_tag_option]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourcesForTagOptionOutput {
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>Information about the resources.</p>
    #[serde(rename = "ResourceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<Vec<ResourceDetail>>,
}

impl Paged for ListResourcesForTagOptionOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedOutput for ListResourcesForTagOptionOutput {
    type Item = ResourceDetail;

    fn into_pagination_page(self) -> Vec<ResourceDetail> {
        self.resource_details.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_service_actions_for_provisioning_artifact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServiceActionsForProvisioningArtifactInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
}

impl Paged for ListServiceActionsForProvisioningArtifactInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListServiceActionsForProvisioningArtifactInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_service_actions_for_provisioning_artifact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServiceActionsForProvisioningArtifactOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object containing information about the self-service actions associated with the provisioning artifact.</p>
    #[serde(rename = "ServiceActionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_summaries: Option<Vec<ServiceActionSummary>>,
}

impl Paged for ListServiceActionsForProvisioningArtifactOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListServiceActionsForProvisioningArtifactOutput {
    type Item = ServiceActionSummary;

    fn into_pagination_page(self) -> Vec<ServiceActionSummary> {
        self.service_action_summaries.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_service_actions]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServiceActionsInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Paged for ListServiceActionsInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListServiceActionsInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_service_actions]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServiceActionsOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object containing information about the service actions associated with the provisioning artifact.</p>
    #[serde(rename = "ServiceActionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_summaries: Option<Vec<ServiceActionSummary>>,
}

impl Paged for ListServiceActionsOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ListServiceActionsOutput {
    type Item = ServiceActionSummary;

    fn into_pagination_page(self) -> Vec<ServiceActionSummary> {
        self.service_action_summaries.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::list_stack_instances_for_provisioned_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListStackInstancesForProvisionedProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductId")]
    pub provisioned_product_id: String,
}

/// see [ServiceCatalog::list_stack_instances_for_provisioned_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListStackInstancesForProvisionedProductOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>List of stack instances.</p>
    #[serde(rename = "StackInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_instances: Option<Vec<StackInstance>>,
}

/// <p>Filters to use when listing TagOptions.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagOptionsFilters {
    /// <p>The active state.</p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p>The TagOption key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The TagOption value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// see [ServiceCatalog::list_tag_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagOptionsInput {
    /// <p>The search filters. If no search filters are specified, the output includes all TagOptions.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ListTagOptionsFilters>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Paged for ListTagOptionsInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ListTagOptionsInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::list_tag_options]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagOptionsOutput {
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>Information about the TagOptions.</p>
    #[serde(rename = "TagOptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_details: Option<Vec<TagOptionDetail>>,
}

impl Paged for ListTagOptionsOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedOutput for ListTagOptionsOutput {
    type Item = TagOptionDetail;

    fn into_pagination_page(self) -> Vec<TagOptionDetail> {
        self.tag_option_details.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// <p>Information about the organization node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OrganizationNode {
    /// <p>The organization node type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The identifier of the organization node.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The constraints that the administrator has put on the parameter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ParameterConstraints {
    /// <p>The values that the administrator has allowed for the parameter.</p>
    #[serde(rename = "AllowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
}

/// <p>Information about a portfolio.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PortfolioDetail {
    /// <p>The ARN assigned to the portfolio.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the portfolio.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name to use for display purposes.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the portfolio provider.</p>
    #[serde(rename = "ProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

/// <p>Information about the portfolio share.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PortfolioShareDetail {
    /// <p>Indicates whether the shared portfolio is imported by the recipient account. If the recipient is in an organization node, the share is automatically imported, and the field is always set to true.</p>
    #[serde(rename = "Accepted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
    /// <p>The identifier of the recipient entity that received the portfolio share. The recipient entities can be one of the following: </p> <p>1. An external account.</p> <p>2. An organziation member account.</p> <p>3. An organzational unit (OU).</p> <p>4. The organization itself. (This shares with every account in the organization).</p>
    #[serde(rename = "PrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// <p>Indicates whether TagOptions sharing is enabled or disabled for the portfolio share.</p>
    #[serde(rename = "ShareTagOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_tag_options: Option<bool>,
    /// <p>The type of the portfolio share.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about a principal.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Principal {
    /// <p>The ARN of the principal (IAM user, role, or group).</p>
    #[serde(rename = "PrincipalARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    /// <p>The principal type. The supported value is <code>IAM</code>.</p>
    #[serde(rename = "PrincipalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}

/// <p>A single product view aggregation value/count pair, containing metadata about each product to which the calling user has access.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProductViewAggregationValue {
    /// <p>An approximate count of the products that match the value.</p>
    #[serde(rename = "ApproximateCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_count: Option<i64>,
    /// <p>The value of the product view aggregation.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Information about a product view.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProductViewDetail {
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The ARN of the product.</p>
    #[serde(rename = "ProductARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_arn: Option<String>,
    /// <p>Summary information about the product view.</p>
    #[serde(rename = "ProductViewSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    /// <p><p>The status of the product.</p> <ul> <li> <p> <code>AVAILABLE</code> - The product is ready for use.</p> </li> <li> <p> <code>CREATING</code> - Product creation has started; the product is not ready for use.</p> </li> <li> <p> <code>FAILED</code> - An action failed.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Summary information about a product view.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProductViewSummary {
    /// <p>The distributor of the product. Contact the product administrator for the significance of this value.</p>
    #[serde(rename = "Distributor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    /// <p>Indicates whether the product has a default path. If the product does not have a default path, call <a>ListLaunchPaths</a> to disambiguate between paths. Otherwise, <a>ListLaunchPaths</a> is not required, and the output of <a>ProductViewSummary</a> can be used directly with <a>DescribeProvisioningParameters</a>.</p>
    #[serde(rename = "HasDefaultPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_default_path: Option<bool>,
    /// <p>The product view identifier.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the product.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the product. Contact the product administrator for the significance of this value.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>Short description of the product.</p>
    #[serde(rename = "ShortDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    /// <p>The description of the support for this Product.</p>
    #[serde(rename = "SupportDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_description: Option<String>,
    /// <p>The email contact information to obtain support for this Product.</p>
    #[serde(rename = "SupportEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// <p>The URL information to obtain support for this Product.</p>
    #[serde(rename = "SupportUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    /// <p>The product type. Contact the product administrator for the significance of this value. If this value is <code>MARKETPLACE</code>, the product was created by AWS Marketplace.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// see [ServiceCatalog::provision_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ProvisionProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.</p>
    #[serde(rename = "NotificationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    /// <p>The path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use <a>ListLaunchPaths</a>. You must provide the name or ID, but not both.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The name of the path. You must provide the name or ID, but not both.</p>
    #[serde(rename = "PathName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_name: Option<String>,
    /// <p>The product identifier. You must provide the name or ID, but not both.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The name of the product. You must provide the name or ID, but not both.</p>
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// <p>An idempotency token that uniquely identifies the provisioning request.</p>
    #[serde(rename = "ProvisionToken")]
    pub provision_token: String,
    /// <p>A user-friendly name for the provisioned product. This value must be unique for the AWS account and cannot be updated after the product is provisioned.</p>
    #[serde(rename = "ProvisionedProductName")]
    pub provisioned_product_name: String,
    /// <p>The identifier of the provisioning artifact. You must provide the name or ID, but not both.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>The name of the provisioning artifact. You must provide the name or ID, but not both.</p>
    #[serde(rename = "ProvisioningArtifactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_name: Option<String>,
    /// <p>Parameters specified by the administrator that are required for provisioning the product.</p>
    #[serde(rename = "ProvisioningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<ProvisioningParameter>>,
    /// <p>An object that contains information about the provisioning preferences for a stack set.</p>
    #[serde(rename = "ProvisioningPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_preferences: Option<ProvisioningPreferences>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [ServiceCatalog::provision_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisionProductOutput {
    /// <p>Information about the result of provisioning the product.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

/// <p>Information about a provisioned product.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisionedProductAttribute {
    /// <p>The ARN of the provisioned product.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    /// <p><p>The record identifier of the last request performed on this provisioned product of the following types:</p> <ul> <li> <p> ProvisionedProduct </p> </li> <li> <p> UpdateProvisionedProduct </p> </li> <li> <p> ExecuteProvisionedProductPlan </p> </li> <li> <p> TerminateProvisionedProduct </p> </li> </ul></p>
    #[serde(rename = "LastProvisioningRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_provisioning_record_id: Option<String>,
    /// <p>The record identifier of the last request performed on this provisioned product.</p>
    #[serde(rename = "LastRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_record_id: Option<String>,
    /// <p><p>The record identifier of the last successful request performed on this provisioned product of the following types:</p> <ul> <li> <p> ProvisionedProduct </p> </li> <li> <p> UpdateProvisionedProduct </p> </li> <li> <p> ExecuteProvisionedProductPlan </p> </li> <li> <p> TerminateProvisionedProduct </p> </li> </ul></p>
    #[serde(rename = "LastSuccessfulProvisioningRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_provisioning_record_id: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The assigned identifier for the resource, such as an EC2 instance ID or an S3 bucket name.</p>
    #[serde(rename = "PhysicalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_id: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The name of the product.</p>
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>The name of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_name: Option<String>,
    /// <p><p>The current status of the provisioned product.</p> <ul> <li> <p> <code>AVAILABLE</code> - Stable state, ready to perform any operation. The most recent operation succeeded and completed.</p> </li> <li> <p> <code>UNDER<em>CHANGE</code> - Transitive state. Operations performed might not have valid results. Wait for an <code>AVAILABLE</code> status before performing operations.</p> </li> <li> <p> <code>TAINTED</code> - Stable state, ready to perform any operation. The stack has completed the requested operation but is not exactly what was requested. For example, a request to update to a new version failed and the stack rolled back to the current version.</p> </li> <li> <p> <code>ERROR</code> - An unexpected error occurred. The provisioned product exists but the stack is not running. For example, CloudFormation received a parameter value that was not valid and could not launch the stack.</p> </li> <li> <p> <code>PLAN</em>IN_PROGRESS</code> - Transitive state. The plan operations were performed to provision a new product, but resources have not yet been created. After reviewing the list of resources to be created, execute the plan. Wait for an <code>AVAILABLE</code> status before performing operations.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The current status message of the provisioned product.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of provisioned product. The supported values are <code>CFN_STACK</code> and <code>CFN_STACKSET</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM user.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    /// <p>The ARN of the IAM user in the session. This ARN might contain a session ID.</p>
    #[serde(rename = "UserArnSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn_session: Option<String>,
}

/// <p>Information about a provisioned product.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisionedProductDetail {
    /// <p>The ARN of the provisioned product.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    #[serde(rename = "IdempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    /// <p><p>The record identifier of the last request performed on this provisioned product of the following types:</p> <ul> <li> <p> ProvisionedProduct </p> </li> <li> <p> UpdateProvisionedProduct </p> </li> <li> <p> ExecuteProvisionedProductPlan </p> </li> <li> <p> TerminateProvisionedProduct </p> </li> </ul></p>
    #[serde(rename = "LastProvisioningRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_provisioning_record_id: Option<String>,
    /// <p>The record identifier of the last request performed on this provisioned product.</p>
    #[serde(rename = "LastRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_record_id: Option<String>,
    /// <p><p>The record identifier of the last successful request performed on this provisioned product of the following types:</p> <ul> <li> <p> ProvisionedProduct </p> </li> <li> <p> UpdateProvisionedProduct </p> </li> <li> <p> ExecuteProvisionedProductPlan </p> </li> <li> <p> TerminateProvisionedProduct </p> </li> </ul></p>
    #[serde(rename = "LastSuccessfulProvisioningRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_provisioning_record_id: Option<String>,
    /// <p>The ARN of the launch role associated with the provisioned product.</p>
    #[serde(rename = "LaunchRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_role_arn: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p><p>The current status of the provisioned product.</p> <ul> <li> <p> <code>AVAILABLE</code> - Stable state, ready to perform any operation. The most recent operation succeeded and completed.</p> </li> <li> <p> <code>UNDER<em>CHANGE</code> - Transitive state. Operations performed might not have valid results. Wait for an <code>AVAILABLE</code> status before performing operations.</p> </li> <li> <p> <code>TAINTED</code> - Stable state, ready to perform any operation. The stack has completed the requested operation but is not exactly what was requested. For example, a request to update to a new version failed and the stack rolled back to the current version.</p> </li> <li> <p> <code>ERROR</code> - An unexpected error occurred. The provisioned product exists but the stack is not running. For example, CloudFormation received a parameter value that was not valid and could not launch the stack.</p> </li> <li> <p> <code>PLAN</em>IN_PROGRESS</code> - Transitive state. The plan operations were performed to provision a new product, but resources have not yet been created. After reviewing the list of resources to be created, execute the plan. Wait for an <code>AVAILABLE</code> status before performing operations.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The current status message of the provisioned product.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The type of provisioned product. The supported values are <code>CFN_STACK</code> and <code>CFN_STACKSET</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about a plan.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisionedProductPlanDetails {
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.</p>
    #[serde(rename = "NotificationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    /// <p>The path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use <a>ListLaunchPaths</a>.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// <p>The name of the plan.</p>
    #[serde(rename = "PlanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// <p>The plan type.</p>
    #[serde(rename = "PlanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProvisionProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "ProvisionProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_name: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>Parameters specified by the administrator that are required for provisioning the product.</p>
    #[serde(rename = "ProvisioningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<UpdateProvisioningParameter>>,
    /// <p>The status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The status message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The time when the plan was last updated.</p>
    #[serde(rename = "UpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<f64>,
}

/// <p>Summary information about a plan.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisionedProductPlanSummary {
    /// <p>The plan identifier.</p>
    #[serde(rename = "PlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// <p>The name of the plan.</p>
    #[serde(rename = "PlanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// <p>The plan type.</p>
    #[serde(rename = "PlanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProvisionProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "ProvisionProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_name: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
}

/// <p>Information about a provisioning artifact. A provisioning artifact is also known as a product version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisioningArtifact {
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the provisioning artifact.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Information set by the administrator to provide guidance to end users about which provisioning artifacts to use.</p>
    #[serde(rename = "Guidance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the provisioning artifact.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a provisioning artifact (also known as a version) for a product.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisioningArtifactDetail {
    /// <p>Indicates whether the product version is active.</p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the provisioning artifact.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Information set by the administrator to provide guidance to end users about which provisioning artifacts to use.</p>
    #[serde(rename = "Guidance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the provisioning artifact.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The type of provisioning artifact.</p> <ul> <li> <p> <code>CLOUD<em>FORMATION</em>TEMPLATE</code> - AWS CloudFormation template</p> </li> <li> <p> <code>MARKETPLACE<em>AMI</code> - AWS Marketplace AMI</p> </li> <li> <p> <code>MARKETPLACE</em>CAR</code> - AWS Marketplace Clusters and AWS Resources</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Provisioning artifact output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisioningArtifactOutput {
    /// <p>Description of the provisioning artifact output key.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The provisioning artifact output key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// <p>Information about a parameter used to provision a product.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisioningArtifactParameter {
    /// <p>The default value.</p>
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>The description of the parameter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If this value is true, the value for this parameter is obfuscated from view when the parameter is retrieved. This parameter is used to hide sensitive information.</p>
    #[serde(rename = "IsNoEcho")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_no_echo: Option<bool>,
    /// <p>Constraints that the administrator has put on a parameter.</p>
    #[serde(rename = "ParameterConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_constraints: Option<ParameterConstraints>,
    /// <p>The parameter key.</p>
    #[serde(rename = "ParameterKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_key: Option<String>,
    /// <p>The parameter type.</p>
    #[serde(rename = "ParameterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<String>,
}

/// <p>The user-defined preferences that will be applied during product provisioning, unless overridden by <code>ProvisioningPreferences</code> or <code>UpdateProvisioningPreferences</code>.</p> <p>For more information on maximum concurrent accounts and failure tolerance, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-concepts.html#stackset-ops-options">Stack set operation options</a> in the <i>AWS CloudFormation User Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisioningArtifactPreferences {
    /// <p>One or more AWS accounts where stack instances are deployed from the stack set. These accounts can be scoped in <code>ProvisioningPreferences$StackSetAccounts</code> and <code>UpdateProvisioningPreferences$StackSetAccounts</code>.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p>
    #[serde(rename = "StackSetAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_accounts: Option<Vec<String>>,
    /// <p>One or more AWS Regions where stack instances are deployed from the stack set. These regions can be scoped in <code>ProvisioningPreferences$StackSetRegions</code> and <code>UpdateProvisioningPreferences$StackSetRegions</code>.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p>
    #[serde(rename = "StackSetRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_regions: Option<Vec<String>>,
}

/// <p>Information about a provisioning artifact (also known as a version) for a product.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ProvisioningArtifactProperties {
    /// <p>The description of the provisioning artifact, including how it differs from the previous provisioning artifact.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If set to true, AWS Service Catalog stops validating the specified provisioning artifact even if it is invalid.</p>
    #[serde(rename = "DisableTemplateValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_template_validation: Option<bool>,
    /// <p>Specify the template source with one of the following options, but not both. Keys accepted: [ <code>LoadTemplateFromURL</code>, <code>ImportFromPhysicalId</code> ]</p> <p>The URL of the CloudFormation template in Amazon S3. Specify the URL in JSON format as follows:</p> <p> <code>"LoadTemplateFromURL": "https://s3.amazonaws.com/cf-templates-ozkq9d3hgiq2-us-east-1/..."</code> </p> <p> <code>ImportFromPhysicalId</code>: The physical id of the resource that contains the template. Currently only supports CloudFormation stack arn. Specify the physical id in JSON format as follows: <code>ImportFromPhysicalId: “arn:aws:cloudformation:[us-east-1]:[accountId]:stack/[StackName]/[resourceId]</code> </p>
    #[serde(rename = "Info")]
    pub info: ::std::collections::HashMap<String, String>,
    /// <p>The name of the provisioning artifact (for example, v1 v2beta). No spaces are allowed.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The type of provisioning artifact.</p> <ul> <li> <p> <code>CLOUD<em>FORMATION</em>TEMPLATE</code> - AWS CloudFormation template</p> </li> <li> <p> <code>MARKETPLACE<em>AMI</code> - AWS Marketplace AMI</p> </li> <li> <p> <code>MARKETPLACE</em>CAR</code> - AWS Marketplace Clusters and AWS Resources</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Summary information about a provisioning artifact (also known as a version) for a product.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisioningArtifactSummary {
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the provisioning artifact.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the provisioning artifact.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The metadata for the provisioning artifact. This is used with AWS Marketplace products.</p>
    #[serde(rename = "ProvisioningArtifactMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_metadata: Option<::std::collections::HashMap<String, String>>,
}

/// <p>An object that contains summary information about a product view and a provisioning artifact.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisioningArtifactView {
    /// <p>Summary information about a product view.</p>
    #[serde(rename = "ProductViewSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    /// <p>Information about a provisioning artifact. A provisioning artifact is also known as a product version.</p>
    #[serde(rename = "ProvisioningArtifact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact: Option<ProvisioningArtifact>,
}

/// <p>Information about a parameter used to provision a product.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ProvisioningParameter {
    /// <p>The parameter key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The parameter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The user-defined preferences that will be applied when updating a provisioned product. Not all preferences are applicable to all provisioned product type</p> <p>One or more AWS accounts that will have access to the provisioned product.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>The AWS accounts specified should be within the list of accounts in the <code>STACKSET</code> constraint. To get the list of accounts in the <code>STACKSET</code> constraint, use the <code>DescribeProvisioningParameters</code> operation.</p> <p>If no values are specified, the default value is all accounts from the <code>STACKSET</code> constraint.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ProvisioningPreferences {
    /// <p>One or more AWS accounts where the provisioned product will be available.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>The specified accounts should be within the list of accounts from the <code>STACKSET</code> constraint. To get the list of accounts in the <code>STACKSET</code> constraint, use the <code>DescribeProvisioningParameters</code> operation.</p> <p>If no values are specified, the default value is all acounts from the <code>STACKSET</code> constraint.</p>
    #[serde(rename = "StackSetAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_accounts: Option<Vec<String>>,
    /// <p>The number of accounts, per region, for which this operation can fail before AWS Service Catalog stops the operation in that region. If the operation is stopped in a region, AWS Service Catalog doesn't attempt the operation in any subsequent regions.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetFailureToleranceCount</code> or <code>StackSetFailureTolerancePercentage</code>, but not both.</p> <p>The default value is <code>0</code> if no value is specified.</p>
    #[serde(rename = "StackSetFailureToleranceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_count: Option<i64>,
    /// <p>The percentage of accounts, per region, for which this stack operation can fail before AWS Service Catalog stops the operation in that region. If the operation is stopped in a region, AWS Service Catalog doesn't attempt the operation in any subsequent regions.</p> <p>When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetFailureToleranceCount</code> or <code>StackSetFailureTolerancePercentage</code>, but not both.</p>
    #[serde(rename = "StackSetFailureTolerancePercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_percentage: Option<i64>,
    /// <p>The maximum number of accounts in which to perform this operation at one time. This is dependent on the value of <code>StackSetFailureToleranceCount</code>. <code>StackSetMaxConcurrentCount</code> is at most one more than the <code>StackSetFailureToleranceCount</code>.</p> <p>Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetMaxConcurrentCount</code> or <code>StackSetMaxConcurrentPercentage</code>, but not both.</p>
    #[serde(rename = "StackSetMaxConcurrencyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_count: Option<i64>,
    /// <p>The maximum percentage of accounts in which to perform this operation at one time.</p> <p>When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number. This is true except in cases where rounding down would result is zero. In this case, AWS Service Catalog sets the number as <code>1</code> instead.</p> <p>Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetMaxConcurrentCount</code> or <code>StackSetMaxConcurrentPercentage</code>, but not both.</p>
    #[serde(rename = "StackSetMaxConcurrencyPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_percentage: Option<i64>,
    /// <p>One or more AWS Regions where the provisioned product will be available.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>The specified regions should be within the list of regions from the <code>STACKSET</code> constraint. To get the list of regions in the <code>STACKSET</code> constraint, use the <code>DescribeProvisioningParameters</code> operation.</p> <p>If no values are specified, the default value is all regions from the <code>STACKSET</code> constraint.</p>
    #[serde(rename = "StackSetRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_regions: Option<Vec<String>>,
}

/// <p>Information about a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecordDetail {
    /// <p>The UTC time stamp of the creation time.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The ARN of the launch role associated with the provisioned product.</p>
    #[serde(rename = "LaunchRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_role_arn: Option<String>,
    /// <p>The path identifier.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    /// <p>The user-friendly name of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    /// <p>The type of provisioned product. The supported values are <code>CFN_STACK</code> and <code>CFN_STACKSET</code>.</p>
    #[serde(rename = "ProvisionedProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_type: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>The errors that occurred.</p>
    #[serde(rename = "RecordErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_errors: Option<Vec<RecordError>>,
    /// <p>The identifier of the record.</p>
    #[serde(rename = "RecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "RecordTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_tags: Option<Vec<RecordTag>>,
    /// <p><p>The record type.</p> <ul> <li> <p> <code>PROVISION<em>PRODUCT</code> </p> </li> <li> <p> <code>UPDATE</em>PROVISIONED<em>PRODUCT</code> </p> </li> <li> <p> <code>TERMINATE</em>PROVISIONED_PRODUCT</code> </p> </li> </ul></p>
    #[serde(rename = "RecordType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
    /// <p><p>The status of the provisioned product.</p> <ul> <li> <p> <code>CREATED</code> - The request was created but the operation has not started.</p> </li> <li> <p> <code>IN<em>PROGRESS</code> - The requested operation is in progress.</p> </li> <li> <p> <code>IN</em>PROGRESS<em>IN</em>ERROR</code> - The provisioned product is under change but the requested operation failed and some remediation is occurring. For example, a rollback.</p> </li> <li> <p> <code>SUCCEEDED</code> - The requested operation has successfully completed.</p> </li> <li> <p> <code>FAILED</code> - The requested operation has unsuccessfully completed. Investigate using the error messages returned.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The time when the record was last updated.</p>
    #[serde(rename = "UpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<f64>,
}

/// <p>The error code and description resulting from an operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecordError {
    /// <p>The numeric value of the error.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The description of the error.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p>The output for the product created as the result of a request. For example, the output for a CloudFormation-backed product that creates an S3 bucket would include the S3 bucket URL.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecordOutput {
    /// <p>The description of the output.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The output key.</p>
    #[serde(rename = "OutputKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_key: Option<String>,
    /// <p>The output value.</p>
    #[serde(rename = "OutputValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_value: Option<String>,
}

/// <p>Information about a tag, which is a key-value pair.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecordTag {
    /// <p>The key for this tag.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value for this tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// see [ServiceCatalog::reject_portfolio_share]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RejectPortfolioShareInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>The type of shared portfolios to reject. The default is to reject imported portfolios.</p> <ul> <li> <p> <code>AWS_ORGANIZATIONS</code> - Reject portfolios shared by the management account of your organization.</p> </li> <li> <p> <code>IMPORTED</code> - Reject imported portfolios.</p> </li> <li> <p> <code>AWS_SERVICECATALOG</code> - Not supported. (Throws ResourceNotFoundException.)</p> </li> </ul> <p>For example, <code>aws servicecatalog reject-portfolio-share --portfolio-id "port-2qwzkwxt3y5fk" --portfolio-share-type AWS_ORGANIZATIONS</code> </p>
    #[serde(rename = "PortfolioShareType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_type: Option<String>,
}

/// see [ServiceCatalog::reject_portfolio_share]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RejectPortfolioShareOutput {}

/// <p>Information about a resource change that will occur when a plan is executed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceChange {
    /// <p>The change action.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>Information about the resource changes.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<ResourceChangeDetail>>,
    /// <p>The ID of the resource, as defined in the CloudFormation template.</p>
    #[serde(rename = "LogicalResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    /// <p>The ID of the resource, if it was already created.</p>
    #[serde(rename = "PhysicalResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    /// <p>If the change type is <code>Modify</code>, indicates whether the existing resource is deleted and replaced with a new one.</p>
    #[serde(rename = "Replacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    /// <p>The type of resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The change scope.</p>
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
}

/// <p>Information about a change to a resource attribute.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceChangeDetail {
    /// <p>The ID of the entity that caused the change.</p>
    #[serde(rename = "CausingEntity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causing_entity: Option<String>,
    /// <p>For static evaluations, the value of the resource attribute will change and the new value is known. For dynamic evaluations, the value might change, and any new value will be determined when the plan is updated.</p>
    #[serde(rename = "Evaluation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<String>,
    /// <p>Information about the resource attribute to be modified.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ResourceTargetDefinition>,
}

/// <p>Information about a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceDetail {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The creation time of the resource.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The description of the resource.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the resource.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the resource.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a change to a resource attribute.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceTargetDefinition {
    /// <p>The attribute to be changed.</p>
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    /// <p>If the attribute is <code>Properties</code>, the value is the name of the property. Otherwise, the value is null.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If the attribute is <code>Properties</code>, indicates whether a change to this property causes the resource to be re-created.</p>
    #[serde(rename = "RequiresRecreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_recreation: Option<String>,
}

/// see [ServiceCatalog::scan_provisioned_products]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ScanProvisionedProductsInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    #[serde(rename = "AccessLevelFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Paged for ScanProvisionedProductsInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for ScanProvisionedProductsInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::scan_provisioned_products]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScanProvisionedProductsOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the provisioned products.</p>
    #[serde(rename = "ProvisionedProducts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_products: Option<Vec<ProvisionedProductDetail>>,
}

impl Paged for ScanProvisionedProductsOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for ScanProvisionedProductsOutput {
    type Item = ProvisionedProductDetail;

    fn into_pagination_page(self) -> Vec<ProvisionedProductDetail> {
        self.provisioned_products.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::search_products_as_admin]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchProductsAsAdminInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The search filters. If no search filters are specified, the output includes all products to which the administrator has access.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "PortfolioId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_id: Option<String>,
    /// <p>Access level of the source of the product.</p>
    #[serde(rename = "ProductSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_source: Option<String>,
    /// <p>The sort field. If no value is specified, the results are not sorted.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order. If no value is specified, the results are not sorted.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl Paged for SearchProductsAsAdminInput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.page_token)
    }
}

impl PagedRequest for SearchProductsAsAdminInput {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.page_token = key;
    }
}

/// see [ServiceCatalog::search_products_as_admin]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchProductsAsAdminOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the product views.</p>
    #[serde(rename = "ProductViewDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_details: Option<Vec<ProductViewDetail>>,
}

impl Paged for SearchProductsAsAdminOutput {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_page_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_page_token)
    }
}

impl PagedOutput for SearchProductsAsAdminOutput {
    type Item = ProductViewDetail;

    fn into_pagination_page(self) -> Vec<ProductViewDetail> {
        self.product_view_details.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [ServiceCatalog::search_products]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchProductsInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The search filters. If no search filters are specified, the output includes all products to which the caller has access.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The sort field. If no value is specified, the results are not sorted.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order. If no value is specified, the results are not sorted.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// see [ServiceCatalog::search_products]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchProductsOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The product view aggregations.</p>
    #[serde(rename = "ProductViewAggregations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_aggregations:
        Option<::std::collections::HashMap<String, Vec<ProductViewAggregationValue>>>,
    /// <p>Information about the product views.</p>
    #[serde(rename = "ProductViewSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summaries: Option<Vec<ProductViewSummary>>,
}

/// see [ServiceCatalog::search_provisioned_products]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchProvisionedProductsInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    #[serde(rename = "AccessLevelFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    /// <p>The search filters.</p> <p>When the key is <code>SearchQuery</code>, the searchable fields are <code>arn</code>, <code>createdTime</code>, <code>id</code>, <code>lastRecordId</code>, <code>idempotencyToken</code>, <code>name</code>, <code>physicalId</code>, <code>productId</code>, <code>provisioningArtifact</code>, <code>type</code>, <code>status</code>, <code>tags</code>, <code>userArn</code>, <code>userArnSession</code>, <code>lastProvisioningRecordId</code>, <code>lastSuccessfulProvisioningRecordId</code>, <code>productName</code>, and <code>provisioningArtifactName</code>.</p> <p>Example: <code>"SearchQuery":["status:AVAILABLE"]</code> </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of items to return with this call.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    #[serde(rename = "PageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The sort field. If no value is specified, the results are not sorted. The valid values are <code>arn</code>, <code>id</code>, <code>name</code>, and <code>lastRecordId</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order. If no value is specified, the results are not sorted.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// see [ServiceCatalog::search_provisioned_products]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchProvisionedProductsOutput {
    /// <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Information about the provisioned products.</p>
    #[serde(rename = "ProvisionedProducts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_products: Option<Vec<ProvisionedProductAttribute>>,
    /// <p>The number of provisioned products found.</p>
    #[serde(rename = "TotalResultsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_results_count: Option<i64>,
}

/// <p>A self-service action association consisting of the Action ID, the Product ID, and the Provisioning Artifact ID.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ServiceActionAssociation {
    /// <p>The product identifier. For example, <code>prod-abcdzk7xy33qa</code>.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact. For example, <code>pa-4abcdjnxjj6ne</code>.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
    /// <p>The self-service action identifier. For example, <code>act-fs7abcd89wxyz</code>.</p>
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
}

/// <p>An object containing detailed information about the self-service action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceActionDetail {
    /// <p>A map that defines the self-service action.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<::std::collections::HashMap<String, String>>,
    /// <p>Summary information about the self-service action.</p>
    #[serde(rename = "ServiceActionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_summary: Option<ServiceActionSummary>,
}

/// <p>Detailed information about the self-service action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceActionSummary {
    /// <p>The self-service action definition type. For example, <code>SSM_AUTOMATION</code>.</p>
    #[serde(rename = "DefinitionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_type: Option<String>,
    /// <p>The self-service action description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The self-service action identifier.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The self-service action name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about the portfolio share operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ShareDetails {
    /// <p>List of errors.</p>
    #[serde(rename = "ShareErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_errors: Option<Vec<ShareError>>,
    /// <p>List of accounts for whom the operation succeeded.</p>
    #[serde(rename = "SuccessfulShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_shares: Option<Vec<String>>,
}

/// <p>Errors that occurred during the portfolio share operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ShareError {
    /// <p>List of accounts impacted by the error.</p>
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    /// <p>Error type that happened when processing the operation.</p>
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>Information about the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>An AWS CloudFormation stack, in a specific account and region, that's part of a stack set operation. A stack instance is a reference to an attempted or actual stack in a given account within a given region. A stack instance can exist without a stack—for example, if the stack couldn't be created for some reason. A stack instance is associated with only one stack set. Each stack instance contains the ID of its associated stack set, as well as the ID of the actual stack and the stack status. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StackInstance {
    /// <p>The name of the AWS account that the stack instance is associated with.</p>
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// <p>The name of the AWS region that the stack instance is associated with.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p><p>The status of the stack instance, in terms of its synchronization with its associated stack set. </p> <ul> <li> <p> <code>INOPERABLE</code>: A <code>DeleteStackInstances</code> operation has failed and left the stack in an unstable state. Stacks in this state are excluded from further <code>UpdateStackSet</code> operations. You might need to perform a <code>DeleteStackInstances</code> operation, with <code>RetainStacks</code> set to true, to delete the stack instance, and then delete the stack manually. </p> </li> <li> <p> <code>OUTDATED</code>: The stack isn&#39;t currently up to date with the stack set because either the associated stack failed during a <code>CreateStackSet</code> or <code>UpdateStackSet</code> operation, or the stack was part of a <code>CreateStackSet</code> or <code>UpdateStackSet</code> operation that failed or was stopped before the stack was created or updated.</p> </li> <li> <p> <code>CURRENT</code>: The stack is currently up to date with the stack set.</p> </li> </ul></p>
    #[serde(rename = "StackInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_instance_status: Option<String>,
}

/// <p>Information about a tag. A tag is a key-value pair. Tags are propagated to the resources created when provisioning a product.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The tag key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value for this key.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Information about a TagOption.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagOptionDetail {
    /// <p>The TagOption active state.</p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The TagOption key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The AWS account Id of the owner account that created the TagOption.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The TagOption value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Summary information about a TagOption.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagOptionSummary {
    /// <p>The TagOption key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The TagOption value.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// see [ServiceCatalog::terminate_provisioned_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TerminateProvisionedProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>If set to true, AWS Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources.</p>
    #[serde(rename = "IgnoreErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_errors: Option<bool>,
    /// <p>The identifier of the provisioned product. You cannot specify both <code>ProvisionedProductName</code> and <code>ProvisionedProductId</code>.</p>
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    /// <p>The name of the provisioned product. You cannot specify both <code>ProvisionedProductName</code> and <code>ProvisionedProductId</code>.</p>
    #[serde(rename = "ProvisionedProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    /// <p>When this boolean parameter is set to true, the <code>TerminateProvisionedProduct</code> API deletes the Service Catalog provisioned product. However, it does not remove the CloudFormation stack, stack set, or the underlying resources of the deleted provisioned product. The default value is false.</p>
    #[serde(rename = "RetainPhysicalResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_physical_resources: Option<bool>,
    /// <p>An idempotency token that uniquely identifies the termination request. This token is only valid during the termination process. After the provisioned product is terminated, subsequent requests to terminate the same provisioned product always return <b>ResourceNotFound</b>.</p>
    #[serde(rename = "TerminateToken")]
    pub terminate_token: String,
}

/// see [ServiceCatalog::terminate_provisioned_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TerminateProvisionedProductOutput {
    /// <p>Information about the result of this request.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

/// see [ServiceCatalog::update_constraint]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConstraintInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The updated description of the constraint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the constraint.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p><p>The constraint parameters, in JSON format. The syntax depends on the constraint type as follows:</p> <dl> <dt>LAUNCH</dt> <dd> <p>You are required to specify either the <code>RoleArn</code> or the <code>LocalRoleName</code> but can&#39;t use both.</p> <p>Specify the <code>RoleArn</code> property as follows:</p> <p> <code>{&quot;RoleArn&quot; : &quot;arn:aws:iam::123456789012:role/LaunchRole&quot;}</code> </p> <p>Specify the <code>LocalRoleName</code> property as follows:</p> <p> <code>{&quot;LocalRoleName&quot;: &quot;SCBasicLaunchRole&quot;}</code> </p> <p>If you specify the <code>LocalRoleName</code> property, when an account uses the launch constraint, the IAM role with that name in the account will be used. This allows launch-role constraints to be account-agnostic so the administrator can create fewer resources per shared account.</p> <note> <p>The given role name must exist in the account used to create the launch constraint and the account of the user who launches a product with this launch constraint.</p> </note> <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p> <p>You also cannot have more than one <code>LAUNCH</code> constraint on a product and portfolio.</p> </dd> <dt>NOTIFICATION</dt> <dd> <p>Specify the <code>NotificationArns</code> property as follows:</p> <p> <code>{&quot;NotificationArns&quot; : [&quot;arn:aws:sns:us-east-1:123456789012:Topic&quot;]}</code> </p> </dd> <dt>RESOURCE<em>UPDATE</dt> <dd> <p>Specify the <code>TagUpdatesOnProvisionedProduct</code> property as follows:</p> <p> <code>{&quot;Version&quot;:&quot;2.0&quot;,&quot;Properties&quot;:{&quot;TagUpdateOnProvisionedProduct&quot;:&quot;String&quot;}}</code> </p> <p>The <code>TagUpdatesOnProvisionedProduct</code> property accepts a string value of <code>ALLOWED</code> or <code>NOT</em>ALLOWED</code>.</p> </dd> <dt>STACKSET</dt> <dd> <p>Specify the <code>Parameters</code> property as follows:</p> <p> <code>{&quot;Version&quot;: &quot;String&quot;, &quot;Properties&quot;: {&quot;AccountList&quot;: [ &quot;String&quot; ], &quot;RegionList&quot;: [ &quot;String&quot; ], &quot;AdminRole&quot;: &quot;String&quot;, &quot;ExecutionRole&quot;: &quot;String&quot;}}</code> </p> <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p> <p>You also cannot have more than one <code>STACKSET</code> constraint on a product and portfolio.</p> <p>Products with a <code>STACKSET</code> constraint will launch an AWS CloudFormation stack set.</p> </dd> <dt>TEMPLATE</dt> <dd> <p>Specify the <code>Rules</code> property. For more information, see <a href="http://docs.aws.amazon.com/servicecatalog/latest/adminguide/reference-template_constraint_rules.html">Template Constraint Rules</a>.</p> </dd> </dl></p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
}

/// see [ServiceCatalog::update_constraint]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConstraintOutput {
    /// <p>Information about the constraint.</p>
    #[serde(rename = "ConstraintDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    /// <p>The constraint parameters.</p>
    #[serde(rename = "ConstraintParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_parameters: Option<String>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ServiceCatalog::update_portfolio]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePortfolioInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The tags to add.</p>
    #[serde(rename = "AddTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<Vec<Tag>>,
    /// <p>The updated description of the portfolio.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name to use for display purposes.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The portfolio identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The updated name of the portfolio provider.</p>
    #[serde(rename = "ProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// <p>The tags to remove.</p>
    #[serde(rename = "RemoveTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
}

/// see [ServiceCatalog::update_portfolio]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePortfolioOutput {
    /// <p>Information about the portfolio.</p>
    #[serde(rename = "PortfolioDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    /// <p>Information about the tags associated with the portfolio.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [ServiceCatalog::update_portfolio_share]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePortfolioShareInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The AWS Account Id of the recipient account. This field is required when updating an external account to account type share.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "OrganizationNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_node: Option<OrganizationNode>,
    /// <p>The unique identifier of the portfolio for which the share will be updated.</p>
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// <p>A flag to enable or disable TagOptions sharing for the portfolio share. If this field is not provided, the current state of TagOptions sharing on the portfolio share will not be modified.</p>
    #[serde(rename = "ShareTagOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_tag_options: Option<bool>,
}

/// see [ServiceCatalog::update_portfolio_share]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePortfolioShareOutput {
    /// <p>The token that tracks the status of the <code>UpdatePortfolioShare</code> operation for external account to account or organizational type sharing.</p>
    #[serde(rename = "PortfolioShareToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_token: Option<String>,
    /// <p>The status of <code>UpdatePortfolioShare</code> operation. You can also obtain the operation status using <code>DescribePortfolioShareStatus</code> API. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ServiceCatalog::update_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The tags to add to the product.</p>
    #[serde(rename = "AddTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<Vec<Tag>>,
    /// <p>The updated description of the product.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated distributor of the product.</p>
    #[serde(rename = "Distributor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The updated product name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The updated owner of the product.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The tags to remove from the product.</p>
    #[serde(rename = "RemoveTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
    /// <p>The updated support description for the product.</p>
    #[serde(rename = "SupportDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_description: Option<String>,
    /// <p>The updated support email for the product.</p>
    #[serde(rename = "SupportEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// <p>The updated support URL for the product.</p>
    #[serde(rename = "SupportUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
}

/// see [ServiceCatalog::update_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProductOutput {
    /// <p>Information about the product view.</p>
    #[serde(rename = "ProductViewDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    /// <p>Information about the tags associated with the product.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [ServiceCatalog::update_provisioned_product]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProvisionedProductInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The path identifier. This value is optional if the product has a default path, and required if the product has more than one path. You must provide the name or ID, but not both.</p>
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// <p>The name of the path. You must provide the name or ID, but not both.</p>
    #[serde(rename = "PathName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_name: Option<String>,
    /// <p>The identifier of the product. You must provide the name or ID, but not both.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>The name of the product. You must provide the name or ID, but not both.</p>
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// <p>The identifier of the provisioned product. You must provide the name or ID, but not both.</p>
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    /// <p>The name of the provisioned product. You cannot specify both <code>ProvisionedProductName</code> and <code>ProvisionedProductId</code>.</p>
    #[serde(rename = "ProvisionedProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    /// <p>The name of the provisioning artifact. You must provide the name or ID, but not both.</p>
    #[serde(rename = "ProvisioningArtifactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_name: Option<String>,
    /// <p>The new parameters.</p>
    #[serde(rename = "ProvisioningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<UpdateProvisioningParameter>>,
    /// <p>An object that contains information about the provisioning preferences for a stack set.</p>
    #[serde(rename = "ProvisioningPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_preferences: Option<UpdateProvisioningPreferences>,
    /// <p>One or more tags. Requires the product to have <code>RESOURCE_UPDATE</code> constraint with <code>TagUpdatesOnProvisionedProduct</code> set to <code>ALLOWED</code> to allow tag updates.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The idempotency token that uniquely identifies the provisioning update request.</p>
    #[serde(rename = "UpdateToken")]
    pub update_token: String,
}

/// see [ServiceCatalog::update_provisioned_product]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProvisionedProductOutput {
    /// <p>Information about the result of the request.</p>
    #[serde(rename = "RecordDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

/// see [ServiceCatalog::update_provisioned_product_properties]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProvisionedProductPropertiesInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>The idempotency token that uniquely identifies the provisioning product update request.</p>
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    /// <p>The identifier of the provisioned product.</p>
    #[serde(rename = "ProvisionedProductId")]
    pub provisioned_product_id: String,
    /// <p>A map that contains the provisioned product properties to be updated.</p> <p>The <code>LAUNCH_ROLE</code> key accepts role ARNs. This key allows an administrator to call <code>UpdateProvisionedProductProperties</code> to update the launch role that is associated with a provisioned product. This role is used when an end user calls a provisioning operation such as <code>UpdateProvisionedProduct</code>, <code>TerminateProvisionedProduct</code>, or <code>ExecuteProvisionedProductServiceAction</code>. Only a role ARN is valid. A user ARN is invalid. </p> <p>The <code>OWNER</code> key accepts user ARNs and role ARNs. The owner is the user that has permission to see, update, terminate, and execute service actions in the provisioned product.</p> <p>The administrator can change the owner of a provisioned product to another IAM user within the same account. Both end user owners and administrators can see ownership history of the provisioned product using the <code>ListRecordHistory</code> API. The new owner can describe all past records for the provisioned product using the <code>DescribeRecord</code> API. The previous owner can no longer use <code>DescribeRecord</code>, but can still see the product's history from when he was an owner using <code>ListRecordHistory</code>.</p> <p>If a provisioned product ownership is assigned to an end user, they can see and perform any action through the API or Service Catalog console such as update, terminate, and execute service actions. If an end user provisions a product and the owner is updated to someone else, they will no longer be able to see or perform any actions through API or the Service Catalog console on that provisioned product.</p>
    #[serde(rename = "ProvisionedProductProperties")]
    pub provisioned_product_properties: ::std::collections::HashMap<String, String>,
}

/// see [ServiceCatalog::update_provisioned_product_properties]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProvisionedProductPropertiesOutput {
    /// <p>The provisioned product identifier.</p>
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    /// <p>A map that contains the properties updated.</p>
    #[serde(rename = "ProvisionedProductProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_properties: Option<::std::collections::HashMap<String, String>>,
    /// <p>The identifier of the record.</p>
    #[serde(rename = "RecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    /// <p>The status of the request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [ServiceCatalog::update_provisioning_artifact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProvisioningArtifactInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>Indicates whether the product version is active.</p> <p>Inactive provisioning artifacts are invisible to end users. End users cannot launch or update a provisioned product from an inactive provisioning artifact.</p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p>The updated description of the provisioning artifact.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Information set by the administrator to provide guidance to end users about which provisioning artifacts to use.</p> <p>The <code>DEFAULT</code> value indicates that the product version is active.</p> <p>The administrator can set the guidance to <code>DEPRECATED</code> to inform users that the product version is deprecated. Users are able to make updates to a provisioned product of a deprecated version but cannot launch new provisioned products using a deprecated version.</p>
    #[serde(rename = "Guidance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
    /// <p>The updated name of the provisioning artifact.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The product identifier.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The identifier of the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,
}

/// see [ServiceCatalog::update_provisioning_artifact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProvisioningArtifactOutput {
    /// <p>The URL of the CloudFormation template in Amazon S3.</p>
    #[serde(rename = "Info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<::std::collections::HashMap<String, String>>,
    /// <p>Information about the provisioning artifact.</p>
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    /// <p>The status of the current request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The parameter key-value pair used to update a provisioned product.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UpdateProvisioningParameter {
    /// <p>The parameter key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>If set to true, <code>Value</code> is ignored and the previous parameter value is kept.</p>
    #[serde(rename = "UsePreviousValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_previous_value: Option<bool>,
    /// <p>The parameter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The user-defined preferences that will be applied when updating a provisioned product. Not all preferences are applicable to all provisioned product types.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProvisioningPreferences {
    /// <p>One or more AWS accounts that will have access to the provisioned product.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>The AWS accounts specified should be within the list of accounts in the <code>STACKSET</code> constraint. To get the list of accounts in the <code>STACKSET</code> constraint, use the <code>DescribeProvisioningParameters</code> operation.</p> <p>If no values are specified, the default value is all accounts from the <code>STACKSET</code> constraint.</p>
    #[serde(rename = "StackSetAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_accounts: Option<Vec<String>>,
    /// <p>The number of accounts, per region, for which this operation can fail before AWS Service Catalog stops the operation in that region. If the operation is stopped in a region, AWS Service Catalog doesn't attempt the operation in any subsequent regions.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetFailureToleranceCount</code> or <code>StackSetFailureTolerancePercentage</code>, but not both.</p> <p>The default value is <code>0</code> if no value is specified.</p>
    #[serde(rename = "StackSetFailureToleranceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_count: Option<i64>,
    /// <p>The percentage of accounts, per region, for which this stack operation can fail before AWS Service Catalog stops the operation in that region. If the operation is stopped in a region, AWS Service Catalog doesn't attempt the operation in any subsequent regions.</p> <p>When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetFailureToleranceCount</code> or <code>StackSetFailureTolerancePercentage</code>, but not both.</p>
    #[serde(rename = "StackSetFailureTolerancePercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_percentage: Option<i64>,
    /// <p>The maximum number of accounts in which to perform this operation at one time. This is dependent on the value of <code>StackSetFailureToleranceCount</code>. <code>StackSetMaxConcurrentCount</code> is at most one more than the <code>StackSetFailureToleranceCount</code>.</p> <p>Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetMaxConcurrentCount</code> or <code>StackSetMaxConcurrentPercentage</code>, but not both.</p>
    #[serde(rename = "StackSetMaxConcurrencyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_count: Option<i64>,
    /// <p>The maximum percentage of accounts in which to perform this operation at one time.</p> <p>When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number. This is true except in cases where rounding down would result is zero. In this case, AWS Service Catalog sets the number as <code>1</code> instead.</p> <p>Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>Conditional: You must specify either <code>StackSetMaxConcurrentCount</code> or <code>StackSetMaxConcurrentPercentage</code>, but not both.</p>
    #[serde(rename = "StackSetMaxConcurrencyPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_percentage: Option<i64>,
    /// <p><p>Determines what action AWS Service Catalog performs to a stack set or a stack instance represented by the provisioned product. The default value is <code>UPDATE</code> if nothing is specified.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <dl> <dt>CREATE</dt> <dd> <p>Creates a new stack instance in the stack set represented by the provisioned product. In this case, only new stack instances are created based on accounts and regions; if new ProductId or ProvisioningArtifactID are passed, they will be ignored.</p> </dd> <dt>UPDATE</dt> <dd> <p>Updates the stack set represented by the provisioned product and also its stack instances.</p> </dd> <dt>DELETE</dt> <dd> <p>Deletes a stack instance in the stack set represented by the provisioned product.</p> </dd> </dl></p>
    #[serde(rename = "StackSetOperationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_operation_type: Option<String>,
    /// <p>One or more AWS Regions where the provisioned product will be available.</p> <p>Applicable only to a <code>CFN_STACKSET</code> provisioned product type.</p> <p>The specified regions should be within the list of regions from the <code>STACKSET</code> constraint. To get the list of regions in the <code>STACKSET</code> constraint, use the <code>DescribeProvisioningParameters</code> operation.</p> <p>If no values are specified, the default value is all regions from the <code>STACKSET</code> constraint.</p>
    #[serde(rename = "StackSetRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_regions: Option<Vec<String>>,
}

/// see [ServiceCatalog::update_service_action]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateServiceActionInput {
    /// <p><p>The language code.</p> <ul> <li> <p> <code>en</code> - English (default)</p> </li> <li> <p> <code>jp</code> - Japanese</p> </li> <li> <p> <code>zh</code> - Chinese</p> </li> </ul></p>
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// <p>A map that defines the self-service action.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<::std::collections::HashMap<String, String>>,
    /// <p>The self-service action description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The self-service action identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The self-service action name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// see [ServiceCatalog::update_service_action]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateServiceActionOutput {
    /// <p>Detailed information about the self-service action.</p>
    #[serde(rename = "ServiceActionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_detail: Option<ServiceActionDetail>,
}

/// see [ServiceCatalog::update_tag_option]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTagOptionInput {
    /// <p>The updated active state.</p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p>The TagOption identifier.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The updated value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// see [ServiceCatalog::update_tag_option]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTagOptionOutput {
    /// <p>Information about the TagOption.</p>
    #[serde(rename = "TagOptionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}

/// <p>Additional information provided by the administrator.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UsageInstruction {
    /// <p>The usage instruction type for the value.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The usage instruction value for this type.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Errors returned by AcceptPortfolioShare
#[derive(Debug, PartialEq)]
pub enum AcceptPortfolioShareError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AcceptPortfolioShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptPortfolioShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(AcceptPortfolioShareError::InvalidParameters(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AcceptPortfolioShareError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AcceptPortfolioShareError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptPortfolioShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptPortfolioShareError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            AcceptPortfolioShareError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AcceptPortfolioShareError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AcceptPortfolioShareError {}
/// Errors returned by AssociateBudgetWithResource
#[derive(Debug, PartialEq)]
pub enum AssociateBudgetWithResourceError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AssociateBudgetWithResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateBudgetWithResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(
                        AssociateBudgetWithResourceError::DuplicateResource(err.msg),
                    )
                }
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        AssociateBudgetWithResourceError::InvalidParameters(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AssociateBudgetWithResourceError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateBudgetWithResourceError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateBudgetWithResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateBudgetWithResourceError::DuplicateResource(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateBudgetWithResourceError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateBudgetWithResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AssociateBudgetWithResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateBudgetWithResourceError {}
/// Errors returned by AssociatePrincipalWithPortfolio
#[derive(Debug, PartialEq)]
pub enum AssociatePrincipalWithPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AssociatePrincipalWithPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociatePrincipalWithPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        AssociatePrincipalWithPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        AssociatePrincipalWithPortfolioError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociatePrincipalWithPortfolioError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociatePrincipalWithPortfolioError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociatePrincipalWithPortfolioError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePrincipalWithPortfolioError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePrincipalWithPortfolioError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociatePrincipalWithPortfolioError {}
/// Errors returned by AssociateProductWithPortfolio
#[derive(Debug, PartialEq)]
pub enum AssociateProductWithPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AssociateProductWithPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateProductWithPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        AssociateProductWithPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AssociateProductWithPortfolioError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateProductWithPortfolioError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateProductWithPortfolioError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateProductWithPortfolioError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateProductWithPortfolioError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AssociateProductWithPortfolioError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateProductWithPortfolioError {}
/// Errors returned by AssociateServiceActionWithProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum AssociateServiceActionWithProvisioningArtifactError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AssociateServiceActionWithProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateServiceActionWithProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(
                        AssociateServiceActionWithProvisioningArtifactError::DuplicateResource(
                            err.msg,
                        ),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        AssociateServiceActionWithProvisioningArtifactError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateServiceActionWithProvisioningArtifactError::ResourceNotFound(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateServiceActionWithProvisioningArtifactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateServiceActionWithProvisioningArtifactError::DuplicateResource(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateServiceActionWithProvisioningArtifactError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateServiceActionWithProvisioningArtifactError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateServiceActionWithProvisioningArtifactError {}
/// Errors returned by AssociateTagOptionWithResource
#[derive(Debug, PartialEq)]
pub enum AssociateTagOptionWithResourceError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl AssociateTagOptionWithResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateTagOptionWithResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(
                        AssociateTagOptionWithResourceError::DuplicateResource(err.msg),
                    )
                }
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        AssociateTagOptionWithResourceError::InvalidParameters(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(AssociateTagOptionWithResourceError::InvalidState(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        AssociateTagOptionWithResourceError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateTagOptionWithResourceError::ResourceNotFound(err.msg),
                    )
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(
                        AssociateTagOptionWithResourceError::TagOptionNotMigrated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateTagOptionWithResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateTagOptionWithResourceError::DuplicateResource(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateTagOptionWithResourceError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateTagOptionWithResourceError::InvalidState(ref cause) => write!(f, "{}", cause),
            AssociateTagOptionWithResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AssociateTagOptionWithResourceError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateTagOptionWithResourceError::TagOptionNotMigrated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateTagOptionWithResourceError {}
/// Errors returned by BatchAssociateServiceActionWithProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum BatchAssociateServiceActionWithProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl BatchAssociateServiceActionWithProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchAssociateServiceActionWithProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        BatchAssociateServiceActionWithProvisioningArtifactError::InvalidParameters(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchAssociateServiceActionWithProvisioningArtifactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchAssociateServiceActionWithProvisioningArtifactError::InvalidParameters(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchAssociateServiceActionWithProvisioningArtifactError {}
/// Errors returned by BatchDisassociateServiceActionFromProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum BatchDisassociateServiceActionFromProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl BatchDisassociateServiceActionFromProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchDisassociateServiceActionFromProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => return RusotoError::Service(
                    BatchDisassociateServiceActionFromProvisioningArtifactError::InvalidParameters(
                        err.msg,
                    ),
                ),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchDisassociateServiceActionFromProvisioningArtifactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDisassociateServiceActionFromProvisioningArtifactError::InvalidParameters(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDisassociateServiceActionFromProvisioningArtifactError {}
/// Errors returned by CopyProduct
#[derive(Debug, PartialEq)]
pub enum CopyProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CopyProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopyProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(CopyProductError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CopyProductError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CopyProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CopyProductError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            CopyProductError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CopyProductError {}
/// Errors returned by CreateConstraint
#[derive(Debug, PartialEq)]
pub enum CreateConstraintError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateConstraintError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConstraintError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(CreateConstraintError::DuplicateResource(err.msg))
                }
                "InvalidParametersException" => {
                    return RusotoError::Service(CreateConstraintError::InvalidParameters(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateConstraintError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateConstraintError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateConstraintError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConstraintError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            CreateConstraintError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            CreateConstraintError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateConstraintError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConstraintError {}
/// Errors returned by CreatePortfolio
#[derive(Debug, PartialEq)]
pub enum CreatePortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl CreatePortfolioError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(CreatePortfolioError::InvalidParameters(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreatePortfolioError::LimitExceeded(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(CreatePortfolioError::TagOptionNotMigrated(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePortfolioError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePortfolioError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            CreatePortfolioError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreatePortfolioError::TagOptionNotMigrated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePortfolioError {}
/// Errors returned by CreatePortfolioShare
#[derive(Debug, PartialEq)]
pub enum CreatePortfolioShareError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreatePortfolioShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePortfolioShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(CreatePortfolioShareError::InvalidParameters(
                        err.msg,
                    ))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(CreatePortfolioShareError::InvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreatePortfolioShareError::LimitExceeded(err.msg))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(CreatePortfolioShareError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreatePortfolioShareError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePortfolioShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePortfolioShareError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            CreatePortfolioShareError::InvalidState(ref cause) => write!(f, "{}", cause),
            CreatePortfolioShareError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreatePortfolioShareError::OperationNotSupported(ref cause) => write!(f, "{}", cause),
            CreatePortfolioShareError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePortfolioShareError {}
/// Errors returned by CreateProduct
#[derive(Debug, PartialEq)]
pub enum CreateProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl CreateProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(CreateProductError::InvalidParameters(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateProductError::LimitExceeded(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(CreateProductError::TagOptionNotMigrated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProductError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            CreateProductError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateProductError::TagOptionNotMigrated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProductError {}
/// Errors returned by CreateProvisionedProductPlan
#[derive(Debug, PartialEq)]
pub enum CreateProvisionedProductPlanError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateProvisionedProductPlanError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateProvisionedProductPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        CreateProvisionedProductPlanError::InvalidParameters(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(CreateProvisionedProductPlanError::InvalidState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        CreateProvisionedProductPlanError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateProvisionedProductPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProvisionedProductPlanError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateProvisionedProductPlanError::InvalidState(ref cause) => write!(f, "{}", cause),
            CreateProvisionedProductPlanError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateProvisionedProductPlanError {}
/// Errors returned by CreateProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum CreateProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        CreateProvisioningArtifactError::InvalidParameters(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateProvisioningArtifactError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateProvisioningArtifactError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateProvisioningArtifactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProvisioningArtifactError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            CreateProvisioningArtifactError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateProvisioningArtifactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProvisioningArtifactError {}
/// Errors returned by CreateServiceAction
#[derive(Debug, PartialEq)]
pub enum CreateServiceActionError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
}

impl CreateServiceActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(CreateServiceActionError::InvalidParameters(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateServiceActionError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateServiceActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateServiceActionError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            CreateServiceActionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateServiceActionError {}
/// Errors returned by CreateTagOption
#[derive(Debug, PartialEq)]
pub enum CreateTagOptionError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl CreateTagOptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTagOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(CreateTagOptionError::DuplicateResource(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateTagOptionError::LimitExceeded(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(CreateTagOptionError::TagOptionNotMigrated(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTagOptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTagOptionError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            CreateTagOptionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateTagOptionError::TagOptionNotMigrated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTagOptionError {}
/// Errors returned by DeleteConstraint
#[derive(Debug, PartialEq)]
pub enum DeleteConstraintError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteConstraintError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConstraintError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DeleteConstraintError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteConstraintError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConstraintError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConstraintError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            DeleteConstraintError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConstraintError {}
/// Errors returned by DeletePortfolio
#[derive(Debug, PartialEq)]
pub enum DeletePortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl DeletePortfolioError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DeletePortfolioError::InvalidParameters(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeletePortfolioError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePortfolioError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(DeletePortfolioError::TagOptionNotMigrated(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePortfolioError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePortfolioError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            DeletePortfolioError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeletePortfolioError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeletePortfolioError::TagOptionNotMigrated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePortfolioError {}
/// Errors returned by DeletePortfolioShare
#[derive(Debug, PartialEq)]
pub enum DeletePortfolioShareError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeletePortfolioShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePortfolioShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DeletePortfolioShareError::InvalidParameters(
                        err.msg,
                    ))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(DeletePortfolioShareError::InvalidState(err.msg))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(DeletePortfolioShareError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePortfolioShareError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePortfolioShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePortfolioShareError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            DeletePortfolioShareError::InvalidState(ref cause) => write!(f, "{}", cause),
            DeletePortfolioShareError::OperationNotSupported(ref cause) => write!(f, "{}", cause),
            DeletePortfolioShareError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePortfolioShareError {}
/// Errors returned by DeleteProduct
#[derive(Debug, PartialEq)]
pub enum DeleteProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl DeleteProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DeleteProductError::InvalidParameters(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteProductError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProductError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(DeleteProductError::TagOptionNotMigrated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProductError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            DeleteProductError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteProductError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteProductError::TagOptionNotMigrated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProductError {}
/// Errors returned by DeleteProvisionedProductPlan
#[derive(Debug, PartialEq)]
pub enum DeleteProvisionedProductPlanError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteProvisionedProductPlanError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteProvisionedProductPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DeleteProvisionedProductPlanError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteProvisionedProductPlanError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteProvisionedProductPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProvisionedProductPlanError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteProvisionedProductPlanError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteProvisionedProductPlanError {}
/// Errors returned by DeleteProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum DeleteProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DeleteProvisioningArtifactError::InvalidParameters(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteProvisioningArtifactError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProvisioningArtifactError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteProvisioningArtifactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProvisioningArtifactError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            DeleteProvisioningArtifactError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteProvisioningArtifactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProvisioningArtifactError {}
/// Errors returned by DeleteServiceAction
#[derive(Debug, PartialEq)]
pub enum DeleteServiceActionError {
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteServiceActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteServiceActionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteServiceActionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteServiceActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteServiceActionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteServiceActionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteServiceActionError {}
/// Errors returned by DeleteTagOption
#[derive(Debug, PartialEq)]
pub enum DeleteTagOptionError {
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl DeleteTagOptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteTagOptionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTagOptionError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(DeleteTagOptionError::TagOptionNotMigrated(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTagOptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTagOptionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteTagOptionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTagOptionError::TagOptionNotMigrated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTagOptionError {}
/// Errors returned by DescribeConstraint
#[derive(Debug, PartialEq)]
pub enum DescribeConstraintError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeConstraintError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConstraintError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeConstraintError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConstraintError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConstraintError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeConstraintError {}
/// Errors returned by DescribeCopyProductStatus
#[derive(Debug, PartialEq)]
pub enum DescribeCopyProductStatusError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeCopyProductStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCopyProductStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeCopyProductStatusError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCopyProductStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCopyProductStatusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCopyProductStatusError {}
/// Errors returned by DescribePortfolio
#[derive(Debug, PartialEq)]
pub enum DescribePortfolioError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribePortfolioError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePortfolioError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribePortfolioError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePortfolioError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePortfolioError {}
/// Errors returned by DescribePortfolioShareStatus
#[derive(Debug, PartialEq)]
pub enum DescribePortfolioShareStatusError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribePortfolioShareStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribePortfolioShareStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DescribePortfolioShareStatusError::InvalidParameters(err.msg),
                    )
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        DescribePortfolioShareStatusError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribePortfolioShareStatusError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribePortfolioShareStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePortfolioShareStatusError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePortfolioShareStatusError::OperationNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePortfolioShareStatusError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribePortfolioShareStatusError {}
/// Errors returned by DescribePortfolioShares
#[derive(Debug, PartialEq)]
pub enum DescribePortfolioSharesError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribePortfolioSharesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePortfolioSharesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DescribePortfolioSharesError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePortfolioSharesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribePortfolioSharesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePortfolioSharesError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            DescribePortfolioSharesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePortfolioSharesError {}
/// Errors returned by DescribeProduct
#[derive(Debug, PartialEq)]
pub enum DescribeProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DescribeProductError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProductError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProductError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            DescribeProductError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProductError {}
/// Errors returned by DescribeProductAsAdmin
#[derive(Debug, PartialEq)]
pub enum DescribeProductAsAdminError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProductAsAdminError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProductAsAdminError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DescribeProductAsAdminError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProductAsAdminError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProductAsAdminError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProductAsAdminError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            DescribeProductAsAdminError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProductAsAdminError {}
/// Errors returned by DescribeProductView
#[derive(Debug, PartialEq)]
pub enum DescribeProductViewError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProductViewError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProductViewError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(DescribeProductViewError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProductViewError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProductViewError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProductViewError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            DescribeProductViewError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProductViewError {}
/// Errors returned by DescribeProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum DescribeProvisionedProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProvisionedProductError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeProvisionedProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DescribeProvisionedProductError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProvisionedProductError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProvisionedProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProvisionedProductError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            DescribeProvisionedProductError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProvisionedProductError {}
/// Errors returned by DescribeProvisionedProductPlan
#[derive(Debug, PartialEq)]
pub enum DescribeProvisionedProductPlanError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProvisionedProductPlanError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeProvisionedProductPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DescribeProvisionedProductPlanError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeProvisionedProductPlanError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProvisionedProductPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProvisionedProductPlanError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeProvisionedProductPlanError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeProvisionedProductPlanError {}
/// Errors returned by DescribeProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum DescribeProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DescribeProvisioningArtifactError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeProvisioningArtifactError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProvisioningArtifactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProvisioningArtifactError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeProvisioningArtifactError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeProvisioningArtifactError {}
/// Errors returned by DescribeProvisioningParameters
#[derive(Debug, PartialEq)]
pub enum DescribeProvisioningParametersError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeProvisioningParametersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeProvisioningParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DescribeProvisioningParametersError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeProvisioningParametersError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProvisioningParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProvisioningParametersError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeProvisioningParametersError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeProvisioningParametersError {}
/// Errors returned by DescribeRecord
#[derive(Debug, PartialEq)]
pub enum DescribeRecordError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeRecordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRecordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRecordError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRecordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRecordError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRecordError {}
/// Errors returned by DescribeServiceAction
#[derive(Debug, PartialEq)]
pub enum DescribeServiceActionError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeServiceActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeServiceActionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeServiceActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServiceActionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeServiceActionError {}
/// Errors returned by DescribeServiceActionExecutionParameters
#[derive(Debug, PartialEq)]
pub enum DescribeServiceActionExecutionParametersError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeServiceActionExecutionParametersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeServiceActionExecutionParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DescribeServiceActionExecutionParametersError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeServiceActionExecutionParametersError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeServiceActionExecutionParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServiceActionExecutionParametersError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeServiceActionExecutionParametersError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeServiceActionExecutionParametersError {}
/// Errors returned by DescribeTagOption
#[derive(Debug, PartialEq)]
pub enum DescribeTagOptionError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl DescribeTagOptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTagOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTagOptionError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(DescribeTagOptionError::TagOptionNotMigrated(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTagOptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTagOptionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTagOptionError::TagOptionNotMigrated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTagOptionError {}
/// Errors returned by DisableAWSOrganizationsAccess
#[derive(Debug, PartialEq)]
pub enum DisableAWSOrganizationsAccessError {
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisableAWSOrganizationsAccessError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisableAWSOrganizationsAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(DisableAWSOrganizationsAccessError::InvalidState(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        DisableAWSOrganizationsAccessError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisableAWSOrganizationsAccessError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableAWSOrganizationsAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableAWSOrganizationsAccessError::InvalidState(ref cause) => write!(f, "{}", cause),
            DisableAWSOrganizationsAccessError::OperationNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableAWSOrganizationsAccessError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisableAWSOrganizationsAccessError {}
/// Errors returned by DisassociateBudgetFromResource
#[derive(Debug, PartialEq)]
pub enum DisassociateBudgetFromResourceError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociateBudgetFromResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateBudgetFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateBudgetFromResourceError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateBudgetFromResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateBudgetFromResourceError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateBudgetFromResourceError {}
/// Errors returned by DisassociatePrincipalFromPortfolio
#[derive(Debug, PartialEq)]
pub enum DisassociatePrincipalFromPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociatePrincipalFromPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociatePrincipalFromPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DisassociatePrincipalFromPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociatePrincipalFromPortfolioError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociatePrincipalFromPortfolioError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociatePrincipalFromPortfolioError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePrincipalFromPortfolioError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociatePrincipalFromPortfolioError {}
/// Errors returned by DisassociateProductFromPortfolio
#[derive(Debug, PartialEq)]
pub enum DisassociateProductFromPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>A resource that is currently in use. Ensure that the resource is not in use and retry the operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociateProductFromPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateProductFromPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        DisassociateProductFromPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        DisassociateProductFromPortfolioError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateProductFromPortfolioError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateProductFromPortfolioError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateProductFromPortfolioError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateProductFromPortfolioError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateProductFromPortfolioError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateProductFromPortfolioError {}
/// Errors returned by DisassociateServiceActionFromProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum DisassociateServiceActionFromProvisioningArtifactError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociateServiceActionFromProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateServiceActionFromProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateServiceActionFromProvisioningArtifactError::ResourceNotFound(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateServiceActionFromProvisioningArtifactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateServiceActionFromProvisioningArtifactError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateServiceActionFromProvisioningArtifactError {}
/// Errors returned by DisassociateTagOptionFromResource
#[derive(Debug, PartialEq)]
pub enum DisassociateTagOptionFromResourceError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl DisassociateTagOptionFromResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateTagOptionFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateTagOptionFromResourceError::ResourceNotFound(err.msg),
                    )
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(
                        DisassociateTagOptionFromResourceError::TagOptionNotMigrated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateTagOptionFromResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateTagOptionFromResourceError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateTagOptionFromResourceError::TagOptionNotMigrated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateTagOptionFromResourceError {}
/// Errors returned by EnableAWSOrganizationsAccess
#[derive(Debug, PartialEq)]
pub enum EnableAWSOrganizationsAccessError {
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl EnableAWSOrganizationsAccessError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<EnableAWSOrganizationsAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(EnableAWSOrganizationsAccessError::InvalidState(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        EnableAWSOrganizationsAccessError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        EnableAWSOrganizationsAccessError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableAWSOrganizationsAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableAWSOrganizationsAccessError::InvalidState(ref cause) => write!(f, "{}", cause),
            EnableAWSOrganizationsAccessError::OperationNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            EnableAWSOrganizationsAccessError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for EnableAWSOrganizationsAccessError {}
/// Errors returned by ExecuteProvisionedProductPlan
#[derive(Debug, PartialEq)]
pub enum ExecuteProvisionedProductPlanError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ExecuteProvisionedProductPlanError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ExecuteProvisionedProductPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ExecuteProvisionedProductPlanError::InvalidParameters(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(ExecuteProvisionedProductPlanError::InvalidState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ExecuteProvisionedProductPlanError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ExecuteProvisionedProductPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteProvisionedProductPlanError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            ExecuteProvisionedProductPlanError::InvalidState(ref cause) => write!(f, "{}", cause),
            ExecuteProvisionedProductPlanError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ExecuteProvisionedProductPlanError {}
/// Errors returned by ExecuteProvisionedProductServiceAction
#[derive(Debug, PartialEq)]
pub enum ExecuteProvisionedProductServiceActionError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ExecuteProvisionedProductServiceActionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ExecuteProvisionedProductServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ExecuteProvisionedProductServiceActionError::InvalidParameters(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(
                        ExecuteProvisionedProductServiceActionError::InvalidState(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ExecuteProvisionedProductServiceActionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ExecuteProvisionedProductServiceActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteProvisionedProductServiceActionError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            ExecuteProvisionedProductServiceActionError::InvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            ExecuteProvisionedProductServiceActionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ExecuteProvisionedProductServiceActionError {}
/// Errors returned by GetAWSOrganizationsAccessStatus
#[derive(Debug, PartialEq)]
pub enum GetAWSOrganizationsAccessStatusError {
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetAWSOrganizationsAccessStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAWSOrganizationsAccessStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        GetAWSOrganizationsAccessStatusError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetAWSOrganizationsAccessStatusError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAWSOrganizationsAccessStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAWSOrganizationsAccessStatusError::OperationNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAWSOrganizationsAccessStatusError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetAWSOrganizationsAccessStatusError {}
/// Errors returned by GetProvisionedProductOutputs
#[derive(Debug, PartialEq)]
pub enum GetProvisionedProductOutputsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetProvisionedProductOutputsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetProvisionedProductOutputsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        GetProvisionedProductOutputsError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetProvisionedProductOutputsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetProvisionedProductOutputsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetProvisionedProductOutputsError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            GetProvisionedProductOutputsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetProvisionedProductOutputsError {}
/// Errors returned by ImportAsProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum ImportAsProvisionedProductError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ImportAsProvisionedProductError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ImportAsProvisionedProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(
                        ImportAsProvisionedProductError::DuplicateResource(err.msg),
                    )
                }
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ImportAsProvisionedProductError::InvalidParameters(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(ImportAsProvisionedProductError::InvalidState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ImportAsProvisionedProductError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ImportAsProvisionedProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportAsProvisionedProductError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            ImportAsProvisionedProductError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            ImportAsProvisionedProductError::InvalidState(ref cause) => write!(f, "{}", cause),
            ImportAsProvisionedProductError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportAsProvisionedProductError {}
/// Errors returned by ListAcceptedPortfolioShares
#[derive(Debug, PartialEq)]
pub enum ListAcceptedPortfolioSharesError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
}

impl ListAcceptedPortfolioSharesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAcceptedPortfolioSharesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListAcceptedPortfolioSharesError::InvalidParameters(err.msg),
                    )
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        ListAcceptedPortfolioSharesError::OperationNotSupported(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAcceptedPortfolioSharesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAcceptedPortfolioSharesError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAcceptedPortfolioSharesError::OperationNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListAcceptedPortfolioSharesError {}
/// Errors returned by ListBudgetsForResource
#[derive(Debug, PartialEq)]
pub enum ListBudgetsForResourceError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListBudgetsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBudgetsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListBudgetsForResourceError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListBudgetsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListBudgetsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBudgetsForResourceError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            ListBudgetsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBudgetsForResourceError {}
/// Errors returned by ListConstraintsForPortfolio
#[derive(Debug, PartialEq)]
pub enum ListConstraintsForPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListConstraintsForPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListConstraintsForPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListConstraintsForPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListConstraintsForPortfolioError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListConstraintsForPortfolioError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListConstraintsForPortfolioError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            ListConstraintsForPortfolioError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListConstraintsForPortfolioError {}
/// Errors returned by ListLaunchPaths
#[derive(Debug, PartialEq)]
pub enum ListLaunchPathsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListLaunchPathsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLaunchPathsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListLaunchPathsError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListLaunchPathsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLaunchPathsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLaunchPathsError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            ListLaunchPathsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLaunchPathsError {}
/// Errors returned by ListOrganizationPortfolioAccess
#[derive(Debug, PartialEq)]
pub enum ListOrganizationPortfolioAccessError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListOrganizationPortfolioAccessError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListOrganizationPortfolioAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListOrganizationPortfolioAccessError::InvalidParameters(err.msg),
                    )
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        ListOrganizationPortfolioAccessError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListOrganizationPortfolioAccessError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOrganizationPortfolioAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOrganizationPortfolioAccessError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            ListOrganizationPortfolioAccessError::OperationNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            ListOrganizationPortfolioAccessError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListOrganizationPortfolioAccessError {}
/// Errors returned by ListPortfolioAccess
#[derive(Debug, PartialEq)]
pub enum ListPortfolioAccessError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListPortfolioAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPortfolioAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListPortfolioAccessError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPortfolioAccessError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPortfolioAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPortfolioAccessError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            ListPortfolioAccessError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPortfolioAccessError {}
/// Errors returned by ListPortfolios
#[derive(Debug, PartialEq)]
pub enum ListPortfoliosError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl ListPortfoliosError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPortfoliosError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListPortfoliosError::InvalidParameters(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPortfoliosError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPortfoliosError::InvalidParameters(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPortfoliosError {}
/// Errors returned by ListPortfoliosForProduct
#[derive(Debug, PartialEq)]
pub enum ListPortfoliosForProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListPortfoliosForProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPortfoliosForProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListPortfoliosForProductError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPortfoliosForProductError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPortfoliosForProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPortfoliosForProductError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            ListPortfoliosForProductError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPortfoliosForProductError {}
/// Errors returned by ListPrincipalsForPortfolio
#[derive(Debug, PartialEq)]
pub enum ListPrincipalsForPortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListPrincipalsForPortfolioError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListPrincipalsForPortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListPrincipalsForPortfolioError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPrincipalsForPortfolioError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPrincipalsForPortfolioError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPrincipalsForPortfolioError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            ListPrincipalsForPortfolioError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPrincipalsForPortfolioError {}
/// Errors returned by ListProvisionedProductPlans
#[derive(Debug, PartialEq)]
pub enum ListProvisionedProductPlansError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListProvisionedProductPlansError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListProvisionedProductPlansError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListProvisionedProductPlansError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListProvisionedProductPlansError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProvisionedProductPlansError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProvisionedProductPlansError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            ListProvisionedProductPlansError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProvisionedProductPlansError {}
/// Errors returned by ListProvisioningArtifacts
#[derive(Debug, PartialEq)]
pub enum ListProvisioningArtifactsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListProvisioningArtifactsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProvisioningArtifactsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListProvisioningArtifactsError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListProvisioningArtifactsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProvisioningArtifactsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProvisioningArtifactsError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            ListProvisioningArtifactsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProvisioningArtifactsError {}
/// Errors returned by ListProvisioningArtifactsForServiceAction
#[derive(Debug, PartialEq)]
pub enum ListProvisioningArtifactsForServiceActionError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListProvisioningArtifactsForServiceActionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListProvisioningArtifactsForServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListProvisioningArtifactsForServiceActionError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListProvisioningArtifactsForServiceActionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProvisioningArtifactsForServiceActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProvisioningArtifactsForServiceActionError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            ListProvisioningArtifactsForServiceActionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListProvisioningArtifactsForServiceActionError {}
/// Errors returned by ListRecordHistory
#[derive(Debug, PartialEq)]
pub enum ListRecordHistoryError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl ListRecordHistoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRecordHistoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListRecordHistoryError::InvalidParameters(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRecordHistoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRecordHistoryError::InvalidParameters(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRecordHistoryError {}
/// Errors returned by ListResourcesForTagOption
#[derive(Debug, PartialEq)]
pub enum ListResourcesForTagOptionError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl ListResourcesForTagOptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourcesForTagOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListResourcesForTagOptionError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListResourcesForTagOptionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(
                        ListResourcesForTagOptionError::TagOptionNotMigrated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResourcesForTagOptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourcesForTagOptionError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            ListResourcesForTagOptionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListResourcesForTagOptionError::TagOptionNotMigrated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListResourcesForTagOptionError {}
/// Errors returned by ListServiceActions
#[derive(Debug, PartialEq)]
pub enum ListServiceActionsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl ListServiceActionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListServiceActionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListServiceActionsError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListServiceActionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListServiceActionsError::InvalidParameters(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListServiceActionsError {}
/// Errors returned by ListServiceActionsForProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum ListServiceActionsForProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListServiceActionsForProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListServiceActionsForProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListServiceActionsForProvisioningArtifactError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListServiceActionsForProvisioningArtifactError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListServiceActionsForProvisioningArtifactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListServiceActionsForProvisioningArtifactError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            ListServiceActionsForProvisioningArtifactError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListServiceActionsForProvisioningArtifactError {}
/// Errors returned by ListStackInstancesForProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum ListStackInstancesForProvisionedProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListStackInstancesForProvisionedProductError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListStackInstancesForProvisionedProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        ListStackInstancesForProvisionedProductError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListStackInstancesForProvisionedProductError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListStackInstancesForProvisionedProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListStackInstancesForProvisionedProductError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            ListStackInstancesForProvisionedProductError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListStackInstancesForProvisionedProductError {}
/// Errors returned by ListTagOptions
#[derive(Debug, PartialEq)]
pub enum ListTagOptionsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl ListTagOptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagOptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ListTagOptionsError::InvalidParameters(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(ListTagOptionsError::TagOptionNotMigrated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagOptionsError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            ListTagOptionsError::TagOptionNotMigrated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagOptionsError {}
/// Errors returned by ProvisionProduct
#[derive(Debug, PartialEq)]
pub enum ProvisionProductError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ProvisionProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ProvisionProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(ProvisionProductError::DuplicateResource(err.msg))
                }
                "InvalidParametersException" => {
                    return RusotoError::Service(ProvisionProductError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ProvisionProductError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ProvisionProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProvisionProductError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            ProvisionProductError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            ProvisionProductError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ProvisionProductError {}
/// Errors returned by RejectPortfolioShare
#[derive(Debug, PartialEq)]
pub enum RejectPortfolioShareError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl RejectPortfolioShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RejectPortfolioShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RejectPortfolioShareError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RejectPortfolioShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RejectPortfolioShareError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RejectPortfolioShareError {}
/// Errors returned by ScanProvisionedProducts
#[derive(Debug, PartialEq)]
pub enum ScanProvisionedProductsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl ScanProvisionedProductsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ScanProvisionedProductsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(ScanProvisionedProductsError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ScanProvisionedProductsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScanProvisionedProductsError::InvalidParameters(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ScanProvisionedProductsError {}
/// Errors returned by SearchProducts
#[derive(Debug, PartialEq)]
pub enum SearchProductsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl SearchProductsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchProductsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(SearchProductsError::InvalidParameters(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchProductsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchProductsError::InvalidParameters(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchProductsError {}
/// Errors returned by SearchProductsAsAdmin
#[derive(Debug, PartialEq)]
pub enum SearchProductsAsAdminError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl SearchProductsAsAdminError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchProductsAsAdminError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(SearchProductsAsAdminError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SearchProductsAsAdminError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchProductsAsAdminError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchProductsAsAdminError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            SearchProductsAsAdminError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchProductsAsAdminError {}
/// Errors returned by SearchProvisionedProducts
#[derive(Debug, PartialEq)]
pub enum SearchProvisionedProductsError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
}

impl SearchProvisionedProductsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchProvisionedProductsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(SearchProvisionedProductsError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchProvisionedProductsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchProvisionedProductsError::InvalidParameters(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchProvisionedProductsError {}
/// Errors returned by TerminateProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum TerminateProvisionedProductError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl TerminateProvisionedProductError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<TerminateProvisionedProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        TerminateProvisionedProductError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TerminateProvisionedProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TerminateProvisionedProductError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TerminateProvisionedProductError {}
/// Errors returned by UpdateConstraint
#[derive(Debug, PartialEq)]
pub enum UpdateConstraintError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateConstraintError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConstraintError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdateConstraintError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateConstraintError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateConstraintError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConstraintError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            UpdateConstraintError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateConstraintError {}
/// Errors returned by UpdatePortfolio
#[derive(Debug, PartialEq)]
pub enum UpdatePortfolioError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the operation.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl UpdatePortfolioError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePortfolioError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdatePortfolioError::InvalidParameters(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdatePortfolioError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdatePortfolioError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(UpdatePortfolioError::TagOptionNotMigrated(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdatePortfolioError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePortfolioError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            UpdatePortfolioError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdatePortfolioError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdatePortfolioError::TagOptionNotMigrated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePortfolioError {}
/// Errors returned by UpdatePortfolioShare
#[derive(Debug, PartialEq)]
pub enum UpdatePortfolioShareError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdatePortfolioShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePortfolioShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdatePortfolioShareError::InvalidParameters(
                        err.msg,
                    ))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(UpdatePortfolioShareError::InvalidState(err.msg))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(UpdatePortfolioShareError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdatePortfolioShareError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdatePortfolioShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePortfolioShareError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            UpdatePortfolioShareError::InvalidState(ref cause) => write!(f, "{}", cause),
            UpdatePortfolioShareError::OperationNotSupported(ref cause) => write!(f, "{}", cause),
            UpdatePortfolioShareError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePortfolioShareError {}
/// Errors returned by UpdateProduct
#[derive(Debug, PartialEq)]
pub enum UpdateProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl UpdateProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdateProductError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProductError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(UpdateProductError::TagOptionNotMigrated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateProductError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            UpdateProductError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateProductError::TagOptionNotMigrated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateProductError {}
/// Errors returned by UpdateProvisionedProduct
#[derive(Debug, PartialEq)]
pub enum UpdateProvisionedProductError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateProvisionedProductError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProvisionedProductError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdateProvisionedProductError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProvisionedProductError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateProvisionedProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateProvisionedProductError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            UpdateProvisionedProductError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateProvisionedProductError {}
/// Errors returned by UpdateProvisionedProductProperties
#[derive(Debug, PartialEq)]
pub enum UpdateProvisionedProductPropertiesError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying the operation.</p>
    InvalidState(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateProvisionedProductPropertiesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateProvisionedProductPropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        UpdateProvisionedProductPropertiesError::InvalidParameters(err.msg),
                    )
                }
                "InvalidStateException" => {
                    return RusotoError::Service(
                        UpdateProvisionedProductPropertiesError::InvalidState(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateProvisionedProductPropertiesError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateProvisionedProductPropertiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateProvisionedProductPropertiesError::InvalidParameters(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateProvisionedProductPropertiesError::InvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateProvisionedProductPropertiesError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateProvisionedProductPropertiesError {}
/// Errors returned by UpdateProvisioningArtifact
#[derive(Debug, PartialEq)]
pub enum UpdateProvisioningArtifactError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateProvisioningArtifactError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateProvisioningArtifactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(
                        UpdateProvisioningArtifactError::InvalidParameters(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProvisioningArtifactError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateProvisioningArtifactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateProvisioningArtifactError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            UpdateProvisioningArtifactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateProvisioningArtifactError {}
/// Errors returned by UpdateServiceAction
#[derive(Debug, PartialEq)]
pub enum UpdateServiceActionError {
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateServiceActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateServiceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdateServiceActionError::InvalidParameters(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateServiceActionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateServiceActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateServiceActionError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            UpdateServiceActionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateServiceActionError {}
/// Errors returned by UpdateTagOption
#[derive(Debug, PartialEq)]
pub enum UpdateTagOptionError {
    /// <p>The specified resource is a duplicate.</p>
    DuplicateResource(String),
    /// <p>One or more parameters provided to the operation are not valid.</p>
    InvalidParameters(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Please use the AWS console to perform the migration process before retrying the operation.</p>
    TagOptionNotMigrated(String),
}

impl UpdateTagOptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTagOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(UpdateTagOptionError::DuplicateResource(err.msg))
                }
                "InvalidParametersException" => {
                    return RusotoError::Service(UpdateTagOptionError::InvalidParameters(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateTagOptionError::ResourceNotFound(err.msg))
                }
                "TagOptionNotMigratedException" => {
                    return RusotoError::Service(UpdateTagOptionError::TagOptionNotMigrated(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateTagOptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTagOptionError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            UpdateTagOptionError::InvalidParameters(ref cause) => write!(f, "{}", cause),
            UpdateTagOptionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTagOptionError::TagOptionNotMigrated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTagOptionError {}
/// Trait representing the capabilities of the AWS Service Catalog API. AWS Service Catalog clients implement this trait.
#[async_trait]
pub trait ServiceCatalog: Clone + Sync + Send + 'static {
    /// <p>Accepts an offer to share the specified portfolio.</p>
    async fn accept_portfolio_share(
        &self,
        input: AcceptPortfolioShareInput,
    ) -> Result<AcceptPortfolioShareOutput, RusotoError<AcceptPortfolioShareError>>;

    /// <p>Associates the specified budget with the specified resource.</p>
    async fn associate_budget_with_resource(
        &self,
        input: AssociateBudgetWithResourceInput,
    ) -> Result<AssociateBudgetWithResourceOutput, RusotoError<AssociateBudgetWithResourceError>>;

    /// <p>Associates the specified principal ARN with the specified portfolio.</p>
    async fn associate_principal_with_portfolio(
        &self,
        input: AssociatePrincipalWithPortfolioInput,
    ) -> Result<
        AssociatePrincipalWithPortfolioOutput,
        RusotoError<AssociatePrincipalWithPortfolioError>,
    >;

    /// <p>Associates the specified product with the specified portfolio.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn associate_product_with_portfolio(
        &self,
        input: AssociateProductWithPortfolioInput,
    ) -> Result<AssociateProductWithPortfolioOutput, RusotoError<AssociateProductWithPortfolioError>>;

    /// <p>Associates a self-service action with a provisioning artifact.</p>
    async fn associate_service_action_with_provisioning_artifact(
        &self,
        input: AssociateServiceActionWithProvisioningArtifactInput,
    ) -> Result<
        AssociateServiceActionWithProvisioningArtifactOutput,
        RusotoError<AssociateServiceActionWithProvisioningArtifactError>,
    >;

    /// <p>Associate the specified TagOption with the specified portfolio or product.</p>
    async fn associate_tag_option_with_resource(
        &self,
        input: AssociateTagOptionWithResourceInput,
    ) -> Result<
        AssociateTagOptionWithResourceOutput,
        RusotoError<AssociateTagOptionWithResourceError>,
    >;

    /// <p>Associates multiple self-service actions with provisioning artifacts.</p>
    async fn batch_associate_service_action_with_provisioning_artifact(
        &self,
        input: BatchAssociateServiceActionWithProvisioningArtifactInput,
    ) -> Result<
        BatchAssociateServiceActionWithProvisioningArtifactOutput,
        RusotoError<BatchAssociateServiceActionWithProvisioningArtifactError>,
    >;

    /// <p>Disassociates a batch of self-service actions from the specified provisioning artifact.</p>
    async fn batch_disassociate_service_action_from_provisioning_artifact(
        &self,
        input: BatchDisassociateServiceActionFromProvisioningArtifactInput,
    ) -> Result<
        BatchDisassociateServiceActionFromProvisioningArtifactOutput,
        RusotoError<BatchDisassociateServiceActionFromProvisioningArtifactError>,
    >;

    /// <p>Copies the specified source product to the specified target product or a new product.</p> <p>You can copy a product to the same account or another account. You can copy a product to the same region or another region.</p> <p>This operation is performed asynchronously. To track the progress of the operation, use <a>DescribeCopyProductStatus</a>.</p>
    async fn copy_product(
        &self,
        input: CopyProductInput,
    ) -> Result<CopyProductOutput, RusotoError<CopyProductError>>;

    /// <p>Creates a constraint.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn create_constraint(
        &self,
        input: CreateConstraintInput,
    ) -> Result<CreateConstraintOutput, RusotoError<CreateConstraintError>>;

    /// <p>Creates a portfolio.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn create_portfolio(
        &self,
        input: CreatePortfolioInput,
    ) -> Result<CreatePortfolioOutput, RusotoError<CreatePortfolioError>>;

    /// <p>Shares the specified portfolio with the specified account or organization node. Shares to an organization node can only be created by the management account of an organization or by a delegated administrator. You can share portfolios to an organization, an organizational unit, or a specific account.</p> <p>Note that if a delegated admin is de-registered, they can no longer create portfolio shares.</p> <p> <code>AWSOrganizationsAccess</code> must be enabled in order to create a portfolio share to an organization node.</p> <p>You can't share a shared resource, including portfolios that contain a shared product.</p> <p>If the portfolio share with the specified account or organization node already exists, this action will have no effect and will not return an error. To update an existing share, you must use the <code> UpdatePortfolioShare</code> API instead.</p>
    async fn create_portfolio_share(
        &self,
        input: CreatePortfolioShareInput,
    ) -> Result<CreatePortfolioShareOutput, RusotoError<CreatePortfolioShareError>>;

    /// <p>Creates a product.</p> <p>A delegated admin is authorized to invoke this command.</p> <p>The user or role that performs this operation must have the <code>cloudformation:GetTemplate</code> IAM policy permission. This policy permission is required when using the <code>ImportFromPhysicalId</code> template source in the information data section.</p>
    async fn create_product(
        &self,
        input: CreateProductInput,
    ) -> Result<CreateProductOutput, RusotoError<CreateProductError>>;

    /// <p>Creates a plan. A plan includes the list of resources to be created (when provisioning a new product) or modified (when updating a provisioned product) when the plan is executed.</p> <p>You can create one plan per provisioned product. To create a plan for an existing provisioned product, the product status must be AVAILBLE or TAINTED.</p> <p>To view the resource changes in the change set, use <a>DescribeProvisionedProductPlan</a>. To create or modify the provisioned product, use <a>ExecuteProvisionedProductPlan</a>.</p>
    async fn create_provisioned_product_plan(
        &self,
        input: CreateProvisionedProductPlanInput,
    ) -> Result<CreateProvisionedProductPlanOutput, RusotoError<CreateProvisionedProductPlanError>>;

    /// <p>Creates a provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot create a provisioning artifact for a product that was shared with you.</p> <p>The user or role that performs this operation must have the <code>cloudformation:GetTemplate</code> IAM policy permission. This policy permission is required when using the <code>ImportFromPhysicalId</code> template source in the information data section.</p>
    async fn create_provisioning_artifact(
        &self,
        input: CreateProvisioningArtifactInput,
    ) -> Result<CreateProvisioningArtifactOutput, RusotoError<CreateProvisioningArtifactError>>;

    /// <p>Creates a self-service action.</p>
    async fn create_service_action(
        &self,
        input: CreateServiceActionInput,
    ) -> Result<CreateServiceActionOutput, RusotoError<CreateServiceActionError>>;

    /// <p>Creates a TagOption.</p>
    async fn create_tag_option(
        &self,
        input: CreateTagOptionInput,
    ) -> Result<CreateTagOptionOutput, RusotoError<CreateTagOptionError>>;

    /// <p>Deletes the specified constraint.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn delete_constraint(
        &self,
        input: DeleteConstraintInput,
    ) -> Result<DeleteConstraintOutput, RusotoError<DeleteConstraintError>>;

    /// <p>Deletes the specified portfolio.</p> <p>You cannot delete a portfolio if it was shared with you or if it has associated products, users, constraints, or shared accounts.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn delete_portfolio(
        &self,
        input: DeletePortfolioInput,
    ) -> Result<DeletePortfolioOutput, RusotoError<DeletePortfolioError>>;

    /// <p>Stops sharing the specified portfolio with the specified account or organization node. Shares to an organization node can only be deleted by the management account of an organization or by a delegated administrator.</p> <p>Note that if a delegated admin is de-registered, portfolio shares created from that account are removed.</p>
    async fn delete_portfolio_share(
        &self,
        input: DeletePortfolioShareInput,
    ) -> Result<DeletePortfolioShareOutput, RusotoError<DeletePortfolioShareError>>;

    /// <p>Deletes the specified product.</p> <p>You cannot delete a product if it was shared with you or is associated with a portfolio.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn delete_product(
        &self,
        input: DeleteProductInput,
    ) -> Result<DeleteProductOutput, RusotoError<DeleteProductError>>;

    /// <p>Deletes the specified plan.</p>
    async fn delete_provisioned_product_plan(
        &self,
        input: DeleteProvisionedProductPlanInput,
    ) -> Result<DeleteProvisionedProductPlanOutput, RusotoError<DeleteProvisionedProductPlanError>>;

    /// <p>Deletes the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot delete a provisioning artifact associated with a product that was shared with you. You cannot delete the last provisioning artifact for a product, because a product must have at least one provisioning artifact.</p>
    async fn delete_provisioning_artifact(
        &self,
        input: DeleteProvisioningArtifactInput,
    ) -> Result<DeleteProvisioningArtifactOutput, RusotoError<DeleteProvisioningArtifactError>>;

    /// <p>Deletes a self-service action.</p>
    async fn delete_service_action(
        &self,
        input: DeleteServiceActionInput,
    ) -> Result<DeleteServiceActionOutput, RusotoError<DeleteServiceActionError>>;

    /// <p>Deletes the specified TagOption.</p> <p>You cannot delete a TagOption if it is associated with a product or portfolio.</p>
    async fn delete_tag_option(
        &self,
        input: DeleteTagOptionInput,
    ) -> Result<DeleteTagOptionOutput, RusotoError<DeleteTagOptionError>>;

    /// <p>Gets information about the specified constraint.</p>
    async fn describe_constraint(
        &self,
        input: DescribeConstraintInput,
    ) -> Result<DescribeConstraintOutput, RusotoError<DescribeConstraintError>>;

    /// <p>Gets the status of the specified copy product operation.</p>
    async fn describe_copy_product_status(
        &self,
        input: DescribeCopyProductStatusInput,
    ) -> Result<DescribeCopyProductStatusOutput, RusotoError<DescribeCopyProductStatusError>>;

    /// <p>Gets information about the specified portfolio.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn describe_portfolio(
        &self,
        input: DescribePortfolioInput,
    ) -> Result<DescribePortfolioOutput, RusotoError<DescribePortfolioError>>;

    /// <p>Gets the status of the specified portfolio share operation. This API can only be called by the management account in the organization or by a delegated admin.</p>
    async fn describe_portfolio_share_status(
        &self,
        input: DescribePortfolioShareStatusInput,
    ) -> Result<DescribePortfolioShareStatusOutput, RusotoError<DescribePortfolioShareStatusError>>;

    /// <p>Returns a summary of each of the portfolio shares that were created for the specified portfolio.</p> <p>You can use this API to determine which accounts or organizational nodes this portfolio have been shared, whether the recipient entity has imported the share, and whether TagOptions are included with the share.</p> <p>The <code>PortfolioId</code> and <code>Type</code> parameters are both required.</p>
    async fn describe_portfolio_shares(
        &self,
        input: DescribePortfolioSharesInput,
    ) -> Result<DescribePortfolioSharesOutput, RusotoError<DescribePortfolioSharesError>>;

    /// <p>Gets information about the specified product.</p>
    async fn describe_product(
        &self,
        input: DescribeProductInput,
    ) -> Result<DescribeProductOutput, RusotoError<DescribeProductError>>;

    /// <p>Gets information about the specified product. This operation is run with administrator access.</p>
    async fn describe_product_as_admin(
        &self,
        input: DescribeProductAsAdminInput,
    ) -> Result<DescribeProductAsAdminOutput, RusotoError<DescribeProductAsAdminError>>;

    /// <p>Gets information about the specified product.</p>
    async fn describe_product_view(
        &self,
        input: DescribeProductViewInput,
    ) -> Result<DescribeProductViewOutput, RusotoError<DescribeProductViewError>>;

    /// <p>Gets information about the specified provisioned product.</p>
    async fn describe_provisioned_product(
        &self,
        input: DescribeProvisionedProductInput,
    ) -> Result<DescribeProvisionedProductOutput, RusotoError<DescribeProvisionedProductError>>;

    /// <p>Gets information about the resource changes for the specified plan.</p>
    async fn describe_provisioned_product_plan(
        &self,
        input: DescribeProvisionedProductPlanInput,
    ) -> Result<
        DescribeProvisionedProductPlanOutput,
        RusotoError<DescribeProvisionedProductPlanError>,
    >;

    /// <p>Gets information about the specified provisioning artifact (also known as a version) for the specified product.</p>
    async fn describe_provisioning_artifact(
        &self,
        input: DescribeProvisioningArtifactInput,
    ) -> Result<DescribeProvisioningArtifactOutput, RusotoError<DescribeProvisioningArtifactError>>;

    /// <p>Gets information about the configuration required to provision the specified product using the specified provisioning artifact.</p> <p>If the output contains a TagOption key with an empty list of values, there is a TagOption conflict for that key. The end user cannot take action to fix the conflict, and launch is not blocked. In subsequent calls to <a>ProvisionProduct</a>, do not include conflicted TagOption keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>". Tag the provisioned product with the value <code>sc-tagoption-conflict-portfolioId-productId</code>.</p>
    async fn describe_provisioning_parameters(
        &self,
        input: DescribeProvisioningParametersInput,
    ) -> Result<
        DescribeProvisioningParametersOutput,
        RusotoError<DescribeProvisioningParametersError>,
    >;

    /// <p><p>Gets information about the specified request operation.</p> <p>Use this operation after calling a request operation (for example, <a>ProvisionProduct</a>, <a>TerminateProvisionedProduct</a>, or <a>UpdateProvisionedProduct</a>). </p> <note> <p>If a provisioned product was transferred to a new owner using <a>UpdateProvisionedProductProperties</a>, the new owner will be able to describe all past records for that product. The previous owner will no longer be able to describe the records, but will be able to use <a>ListRecordHistory</a> to see the product&#39;s history from when he was the owner.</p> </note></p>
    async fn describe_record(
        &self,
        input: DescribeRecordInput,
    ) -> Result<DescribeRecordOutput, RusotoError<DescribeRecordError>>;

    /// <p>Describes a self-service action.</p>
    async fn describe_service_action(
        &self,
        input: DescribeServiceActionInput,
    ) -> Result<DescribeServiceActionOutput, RusotoError<DescribeServiceActionError>>;

    /// <p>Finds the default parameters for a specific self-service action on a specific provisioned product and returns a map of the results to the user.</p>
    async fn describe_service_action_execution_parameters(
        &self,
        input: DescribeServiceActionExecutionParametersInput,
    ) -> Result<
        DescribeServiceActionExecutionParametersOutput,
        RusotoError<DescribeServiceActionExecutionParametersError>,
    >;

    /// <p>Gets information about the specified TagOption.</p>
    async fn describe_tag_option(
        &self,
        input: DescribeTagOptionInput,
    ) -> Result<DescribeTagOptionOutput, RusotoError<DescribeTagOptionError>>;

    /// <p>Disable portfolio sharing through AWS Organizations feature. This feature will not delete your current shares but it will prevent you from creating new shares throughout your organization. Current shares will not be in sync with your organization structure if it changes after calling this API. This API can only be called by the management account in the organization.</p> <p>This API can't be invoked if there are active delegated administrators in the organization.</p> <p>Note that a delegated administrator is not authorized to invoke <code>DisableAWSOrganizationsAccess</code>.</p>
    async fn disable_aws_organizations_access(
        &self,
    ) -> Result<DisableAWSOrganizationsAccessOutput, RusotoError<DisableAWSOrganizationsAccessError>>;

    /// <p>Disassociates the specified budget from the specified resource.</p>
    async fn disassociate_budget_from_resource(
        &self,
        input: DisassociateBudgetFromResourceInput,
    ) -> Result<
        DisassociateBudgetFromResourceOutput,
        RusotoError<DisassociateBudgetFromResourceError>,
    >;

    /// <p>Disassociates a previously associated principal ARN from a specified portfolio.</p>
    async fn disassociate_principal_from_portfolio(
        &self,
        input: DisassociatePrincipalFromPortfolioInput,
    ) -> Result<
        DisassociatePrincipalFromPortfolioOutput,
        RusotoError<DisassociatePrincipalFromPortfolioError>,
    >;

    /// <p>Disassociates the specified product from the specified portfolio. </p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn disassociate_product_from_portfolio(
        &self,
        input: DisassociateProductFromPortfolioInput,
    ) -> Result<
        DisassociateProductFromPortfolioOutput,
        RusotoError<DisassociateProductFromPortfolioError>,
    >;

    /// <p>Disassociates the specified self-service action association from the specified provisioning artifact.</p>
    async fn disassociate_service_action_from_provisioning_artifact(
        &self,
        input: DisassociateServiceActionFromProvisioningArtifactInput,
    ) -> Result<
        DisassociateServiceActionFromProvisioningArtifactOutput,
        RusotoError<DisassociateServiceActionFromProvisioningArtifactError>,
    >;

    /// <p>Disassociates the specified TagOption from the specified resource.</p>
    async fn disassociate_tag_option_from_resource(
        &self,
        input: DisassociateTagOptionFromResourceInput,
    ) -> Result<
        DisassociateTagOptionFromResourceOutput,
        RusotoError<DisassociateTagOptionFromResourceError>,
    >;

    /// <p>Enable portfolio sharing feature through AWS Organizations. This API will allow Service Catalog to receive updates on your organization in order to sync your shares with the current structure. This API can only be called by the management account in the organization.</p> <p>By calling this API Service Catalog will make a call to organizations:EnableAWSServiceAccess on your behalf so that your shares can be in sync with any changes in your AWS Organizations structure.</p> <p>Note that a delegated administrator is not authorized to invoke <code>EnableAWSOrganizationsAccess</code>.</p>
    async fn enable_aws_organizations_access(
        &self,
    ) -> Result<EnableAWSOrganizationsAccessOutput, RusotoError<EnableAWSOrganizationsAccessError>>;

    /// <p>Provisions or modifies a product based on the resource changes for the specified plan.</p>
    async fn execute_provisioned_product_plan(
        &self,
        input: ExecuteProvisionedProductPlanInput,
    ) -> Result<ExecuteProvisionedProductPlanOutput, RusotoError<ExecuteProvisionedProductPlanError>>;

    /// <p>Executes a self-service action against a provisioned product.</p>
    async fn execute_provisioned_product_service_action(
        &self,
        input: ExecuteProvisionedProductServiceActionInput,
    ) -> Result<
        ExecuteProvisionedProductServiceActionOutput,
        RusotoError<ExecuteProvisionedProductServiceActionError>,
    >;

    /// <p>Get the Access Status for AWS Organization portfolio share feature. This API can only be called by the management account in the organization or by a delegated admin.</p>
    async fn get_aws_organizations_access_status(
        &self,
    ) -> Result<
        GetAWSOrganizationsAccessStatusOutput,
        RusotoError<GetAWSOrganizationsAccessStatusError>,
    >;

    /// <p>This API takes either a <code>ProvisonedProductId</code> or a <code>ProvisionedProductName</code>, along with a list of one or more output keys, and responds with the key/value pairs of those outputs.</p>
    async fn get_provisioned_product_outputs(
        &self,
        input: GetProvisionedProductOutputsInput,
    ) -> Result<GetProvisionedProductOutputsOutput, RusotoError<GetProvisionedProductOutputsError>>;

    /// <p>Requests the import of a resource as a Service Catalog provisioned product that is associated to a Service Catalog product and provisioning artifact. Once imported, all supported Service Catalog governance actions are supported on the provisioned product.</p> <p>Resource import only supports CloudFormation stack ARNs. CloudFormation StackSets and non-root nested stacks are not supported.</p> <p>The CloudFormation stack must have one of the following statuses to be imported: <code>CREATE_COMPLETE</code>, <code>UPDATE_COMPLETE</code>, <code>UPDATE_ROLLBACK_COMPLETE</code>, <code>IMPORT_COMPLETE</code>, <code>IMPORT_ROLLBACK_COMPLETE</code>.</p> <p>Import of the resource requires that the CloudFormation stack template matches the associated Service Catalog product provisioning artifact. </p> <p>The user or role that performs this operation must have the <code>cloudformation:GetTemplate</code> and <code>cloudformation:DescribeStacks</code> IAM policy permissions. </p>
    async fn import_as_provisioned_product(
        &self,
        input: ImportAsProvisionedProductInput,
    ) -> Result<ImportAsProvisionedProductOutput, RusotoError<ImportAsProvisionedProductError>>;

    /// <p>Lists all portfolios for which sharing was accepted by this account.</p>
    async fn list_accepted_portfolio_shares(
        &self,
        input: ListAcceptedPortfolioSharesInput,
    ) -> Result<ListAcceptedPortfolioSharesOutput, RusotoError<ListAcceptedPortfolioSharesError>>;

    /// Auto-paginating version of `list_accepted_portfolio_shares`
    fn list_accepted_portfolio_shares_pages<'a>(
        &'a self,
        mut input: ListAcceptedPortfolioSharesInput,
    ) -> RusotoStream<'a, PortfolioDetail, ListAcceptedPortfolioSharesError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_accepted_portfolio_shares(input.clone())
        }))
    }

    /// <p>Lists all the budgets associated to the specified resource.</p>
    async fn list_budgets_for_resource(
        &self,
        input: ListBudgetsForResourceInput,
    ) -> Result<ListBudgetsForResourceOutput, RusotoError<ListBudgetsForResourceError>>;

    /// <p>Lists the constraints for the specified portfolio and product.</p>
    async fn list_constraints_for_portfolio(
        &self,
        input: ListConstraintsForPortfolioInput,
    ) -> Result<ListConstraintsForPortfolioOutput, RusotoError<ListConstraintsForPortfolioError>>;

    /// Auto-paginating version of `list_constraints_for_portfolio`
    fn list_constraints_for_portfolio_pages<'a>(
        &'a self,
        mut input: ListConstraintsForPortfolioInput,
    ) -> RusotoStream<'a, ConstraintDetail, ListConstraintsForPortfolioError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_constraints_for_portfolio(input.clone())
        }))
    }

    /// <p>Lists the paths to the specified product. A path is how the user has access to a specified product, and is necessary when provisioning a product. A path also determines the constraints put on the product.</p>
    async fn list_launch_paths(
        &self,
        input: ListLaunchPathsInput,
    ) -> Result<ListLaunchPathsOutput, RusotoError<ListLaunchPathsError>>;

    /// Auto-paginating version of `list_launch_paths`
    fn list_launch_paths_pages<'a>(
        &'a self,
        mut input: ListLaunchPathsInput,
    ) -> RusotoStream<'a, LaunchPathSummary, ListLaunchPathsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_launch_paths(input.clone())
        }))
    }

    /// <p>Lists the organization nodes that have access to the specified portfolio. This API can only be called by the management account in the organization or by a delegated admin.</p> <p>If a delegated admin is de-registered, they can no longer perform this operation.</p>
    async fn list_organization_portfolio_access(
        &self,
        input: ListOrganizationPortfolioAccessInput,
    ) -> Result<
        ListOrganizationPortfolioAccessOutput,
        RusotoError<ListOrganizationPortfolioAccessError>,
    >;

    /// Auto-paginating version of `list_organization_portfolio_access`
    fn list_organization_portfolio_access_pages<'a>(
        &'a self,
        mut input: ListOrganizationPortfolioAccessInput,
    ) -> RusotoStream<'a, OrganizationNode, ListOrganizationPortfolioAccessError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_organization_portfolio_access(input.clone())
        }))
    }

    /// <p>Lists the account IDs that have access to the specified portfolio.</p> <p>A delegated admin can list the accounts that have access to the shared portfolio. Note that if a delegated admin is de-registered, they can no longer perform this operation.</p>
    async fn list_portfolio_access(
        &self,
        input: ListPortfolioAccessInput,
    ) -> Result<ListPortfolioAccessOutput, RusotoError<ListPortfolioAccessError>>;

    /// <p>Lists all portfolios in the catalog.</p>
    async fn list_portfolios(
        &self,
        input: ListPortfoliosInput,
    ) -> Result<ListPortfoliosOutput, RusotoError<ListPortfoliosError>>;

    /// Auto-paginating version of `list_portfolios`
    fn list_portfolios_pages<'a>(
        &'a self,
        mut input: ListPortfoliosInput,
    ) -> RusotoStream<'a, PortfolioDetail, ListPortfoliosError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_portfolios(input.clone())
        }))
    }

    /// <p>Lists all portfolios that the specified product is associated with.</p>
    async fn list_portfolios_for_product(
        &self,
        input: ListPortfoliosForProductInput,
    ) -> Result<ListPortfoliosForProductOutput, RusotoError<ListPortfoliosForProductError>>;

    /// Auto-paginating version of `list_portfolios_for_product`
    fn list_portfolios_for_product_pages<'a>(
        &'a self,
        mut input: ListPortfoliosForProductInput,
    ) -> RusotoStream<'a, PortfolioDetail, ListPortfoliosForProductError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_portfolios_for_product(input.clone())
        }))
    }

    /// <p>Lists all principal ARNs associated with the specified portfolio.</p>
    async fn list_principals_for_portfolio(
        &self,
        input: ListPrincipalsForPortfolioInput,
    ) -> Result<ListPrincipalsForPortfolioOutput, RusotoError<ListPrincipalsForPortfolioError>>;

    /// Auto-paginating version of `list_principals_for_portfolio`
    fn list_principals_for_portfolio_pages<'a>(
        &'a self,
        mut input: ListPrincipalsForPortfolioInput,
    ) -> RusotoStream<'a, Principal, ListPrincipalsForPortfolioError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_principals_for_portfolio(input.clone())
        }))
    }

    /// <p>Lists the plans for the specified provisioned product or all plans to which the user has access.</p>
    async fn list_provisioned_product_plans(
        &self,
        input: ListProvisionedProductPlansInput,
    ) -> Result<ListProvisionedProductPlansOutput, RusotoError<ListProvisionedProductPlansError>>;

    /// Auto-paginating version of `list_provisioned_product_plans`
    fn list_provisioned_product_plans_pages<'a>(
        &'a self,
        mut input: ListProvisionedProductPlansInput,
    ) -> RusotoStream<'a, ProvisionedProductPlanSummary, ListProvisionedProductPlansError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_provisioned_product_plans(input.clone())
        }))
    }

    /// <p>Lists all provisioning artifacts (also known as versions) for the specified product.</p>
    async fn list_provisioning_artifacts(
        &self,
        input: ListProvisioningArtifactsInput,
    ) -> Result<ListProvisioningArtifactsOutput, RusotoError<ListProvisioningArtifactsError>>;

    /// <p>Lists all provisioning artifacts (also known as versions) for the specified self-service action.</p>
    async fn list_provisioning_artifacts_for_service_action(
        &self,
        input: ListProvisioningArtifactsForServiceActionInput,
    ) -> Result<
        ListProvisioningArtifactsForServiceActionOutput,
        RusotoError<ListProvisioningArtifactsForServiceActionError>,
    >;

    /// Auto-paginating version of `list_provisioning_artifacts_for_service_action`
    fn list_provisioning_artifacts_for_service_action_pages<'a>(
        &'a self,
        mut input: ListProvisioningArtifactsForServiceActionInput,
    ) -> RusotoStream<'a, ProvisioningArtifactView, ListProvisioningArtifactsForServiceActionError>
    {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_provisioning_artifacts_for_service_action(input.clone())
        }))
    }

    /// <p>Lists the specified requests or all performed requests.</p>
    async fn list_record_history(
        &self,
        input: ListRecordHistoryInput,
    ) -> Result<ListRecordHistoryOutput, RusotoError<ListRecordHistoryError>>;

    /// Auto-paginating version of `list_record_history`
    fn list_record_history_pages<'a>(
        &'a self,
        mut input: ListRecordHistoryInput,
    ) -> RusotoStream<'a, RecordDetail, ListRecordHistoryError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_record_history(input.clone())
        }))
    }

    /// <p>Lists the resources associated with the specified TagOption.</p>
    async fn list_resources_for_tag_option(
        &self,
        input: ListResourcesForTagOptionInput,
    ) -> Result<ListResourcesForTagOptionOutput, RusotoError<ListResourcesForTagOptionError>>;

    /// Auto-paginating version of `list_resources_for_tag_option`
    fn list_resources_for_tag_option_pages<'a>(
        &'a self,
        mut input: ListResourcesForTagOptionInput,
    ) -> RusotoStream<'a, ResourceDetail, ListResourcesForTagOptionError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_resources_for_tag_option(input.clone())
        }))
    }

    /// <p>Lists all self-service actions.</p>
    async fn list_service_actions(
        &self,
        input: ListServiceActionsInput,
    ) -> Result<ListServiceActionsOutput, RusotoError<ListServiceActionsError>>;

    /// Auto-paginating version of `list_service_actions`
    fn list_service_actions_pages<'a>(
        &'a self,
        mut input: ListServiceActionsInput,
    ) -> RusotoStream<'a, ServiceActionSummary, ListServiceActionsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_service_actions(input.clone())
        }))
    }

    /// <p>Returns a paginated list of self-service actions associated with the specified Product ID and Provisioning Artifact ID.</p>
    async fn list_service_actions_for_provisioning_artifact(
        &self,
        input: ListServiceActionsForProvisioningArtifactInput,
    ) -> Result<
        ListServiceActionsForProvisioningArtifactOutput,
        RusotoError<ListServiceActionsForProvisioningArtifactError>,
    >;

    /// Auto-paginating version of `list_service_actions_for_provisioning_artifact`
    fn list_service_actions_for_provisioning_artifact_pages<'a>(
        &'a self,
        mut input: ListServiceActionsForProvisioningArtifactInput,
    ) -> RusotoStream<'a, ServiceActionSummary, ListServiceActionsForProvisioningArtifactError>
    {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_service_actions_for_provisioning_artifact(input.clone())
        }))
    }

    /// <p>Returns summary information about stack instances that are associated with the specified <code>CFN_STACKSET</code> type provisioned product. You can filter for stack instances that are associated with a specific AWS account name or region. </p>
    async fn list_stack_instances_for_provisioned_product(
        &self,
        input: ListStackInstancesForProvisionedProductInput,
    ) -> Result<
        ListStackInstancesForProvisionedProductOutput,
        RusotoError<ListStackInstancesForProvisionedProductError>,
    >;

    /// <p>Lists the specified TagOptions or all TagOptions.</p>
    async fn list_tag_options(
        &self,
        input: ListTagOptionsInput,
    ) -> Result<ListTagOptionsOutput, RusotoError<ListTagOptionsError>>;

    /// Auto-paginating version of `list_tag_options`
    fn list_tag_options_pages<'a>(
        &'a self,
        mut input: ListTagOptionsInput,
    ) -> RusotoStream<'a, TagOptionDetail, ListTagOptionsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_tag_options(input.clone())
        }))
    }

    /// <p>Provisions the specified product.</p> <p>A provisioned product is a resourced instance of a product. For example, provisioning a product based on a CloudFormation template launches a CloudFormation stack and its underlying resources. You can check the status of this request using <a>DescribeRecord</a>.</p> <p>If the request contains a tag key with an empty list of values, there is a tag conflict for that key. Do not include conflicted keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>".</p>
    async fn provision_product(
        &self,
        input: ProvisionProductInput,
    ) -> Result<ProvisionProductOutput, RusotoError<ProvisionProductError>>;

    /// <p>Rejects an offer to share the specified portfolio.</p>
    async fn reject_portfolio_share(
        &self,
        input: RejectPortfolioShareInput,
    ) -> Result<RejectPortfolioShareOutput, RusotoError<RejectPortfolioShareError>>;

    /// <p>Lists the provisioned products that are available (not terminated).</p> <p>To use additional filtering, see <a>SearchProvisionedProducts</a>.</p>
    async fn scan_provisioned_products(
        &self,
        input: ScanProvisionedProductsInput,
    ) -> Result<ScanProvisionedProductsOutput, RusotoError<ScanProvisionedProductsError>>;

    /// Auto-paginating version of `scan_provisioned_products`
    fn scan_provisioned_products_pages<'a>(
        &'a self,
        mut input: ScanProvisionedProductsInput,
    ) -> RusotoStream<'a, ProvisionedProductDetail, ScanProvisionedProductsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.scan_provisioned_products(input.clone())
        }))
    }

    /// <p>Gets information about the products to which the caller has access.</p>
    async fn search_products(
        &self,
        input: SearchProductsInput,
    ) -> Result<SearchProductsOutput, RusotoError<SearchProductsError>>;

    /// <p>Gets information about the products for the specified portfolio or all products.</p>
    async fn search_products_as_admin(
        &self,
        input: SearchProductsAsAdminInput,
    ) -> Result<SearchProductsAsAdminOutput, RusotoError<SearchProductsAsAdminError>>;

    /// Auto-paginating version of `search_products_as_admin`
    fn search_products_as_admin_pages<'a>(
        &'a self,
        mut input: SearchProductsAsAdminInput,
    ) -> RusotoStream<'a, ProductViewDetail, SearchProductsAsAdminError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.search_products_as_admin(input.clone())
        }))
    }

    /// <p>Gets information about the provisioned products that meet the specified criteria.</p>
    async fn search_provisioned_products(
        &self,
        input: SearchProvisionedProductsInput,
    ) -> Result<SearchProvisionedProductsOutput, RusotoError<SearchProvisionedProductsError>>;

    /// <p>Terminates the specified provisioned product.</p> <p>This operation does not delete any records associated with the provisioned product.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    async fn terminate_provisioned_product(
        &self,
        input: TerminateProvisionedProductInput,
    ) -> Result<TerminateProvisionedProductOutput, RusotoError<TerminateProvisionedProductError>>;

    /// <p>Updates the specified constraint.</p>
    async fn update_constraint(
        &self,
        input: UpdateConstraintInput,
    ) -> Result<UpdateConstraintOutput, RusotoError<UpdateConstraintError>>;

    /// <p>Updates the specified portfolio.</p> <p>You cannot update a product that was shared with you.</p>
    async fn update_portfolio(
        &self,
        input: UpdatePortfolioInput,
    ) -> Result<UpdatePortfolioOutput, RusotoError<UpdatePortfolioError>>;

    /// <p>Updates the specified portfolio share. You can use this API to enable or disable TagOptions sharing for an existing portfolio share. </p> <p>The portfolio share cannot be updated if the <code> CreatePortfolioShare</code> operation is <code>IN_PROGRESS</code>, as the share is not available to recipient entities. In this case, you must wait for the portfolio share to be COMPLETED.</p> <p>You must provide the <code>accountId</code> or organization node in the input, but not both.</p> <p>If the portfolio is shared to both an external account and an organization node, and both shares need to be updated, you must invoke <code>UpdatePortfolioShare</code> separately for each share type. </p> <p>This API cannot be used for removing the portfolio share. You must use <code>DeletePortfolioShare</code> API for that action. </p>
    async fn update_portfolio_share(
        &self,
        input: UpdatePortfolioShareInput,
    ) -> Result<UpdatePortfolioShareOutput, RusotoError<UpdatePortfolioShareError>>;

    /// <p>Updates the specified product.</p>
    async fn update_product(
        &self,
        input: UpdateProductInput,
    ) -> Result<UpdateProductOutput, RusotoError<UpdateProductError>>;

    /// <p>Requests updates to the configuration of the specified provisioned product.</p> <p>If there are tags associated with the object, they cannot be updated or added. Depending on the specific updates requested, this operation can update with no interruption, with some interruption, or replace the provisioned product entirely.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    async fn update_provisioned_product(
        &self,
        input: UpdateProvisionedProductInput,
    ) -> Result<UpdateProvisionedProductOutput, RusotoError<UpdateProvisionedProductError>>;

    /// <p>Requests updates to the properties of the specified provisioned product.</p>
    async fn update_provisioned_product_properties(
        &self,
        input: UpdateProvisionedProductPropertiesInput,
    ) -> Result<
        UpdateProvisionedProductPropertiesOutput,
        RusotoError<UpdateProvisionedProductPropertiesError>,
    >;

    /// <p>Updates the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot update a provisioning artifact for a product that was shared with you.</p>
    async fn update_provisioning_artifact(
        &self,
        input: UpdateProvisioningArtifactInput,
    ) -> Result<UpdateProvisioningArtifactOutput, RusotoError<UpdateProvisioningArtifactError>>;

    /// <p>Updates a self-service action.</p>
    async fn update_service_action(
        &self,
        input: UpdateServiceActionInput,
    ) -> Result<UpdateServiceActionOutput, RusotoError<UpdateServiceActionError>>;

    /// <p>Updates the specified TagOption.</p>
    async fn update_tag_option(
        &self,
        input: UpdateTagOptionInput,
    ) -> Result<UpdateTagOptionOutput, RusotoError<UpdateTagOptionError>>;
}
/// A client for the AWS Service Catalog API.
#[derive(Clone)]
pub struct ServiceCatalogClient {
    client: Client,
    region: region::Region,
}

impl ServiceCatalogClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServiceCatalogClient {
        ServiceCatalogClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServiceCatalogClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ServiceCatalogClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ServiceCatalogClient {
        ServiceCatalogClient { client, region }
    }
}

#[async_trait]
impl ServiceCatalog for ServiceCatalogClient {
    /// <p>Accepts an offer to share the specified portfolio.</p>
    async fn accept_portfolio_share(
        &self,
        input: AcceptPortfolioShareInput,
    ) -> Result<AcceptPortfolioShareOutput, RusotoError<AcceptPortfolioShareError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AcceptPortfolioShare",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AcceptPortfolioShareError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AcceptPortfolioShareOutput, _>()
    }

    /// <p>Associates the specified budget with the specified resource.</p>
    async fn associate_budget_with_resource(
        &self,
        input: AssociateBudgetWithResourceInput,
    ) -> Result<AssociateBudgetWithResourceOutput, RusotoError<AssociateBudgetWithResourceError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociateBudgetWithResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateBudgetWithResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateBudgetWithResourceOutput, _>()
    }

    /// <p>Associates the specified principal ARN with the specified portfolio.</p>
    async fn associate_principal_with_portfolio(
        &self,
        input: AssociatePrincipalWithPortfolioInput,
    ) -> Result<
        AssociatePrincipalWithPortfolioOutput,
        RusotoError<AssociatePrincipalWithPortfolioError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociatePrincipalWithPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociatePrincipalWithPortfolioError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociatePrincipalWithPortfolioOutput, _>()
    }

    /// <p>Associates the specified product with the specified portfolio.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn associate_product_with_portfolio(
        &self,
        input: AssociateProductWithPortfolioInput,
    ) -> Result<AssociateProductWithPortfolioOutput, RusotoError<AssociateProductWithPortfolioError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociateProductWithPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateProductWithPortfolioError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateProductWithPortfolioOutput, _>()
    }

    /// <p>Associates a self-service action with a provisioning artifact.</p>
    async fn associate_service_action_with_provisioning_artifact(
        &self,
        input: AssociateServiceActionWithProvisioningArtifactInput,
    ) -> Result<
        AssociateServiceActionWithProvisioningArtifactOutput,
        RusotoError<AssociateServiceActionWithProvisioningArtifactError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociateServiceActionWithProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                AssociateServiceActionWithProvisioningArtifactError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateServiceActionWithProvisioningArtifactOutput, _>()
    }

    /// <p>Associate the specified TagOption with the specified portfolio or product.</p>
    async fn associate_tag_option_with_resource(
        &self,
        input: AssociateTagOptionWithResourceInput,
    ) -> Result<
        AssociateTagOptionWithResourceOutput,
        RusotoError<AssociateTagOptionWithResourceError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.AssociateTagOptionWithResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateTagOptionWithResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateTagOptionWithResourceOutput, _>()
    }

    /// <p>Associates multiple self-service actions with provisioning artifacts.</p>
    async fn batch_associate_service_action_with_provisioning_artifact(
        &self,
        input: BatchAssociateServiceActionWithProvisioningArtifactInput,
    ) -> Result<
        BatchAssociateServiceActionWithProvisioningArtifactOutput,
        RusotoError<BatchAssociateServiceActionWithProvisioningArtifactError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.BatchAssociateServiceActionWithProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                BatchAssociateServiceActionWithProvisioningArtifactError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<BatchAssociateServiceActionWithProvisioningArtifactOutput, _>()
    }

    /// <p>Disassociates a batch of self-service actions from the specified provisioning artifact.</p>
    async fn batch_disassociate_service_action_from_provisioning_artifact(
        &self,
        input: BatchDisassociateServiceActionFromProvisioningArtifactInput,
    ) -> Result<
        BatchDisassociateServiceActionFromProvisioningArtifactOutput,
        RusotoError<BatchDisassociateServiceActionFromProvisioningArtifactError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.BatchDisassociateServiceActionFromProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                BatchDisassociateServiceActionFromProvisioningArtifactError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<BatchDisassociateServiceActionFromProvisioningArtifactOutput, _>()
    }

    /// <p>Copies the specified source product to the specified target product or a new product.</p> <p>You can copy a product to the same account or another account. You can copy a product to the same region or another region.</p> <p>This operation is performed asynchronously. To track the progress of the operation, use <a>DescribeCopyProductStatus</a>.</p>
    async fn copy_product(
        &self,
        input: CopyProductInput,
    ) -> Result<CopyProductOutput, RusotoError<CopyProductError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.CopyProduct");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CopyProductError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CopyProductOutput, _>()
    }

    /// <p>Creates a constraint.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn create_constraint(
        &self,
        input: CreateConstraintInput,
    ) -> Result<CreateConstraintOutput, RusotoError<CreateConstraintError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateConstraint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateConstraintError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateConstraintOutput, _>()
    }

    /// <p>Creates a portfolio.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn create_portfolio(
        &self,
        input: CreatePortfolioInput,
    ) -> Result<CreatePortfolioOutput, RusotoError<CreatePortfolioError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreatePortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreatePortfolioError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreatePortfolioOutput, _>()
    }

    /// <p>Shares the specified portfolio with the specified account or organization node. Shares to an organization node can only be created by the management account of an organization or by a delegated administrator. You can share portfolios to an organization, an organizational unit, or a specific account.</p> <p>Note that if a delegated admin is de-registered, they can no longer create portfolio shares.</p> <p> <code>AWSOrganizationsAccess</code> must be enabled in order to create a portfolio share to an organization node.</p> <p>You can't share a shared resource, including portfolios that contain a shared product.</p> <p>If the portfolio share with the specified account or organization node already exists, this action will have no effect and will not return an error. To update an existing share, you must use the <code> UpdatePortfolioShare</code> API instead.</p>
    async fn create_portfolio_share(
        &self,
        input: CreatePortfolioShareInput,
    ) -> Result<CreatePortfolioShareOutput, RusotoError<CreatePortfolioShareError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreatePortfolioShare",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreatePortfolioShareError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreatePortfolioShareOutput, _>()
    }

    /// <p>Creates a product.</p> <p>A delegated admin is authorized to invoke this command.</p> <p>The user or role that performs this operation must have the <code>cloudformation:GetTemplate</code> IAM policy permission. This policy permission is required when using the <code>ImportFromPhysicalId</code> template source in the information data section.</p>
    async fn create_product(
        &self,
        input: CreateProductInput,
    ) -> Result<CreateProductOutput, RusotoError<CreateProductError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.CreateProduct");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateProductError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateProductOutput, _>()
    }

    /// <p>Creates a plan. A plan includes the list of resources to be created (when provisioning a new product) or modified (when updating a provisioned product) when the plan is executed.</p> <p>You can create one plan per provisioned product. To create a plan for an existing provisioned product, the product status must be AVAILBLE or TAINTED.</p> <p>To view the resource changes in the change set, use <a>DescribeProvisionedProductPlan</a>. To create or modify the provisioned product, use <a>ExecuteProvisionedProductPlan</a>.</p>
    async fn create_provisioned_product_plan(
        &self,
        input: CreateProvisionedProductPlanInput,
    ) -> Result<CreateProvisionedProductPlanOutput, RusotoError<CreateProvisionedProductPlanError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateProvisionedProductPlanError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateProvisionedProductPlanOutput, _>()
    }

    /// <p>Creates a provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot create a provisioning artifact for a product that was shared with you.</p> <p>The user or role that performs this operation must have the <code>cloudformation:GetTemplate</code> IAM policy permission. This policy permission is required when using the <code>ImportFromPhysicalId</code> template source in the information data section.</p>
    async fn create_provisioning_artifact(
        &self,
        input: CreateProvisioningArtifactInput,
    ) -> Result<CreateProvisioningArtifactOutput, RusotoError<CreateProvisioningArtifactError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateProvisioningArtifactError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateProvisioningArtifactOutput, _>()
    }

    /// <p>Creates a self-service action.</p>
    async fn create_service_action(
        &self,
        input: CreateServiceActionInput,
    ) -> Result<CreateServiceActionOutput, RusotoError<CreateServiceActionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateServiceAction",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateServiceActionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateServiceActionOutput, _>()
    }

    /// <p>Creates a TagOption.</p>
    async fn create_tag_option(
        &self,
        input: CreateTagOptionInput,
    ) -> Result<CreateTagOptionOutput, RusotoError<CreateTagOptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.CreateTagOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateTagOptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateTagOptionOutput, _>()
    }

    /// <p>Deletes the specified constraint.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn delete_constraint(
        &self,
        input: DeleteConstraintInput,
    ) -> Result<DeleteConstraintOutput, RusotoError<DeleteConstraintError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteConstraint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteConstraintError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteConstraintOutput, _>()
    }

    /// <p>Deletes the specified portfolio.</p> <p>You cannot delete a portfolio if it was shared with you or if it has associated products, users, constraints, or shared accounts.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn delete_portfolio(
        &self,
        input: DeletePortfolioInput,
    ) -> Result<DeletePortfolioOutput, RusotoError<DeletePortfolioError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeletePortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeletePortfolioError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeletePortfolioOutput, _>()
    }

    /// <p>Stops sharing the specified portfolio with the specified account or organization node. Shares to an organization node can only be deleted by the management account of an organization or by a delegated administrator.</p> <p>Note that if a delegated admin is de-registered, portfolio shares created from that account are removed.</p>
    async fn delete_portfolio_share(
        &self,
        input: DeletePortfolioShareInput,
    ) -> Result<DeletePortfolioShareOutput, RusotoError<DeletePortfolioShareError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeletePortfolioShare",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeletePortfolioShareError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeletePortfolioShareOutput, _>()
    }

    /// <p>Deletes the specified product.</p> <p>You cannot delete a product if it was shared with you or is associated with a portfolio.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn delete_product(
        &self,
        input: DeleteProductInput,
    ) -> Result<DeleteProductOutput, RusotoError<DeleteProductError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.DeleteProduct");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteProductError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteProductOutput, _>()
    }

    /// <p>Deletes the specified plan.</p>
    async fn delete_provisioned_product_plan(
        &self,
        input: DeleteProvisionedProductPlanInput,
    ) -> Result<DeleteProvisionedProductPlanOutput, RusotoError<DeleteProvisionedProductPlanError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteProvisionedProductPlanError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteProvisionedProductPlanOutput, _>()
    }

    /// <p>Deletes the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot delete a provisioning artifact associated with a product that was shared with you. You cannot delete the last provisioning artifact for a product, because a product must have at least one provisioning artifact.</p>
    async fn delete_provisioning_artifact(
        &self,
        input: DeleteProvisioningArtifactInput,
    ) -> Result<DeleteProvisioningArtifactOutput, RusotoError<DeleteProvisioningArtifactError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteProvisioningArtifactError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteProvisioningArtifactOutput, _>()
    }

    /// <p>Deletes a self-service action.</p>
    async fn delete_service_action(
        &self,
        input: DeleteServiceActionInput,
    ) -> Result<DeleteServiceActionOutput, RusotoError<DeleteServiceActionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteServiceAction",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteServiceActionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteServiceActionOutput, _>()
    }

    /// <p>Deletes the specified TagOption.</p> <p>You cannot delete a TagOption if it is associated with a product or portfolio.</p>
    async fn delete_tag_option(
        &self,
        input: DeleteTagOptionInput,
    ) -> Result<DeleteTagOptionOutput, RusotoError<DeleteTagOptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DeleteTagOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteTagOptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteTagOptionOutput, _>()
    }

    /// <p>Gets information about the specified constraint.</p>
    async fn describe_constraint(
        &self,
        input: DescribeConstraintInput,
    ) -> Result<DescribeConstraintOutput, RusotoError<DescribeConstraintError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeConstraint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeConstraintError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeConstraintOutput, _>()
    }

    /// <p>Gets the status of the specified copy product operation.</p>
    async fn describe_copy_product_status(
        &self,
        input: DescribeCopyProductStatusInput,
    ) -> Result<DescribeCopyProductStatusOutput, RusotoError<DescribeCopyProductStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeCopyProductStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeCopyProductStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeCopyProductStatusOutput, _>()
    }

    /// <p>Gets information about the specified portfolio.</p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn describe_portfolio(
        &self,
        input: DescribePortfolioInput,
    ) -> Result<DescribePortfolioOutput, RusotoError<DescribePortfolioError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribePortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribePortfolioError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribePortfolioOutput, _>()
    }

    /// <p>Gets the status of the specified portfolio share operation. This API can only be called by the management account in the organization or by a delegated admin.</p>
    async fn describe_portfolio_share_status(
        &self,
        input: DescribePortfolioShareStatusInput,
    ) -> Result<DescribePortfolioShareStatusOutput, RusotoError<DescribePortfolioShareStatusError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribePortfolioShareStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribePortfolioShareStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribePortfolioShareStatusOutput, _>()
    }

    /// <p>Returns a summary of each of the portfolio shares that were created for the specified portfolio.</p> <p>You can use this API to determine which accounts or organizational nodes this portfolio have been shared, whether the recipient entity has imported the share, and whether TagOptions are included with the share.</p> <p>The <code>PortfolioId</code> and <code>Type</code> parameters are both required.</p>
    async fn describe_portfolio_shares(
        &self,
        input: DescribePortfolioSharesInput,
    ) -> Result<DescribePortfolioSharesOutput, RusotoError<DescribePortfolioSharesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribePortfolioShares",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribePortfolioSharesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribePortfolioSharesOutput, _>()
    }

    /// <p>Gets information about the specified product.</p>
    async fn describe_product(
        &self,
        input: DescribeProductInput,
    ) -> Result<DescribeProductOutput, RusotoError<DescribeProductError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeProductError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeProductOutput, _>()
    }

    /// <p>Gets information about the specified product. This operation is run with administrator access.</p>
    async fn describe_product_as_admin(
        &self,
        input: DescribeProductAsAdminInput,
    ) -> Result<DescribeProductAsAdminOutput, RusotoError<DescribeProductAsAdminError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProductAsAdmin",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeProductAsAdminError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeProductAsAdminOutput, _>()
    }

    /// <p>Gets information about the specified product.</p>
    async fn describe_product_view(
        &self,
        input: DescribeProductViewInput,
    ) -> Result<DescribeProductViewOutput, RusotoError<DescribeProductViewError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProductView",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeProductViewError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeProductViewOutput, _>()
    }

    /// <p>Gets information about the specified provisioned product.</p>
    async fn describe_provisioned_product(
        &self,
        input: DescribeProvisionedProductInput,
    ) -> Result<DescribeProvisionedProductOutput, RusotoError<DescribeProvisionedProductError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisionedProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeProvisionedProductError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeProvisionedProductOutput, _>()
    }

    /// <p>Gets information about the resource changes for the specified plan.</p>
    async fn describe_provisioned_product_plan(
        &self,
        input: DescribeProvisionedProductPlanInput,
    ) -> Result<
        DescribeProvisionedProductPlanOutput,
        RusotoError<DescribeProvisionedProductPlanError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeProvisionedProductPlanError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeProvisionedProductPlanOutput, _>()
    }

    /// <p>Gets information about the specified provisioning artifact (also known as a version) for the specified product.</p>
    async fn describe_provisioning_artifact(
        &self,
        input: DescribeProvisioningArtifactInput,
    ) -> Result<DescribeProvisioningArtifactOutput, RusotoError<DescribeProvisioningArtifactError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeProvisioningArtifactError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeProvisioningArtifactOutput, _>()
    }

    /// <p>Gets information about the configuration required to provision the specified product using the specified provisioning artifact.</p> <p>If the output contains a TagOption key with an empty list of values, there is a TagOption conflict for that key. The end user cannot take action to fix the conflict, and launch is not blocked. In subsequent calls to <a>ProvisionProduct</a>, do not include conflicted TagOption keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>". Tag the provisioned product with the value <code>sc-tagoption-conflict-portfolioId-productId</code>.</p>
    async fn describe_provisioning_parameters(
        &self,
        input: DescribeProvisioningParametersInput,
    ) -> Result<
        DescribeProvisioningParametersOutput,
        RusotoError<DescribeProvisioningParametersError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeProvisioningParameters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeProvisioningParametersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeProvisioningParametersOutput, _>()
    }

    /// <p><p>Gets information about the specified request operation.</p> <p>Use this operation after calling a request operation (for example, <a>ProvisionProduct</a>, <a>TerminateProvisionedProduct</a>, or <a>UpdateProvisionedProduct</a>). </p> <note> <p>If a provisioned product was transferred to a new owner using <a>UpdateProvisionedProductProperties</a>, the new owner will be able to describe all past records for that product. The previous owner will no longer be able to describe the records, but will be able to use <a>ListRecordHistory</a> to see the product&#39;s history from when he was the owner.</p> </note></p>
    async fn describe_record(
        &self,
        input: DescribeRecordInput,
    ) -> Result<DescribeRecordOutput, RusotoError<DescribeRecordError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.DescribeRecord");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeRecordError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeRecordOutput, _>()
    }

    /// <p>Describes a self-service action.</p>
    async fn describe_service_action(
        &self,
        input: DescribeServiceActionInput,
    ) -> Result<DescribeServiceActionOutput, RusotoError<DescribeServiceActionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeServiceAction",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeServiceActionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeServiceActionOutput, _>()
    }

    /// <p>Finds the default parameters for a specific self-service action on a specific provisioned product and returns a map of the results to the user.</p>
    async fn describe_service_action_execution_parameters(
        &self,
        input: DescribeServiceActionExecutionParametersInput,
    ) -> Result<
        DescribeServiceActionExecutionParametersOutput,
        RusotoError<DescribeServiceActionExecutionParametersError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeServiceActionExecutionParameters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeServiceActionExecutionParametersError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeServiceActionExecutionParametersOutput, _>()
    }

    /// <p>Gets information about the specified TagOption.</p>
    async fn describe_tag_option(
        &self,
        input: DescribeTagOptionInput,
    ) -> Result<DescribeTagOptionOutput, RusotoError<DescribeTagOptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DescribeTagOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTagOptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeTagOptionOutput, _>()
    }

    /// <p>Disable portfolio sharing through AWS Organizations feature. This feature will not delete your current shares but it will prevent you from creating new shares throughout your organization. Current shares will not be in sync with your organization structure if it changes after calling this API. This API can only be called by the management account in the organization.</p> <p>This API can't be invoked if there are active delegated administrators in the organization.</p> <p>Note that a delegated administrator is not authorized to invoke <code>DisableAWSOrganizationsAccess</code>.</p>
    async fn disable_aws_organizations_access(
        &self,
    ) -> Result<DisableAWSOrganizationsAccessOutput, RusotoError<DisableAWSOrganizationsAccessError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisableAWSOrganizationsAccess",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DisableAWSOrganizationsAccessError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisableAWSOrganizationsAccessOutput, _>()
    }

    /// <p>Disassociates the specified budget from the specified resource.</p>
    async fn disassociate_budget_from_resource(
        &self,
        input: DisassociateBudgetFromResourceInput,
    ) -> Result<
        DisassociateBudgetFromResourceOutput,
        RusotoError<DisassociateBudgetFromResourceError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociateBudgetFromResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisassociateBudgetFromResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateBudgetFromResourceOutput, _>()
    }

    /// <p>Disassociates a previously associated principal ARN from a specified portfolio.</p>
    async fn disassociate_principal_from_portfolio(
        &self,
        input: DisassociatePrincipalFromPortfolioInput,
    ) -> Result<
        DisassociatePrincipalFromPortfolioOutput,
        RusotoError<DisassociatePrincipalFromPortfolioError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociatePrincipalFromPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DisassociatePrincipalFromPortfolioError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociatePrincipalFromPortfolioOutput, _>()
    }

    /// <p>Disassociates the specified product from the specified portfolio. </p> <p>A delegated admin is authorized to invoke this command.</p>
    async fn disassociate_product_from_portfolio(
        &self,
        input: DisassociateProductFromPortfolioInput,
    ) -> Result<
        DisassociateProductFromPortfolioOutput,
        RusotoError<DisassociateProductFromPortfolioError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociateProductFromPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DisassociateProductFromPortfolioError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateProductFromPortfolioOutput, _>()
    }

    /// <p>Disassociates the specified self-service action association from the specified provisioning artifact.</p>
    async fn disassociate_service_action_from_provisioning_artifact(
        &self,
        input: DisassociateServiceActionFromProvisioningArtifactInput,
    ) -> Result<
        DisassociateServiceActionFromProvisioningArtifactOutput,
        RusotoError<DisassociateServiceActionFromProvisioningArtifactError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociateServiceActionFromProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DisassociateServiceActionFromProvisioningArtifactError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateServiceActionFromProvisioningArtifactOutput, _>()
    }

    /// <p>Disassociates the specified TagOption from the specified resource.</p>
    async fn disassociate_tag_option_from_resource(
        &self,
        input: DisassociateTagOptionFromResourceInput,
    ) -> Result<
        DisassociateTagOptionFromResourceOutput,
        RusotoError<DisassociateTagOptionFromResourceError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.DisassociateTagOptionFromResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DisassociateTagOptionFromResourceError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateTagOptionFromResourceOutput, _>()
    }

    /// <p>Enable portfolio sharing feature through AWS Organizations. This API will allow Service Catalog to receive updates on your organization in order to sync your shares with the current structure. This API can only be called by the management account in the organization.</p> <p>By calling this API Service Catalog will make a call to organizations:EnableAWSServiceAccess on your behalf so that your shares can be in sync with any changes in your AWS Organizations structure.</p> <p>Note that a delegated administrator is not authorized to invoke <code>EnableAWSOrganizationsAccess</code>.</p>
    async fn enable_aws_organizations_access(
        &self,
    ) -> Result<EnableAWSOrganizationsAccessOutput, RusotoError<EnableAWSOrganizationsAccessError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.EnableAWSOrganizationsAccess",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, EnableAWSOrganizationsAccessError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<EnableAWSOrganizationsAccessOutput, _>()
    }

    /// <p>Provisions or modifies a product based on the resource changes for the specified plan.</p>
    async fn execute_provisioned_product_plan(
        &self,
        input: ExecuteProvisionedProductPlanInput,
    ) -> Result<ExecuteProvisionedProductPlanOutput, RusotoError<ExecuteProvisionedProductPlanError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ExecuteProvisionedProductPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ExecuteProvisionedProductPlanError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ExecuteProvisionedProductPlanOutput, _>()
    }

    /// <p>Executes a self-service action against a provisioned product.</p>
    async fn execute_provisioned_product_service_action(
        &self,
        input: ExecuteProvisionedProductServiceActionInput,
    ) -> Result<
        ExecuteProvisionedProductServiceActionOutput,
        RusotoError<ExecuteProvisionedProductServiceActionError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ExecuteProvisionedProductServiceAction",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ExecuteProvisionedProductServiceActionError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ExecuteProvisionedProductServiceActionOutput, _>()
    }

    /// <p>Get the Access Status for AWS Organization portfolio share feature. This API can only be called by the management account in the organization or by a delegated admin.</p>
    async fn get_aws_organizations_access_status(
        &self,
    ) -> Result<
        GetAWSOrganizationsAccessStatusOutput,
        RusotoError<GetAWSOrganizationsAccessStatusError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.GetAWSOrganizationsAccessStatus",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, GetAWSOrganizationsAccessStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetAWSOrganizationsAccessStatusOutput, _>()
    }

    /// <p>This API takes either a <code>ProvisonedProductId</code> or a <code>ProvisionedProductName</code>, along with a list of one or more output keys, and responds with the key/value pairs of those outputs.</p>
    async fn get_provisioned_product_outputs(
        &self,
        input: GetProvisionedProductOutputsInput,
    ) -> Result<GetProvisionedProductOutputsOutput, RusotoError<GetProvisionedProductOutputsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.GetProvisionedProductOutputs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetProvisionedProductOutputsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetProvisionedProductOutputsOutput, _>()
    }

    /// <p>Requests the import of a resource as a Service Catalog provisioned product that is associated to a Service Catalog product and provisioning artifact. Once imported, all supported Service Catalog governance actions are supported on the provisioned product.</p> <p>Resource import only supports CloudFormation stack ARNs. CloudFormation StackSets and non-root nested stacks are not supported.</p> <p>The CloudFormation stack must have one of the following statuses to be imported: <code>CREATE_COMPLETE</code>, <code>UPDATE_COMPLETE</code>, <code>UPDATE_ROLLBACK_COMPLETE</code>, <code>IMPORT_COMPLETE</code>, <code>IMPORT_ROLLBACK_COMPLETE</code>.</p> <p>Import of the resource requires that the CloudFormation stack template matches the associated Service Catalog product provisioning artifact. </p> <p>The user or role that performs this operation must have the <code>cloudformation:GetTemplate</code> and <code>cloudformation:DescribeStacks</code> IAM policy permissions. </p>
    async fn import_as_provisioned_product(
        &self,
        input: ImportAsProvisionedProductInput,
    ) -> Result<ImportAsProvisionedProductOutput, RusotoError<ImportAsProvisionedProductError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ImportAsProvisionedProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ImportAsProvisionedProductError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ImportAsProvisionedProductOutput, _>()
    }

    /// <p>Lists all portfolios for which sharing was accepted by this account.</p>
    async fn list_accepted_portfolio_shares(
        &self,
        input: ListAcceptedPortfolioSharesInput,
    ) -> Result<ListAcceptedPortfolioSharesOutput, RusotoError<ListAcceptedPortfolioSharesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListAcceptedPortfolioShares",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAcceptedPortfolioSharesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListAcceptedPortfolioSharesOutput, _>()
    }

    /// <p>Lists all the budgets associated to the specified resource.</p>
    async fn list_budgets_for_resource(
        &self,
        input: ListBudgetsForResourceInput,
    ) -> Result<ListBudgetsForResourceOutput, RusotoError<ListBudgetsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListBudgetsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListBudgetsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListBudgetsForResourceOutput, _>()
    }

    /// <p>Lists the constraints for the specified portfolio and product.</p>
    async fn list_constraints_for_portfolio(
        &self,
        input: ListConstraintsForPortfolioInput,
    ) -> Result<ListConstraintsForPortfolioOutput, RusotoError<ListConstraintsForPortfolioError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListConstraintsForPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListConstraintsForPortfolioError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListConstraintsForPortfolioOutput, _>()
    }

    /// <p>Lists the paths to the specified product. A path is how the user has access to a specified product, and is necessary when provisioning a product. A path also determines the constraints put on the product.</p>
    async fn list_launch_paths(
        &self,
        input: ListLaunchPathsInput,
    ) -> Result<ListLaunchPathsOutput, RusotoError<ListLaunchPathsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListLaunchPaths",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLaunchPathsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListLaunchPathsOutput, _>()
    }

    /// <p>Lists the organization nodes that have access to the specified portfolio. This API can only be called by the management account in the organization or by a delegated admin.</p> <p>If a delegated admin is de-registered, they can no longer perform this operation.</p>
    async fn list_organization_portfolio_access(
        &self,
        input: ListOrganizationPortfolioAccessInput,
    ) -> Result<
        ListOrganizationPortfolioAccessOutput,
        RusotoError<ListOrganizationPortfolioAccessError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListOrganizationPortfolioAccess",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListOrganizationPortfolioAccessError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListOrganizationPortfolioAccessOutput, _>()
    }

    /// <p>Lists the account IDs that have access to the specified portfolio.</p> <p>A delegated admin can list the accounts that have access to the shared portfolio. Note that if a delegated admin is de-registered, they can no longer perform this operation.</p>
    async fn list_portfolio_access(
        &self,
        input: ListPortfolioAccessInput,
    ) -> Result<ListPortfolioAccessOutput, RusotoError<ListPortfolioAccessError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListPortfolioAccess",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListPortfolioAccessError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListPortfolioAccessOutput, _>()
    }

    /// <p>Lists all portfolios in the catalog.</p>
    async fn list_portfolios(
        &self,
        input: ListPortfoliosInput,
    ) -> Result<ListPortfoliosOutput, RusotoError<ListPortfoliosError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.ListPortfolios");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListPortfoliosError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListPortfoliosOutput, _>()
    }

    /// <p>Lists all portfolios that the specified product is associated with.</p>
    async fn list_portfolios_for_product(
        &self,
        input: ListPortfoliosForProductInput,
    ) -> Result<ListPortfoliosForProductOutput, RusotoError<ListPortfoliosForProductError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListPortfoliosForProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListPortfoliosForProductError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListPortfoliosForProductOutput, _>()
    }

    /// <p>Lists all principal ARNs associated with the specified portfolio.</p>
    async fn list_principals_for_portfolio(
        &self,
        input: ListPrincipalsForPortfolioInput,
    ) -> Result<ListPrincipalsForPortfolioOutput, RusotoError<ListPrincipalsForPortfolioError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListPrincipalsForPortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListPrincipalsForPortfolioError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListPrincipalsForPortfolioOutput, _>()
    }

    /// <p>Lists the plans for the specified provisioned product or all plans to which the user has access.</p>
    async fn list_provisioned_product_plans(
        &self,
        input: ListProvisionedProductPlansInput,
    ) -> Result<ListProvisionedProductPlansOutput, RusotoError<ListProvisionedProductPlansError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListProvisionedProductPlans",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListProvisionedProductPlansError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListProvisionedProductPlansOutput, _>()
    }

    /// <p>Lists all provisioning artifacts (also known as versions) for the specified product.</p>
    async fn list_provisioning_artifacts(
        &self,
        input: ListProvisioningArtifactsInput,
    ) -> Result<ListProvisioningArtifactsOutput, RusotoError<ListProvisioningArtifactsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListProvisioningArtifacts",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListProvisioningArtifactsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListProvisioningArtifactsOutput, _>()
    }

    /// <p>Lists all provisioning artifacts (also known as versions) for the specified self-service action.</p>
    async fn list_provisioning_artifacts_for_service_action(
        &self,
        input: ListProvisioningArtifactsForServiceActionInput,
    ) -> Result<
        ListProvisioningArtifactsForServiceActionOutput,
        RusotoError<ListProvisioningArtifactsForServiceActionError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListProvisioningArtifactsForServiceAction",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListProvisioningArtifactsForServiceActionError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListProvisioningArtifactsForServiceActionOutput, _>()
    }

    /// <p>Lists the specified requests or all performed requests.</p>
    async fn list_record_history(
        &self,
        input: ListRecordHistoryInput,
    ) -> Result<ListRecordHistoryOutput, RusotoError<ListRecordHistoryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListRecordHistory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRecordHistoryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListRecordHistoryOutput, _>()
    }

    /// <p>Lists the resources associated with the specified TagOption.</p>
    async fn list_resources_for_tag_option(
        &self,
        input: ListResourcesForTagOptionInput,
    ) -> Result<ListResourcesForTagOptionOutput, RusotoError<ListResourcesForTagOptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListResourcesForTagOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListResourcesForTagOptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListResourcesForTagOptionOutput, _>()
    }

    /// <p>Lists all self-service actions.</p>
    async fn list_service_actions(
        &self,
        input: ListServiceActionsInput,
    ) -> Result<ListServiceActionsOutput, RusotoError<ListServiceActionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListServiceActions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListServiceActionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListServiceActionsOutput, _>()
    }

    /// <p>Returns a paginated list of self-service actions associated with the specified Product ID and Provisioning Artifact ID.</p>
    async fn list_service_actions_for_provisioning_artifact(
        &self,
        input: ListServiceActionsForProvisioningArtifactInput,
    ) -> Result<
        ListServiceActionsForProvisioningArtifactOutput,
        RusotoError<ListServiceActionsForProvisioningArtifactError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListServiceActionsForProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListServiceActionsForProvisioningArtifactError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListServiceActionsForProvisioningArtifactOutput, _>()
    }

    /// <p>Returns summary information about stack instances that are associated with the specified <code>CFN_STACKSET</code> type provisioned product. You can filter for stack instances that are associated with a specific AWS account name or region. </p>
    async fn list_stack_instances_for_provisioned_product(
        &self,
        input: ListStackInstancesForProvisionedProductInput,
    ) -> Result<
        ListStackInstancesForProvisionedProductOutput,
        RusotoError<ListStackInstancesForProvisionedProductError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ListStackInstancesForProvisionedProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListStackInstancesForProvisionedProductError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListStackInstancesForProvisionedProductOutput, _>()
    }

    /// <p>Lists the specified TagOptions or all TagOptions.</p>
    async fn list_tag_options(
        &self,
        input: ListTagOptionsInput,
    ) -> Result<ListTagOptionsOutput, RusotoError<ListTagOptionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.ListTagOptions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagOptionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagOptionsOutput, _>()
    }

    /// <p>Provisions the specified product.</p> <p>A provisioned product is a resourced instance of a product. For example, provisioning a product based on a CloudFormation template launches a CloudFormation stack and its underlying resources. You can check the status of this request using <a>DescribeRecord</a>.</p> <p>If the request contains a tag key with an empty list of values, there is a tag conflict for that key. Do not include conflicted keys as tags, or this causes the error "Parameter validation failed: Missing required parameter in Tags[<i>N</i>]:<i>Value</i>".</p>
    async fn provision_product(
        &self,
        input: ProvisionProductInput,
    ) -> Result<ProvisionProductOutput, RusotoError<ProvisionProductError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ProvisionProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ProvisionProductError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ProvisionProductOutput, _>()
    }

    /// <p>Rejects an offer to share the specified portfolio.</p>
    async fn reject_portfolio_share(
        &self,
        input: RejectPortfolioShareInput,
    ) -> Result<RejectPortfolioShareOutput, RusotoError<RejectPortfolioShareError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.RejectPortfolioShare",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RejectPortfolioShareError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RejectPortfolioShareOutput, _>()
    }

    /// <p>Lists the provisioned products that are available (not terminated).</p> <p>To use additional filtering, see <a>SearchProvisionedProducts</a>.</p>
    async fn scan_provisioned_products(
        &self,
        input: ScanProvisionedProductsInput,
    ) -> Result<ScanProvisionedProductsOutput, RusotoError<ScanProvisionedProductsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.ScanProvisionedProducts",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ScanProvisionedProductsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ScanProvisionedProductsOutput, _>()
    }

    /// <p>Gets information about the products to which the caller has access.</p>
    async fn search_products(
        &self,
        input: SearchProductsInput,
    ) -> Result<SearchProductsOutput, RusotoError<SearchProductsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.SearchProducts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchProductsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<SearchProductsOutput, _>()
    }

    /// <p>Gets information about the products for the specified portfolio or all products.</p>
    async fn search_products_as_admin(
        &self,
        input: SearchProductsAsAdminInput,
    ) -> Result<SearchProductsAsAdminOutput, RusotoError<SearchProductsAsAdminError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.SearchProductsAsAdmin",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchProductsAsAdminError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<SearchProductsAsAdminOutput, _>()
    }

    /// <p>Gets information about the provisioned products that meet the specified criteria.</p>
    async fn search_provisioned_products(
        &self,
        input: SearchProvisionedProductsInput,
    ) -> Result<SearchProvisionedProductsOutput, RusotoError<SearchProvisionedProductsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.SearchProvisionedProducts",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchProvisionedProductsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SearchProvisionedProductsOutput, _>()
    }

    /// <p>Terminates the specified provisioned product.</p> <p>This operation does not delete any records associated with the provisioned product.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    async fn terminate_provisioned_product(
        &self,
        input: TerminateProvisionedProductInput,
    ) -> Result<TerminateProvisionedProductOutput, RusotoError<TerminateProvisionedProductError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.TerminateProvisionedProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TerminateProvisionedProductError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<TerminateProvisionedProductOutput, _>()
    }

    /// <p>Updates the specified constraint.</p>
    async fn update_constraint(
        &self,
        input: UpdateConstraintInput,
    ) -> Result<UpdateConstraintOutput, RusotoError<UpdateConstraintError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateConstraint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateConstraintError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateConstraintOutput, _>()
    }

    /// <p>Updates the specified portfolio.</p> <p>You cannot update a product that was shared with you.</p>
    async fn update_portfolio(
        &self,
        input: UpdatePortfolioInput,
    ) -> Result<UpdatePortfolioOutput, RusotoError<UpdatePortfolioError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdatePortfolio",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdatePortfolioError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdatePortfolioOutput, _>()
    }

    /// <p>Updates the specified portfolio share. You can use this API to enable or disable TagOptions sharing for an existing portfolio share. </p> <p>The portfolio share cannot be updated if the <code> CreatePortfolioShare</code> operation is <code>IN_PROGRESS</code>, as the share is not available to recipient entities. In this case, you must wait for the portfolio share to be COMPLETED.</p> <p>You must provide the <code>accountId</code> or organization node in the input, but not both.</p> <p>If the portfolio is shared to both an external account and an organization node, and both shares need to be updated, you must invoke <code>UpdatePortfolioShare</code> separately for each share type. </p> <p>This API cannot be used for removing the portfolio share. You must use <code>DeletePortfolioShare</code> API for that action. </p>
    async fn update_portfolio_share(
        &self,
        input: UpdatePortfolioShareInput,
    ) -> Result<UpdatePortfolioShareOutput, RusotoError<UpdatePortfolioShareError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdatePortfolioShare",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdatePortfolioShareError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdatePortfolioShareOutput, _>()
    }

    /// <p>Updates the specified product.</p>
    async fn update_product(
        &self,
        input: UpdateProductInput,
    ) -> Result<UpdateProductOutput, RusotoError<UpdateProductError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWS242ServiceCatalogService.UpdateProduct");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateProductError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateProductOutput, _>()
    }

    /// <p>Requests updates to the configuration of the specified provisioned product.</p> <p>If there are tags associated with the object, they cannot be updated or added. Depending on the specific updates requested, this operation can update with no interruption, with some interruption, or replace the provisioned product entirely.</p> <p>You can check the status of this request using <a>DescribeRecord</a>.</p>
    async fn update_provisioned_product(
        &self,
        input: UpdateProvisionedProductInput,
    ) -> Result<UpdateProvisionedProductOutput, RusotoError<UpdateProvisionedProductError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateProvisionedProduct",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateProvisionedProductError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateProvisionedProductOutput, _>()
    }

    /// <p>Requests updates to the properties of the specified provisioned product.</p>
    async fn update_provisioned_product_properties(
        &self,
        input: UpdateProvisionedProductPropertiesInput,
    ) -> Result<
        UpdateProvisionedProductPropertiesOutput,
        RusotoError<UpdateProvisionedProductPropertiesError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateProvisionedProductProperties",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                UpdateProvisionedProductPropertiesError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateProvisionedProductPropertiesOutput, _>()
    }

    /// <p>Updates the specified provisioning artifact (also known as a version) for the specified product.</p> <p>You cannot update a provisioning artifact for a product that was shared with you.</p>
    async fn update_provisioning_artifact(
        &self,
        input: UpdateProvisioningArtifactInput,
    ) -> Result<UpdateProvisioningArtifactOutput, RusotoError<UpdateProvisioningArtifactError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateProvisioningArtifact",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateProvisioningArtifactError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateProvisioningArtifactOutput, _>()
    }

    /// <p>Updates a self-service action.</p>
    async fn update_service_action(
        &self,
        input: UpdateServiceActionInput,
    ) -> Result<UpdateServiceActionOutput, RusotoError<UpdateServiceActionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateServiceAction",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateServiceActionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateServiceActionOutput, _>()
    }

    /// <p>Updates the specified TagOption.</p>
    async fn update_tag_option(
        &self,
        input: UpdateTagOptionInput,
    ) -> Result<UpdateTagOptionOutput, RusotoError<UpdateTagOptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWS242ServiceCatalogService.UpdateTagOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateTagOptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateTagOptionOutput, _>()
    }
}
