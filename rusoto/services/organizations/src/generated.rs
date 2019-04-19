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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AcceptHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want to accept.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AcceptHandshakeResponse {
    /// <p>A structure that contains details about the accepted handshake.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

/// <p>Contains information about an AWS account that is a member of an organization.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Account {
    /// <p>The Amazon Resource Name (ARN) of the account.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The email address associated with the AWS account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for this parameter is a string of characters that represents a standard Internet email address.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The unique identifier (ID) of the account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The method by which the account joined the organization.</p>
    #[serde(rename = "JoinedMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_method: Option<String>,
    /// <p>The date the account became a part of the organization.</p>
    #[serde(rename = "JoinedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_timestamp: Option<f64>,
    /// <p>The friendly name of the account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the account in the organization.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachPolicyRequest {
    /// <p>The unique identifier (ID) of the policy that you want to attach to the target. You can get the ID for the policy by calling the <a>ListPolicies</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// <p><p>The unique identifier (ID) of the root, OU, or account that you want to attach the policy to. You can get the ID by calling the <a>ListRoots</a>, <a>ListOrganizationalUnitsForParent</a>, or <a>ListAccounts</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    pub target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want to cancel. You can get the ID from the <a>ListHandshakesForOrganization</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CancelHandshakeResponse {
    /// <p>A structure that contains details about the handshake that you canceled.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

/// <p>Contains a list of child entities, either OUs or accounts.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Child {
    /// <p><p>The unique identifier (ID) of this child entity.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a child ID string requires one of the following:</p> <ul> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The type of this child entity.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAccountRequest {
    /// <p>The friendly name of the member account.</p>
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// <p>The email address of the owner to assign to the new member account. This email address must not already be associated with another AWS account. You must use a valid email address to complete account creation. You can't access the root user of the account or remove an account that was created with an invalid email address.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>If set to <code>ALLOW</code>, the new account enables IAM users to access account billing information <i>if</i> they have the required permissions. If set to <code>DENY</code>, only the root user of the new account can access account billing information. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> <p>If you don't specify this parameter, the value defaults to <code>ALLOW</code>, and IAM users and roles with the required permissions can access billing information for the new account.</p>
    #[serde(rename = "IamUserAccessToBilling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_access_to_billing: Option<String>,
    /// <p>(Optional)</p> <p>The name of an IAM role that AWS Organizations automatically preconfigures in the new member account. This role trusts the master account, allowing users in the master account to assume the role, as permitted by the master account administrator. The role has administrator permissions in the new member account.</p> <p>If you don't specify this parameter, the role name defaults to <code>OrganizationAccountAccessRole</code>.</p> <p>For more information about how to use this role to access the member account, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_access.html#orgs_manage_accounts_create-cross-account-role">Accessing and Administering the Member Accounts in Your Organization</a> in the <i>AWS Organizations User Guide</i>, and steps 2 and 3 in <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/tutorial_cross-account-with-roles.html">Tutorial: Delegate Access Across AWS Accounts Using IAM Roles</a> in the <i>IAM User Guide</i>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of characters that can consist of uppercase letters, lowercase letters, digits with no spaces, and any of the following characters: =,.@-</p>
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAccountResponse {
    /// <p>A structure that contains details about the request to create an account. This response structure might not be fully populated when you first receive it because account creation is an asynchronous process. You can pass the returned <code>CreateAccountStatus</code> ID as a parameter to <a>DescribeCreateAccountStatus</a> to get status about the progress of the request at later times. You can also check the AWS CloudTrail log for the <code>CreateAccountResult</code> event. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_monitoring.html">Monitoring the Activity in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "CreateAccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_status: Option<CreateAccountStatus>,
}

/// <p>Contains the status about a <a>CreateAccount</a> request to create an AWS account in an organization.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAccountStatus {
    /// <p>If the account was created successfully, the unique identifier (ID) of the new account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The account name given to the account when it was created.</p>
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// <p>The date and time that the account was created and the request completed.</p>
    #[serde(rename = "CompletedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_timestamp: Option<f64>,
    /// <p><p>If the request failed, a description of the reason for the failure.</p> <ul> <li> <p>ACCOUNT<em>LIMIT</em>EXCEEDED: The account could not be created because you have reached the limit on the number of accounts in your organization.</p> </li> <li> <p>EMAIL<em>ALREADY</em>EXISTS: The account could not be created because another AWS account with that email address already exists.</p> </li> <li> <p>INVALID<em>ADDRESS: The account could not be created because the address you provided is not valid.</p> </li> <li> <p>INVALID</em>EMAIL: The account could not be created because the email address you provided is not valid.</p> </li> <li> <p>INTERNAL_FAILURE: The account could not be created because of an internal failure. Try again later. If the problem persists, contact Customer Support.</p> </li> </ul></p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The unique identifier (ID) that references this request. You get this value from the response of the initial <a>CreateAccount</a> request to create the account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an create account request ID string requires "car-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The date and time that the request was made for the account creation.</p>
    #[serde(rename = "RequestedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_timestamp: Option<f64>,
    /// <p>The status of the request.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateOrganizationRequest {
    /// <p><p>Specifies the feature set supported by the new organization. Each feature set supports different levels of functionality.</p> <ul> <li> <p> <i>CONSOLIDATED<em>BILLING</i>: All member accounts have their bills consolidated to and paid by the master account. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>getting-started<em>concepts.html#feature-set-cb-only&quot;&gt;Consolidated billing</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p> <i>ALL</i>: In addition to all the features supported by the consolidated billing feature set, the master account can also apply any type of policy to any member account in the organization. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>getting-started_concepts.html#feature-set-all&quot;&gt;All features</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul></p>
    #[serde(rename = "FeatureSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateOrganizationResponse {
    /// <p>A structure that contains details about the newly created organization.</p>
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateOrganizationalUnitRequest {
    /// <p>The friendly name to assign to the new OU.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The unique identifier (ID) of the parent root or OU in which you want to create the new OU.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateOrganizationalUnitResponse {
    /// <p>A structure that contains details about the newly created OU.</p>
    #[serde(rename = "OrganizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePolicyRequest {
    /// <p>The policy content to add to the new policy. For example, if you create a <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">service control policy</a> (SCP), this string must be JSON text that specifies the permissions that admins in attached accounts can delegate to their users, groups, and roles. For more information about the SCP syntax, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>An optional description to assign to the policy.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The friendly name to assign to the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The type of policy to create.</p> <note> <p>In the current release, the only type of policy that you can create is a service control policy (SCP).</p> </note></p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreatePolicyResponse {
    /// <p>A structure that contains details about the newly created policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeclineHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want to decline. You can get the ID from the <a>ListHandshakesForAccount</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeclineHandshakeResponse {
    /// <p>A structure that contains details about the declined handshake. The state is updated to show the value <code>DECLINED</code>.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteOrganizationalUnitRequest {
    /// <p>The unique identifier (ID) of the organizational unit that you want to delete. You can get the ID from the <a>ListOrganizationalUnitsForParent</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second "-" dash and from 8 to 32 additional lower-case letters or digits.</p>
    #[serde(rename = "OrganizationalUnitId")]
    pub organizational_unit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePolicyRequest {
    /// <p>The unique identifier (ID) of the policy that you want to delete. You can get the ID from the <a>ListPolicies</a> or <a>ListPoliciesForTarget</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAccountRequest {
    /// <p>The unique identifier (ID) of the AWS account that you want information about. You can get the ID from the <a>ListAccounts</a> or <a>ListAccountsForParent</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeAccountResponse {
    /// <p>A structure that contains information about the requested account.</p>
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCreateAccountStatusRequest {
    /// <p>Specifies the <code>operationId</code> that uniquely identifies the request. You can get the ID from the response to an earlier <a>CreateAccount</a> request, or from the <a>ListCreateAccountStatus</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an create account request ID string requires "car-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "CreateAccountRequestId")]
    pub create_account_request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeCreateAccountStatusResponse {
    /// <p>A structure that contains the current status of an account creation request.</p>
    #[serde(rename = "CreateAccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_status: Option<CreateAccountStatus>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want information about. You can get the ID from the original call to <a>InviteAccountToOrganization</a>, or from a call to <a>ListHandshakesForAccount</a> or <a>ListHandshakesForOrganization</a>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeHandshakeResponse {
    /// <p>A structure that contains information about the specified handshake.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeOrganizationResponse {
    /// <p>A structure that contains information about the organization.</p>
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeOrganizationalUnitRequest {
    /// <p>The unique identifier (ID) of the organizational unit that you want details about. You can get the ID from the <a>ListOrganizationalUnitsForParent</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second "-" dash and from 8 to 32 additional lower-case letters or digits.</p>
    #[serde(rename = "OrganizationalUnitId")]
    pub organizational_unit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeOrganizationalUnitResponse {
    /// <p>A structure that contains details about the specified OU.</p>
    #[serde(rename = "OrganizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePolicyRequest {
    /// <p>The unique identifier (ID) of the policy that you want details about. You can get the ID from the <a>ListPolicies</a> or <a>ListPoliciesForTarget</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribePolicyResponse {
    /// <p>A structure that contains details about the specified policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachPolicyRequest {
    /// <p>The unique identifier (ID) of the policy you want to detach. You can get the ID from the <a>ListPolicies</a> or <a>ListPoliciesForTarget</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// <p><p>The unique identifier (ID) of the root, OU, or account from which you want to detach the policy. You can get the ID from the <a>ListRoots</a>, <a>ListOrganizationalUnitsForParent</a>, or <a>ListAccounts</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    pub target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableAWSServiceAccessRequest {
    /// <p>The service principal name of the AWS service for which you want to disable integration with your organization. This is typically in the form of a URL, such as <code> <i>service-abbreviation</i>.amazonaws.com</code>.</p>
    #[serde(rename = "ServicePrincipal")]
    pub service_principal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisablePolicyTypeRequest {
    /// <p>The policy type that you want to disable in this root.</p>
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
    /// <p>The unique identifier (ID) of the root in which you want to disable a policy type. You can get the ID from the <a>ListRoots</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a root ID string requires "r-" followed by from 4 to 32 lower-case letters or digits.</p>
    #[serde(rename = "RootId")]
    pub root_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisablePolicyTypeResponse {
    /// <p>A structure that shows the root with the updated list of enabled policy types.</p>
    #[serde(rename = "Root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableAWSServiceAccessRequest {
    /// <p>The service principal name of the AWS service for which you want to enable integration with your organization. This is typically in the form of a URL, such as <code> <i>service-abbreviation</i>.amazonaws.com</code>.</p>
    #[serde(rename = "ServicePrincipal")]
    pub service_principal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableAllFeaturesRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EnableAllFeaturesResponse {
    /// <p>A structure that contains details about the handshake created to support this request to enable all features in the organization.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnablePolicyTypeRequest {
    /// <p>The policy type that you want to enable.</p>
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
    /// <p>The unique identifier (ID) of the root in which you want to enable a policy type. You can get the ID from the <a>ListRoots</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a root ID string requires "r-" followed by from 4 to 32 lower-case letters or digits.</p>
    #[serde(rename = "RootId")]
    pub root_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EnablePolicyTypeResponse {
    /// <p>A structure that shows the root with the updated list of enabled policy types.</p>
    #[serde(rename = "Root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
}

/// <p>A structure that contains details of a service principal that is enabled to integrate with AWS Organizations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EnabledServicePrincipal {
    /// <p>The date that the service principal was enabled for integration with AWS Organizations.</p>
    #[serde(rename = "DateEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_enabled: Option<f64>,
    /// <p>The name of the service principal. This is typically in the form of a URL, such as: <code> <i>servicename</i>.amazonaws.com</code>.</p>
    #[serde(rename = "ServicePrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

/// <p>Contains information that must be exchanged to securely establish a relationship between two accounts (an <i>originator</i> and a <i>recipient</i>). For example, when a master account (the originator) invites another account (the recipient) to join its organization, the two accounts exchange information as a series of handshake requests and responses.</p> <p> <b>Note:</b> Handshakes that are CANCELED, ACCEPTED, or DECLINED show up in lists for only 30 days after entering that state After that they are deleted.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Handshake {
    /// <p><p>The type of handshake, indicating what action occurs when the recipient accepts the handshake. The following handshake types are supported:</p> <ul> <li> <p> <b>INVITE</b>: This type of handshake represents a request to join an organization. It is always sent from the master account to only non-member accounts.</p> </li> <li> <p> <b>ENABLE<em>ALL</em>FEATURES</b>: This type of handshake represents a request to enable all features in an organization. It is always sent from the master account to only <i>invited</i> member accounts. Created accounts do not receive this because those accounts were created by the organization&#39;s master account and approval is inferred.</p> </li> <li> <p> <b>APPROVE<em>ALL</em>FEATURES</b>: This type of handshake is sent from the Organizations service when all member accounts have approved the <code>ENABLE<em>ALL</em>FEATURES</code> invitation. It is sent only to the master account and signals the master that it can finalize the process to enable all features.</p> </li> </ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of a handshake.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the handshake expires. If the recipient of the handshake request fails to respond before the specified date and time, the handshake becomes inactive and is no longer valid.</p>
    #[serde(rename = "ExpirationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_timestamp: Option<f64>,
    /// <p>The unique identifier (ID) of a handshake. The originating account creates the ID when it initiates the handshake.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Information about the two accounts that are participating in the handshake.</p>
    #[serde(rename = "Parties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parties: Option<Vec<HandshakeParty>>,
    /// <p>The date and time that the handshake request was made.</p>
    #[serde(rename = "RequestedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_timestamp: Option<f64>,
    /// <p>Additional information that is needed to process the handshake.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<HandshakeResource>>,
    /// <p><p>The current state of the handshake. Use the state to trace the flow of the handshake through the process from its creation to its acceptance. The meaning of each of the valid values is as follows:</p> <ul> <li> <p> <b>REQUESTED</b>: This handshake was sent to multiple recipients (applicable to only some handshake types) and not all recipients have responded yet. The request stays in this state until all recipients respond.</p> </li> <li> <p> <b>OPEN</b>: This handshake was sent to multiple recipients (applicable to only some policy types) and all recipients have responded, allowing the originator to complete the handshake action.</p> </li> <li> <p> <b>CANCELED</b>: This handshake is no longer active because it was canceled by the originating account.</p> </li> <li> <p> <b>ACCEPTED</b>: This handshake is complete because it has been accepted by the recipient.</p> </li> <li> <p> <b>DECLINED</b>: This handshake is no longer active because it was declined by the recipient account.</p> </li> <li> <p> <b>EXPIRED</b>: This handshake is no longer active because the originator did not receive a response of any kind from the recipient before the expiration time (15 days).</p> </li> </ul></p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Specifies the criteria that are used to select the handshakes for the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct HandshakeFilter {
    /// <p>Specifies the type of handshake action.</p> <p>If you specify <code>ActionType</code>, you cannot also specify <code>ParentHandshakeId</code>.</p>
    #[serde(rename = "ActionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// <p>Specifies the parent handshake. Only used for handshake types that are a child of another type.</p> <p>If you specify <code>ParentHandshakeId</code>, you cannot also specify <code>ActionType</code>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "ParentHandshakeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_handshake_id: Option<String>,
}

/// <p>Identifies a participant in a handshake.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandshakeParty {
    /// <p>The unique identifier (ID) for the party.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The type of party.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Contains additional data that is needed to process a handshake.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HandshakeResource {
    /// <p>When needed, contains an additional array of <code>HandshakeResource</code> objects.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<HandshakeResource>>,
    /// <p><p>The type of information being passed, specifying how the value is to be interpreted by the other party:</p> <ul> <li> <p> <code>ACCOUNT</code> - Specifies an AWS account ID number.</p> </li> <li> <p> <code>ORGANIZATION</code> - Specifies an organization ID number.</p> </li> <li> <p> <code>EMAIL</code> - Specifies the email address that is associated with the account that receives the handshake. </p> </li> <li> <p> <code>OWNER<em>EMAIL</code> - Specifies the email address associated with the master account. Included as information about an organization. </p> </li> <li> <p> <code>OWNER</em>NAME</code> - Specifies the name associated with the master account. Included as information about an organization. </p> </li> <li> <p> <code>NOTES</code> - Additional text provided by the handshake initiator and intended for the recipient to read.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The information that is passed to the other party in the handshake. The format of the value string must match the requirements of the specified type.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InviteAccountToOrganizationRequest {
    /// <p>Additional information that you want to include in the generated email to the recipient account owner.</p>
    #[serde(rename = "Notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// <p>The identifier (ID) of the AWS account that you want to invite to join your organization. This is a JSON object that contains the following elements: </p> <p> <code>{ "Type": "ACCOUNT", "Id": "&lt;<i> <b>account id number</b> </i>&gt;" }</code> </p> <p>If you use the AWS CLI, you can submit this as a single string, similar to the following example:</p> <p> <code>--target Id=123456789012,Type=ACCOUNT</code> </p> <p>If you specify <code>"Type": "ACCOUNT"</code>, then you must provide the AWS account ID number as the <code>Id</code>. If you specify <code>"Type": "EMAIL"</code>, then you must specify the email address that is associated with the account.</p> <p> <code>--target Id=diego@example.com,Type=EMAIL</code> </p>
    #[serde(rename = "Target")]
    pub target: HandshakeParty,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InviteAccountToOrganizationResponse {
    /// <p>A structure that contains details about the handshake that is created to support this invitation request.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAWSServiceAccessForOrganizationRequest {
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAWSServiceAccessForOrganizationResponse {
    /// <p>A list of the service principals for the services that are enabled to integrate with your organization. Each principal is a structure that includes the name and the date that it was enabled for integration with AWS Organizations.</p>
    #[serde(rename = "EnabledServicePrincipals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_service_principals: Option<Vec<EnabledServicePrincipal>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAccountsForParentRequest {
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique identifier (ID) for the parent root or organization unit (OU) whose accounts you want to list.</p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAccountsForParentResponse {
    /// <p>A list of the accounts in the specified root or OU.</p>
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAccountsRequest {
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAccountsResponse {
    /// <p>A list of objects in the organization.</p>
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListChildrenRequest {
    /// <p>Filters the output to include only the specified child type.</p>
    #[serde(rename = "ChildType")]
    pub child_type: String,
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The unique identifier (ID) for the parent root or OU whose children you want to list.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListChildrenResponse {
    /// <p>The list of children of the specified parent container.</p>
    #[serde(rename = "Children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Child>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCreateAccountStatusRequest {
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of one or more states that you want included in the response. If this parameter is not present, then all requests are included in the response.</p>
    #[serde(rename = "States")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListCreateAccountStatusResponse {
    /// <p>A list of objects with details about the requests. Certain elements, such as the accountId number, are present in the output only after the account has been successfully created.</p>
    #[serde(rename = "CreateAccountStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_statuses: Option<Vec<CreateAccountStatus>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListHandshakesForAccountRequest {
    /// <p>Filters the handshakes that you want included in the response. The default is all types. Use the <code>ActionType</code> element to limit the output to only a specified type, such as <code>INVITE</code>, <code>ENABLE-FULL-CONTROL</code>, or <code>APPROVE-FULL-CONTROL</code>. Alternatively, for the <code>ENABLE-FULL-CONTROL</code> handshake that generates a separate child handshake for each member account, you can specify <code>ParentHandshakeId</code> to see only the handshakes that were generated by that parent request.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HandshakeFilter>,
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListHandshakesForAccountResponse {
    /// <p>A list of <a>Handshake</a> objects with details about each of the handshakes that is associated with the specified account.</p>
    #[serde(rename = "Handshakes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshakes: Option<Vec<Handshake>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListHandshakesForOrganizationRequest {
    /// <p>A filter of the handshakes that you want included in the response. The default is all types. Use the <code>ActionType</code> element to limit the output to only a specified type, such as <code>INVITE</code>, <code>ENABLE-ALL-FEATURES</code>, or <code>APPROVE-ALL-FEATURES</code>. Alternatively, for the <code>ENABLE-ALL-FEATURES</code> handshake that generates a separate child handshake for each member account, you can specify the <code>ParentHandshakeId</code> to see only the handshakes that were generated by that parent request.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HandshakeFilter>,
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListHandshakesForOrganizationResponse {
    /// <p>A list of <a>Handshake</a> objects with details about each of the handshakes that are associated with an organization.</p>
    #[serde(rename = "Handshakes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshakes: Option<Vec<Handshake>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOrganizationalUnitsForParentRequest {
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The unique identifier (ID) of the root or OU whose child OUs you want to list.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListOrganizationalUnitsForParentResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the OUs in the specified root or parent OU.</p>
    #[serde(rename = "OrganizationalUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_units: Option<Vec<OrganizationalUnit>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListParentsRequest {
    /// <p><p>The unique identifier (ID) of the OU or account whose parent containers you want to list. Do not specify a root.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a child ID string requires one of the following:</p> <ul> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ChildId")]
    pub child_id: String,
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListParentsResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of parents for the specified child account or OU.</p>
    #[serde(rename = "Parents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<Parent>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPoliciesForTargetRequest {
    /// <p>The type of policy that you want to include in the returned list.</p>
    #[serde(rename = "Filter")]
    pub filter: String,
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The unique identifier (ID) of the root, organizational unit, or account whose policies you want to list.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    pub target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPoliciesForTargetResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of policies that match the criteria in the request.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<PolicySummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPoliciesRequest {
    /// <p>Specifies the type of policy that you want to include in the response.</p>
    #[serde(rename = "Filter")]
    pub filter: String,
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPoliciesResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of policies that match the filter criteria in the request. The output list does not include the policy contents. To see the content for a policy, see <a>DescribePolicy</a>.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<PolicySummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRootsRequest {
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListRootsResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of roots that are defined in an organization.</p>
    #[serde(rename = "Roots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<Vec<Root>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTargetsForPolicyRequest {
    /// <p>(Optional) Use this to limit the number of results you want included per page in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique identifier (ID) of the policy for which you want to know its attachments.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTargetsForPolicyResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of structures, each of which contains details about one of the entities to which the specified policy is attached.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<PolicyTargetSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MoveAccountRequest {
    /// <p>The unique identifier (ID) of the account that you want to move.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p><p>The unique identifier (ID) of the root or organizational unit that you want to move the account to.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "DestinationParentId")]
    pub destination_parent_id: String,
    /// <p><p>The unique identifier (ID) of the root or organizational unit that you want to move the account from.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "SourceParentId")]
    pub source_parent_id: String,
}

/// <p>Contains details about an organization. An organization is a collection of accounts that are centrally managed together using consolidated billing, organized hierarchically with organizational units (OUs), and controlled with policies .</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Organization {
    /// <p>The Amazon Resource Name (ARN) of an organization.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p><p>A list of policy types that are enabled for this organization. For example, if your organization has all features enabled, then service control policies (SCPs) are included in the list.</p> <note> <p>Even if a policy type is shown as available in the organization, you can separately enable and disable them at the root level by using <a>EnablePolicyType</a> and <a>DisablePolicyType</a>. Use <a>ListRoots</a> to see the status of a policy type in that root.</p> </note></p>
    #[serde(rename = "AvailablePolicyTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_policy_types: Option<Vec<PolicyTypeSummary>>,
    /// <p>Specifies the functionality that currently is available to the organization. If set to "ALL", then all features are enabled and policies can be applied to accounts in the organization. If set to "CONSOLIDATED_BILLING", then only consolidated billing functionality is available. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "FeatureSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
    /// <p>The unique identifier (ID) of an organization.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organization ID string requires "o-" followed by from 10 to 32 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the account that is designated as the master account for the organization.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "MasterAccountArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_arn: Option<String>,
    /// <p>The email address that is associated with the AWS account that is designated as the master account for the organization.</p>
    #[serde(rename = "MasterAccountEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_email: Option<String>,
    /// <p>The unique identifier (ID) of the master account of an organization.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "MasterAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_id: Option<String>,
}

/// <p>Contains details about an organizational unit (OU). An OU is a container of AWS accounts within a root of an organization. Policies that are attached to an OU apply to all accounts contained in that OU and in any child OUs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct OrganizationalUnit {
    /// <p>The Amazon Resource Name (ARN) of this OU.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The unique identifier (ID) associated with this OU.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second "-" dash and from 8 to 32 additional lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The friendly name of this OU.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains information about either a root or an organizational unit (OU) that can contain OUs or accounts in an organization.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Parent {
    /// <p><p>The unique identifier (ID) of the parent entity.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The type of the parent entity.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains rules to be applied to the affected accounts. Policies can be attached directly to accounts, or to roots and OUs to affect all accounts in those hierarchies.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Policy {
    /// <p>The text content of the policy.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>A structure that contains additional details about the policy.</p>
    #[serde(rename = "PolicySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_summary: Option<PolicySummary>,
}

/// <p>Contains information about a policy, but does not include the content. To see the content of a policy, see <a>DescribePolicy</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PolicySummary {
    /// <p>The Amazon Resource Name (ARN) of the policy.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A boolean value that indicates whether the specified policy is an AWS managed policy. If true, then you can attach the policy to roots, OUs, or accounts, but you cannot edit it.</p>
    #[serde(rename = "AwsManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_managed: Option<bool>,
    /// <p>The description of the policy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier (ID) of the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The friendly name of the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of policy.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains information about a root, OU, or account that a policy is attached to.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PolicyTargetSummary {
    /// <p>The Amazon Resource Name (ARN) of the policy target.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The friendly name of the policy target.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The unique identifier (ID) of the policy target.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// <p>The type of the policy target.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains information about a policy type and its status in the associated root.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PolicyTypeSummary {
    /// <p>The status of the policy type as it relates to the associated root. To attach a policy of the specified type to a root or to an OU or account in that root, it must be available in the organization and enabled for that root.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The name of the policy type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveAccountFromOrganizationRequest {
    /// <p>The unique identifier (ID) of the member account that you want to remove from the organization.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

/// <p>Contains details about a root. A root is a top-level parent node in the hierarchy of an organization that can contain organizational units (OUs) and accounts. Every root contains every AWS account in the organization. Each root enables the accounts to be organized in a different way and to have different policy types enabled for use in that root.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Root {
    /// <p>The Amazon Resource Name (ARN) of the root.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The unique identifier (ID) for the root.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a root ID string requires "r-" followed by from 4 to 32 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The friendly name of the root.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The types of policies that are currently enabled for the root and therefore can be attached to the root or to its OUs or accounts.</p> <note> <p>Even if a policy type is shown as available in the organization, you can separately enable and disable them at the root level by using <a>EnablePolicyType</a> and <a>DisablePolicyType</a>. Use <a>DescribeOrganization</a> to see the availability of the policy types in that organization.</p> </note></p>
    #[serde(rename = "PolicyTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_types: Option<Vec<PolicyTypeSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateOrganizationalUnitRequest {
    /// <p>The new name that you want to assign to the OU.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier (ID) of the OU that you want to rename. You can get the ID from the <a>ListOrganizationalUnitsForParent</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second "-" dash and from 8 to 32 additional lower-case letters or digits.</p>
    #[serde(rename = "OrganizationalUnitId")]
    pub organizational_unit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateOrganizationalUnitResponse {
    /// <p>A structure that contains the details about the specified OU, including its new name.</p>
    #[serde(rename = "OrganizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePolicyRequest {
    /// <p>If provided, the new content for the policy. The text must be correctly formatted JSON that complies with the syntax for the policy's type. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>If provided, the new description for the policy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If provided, the new name for the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier (ID) of the policy that you want to update.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdatePolicyResponse {
    /// <p>A structure that contains details about the updated policy, showing the requested changes.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

/// Errors returned by AcceptHandshake
#[derive(Debug, PartialEq)]
pub enum AcceptHandshakeError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The operation that you attempted requires you to have the <code>iam:CreateServiceLinkedRole</code> so that AWS Organizations can create the required service-linked role. You don't have that permission.</p>
    AccessDeniedForDependency(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>The specified handshake is already in the requested state. For example, you can't accept a handshake that was already accepted.</p>
    HandshakeAlreadyInState(String),
    /// <p><p>The requested operation would violate the constraint identified in the reason code.</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. Note that deleted and closed accounts still count toward your limit.</p> <important> <p>If you get this exception immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>ALREADY<em>IN</em>AN<em>ORGANIZATION: The handshake request is invalid because the invited account is already a member of an organization.</p> </li> <li> <p>ORGANIZATION</em>ALREADY<em>HAS</em>ALL<em>FEATURES: The handshake request is invalid because the organization has already enabled all features.</p> </li> <li> <p>INVITE</em>DISABLED<em>DURING</em>ENABLE<em>ALL</em>FEATURES: You can&#39;t issue new invitations to join an organization while it&#39;s in the process of enabling all features. You can resume inviting accounts after you finalize the process when all accounts have agreed to the change.</p> </li> <li> <p>PAYMENT<em>INSTRUMENT</em>REQUIRED: You can&#39;t complete the operation with an account that doesn&#39;t have a payment instrument, such as a credit card, associated with it.</p> </li> <li> <p>ORGANIZATION<em>FROM</em>DIFFERENT<em>SELLER</em>OF<em>RECORD: The request failed because the account is from a different marketplace than the accounts in the organization. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be from the same marketplace.</p> </li> <li> <p>ORGANIZATION</em>MEMBERSHIP<em>CHANGE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to change the membership of an account too quickly after its previous change.</p> </li> </ul></p>
    HandshakeConstraintViolation(String),
    /// <p>We can't find a handshake with the <code>HandshakeId</code> that you specified.</p>
    HandshakeNotFound(String),
    /// <p>You can't perform the operation on the handshake in its current state. For example, you can't cancel a handshake that was already accepted or accept a handshake that was already declined.</p>
    InvalidHandshakeTransition(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl AcceptHandshakeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptHandshakeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(AcceptHandshakeError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(AcceptHandshakeError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "AccessDeniedForDependencyException" => {
                    return RusotoError::Service(AcceptHandshakeError::AccessDeniedForDependency(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(AcceptHandshakeError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "HandshakeAlreadyInStateException" => {
                    return RusotoError::Service(AcceptHandshakeError::HandshakeAlreadyInState(
                        String::from(error_message),
                    ))
                }
                "HandshakeConstraintViolationException" => {
                    return RusotoError::Service(
                        AcceptHandshakeError::HandshakeConstraintViolation(String::from(
                            error_message,
                        )),
                    )
                }
                "HandshakeNotFoundException" => {
                    return RusotoError::Service(AcceptHandshakeError::HandshakeNotFound(
                        String::from(error_message),
                    ))
                }
                "InvalidHandshakeTransitionException" => {
                    return RusotoError::Service(AcceptHandshakeError::InvalidHandshakeTransition(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AcceptHandshakeError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(AcceptHandshakeError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AcceptHandshakeError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AcceptHandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcceptHandshakeError {
    fn description(&self) -> &str {
        match *self {
            AcceptHandshakeError::AWSOrganizationsNotInUse(ref cause) => cause,
            AcceptHandshakeError::AccessDenied(ref cause) => cause,
            AcceptHandshakeError::AccessDeniedForDependency(ref cause) => cause,
            AcceptHandshakeError::ConcurrentModification(ref cause) => cause,
            AcceptHandshakeError::HandshakeAlreadyInState(ref cause) => cause,
            AcceptHandshakeError::HandshakeConstraintViolation(ref cause) => cause,
            AcceptHandshakeError::HandshakeNotFound(ref cause) => cause,
            AcceptHandshakeError::InvalidHandshakeTransition(ref cause) => cause,
            AcceptHandshakeError::InvalidInput(ref cause) => cause,
            AcceptHandshakeError::Service(ref cause) => cause,
            AcceptHandshakeError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachPolicy
#[derive(Debug, PartialEq)]
pub enum AttachPolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>The selected policy is already attached to the specified target.</p>
    DuplicatePolicyAttachment(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>The specified policy type isn't currently enabled in this root. You can't attach policies of the specified type to entities in a root until you enable that type in the root. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p>
    PolicyTypeNotEnabled(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, or account with the <code>TargetId</code> that you specified.</p>
    TargetNotFound(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl AttachPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachPolicyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(AttachPolicyError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachPolicyError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(AttachPolicyError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(AttachPolicyError::ConstraintViolation(
                        String::from(error_message),
                    ))
                }
                "DuplicatePolicyAttachmentException" => {
                    return RusotoError::Service(AttachPolicyError::DuplicatePolicyAttachment(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AttachPolicyError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(AttachPolicyError::PolicyNotFound(String::from(
                        error_message,
                    )))
                }
                "PolicyTypeNotEnabledException" => {
                    return RusotoError::Service(AttachPolicyError::PolicyTypeNotEnabled(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(AttachPolicyError::Service(String::from(
                        error_message,
                    )))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(AttachPolicyError::TargetNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AttachPolicyError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            AttachPolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            AttachPolicyError::AccessDenied(ref cause) => cause,
            AttachPolicyError::ConcurrentModification(ref cause) => cause,
            AttachPolicyError::ConstraintViolation(ref cause) => cause,
            AttachPolicyError::DuplicatePolicyAttachment(ref cause) => cause,
            AttachPolicyError::InvalidInput(ref cause) => cause,
            AttachPolicyError::PolicyNotFound(ref cause) => cause,
            AttachPolicyError::PolicyTypeNotEnabled(ref cause) => cause,
            AttachPolicyError::Service(ref cause) => cause,
            AttachPolicyError::TargetNotFound(ref cause) => cause,
            AttachPolicyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelHandshake
#[derive(Debug, PartialEq)]
pub enum CancelHandshakeError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>The specified handshake is already in the requested state. For example, you can't accept a handshake that was already accepted.</p>
    HandshakeAlreadyInState(String),
    /// <p>We can't find a handshake with the <code>HandshakeId</code> that you specified.</p>
    HandshakeNotFound(String),
    /// <p>You can't perform the operation on the handshake in its current state. For example, you can't cancel a handshake that was already accepted or accept a handshake that was already declined.</p>
    InvalidHandshakeTransition(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl CancelHandshakeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelHandshakeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return RusotoError::Service(CancelHandshakeError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CancelHandshakeError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "HandshakeAlreadyInStateException" => {
                    return RusotoError::Service(CancelHandshakeError::HandshakeAlreadyInState(
                        String::from(error_message),
                    ))
                }
                "HandshakeNotFoundException" => {
                    return RusotoError::Service(CancelHandshakeError::HandshakeNotFound(
                        String::from(error_message),
                    ))
                }
                "InvalidHandshakeTransitionException" => {
                    return RusotoError::Service(CancelHandshakeError::InvalidHandshakeTransition(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CancelHandshakeError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(CancelHandshakeError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CancelHandshakeError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CancelHandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelHandshakeError {
    fn description(&self) -> &str {
        match *self {
            CancelHandshakeError::AccessDenied(ref cause) => cause,
            CancelHandshakeError::ConcurrentModification(ref cause) => cause,
            CancelHandshakeError::HandshakeAlreadyInState(ref cause) => cause,
            CancelHandshakeError::HandshakeNotFound(ref cause) => cause,
            CancelHandshakeError::InvalidHandshakeTransition(ref cause) => cause,
            CancelHandshakeError::InvalidInput(ref cause) => cause,
            CancelHandshakeError::Service(ref cause) => cause,
            CancelHandshakeError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAccount
#[derive(Debug, PartialEq)]
pub enum CreateAccountError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>AWS Organizations couldn't perform the operation because your organization hasn't finished initializing. This can take up to an hour. Try again later. If after one hour you continue to receive this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p>
    FinalizingOrganization(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl CreateAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAccountError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(CreateAccountError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateAccountError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateAccountError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(CreateAccountError::ConstraintViolation(
                        String::from(error_message),
                    ))
                }
                "FinalizingOrganizationException" => {
                    return RusotoError::Service(CreateAccountError::FinalizingOrganization(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateAccountError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateAccountError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateAccountError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAccountError {
    fn description(&self) -> &str {
        match *self {
            CreateAccountError::AWSOrganizationsNotInUse(ref cause) => cause,
            CreateAccountError::AccessDenied(ref cause) => cause,
            CreateAccountError::ConcurrentModification(ref cause) => cause,
            CreateAccountError::ConstraintViolation(ref cause) => cause,
            CreateAccountError::FinalizingOrganization(ref cause) => cause,
            CreateAccountError::InvalidInput(ref cause) => cause,
            CreateAccountError::Service(ref cause) => cause,
            CreateAccountError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateOrganization
#[derive(Debug, PartialEq)]
pub enum CreateOrganizationError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The operation that you attempted requires you to have the <code>iam:CreateServiceLinkedRole</code> so that AWS Organizations can create the required service-linked role. You don't have that permission.</p>
    AccessDeniedForDependency(String),
    /// <p>This account is already a member of an organization. An account can belong to only one organization at a time.</p>
    AlreadyInOrganization(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl CreateOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOrganizationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateOrganizationError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedForDependencyException" => {
                    return RusotoError::Service(
                        CreateOrganizationError::AccessDeniedForDependency(String::from(
                            error_message,
                        )),
                    )
                }
                "AlreadyInOrganizationException" => {
                    return RusotoError::Service(CreateOrganizationError::AlreadyInOrganization(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateOrganizationError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(CreateOrganizationError::ConstraintViolation(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateOrganizationError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateOrganizationError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateOrganizationError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateOrganizationError {
    fn description(&self) -> &str {
        match *self {
            CreateOrganizationError::AccessDenied(ref cause) => cause,
            CreateOrganizationError::AccessDeniedForDependency(ref cause) => cause,
            CreateOrganizationError::AlreadyInOrganization(ref cause) => cause,
            CreateOrganizationError::ConcurrentModification(ref cause) => cause,
            CreateOrganizationError::ConstraintViolation(ref cause) => cause,
            CreateOrganizationError::InvalidInput(ref cause) => cause,
            CreateOrganizationError::Service(ref cause) => cause,
            CreateOrganizationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum CreateOrganizationalUnitError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>An OU with the same name already exists.</p>
    DuplicateOrganizationalUnit(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or OU with the <code>ParentId</code> that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl CreateOrganizationalUnitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOrganizationalUnitError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        CreateOrganizationalUnitError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateOrganizationalUnitError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreateOrganizationalUnitError::ConcurrentModification(String::from(
                            error_message,
                        )),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(
                        CreateOrganizationalUnitError::ConstraintViolation(String::from(
                            error_message,
                        )),
                    )
                }
                "DuplicateOrganizationalUnitException" => {
                    return RusotoError::Service(
                        CreateOrganizationalUnitError::DuplicateOrganizationalUnit(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateOrganizationalUnitError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ParentNotFoundException" => {
                    return RusotoError::Service(CreateOrganizationalUnitError::ParentNotFound(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateOrganizationalUnitError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateOrganizationalUnitError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateOrganizationalUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateOrganizationalUnitError {
    fn description(&self) -> &str {
        match *self {
            CreateOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => cause,
            CreateOrganizationalUnitError::AccessDenied(ref cause) => cause,
            CreateOrganizationalUnitError::ConcurrentModification(ref cause) => cause,
            CreateOrganizationalUnitError::ConstraintViolation(ref cause) => cause,
            CreateOrganizationalUnitError::DuplicateOrganizationalUnit(ref cause) => cause,
            CreateOrganizationalUnitError::InvalidInput(ref cause) => cause,
            CreateOrganizationalUnitError::ParentNotFound(ref cause) => cause,
            CreateOrganizationalUnitError::Service(ref cause) => cause,
            CreateOrganizationalUnitError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePolicy
#[derive(Debug, PartialEq)]
pub enum CreatePolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>A policy with the same name already exists.</p>
    DuplicatePolicy(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The provided policy document doesn't meet the requirements of the specified policy type. For example, the syntax might be incorrect. For details about service control policy syntax, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>AWS Organizations User Guide</i>.</p>
    MalformedPolicyDocument(String),
    /// <p>You can't use the specified policy type with the feature set currently enabled for this organization. For example, you can enable SCPs only after you enable all features in the organization. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html#enable_policies_on_root">Enabling and Disabling a Policy Type on a Root</a> in the <i>AWS Organizations User Guide</i>.</p>
    PolicyTypeNotAvailableForOrganization(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl CreatePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePolicyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(CreatePolicyError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(CreatePolicyError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreatePolicyError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(CreatePolicyError::ConstraintViolation(
                        String::from(error_message),
                    ))
                }
                "DuplicatePolicyException" => {
                    return RusotoError::Service(CreatePolicyError::DuplicatePolicy(String::from(
                        error_message,
                    )))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreatePolicyError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "MalformedPolicyDocumentException" => {
                    return RusotoError::Service(CreatePolicyError::MalformedPolicyDocument(
                        String::from(error_message),
                    ))
                }
                "PolicyTypeNotAvailableForOrganizationException" => {
                    return RusotoError::Service(
                        CreatePolicyError::PolicyTypeNotAvailableForOrganization(String::from(
                            error_message,
                        )),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreatePolicyError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreatePolicyError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreatePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePolicyError {
    fn description(&self) -> &str {
        match *self {
            CreatePolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            CreatePolicyError::AccessDenied(ref cause) => cause,
            CreatePolicyError::ConcurrentModification(ref cause) => cause,
            CreatePolicyError::ConstraintViolation(ref cause) => cause,
            CreatePolicyError::DuplicatePolicy(ref cause) => cause,
            CreatePolicyError::InvalidInput(ref cause) => cause,
            CreatePolicyError::MalformedPolicyDocument(ref cause) => cause,
            CreatePolicyError::PolicyTypeNotAvailableForOrganization(ref cause) => cause,
            CreatePolicyError::Service(ref cause) => cause,
            CreatePolicyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeclineHandshake
#[derive(Debug, PartialEq)]
pub enum DeclineHandshakeError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>The specified handshake is already in the requested state. For example, you can't accept a handshake that was already accepted.</p>
    HandshakeAlreadyInState(String),
    /// <p>We can't find a handshake with the <code>HandshakeId</code> that you specified.</p>
    HandshakeNotFound(String),
    /// <p>You can't perform the operation on the handshake in its current state. For example, you can't cancel a handshake that was already accepted or accept a handshake that was already declined.</p>
    InvalidHandshakeTransition(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DeclineHandshakeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeclineHandshakeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeclineHandshakeError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeclineHandshakeError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "HandshakeAlreadyInStateException" => {
                    return RusotoError::Service(DeclineHandshakeError::HandshakeAlreadyInState(
                        String::from(error_message),
                    ))
                }
                "HandshakeNotFoundException" => {
                    return RusotoError::Service(DeclineHandshakeError::HandshakeNotFound(
                        String::from(error_message),
                    ))
                }
                "InvalidHandshakeTransitionException" => {
                    return RusotoError::Service(DeclineHandshakeError::InvalidHandshakeTransition(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeclineHandshakeError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeclineHandshakeError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeclineHandshakeError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeclineHandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeclineHandshakeError {
    fn description(&self) -> &str {
        match *self {
            DeclineHandshakeError::AccessDenied(ref cause) => cause,
            DeclineHandshakeError::ConcurrentModification(ref cause) => cause,
            DeclineHandshakeError::HandshakeAlreadyInState(ref cause) => cause,
            DeclineHandshakeError::HandshakeNotFound(ref cause) => cause,
            DeclineHandshakeError::InvalidHandshakeTransition(ref cause) => cause,
            DeclineHandshakeError::InvalidInput(ref cause) => cause,
            DeclineHandshakeError::Service(ref cause) => cause,
            DeclineHandshakeError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteOrganization
#[derive(Debug, PartialEq)]
pub enum DeleteOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The organization isn't empty. To delete an organization, you must first remove all accounts except the master account, delete all OUs, and delete all policies.</p>
    OrganizationNotEmpty(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DeleteOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteOrganizationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DeleteOrganizationError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteOrganizationError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteOrganizationError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteOrganizationError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "OrganizationNotEmptyException" => {
                    return RusotoError::Service(DeleteOrganizationError::OrganizationNotEmpty(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteOrganizationError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteOrganizationError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteOrganizationError {
    fn description(&self) -> &str {
        match *self {
            DeleteOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            DeleteOrganizationError::AccessDenied(ref cause) => cause,
            DeleteOrganizationError::ConcurrentModification(ref cause) => cause,
            DeleteOrganizationError::InvalidInput(ref cause) => cause,
            DeleteOrganizationError::OrganizationNotEmpty(ref cause) => cause,
            DeleteOrganizationError::Service(ref cause) => cause,
            DeleteOrganizationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum DeleteOrganizationalUnitError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The specified OU is not empty. Move all accounts to another root or to other OUs, remove all child OUs, and try the operation again.</p>
    OrganizationalUnitNotEmpty(String),
    /// <p>We can't find an OU with the <code>OrganizationalUnitId</code> that you specified.</p>
    OrganizationalUnitNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DeleteOrganizationalUnitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteOrganizationalUnitError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DeleteOrganizationalUnitError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteOrganizationalUnitError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteOrganizationalUnitError::ConcurrentModification(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteOrganizationalUnitError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "OrganizationalUnitNotEmptyException" => {
                    return RusotoError::Service(
                        DeleteOrganizationalUnitError::OrganizationalUnitNotEmpty(String::from(
                            error_message,
                        )),
                    )
                }
                "OrganizationalUnitNotFoundException" => {
                    return RusotoError::Service(
                        DeleteOrganizationalUnitError::OrganizationalUnitNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteOrganizationalUnitError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteOrganizationalUnitError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteOrganizationalUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteOrganizationalUnitError {
    fn description(&self) -> &str {
        match *self {
            DeleteOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => cause,
            DeleteOrganizationalUnitError::AccessDenied(ref cause) => cause,
            DeleteOrganizationalUnitError::ConcurrentModification(ref cause) => cause,
            DeleteOrganizationalUnitError::InvalidInput(ref cause) => cause,
            DeleteOrganizationalUnitError::OrganizationalUnitNotEmpty(ref cause) => cause,
            DeleteOrganizationalUnitError::OrganizationalUnitNotFound(ref cause) => cause,
            DeleteOrganizationalUnitError::Service(ref cause) => cause,
            DeleteOrganizationalUnitError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePolicy
#[derive(Debug, PartialEq)]
pub enum DeletePolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The policy is attached to one or more entities. You must detach it from all roots, OUs, and accounts before performing this operation.</p>
    PolicyInUse(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DeletePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePolicyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DeletePolicyError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DeletePolicyError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeletePolicyError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeletePolicyError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "PolicyInUseException" => {
                    return RusotoError::Service(DeletePolicyError::PolicyInUse(String::from(
                        error_message,
                    )))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(DeletePolicyError::PolicyNotFound(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeletePolicyError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeletePolicyError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePolicyError {
    fn description(&self) -> &str {
        match *self {
            DeletePolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            DeletePolicyError::AccessDenied(ref cause) => cause,
            DeletePolicyError::ConcurrentModification(ref cause) => cause,
            DeletePolicyError::InvalidInput(ref cause) => cause,
            DeletePolicyError::PolicyInUse(ref cause) => cause,
            DeletePolicyError::PolicyNotFound(ref cause) => cause,
            DeletePolicyError::Service(ref cause) => cause,
            DeletePolicyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAccount
#[derive(Debug, PartialEq)]
pub enum DescribeAccountError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the <code>AccountId</code> that you specified, or the account whose credentials you used to make this request isn't a member of an organization.</p>
    AccountNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DescribeAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DescribeAccountError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeAccountError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "AccountNotFoundException" => {
                    return RusotoError::Service(DescribeAccountError::AccountNotFound(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeAccountError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeAccountError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeAccountError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAccountError {
    fn description(&self) -> &str {
        match *self {
            DescribeAccountError::AWSOrganizationsNotInUse(ref cause) => cause,
            DescribeAccountError::AccessDenied(ref cause) => cause,
            DescribeAccountError::AccountNotFound(ref cause) => cause,
            DescribeAccountError::InvalidInput(ref cause) => cause,
            DescribeAccountError::Service(ref cause) => cause,
            DescribeAccountError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCreateAccountStatus
#[derive(Debug, PartialEq)]
pub enum DescribeCreateAccountStatusError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>We can't find an create account request with the <code>CreateAccountRequestId</code> that you specified.</p>
    CreateAccountStatusNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DescribeCreateAccountStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeCreateAccountStatusError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DescribeCreateAccountStatusError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeCreateAccountStatusError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "CreateAccountStatusNotFoundException" => {
                    return RusotoError::Service(
                        DescribeCreateAccountStatusError::CreateAccountStatusNotFound(
                            String::from(error_message),
                        ),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeCreateAccountStatusError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeCreateAccountStatusError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeCreateAccountStatusError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeCreateAccountStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCreateAccountStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeCreateAccountStatusError::AWSOrganizationsNotInUse(ref cause) => cause,
            DescribeCreateAccountStatusError::AccessDenied(ref cause) => cause,
            DescribeCreateAccountStatusError::CreateAccountStatusNotFound(ref cause) => cause,
            DescribeCreateAccountStatusError::InvalidInput(ref cause) => cause,
            DescribeCreateAccountStatusError::Service(ref cause) => cause,
            DescribeCreateAccountStatusError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeHandshake
#[derive(Debug, PartialEq)]
pub enum DescribeHandshakeError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>We can't find a handshake with the <code>HandshakeId</code> that you specified.</p>
    HandshakeNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DescribeHandshakeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeHandshakeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeHandshakeError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DescribeHandshakeError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "HandshakeNotFoundException" => {
                    return RusotoError::Service(DescribeHandshakeError::HandshakeNotFound(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeHandshakeError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeHandshakeError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeHandshakeError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeHandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeHandshakeError {
    fn description(&self) -> &str {
        match *self {
            DescribeHandshakeError::AccessDenied(ref cause) => cause,
            DescribeHandshakeError::ConcurrentModification(ref cause) => cause,
            DescribeHandshakeError::HandshakeNotFound(ref cause) => cause,
            DescribeHandshakeError::InvalidInput(ref cause) => cause,
            DescribeHandshakeError::Service(ref cause) => cause,
            DescribeHandshakeError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeOrganization
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DescribeOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeOrganizationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DescribeOrganizationError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeOrganizationError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DescribeOrganizationError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeOrganizationError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeOrganizationError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOrganizationError {
    fn description(&self) -> &str {
        match *self {
            DescribeOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            DescribeOrganizationError::AccessDenied(ref cause) => cause,
            DescribeOrganizationError::ConcurrentModification(ref cause) => cause,
            DescribeOrganizationError::Service(ref cause) => cause,
            DescribeOrganizationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationalUnitError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find an OU with the <code>OrganizationalUnitId</code> that you specified.</p>
    OrganizationalUnitNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DescribeOrganizationalUnitError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOrganizationalUnitError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DescribeOrganizationalUnitError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeOrganizationalUnitError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeOrganizationalUnitError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "OrganizationalUnitNotFoundException" => {
                    return RusotoError::Service(
                        DescribeOrganizationalUnitError::OrganizationalUnitNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeOrganizationalUnitError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeOrganizationalUnitError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeOrganizationalUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOrganizationalUnitError {
    fn description(&self) -> &str {
        match *self {
            DescribeOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => cause,
            DescribeOrganizationalUnitError::AccessDenied(ref cause) => cause,
            DescribeOrganizationalUnitError::InvalidInput(ref cause) => cause,
            DescribeOrganizationalUnitError::OrganizationalUnitNotFound(ref cause) => cause,
            DescribeOrganizationalUnitError::Service(ref cause) => cause,
            DescribeOrganizationalUnitError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePolicy
#[derive(Debug, PartialEq)]
pub enum DescribePolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DescribePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePolicyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DescribePolicyError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribePolicyError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribePolicyError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(DescribePolicyError::PolicyNotFound(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribePolicyError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribePolicyError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePolicyError {
    fn description(&self) -> &str {
        match *self {
            DescribePolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            DescribePolicyError::AccessDenied(ref cause) => cause,
            DescribePolicyError::InvalidInput(ref cause) => cause,
            DescribePolicyError::PolicyNotFound(ref cause) => cause,
            DescribePolicyError::Service(ref cause) => cause,
            DescribePolicyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachPolicy
#[derive(Debug, PartialEq)]
pub enum DetachPolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The policy isn't attached to the specified target in the specified root.</p>
    PolicyNotAttached(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, or account with the <code>TargetId</code> that you specified.</p>
    TargetNotFound(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DetachPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachPolicyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DetachPolicyError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DetachPolicyError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DetachPolicyError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(DetachPolicyError::ConstraintViolation(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DetachPolicyError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "PolicyNotAttachedException" => {
                    return RusotoError::Service(DetachPolicyError::PolicyNotAttached(
                        String::from(error_message),
                    ))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(DetachPolicyError::PolicyNotFound(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(DetachPolicyError::Service(String::from(
                        error_message,
                    )))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(DetachPolicyError::TargetNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DetachPolicyError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DetachPolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            DetachPolicyError::AccessDenied(ref cause) => cause,
            DetachPolicyError::ConcurrentModification(ref cause) => cause,
            DetachPolicyError::ConstraintViolation(ref cause) => cause,
            DetachPolicyError::InvalidInput(ref cause) => cause,
            DetachPolicyError::PolicyNotAttached(ref cause) => cause,
            DetachPolicyError::PolicyNotFound(ref cause) => cause,
            DetachPolicyError::Service(ref cause) => cause,
            DetachPolicyError::TargetNotFound(ref cause) => cause,
            DetachPolicyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableAWSServiceAccess
#[derive(Debug, PartialEq)]
pub enum DisableAWSServiceAccessError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DisableAWSServiceAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableAWSServiceAccessError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DisableAWSServiceAccessError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DisableAWSServiceAccessError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DisableAWSServiceAccessError::ConcurrentModification(String::from(
                            error_message,
                        )),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(DisableAWSServiceAccessError::ConstraintViolation(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisableAWSServiceAccessError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DisableAWSServiceAccessError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DisableAWSServiceAccessError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisableAWSServiceAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableAWSServiceAccessError {
    fn description(&self) -> &str {
        match *self {
            DisableAWSServiceAccessError::AWSOrganizationsNotInUse(ref cause) => cause,
            DisableAWSServiceAccessError::AccessDenied(ref cause) => cause,
            DisableAWSServiceAccessError::ConcurrentModification(ref cause) => cause,
            DisableAWSServiceAccessError::ConstraintViolation(ref cause) => cause,
            DisableAWSServiceAccessError::InvalidInput(ref cause) => cause,
            DisableAWSServiceAccessError::Service(ref cause) => cause,
            DisableAWSServiceAccessError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DisablePolicyType
#[derive(Debug, PartialEq)]
pub enum DisablePolicyTypeError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The specified policy type isn't currently enabled in this root. You can't attach policies of the specified type to entities in a root until you enable that type in the root. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p>
    PolicyTypeNotEnabled(String),
    /// <p>We can't find a root with the <code>RootId</code> that you specified.</p>
    RootNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl DisablePolicyTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisablePolicyTypeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DisablePolicyTypeError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DisablePolicyTypeError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DisablePolicyTypeError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(DisablePolicyTypeError::ConstraintViolation(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisablePolicyTypeError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "PolicyTypeNotEnabledException" => {
                    return RusotoError::Service(DisablePolicyTypeError::PolicyTypeNotEnabled(
                        String::from(error_message),
                    ))
                }
                "RootNotFoundException" => {
                    return RusotoError::Service(DisablePolicyTypeError::RootNotFound(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DisablePolicyTypeError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DisablePolicyTypeError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisablePolicyTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisablePolicyTypeError {
    fn description(&self) -> &str {
        match *self {
            DisablePolicyTypeError::AWSOrganizationsNotInUse(ref cause) => cause,
            DisablePolicyTypeError::AccessDenied(ref cause) => cause,
            DisablePolicyTypeError::ConcurrentModification(ref cause) => cause,
            DisablePolicyTypeError::ConstraintViolation(ref cause) => cause,
            DisablePolicyTypeError::InvalidInput(ref cause) => cause,
            DisablePolicyTypeError::PolicyTypeNotEnabled(ref cause) => cause,
            DisablePolicyTypeError::RootNotFound(ref cause) => cause,
            DisablePolicyTypeError::Service(ref cause) => cause,
            DisablePolicyTypeError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableAWSServiceAccess
#[derive(Debug, PartialEq)]
pub enum EnableAWSServiceAccessError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl EnableAWSServiceAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableAWSServiceAccessError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        EnableAWSServiceAccessError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(EnableAWSServiceAccessError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        EnableAWSServiceAccessError::ConcurrentModification(String::from(
                            error_message,
                        )),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(EnableAWSServiceAccessError::ConstraintViolation(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(EnableAWSServiceAccessError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnableAWSServiceAccessError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(EnableAWSServiceAccessError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EnableAWSServiceAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableAWSServiceAccessError {
    fn description(&self) -> &str {
        match *self {
            EnableAWSServiceAccessError::AWSOrganizationsNotInUse(ref cause) => cause,
            EnableAWSServiceAccessError::AccessDenied(ref cause) => cause,
            EnableAWSServiceAccessError::ConcurrentModification(ref cause) => cause,
            EnableAWSServiceAccessError::ConstraintViolation(ref cause) => cause,
            EnableAWSServiceAccessError::InvalidInput(ref cause) => cause,
            EnableAWSServiceAccessError::Service(ref cause) => cause,
            EnableAWSServiceAccessError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableAllFeatures
#[derive(Debug, PartialEq)]
pub enum EnableAllFeaturesError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation would violate the constraint identified in the reason code.</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. Note that deleted and closed accounts still count toward your limit.</p> <important> <p>If you get this exception immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>ALREADY<em>IN</em>AN<em>ORGANIZATION: The handshake request is invalid because the invited account is already a member of an organization.</p> </li> <li> <p>ORGANIZATION</em>ALREADY<em>HAS</em>ALL<em>FEATURES: The handshake request is invalid because the organization has already enabled all features.</p> </li> <li> <p>INVITE</em>DISABLED<em>DURING</em>ENABLE<em>ALL</em>FEATURES: You can&#39;t issue new invitations to join an organization while it&#39;s in the process of enabling all features. You can resume inviting accounts after you finalize the process when all accounts have agreed to the change.</p> </li> <li> <p>PAYMENT<em>INSTRUMENT</em>REQUIRED: You can&#39;t complete the operation with an account that doesn&#39;t have a payment instrument, such as a credit card, associated with it.</p> </li> <li> <p>ORGANIZATION<em>FROM</em>DIFFERENT<em>SELLER</em>OF<em>RECORD: The request failed because the account is from a different marketplace than the accounts in the organization. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be from the same marketplace.</p> </li> <li> <p>ORGANIZATION</em>MEMBERSHIP<em>CHANGE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to change the membership of an account too quickly after its previous change.</p> </li> </ul></p>
    HandshakeConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl EnableAllFeaturesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableAllFeaturesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(EnableAllFeaturesError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(EnableAllFeaturesError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(EnableAllFeaturesError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "HandshakeConstraintViolationException" => {
                    return RusotoError::Service(
                        EnableAllFeaturesError::HandshakeConstraintViolation(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(EnableAllFeaturesError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnableAllFeaturesError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(EnableAllFeaturesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EnableAllFeaturesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableAllFeaturesError {
    fn description(&self) -> &str {
        match *self {
            EnableAllFeaturesError::AWSOrganizationsNotInUse(ref cause) => cause,
            EnableAllFeaturesError::AccessDenied(ref cause) => cause,
            EnableAllFeaturesError::ConcurrentModification(ref cause) => cause,
            EnableAllFeaturesError::HandshakeConstraintViolation(ref cause) => cause,
            EnableAllFeaturesError::InvalidInput(ref cause) => cause,
            EnableAllFeaturesError::Service(ref cause) => cause,
            EnableAllFeaturesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by EnablePolicyType
#[derive(Debug, PartialEq)]
pub enum EnablePolicyTypeError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The specified policy type is already enabled in the specified root.</p>
    PolicyTypeAlreadyEnabled(String),
    /// <p>You can't use the specified policy type with the feature set currently enabled for this organization. For example, you can enable SCPs only after you enable all features in the organization. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html#enable_policies_on_root">Enabling and Disabling a Policy Type on a Root</a> in the <i>AWS Organizations User Guide</i>.</p>
    PolicyTypeNotAvailableForOrganization(String),
    /// <p>We can't find a root with the <code>RootId</code> that you specified.</p>
    RootNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl EnablePolicyTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnablePolicyTypeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(EnablePolicyTypeError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(EnablePolicyTypeError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(EnablePolicyTypeError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(EnablePolicyTypeError::ConstraintViolation(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(EnablePolicyTypeError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "PolicyTypeAlreadyEnabledException" => {
                    return RusotoError::Service(EnablePolicyTypeError::PolicyTypeAlreadyEnabled(
                        String::from(error_message),
                    ))
                }
                "PolicyTypeNotAvailableForOrganizationException" => {
                    return RusotoError::Service(
                        EnablePolicyTypeError::PolicyTypeNotAvailableForOrganization(String::from(
                            error_message,
                        )),
                    )
                }
                "RootNotFoundException" => {
                    return RusotoError::Service(EnablePolicyTypeError::RootNotFound(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnablePolicyTypeError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(EnablePolicyTypeError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EnablePolicyTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnablePolicyTypeError {
    fn description(&self) -> &str {
        match *self {
            EnablePolicyTypeError::AWSOrganizationsNotInUse(ref cause) => cause,
            EnablePolicyTypeError::AccessDenied(ref cause) => cause,
            EnablePolicyTypeError::ConcurrentModification(ref cause) => cause,
            EnablePolicyTypeError::ConstraintViolation(ref cause) => cause,
            EnablePolicyTypeError::InvalidInput(ref cause) => cause,
            EnablePolicyTypeError::PolicyTypeAlreadyEnabled(ref cause) => cause,
            EnablePolicyTypeError::PolicyTypeNotAvailableForOrganization(ref cause) => cause,
            EnablePolicyTypeError::RootNotFound(ref cause) => cause,
            EnablePolicyTypeError::Service(ref cause) => cause,
            EnablePolicyTypeError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by InviteAccountToOrganization
#[derive(Debug, PartialEq)]
pub enum InviteAccountToOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>You can't invite an existing account to your organization until you verify that you own the email address associated with the master account. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_create.html#about-email-verification">Email Address Verification</a> in the <i>AWS Organizations User Guide</i>.</p>
    AccountOwnerNotVerified(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>A handshake with the same action and target already exists. For example, if you invited an account to join your organization, the invited account might already have a pending invitation from this organization. If you intend to resend an invitation to an account, ensure that existing handshakes that might be considered duplicates are canceled or declined.</p>
    DuplicateHandshake(String),
    /// <p>AWS Organizations couldn't perform the operation because your organization hasn't finished initializing. This can take up to an hour. Try again later. If after one hour you continue to receive this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p>
    FinalizingOrganization(String),
    /// <p><p>The requested operation would violate the constraint identified in the reason code.</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. Note that deleted and closed accounts still count toward your limit.</p> <important> <p>If you get this exception immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>ALREADY<em>IN</em>AN<em>ORGANIZATION: The handshake request is invalid because the invited account is already a member of an organization.</p> </li> <li> <p>ORGANIZATION</em>ALREADY<em>HAS</em>ALL<em>FEATURES: The handshake request is invalid because the organization has already enabled all features.</p> </li> <li> <p>INVITE</em>DISABLED<em>DURING</em>ENABLE<em>ALL</em>FEATURES: You can&#39;t issue new invitations to join an organization while it&#39;s in the process of enabling all features. You can resume inviting accounts after you finalize the process when all accounts have agreed to the change.</p> </li> <li> <p>PAYMENT<em>INSTRUMENT</em>REQUIRED: You can&#39;t complete the operation with an account that doesn&#39;t have a payment instrument, such as a credit card, associated with it.</p> </li> <li> <p>ORGANIZATION<em>FROM</em>DIFFERENT<em>SELLER</em>OF<em>RECORD: The request failed because the account is from a different marketplace than the accounts in the organization. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be from the same marketplace.</p> </li> <li> <p>ORGANIZATION</em>MEMBERSHIP<em>CHANGE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to change the membership of an account too quickly after its previous change.</p> </li> </ul></p>
    HandshakeConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl InviteAccountToOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<InviteAccountToOrganizationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(InviteAccountToOrganizationError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "AccountOwnerNotVerifiedException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::AccountOwnerNotVerified(String::from(
                            error_message,
                        )),
                    )
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::ConcurrentModification(String::from(
                            error_message,
                        )),
                    )
                }
                "DuplicateHandshakeException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::DuplicateHandshake(String::from(
                            error_message,
                        )),
                    )
                }
                "FinalizingOrganizationException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::FinalizingOrganization(String::from(
                            error_message,
                        )),
                    )
                }
                "HandshakeConstraintViolationException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::HandshakeConstraintViolation(
                            String::from(error_message),
                        ),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(InviteAccountToOrganizationError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(InviteAccountToOrganizationError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(InviteAccountToOrganizationError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for InviteAccountToOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InviteAccountToOrganizationError {
    fn description(&self) -> &str {
        match *self {
            InviteAccountToOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            InviteAccountToOrganizationError::AccessDenied(ref cause) => cause,
            InviteAccountToOrganizationError::AccountOwnerNotVerified(ref cause) => cause,
            InviteAccountToOrganizationError::ConcurrentModification(ref cause) => cause,
            InviteAccountToOrganizationError::DuplicateHandshake(ref cause) => cause,
            InviteAccountToOrganizationError::FinalizingOrganization(ref cause) => cause,
            InviteAccountToOrganizationError::HandshakeConstraintViolation(ref cause) => cause,
            InviteAccountToOrganizationError::InvalidInput(ref cause) => cause,
            InviteAccountToOrganizationError::Service(ref cause) => cause,
            InviteAccountToOrganizationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by LeaveOrganization
#[derive(Debug, PartialEq)]
pub enum LeaveOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the <code>AccountId</code> that you specified, or the account whose credentials you used to make this request isn't a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>You can't remove a master account from an organization. If you want the master account to become a member account in another organization, you must first delete the current organization of the master account.</p>
    MasterCannotLeaveOrganization(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl LeaveOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<LeaveOrganizationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(LeaveOrganizationError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(LeaveOrganizationError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "AccountNotFoundException" => {
                    return RusotoError::Service(LeaveOrganizationError::AccountNotFound(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(LeaveOrganizationError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(LeaveOrganizationError::ConstraintViolation(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(LeaveOrganizationError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "MasterCannotLeaveOrganizationException" => {
                    return RusotoError::Service(
                        LeaveOrganizationError::MasterCannotLeaveOrganization(String::from(
                            error_message,
                        )),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(LeaveOrganizationError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(LeaveOrganizationError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for LeaveOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for LeaveOrganizationError {
    fn description(&self) -> &str {
        match *self {
            LeaveOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            LeaveOrganizationError::AccessDenied(ref cause) => cause,
            LeaveOrganizationError::AccountNotFound(ref cause) => cause,
            LeaveOrganizationError::ConcurrentModification(ref cause) => cause,
            LeaveOrganizationError::ConstraintViolation(ref cause) => cause,
            LeaveOrganizationError::InvalidInput(ref cause) => cause,
            LeaveOrganizationError::MasterCannotLeaveOrganization(ref cause) => cause,
            LeaveOrganizationError::Service(ref cause) => cause,
            LeaveOrganizationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAWSServiceAccessForOrganization
#[derive(Debug, PartialEq)]
pub enum ListAWSServiceAccessForOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListAWSServiceAccessForOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAWSServiceAccessForOrganizationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListAWSServiceAccessForOrganizationError::AWSOrganizationsNotInUse(
                            String::from(error_message),
                        ),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListAWSServiceAccessForOrganizationError::AccessDenied(String::from(
                            error_message,
                        )),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(
                        ListAWSServiceAccessForOrganizationError::ConstraintViolation(
                            String::from(error_message),
                        ),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        ListAWSServiceAccessForOrganizationError::InvalidInput(String::from(
                            error_message,
                        )),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(ListAWSServiceAccessForOrganizationError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListAWSServiceAccessForOrganizationError::TooManyRequests(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAWSServiceAccessForOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAWSServiceAccessForOrganizationError {
    fn description(&self) -> &str {
        match *self {
            ListAWSServiceAccessForOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::AccessDenied(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::ConstraintViolation(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::InvalidInput(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::Service(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAccounts
#[derive(Debug, PartialEq)]
pub enum ListAccountsError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListAccountsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAccountsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(ListAccountsError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAccountsError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListAccountsError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListAccountsError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListAccountsError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAccountsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAccountsError {
    fn description(&self) -> &str {
        match *self {
            ListAccountsError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListAccountsError::AccessDenied(ref cause) => cause,
            ListAccountsError::InvalidInput(ref cause) => cause,
            ListAccountsError::Service(ref cause) => cause,
            ListAccountsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAccountsForParent
#[derive(Debug, PartialEq)]
pub enum ListAccountsForParentError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or OU with the <code>ParentId</code> that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListAccountsForParentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAccountsForParentError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListAccountsForParentError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAccountsForParentError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListAccountsForParentError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ParentNotFoundException" => {
                    return RusotoError::Service(ListAccountsForParentError::ParentNotFound(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListAccountsForParentError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListAccountsForParentError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAccountsForParentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAccountsForParentError {
    fn description(&self) -> &str {
        match *self {
            ListAccountsForParentError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListAccountsForParentError::AccessDenied(ref cause) => cause,
            ListAccountsForParentError::InvalidInput(ref cause) => cause,
            ListAccountsForParentError::ParentNotFound(ref cause) => cause,
            ListAccountsForParentError::Service(ref cause) => cause,
            ListAccountsForParentError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListChildren
#[derive(Debug, PartialEq)]
pub enum ListChildrenError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or OU with the <code>ParentId</code> that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListChildrenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListChildrenError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(ListChildrenError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListChildrenError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListChildrenError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "ParentNotFoundException" => {
                    return RusotoError::Service(ListChildrenError::ParentNotFound(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListChildrenError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListChildrenError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListChildrenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListChildrenError {
    fn description(&self) -> &str {
        match *self {
            ListChildrenError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListChildrenError::AccessDenied(ref cause) => cause,
            ListChildrenError::InvalidInput(ref cause) => cause,
            ListChildrenError::ParentNotFound(ref cause) => cause,
            ListChildrenError::Service(ref cause) => cause,
            ListChildrenError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCreateAccountStatus
#[derive(Debug, PartialEq)]
pub enum ListCreateAccountStatusError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListCreateAccountStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCreateAccountStatusError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListCreateAccountStatusError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListCreateAccountStatusError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListCreateAccountStatusError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListCreateAccountStatusError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListCreateAccountStatusError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListCreateAccountStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCreateAccountStatusError {
    fn description(&self) -> &str {
        match *self {
            ListCreateAccountStatusError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListCreateAccountStatusError::AccessDenied(ref cause) => cause,
            ListCreateAccountStatusError::InvalidInput(ref cause) => cause,
            ListCreateAccountStatusError::Service(ref cause) => cause,
            ListCreateAccountStatusError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListHandshakesForAccount
#[derive(Debug, PartialEq)]
pub enum ListHandshakesForAccountError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListHandshakesForAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHandshakesForAccountError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListHandshakesForAccountError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        ListHandshakesForAccountError::ConcurrentModification(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListHandshakesForAccountError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListHandshakesForAccountError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListHandshakesForAccountError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListHandshakesForAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHandshakesForAccountError {
    fn description(&self) -> &str {
        match *self {
            ListHandshakesForAccountError::AccessDenied(ref cause) => cause,
            ListHandshakesForAccountError::ConcurrentModification(ref cause) => cause,
            ListHandshakesForAccountError::InvalidInput(ref cause) => cause,
            ListHandshakesForAccountError::Service(ref cause) => cause,
            ListHandshakesForAccountError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListHandshakesForOrganization
#[derive(Debug, PartialEq)]
pub enum ListHandshakesForOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListHandshakesForOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListHandshakesForOrganizationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListHandshakesForOrganizationError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListHandshakesForOrganizationError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        ListHandshakesForOrganizationError::ConcurrentModification(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListHandshakesForOrganizationError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListHandshakesForOrganizationError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListHandshakesForOrganizationError::TooManyRequests(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListHandshakesForOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHandshakesForOrganizationError {
    fn description(&self) -> &str {
        match *self {
            ListHandshakesForOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListHandshakesForOrganizationError::AccessDenied(ref cause) => cause,
            ListHandshakesForOrganizationError::ConcurrentModification(ref cause) => cause,
            ListHandshakesForOrganizationError::InvalidInput(ref cause) => cause,
            ListHandshakesForOrganizationError::Service(ref cause) => cause,
            ListHandshakesForOrganizationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOrganizationalUnitsForParent
#[derive(Debug, PartialEq)]
pub enum ListOrganizationalUnitsForParentError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or OU with the <code>ParentId</code> that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListOrganizationalUnitsForParentError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListOrganizationalUnitsForParentError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListOrganizationalUnitsForParentError::AWSOrganizationsNotInUse(
                            String::from(error_message),
                        ),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListOrganizationalUnitsForParentError::AccessDenied(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        ListOrganizationalUnitsForParentError::InvalidInput(String::from(
                            error_message,
                        )),
                    )
                }
                "ParentNotFoundException" => {
                    return RusotoError::Service(
                        ListOrganizationalUnitsForParentError::ParentNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(ListOrganizationalUnitsForParentError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListOrganizationalUnitsForParentError::TooManyRequests(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListOrganizationalUnitsForParentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOrganizationalUnitsForParentError {
    fn description(&self) -> &str {
        match *self {
            ListOrganizationalUnitsForParentError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListOrganizationalUnitsForParentError::AccessDenied(ref cause) => cause,
            ListOrganizationalUnitsForParentError::InvalidInput(ref cause) => cause,
            ListOrganizationalUnitsForParentError::ParentNotFound(ref cause) => cause,
            ListOrganizationalUnitsForParentError::Service(ref cause) => cause,
            ListOrganizationalUnitsForParentError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListParents
#[derive(Debug, PartialEq)]
pub enum ListParentsError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>We can't find an organizational unit (OU) or AWS account with the <code>ChildId</code> that you specified.</p>
    ChildNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListParentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListParentsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(ListParentsError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListParentsError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "ChildNotFoundException" => {
                    return RusotoError::Service(ListParentsError::ChildNotFound(String::from(
                        error_message,
                    )))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListParentsError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListParentsError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListParentsError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListParentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListParentsError {
    fn description(&self) -> &str {
        match *self {
            ListParentsError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListParentsError::AccessDenied(ref cause) => cause,
            ListParentsError::ChildNotFound(ref cause) => cause,
            ListParentsError::InvalidInput(ref cause) => cause,
            ListParentsError::Service(ref cause) => cause,
            ListParentsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPolicies
#[derive(Debug, PartialEq)]
pub enum ListPoliciesError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPoliciesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(ListPoliciesError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListPoliciesError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListPoliciesError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListPoliciesError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListPoliciesError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListPoliciesError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListPoliciesError::AccessDenied(ref cause) => cause,
            ListPoliciesError::InvalidInput(ref cause) => cause,
            ListPoliciesError::Service(ref cause) => cause,
            ListPoliciesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPoliciesForTarget
#[derive(Debug, PartialEq)]
pub enum ListPoliciesForTargetError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, or account with the <code>TargetId</code> that you specified.</p>
    TargetNotFound(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListPoliciesForTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPoliciesForTargetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListPoliciesForTargetError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListPoliciesForTargetError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListPoliciesForTargetError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListPoliciesForTargetError::Service(String::from(
                        error_message,
                    )))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(ListPoliciesForTargetError::TargetNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListPoliciesForTargetError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPoliciesForTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPoliciesForTargetError {
    fn description(&self) -> &str {
        match *self {
            ListPoliciesForTargetError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListPoliciesForTargetError::AccessDenied(ref cause) => cause,
            ListPoliciesForTargetError::InvalidInput(ref cause) => cause,
            ListPoliciesForTargetError::Service(ref cause) => cause,
            ListPoliciesForTargetError::TargetNotFound(ref cause) => cause,
            ListPoliciesForTargetError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRoots
#[derive(Debug, PartialEq)]
pub enum ListRootsError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListRootsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRootsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(ListRootsError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListRootsError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListRootsError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListRootsError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListRootsError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRootsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRootsError {
    fn description(&self) -> &str {
        match *self {
            ListRootsError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListRootsError::AccessDenied(ref cause) => cause,
            ListRootsError::InvalidInput(ref cause) => cause,
            ListRootsError::Service(ref cause) => cause,
            ListRootsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTargetsForPolicy
#[derive(Debug, PartialEq)]
pub enum ListTargetsForPolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl ListTargetsForPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTargetsForPolicyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListTargetsForPolicyError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTargetsForPolicyError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListTargetsForPolicyError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(ListTargetsForPolicyError::PolicyNotFound(
                        String::from(error_message),
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListTargetsForPolicyError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTargetsForPolicyError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTargetsForPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTargetsForPolicyError {
    fn description(&self) -> &str {
        match *self {
            ListTargetsForPolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListTargetsForPolicyError::AccessDenied(ref cause) => cause,
            ListTargetsForPolicyError::InvalidInput(ref cause) => cause,
            ListTargetsForPolicyError::PolicyNotFound(ref cause) => cause,
            ListTargetsForPolicyError::Service(ref cause) => cause,
            ListTargetsForPolicyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by MoveAccount
#[derive(Debug, PartialEq)]
pub enum MoveAccountError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the <code>AccountId</code> that you specified, or the account whose credentials you used to make this request isn't a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>We can't find the destination container (a root or OU) with the <code>ParentId</code> that you specified.</p>
    DestinationParentNotFound(String),
    /// <p>That account is already present in the specified destination.</p>
    DuplicateAccount(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a source root or OU with the <code>ParentId</code> that you specified.</p>
    SourceParentNotFound(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl MoveAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<MoveAccountError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(MoveAccountError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(MoveAccountError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "AccountNotFoundException" => {
                    return RusotoError::Service(MoveAccountError::AccountNotFound(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(MoveAccountError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "DestinationParentNotFoundException" => {
                    return RusotoError::Service(MoveAccountError::DestinationParentNotFound(
                        String::from(error_message),
                    ))
                }
                "DuplicateAccountException" => {
                    return RusotoError::Service(MoveAccountError::DuplicateAccount(String::from(
                        error_message,
                    )))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(MoveAccountError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(MoveAccountError::Service(String::from(
                        error_message,
                    )))
                }
                "SourceParentNotFoundException" => {
                    return RusotoError::Service(MoveAccountError::SourceParentNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(MoveAccountError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for MoveAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for MoveAccountError {
    fn description(&self) -> &str {
        match *self {
            MoveAccountError::AWSOrganizationsNotInUse(ref cause) => cause,
            MoveAccountError::AccessDenied(ref cause) => cause,
            MoveAccountError::AccountNotFound(ref cause) => cause,
            MoveAccountError::ConcurrentModification(ref cause) => cause,
            MoveAccountError::DestinationParentNotFound(ref cause) => cause,
            MoveAccountError::DuplicateAccount(ref cause) => cause,
            MoveAccountError::InvalidInput(ref cause) => cause,
            MoveAccountError::Service(ref cause) => cause,
            MoveAccountError::SourceParentNotFound(ref cause) => cause,
            MoveAccountError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveAccountFromOrganization
#[derive(Debug, PartialEq)]
pub enum RemoveAccountFromOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the <code>AccountId</code> that you specified, or the account whose credentials you used to make this request isn't a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>You can't remove a master account from an organization. If you want the master account to become a member account in another organization, you must first delete the current organization of the master account.</p>
    MasterCannotLeaveOrganization(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl RemoveAccountFromOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RemoveAccountFromOrganizationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(RemoveAccountFromOrganizationError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "AccountNotFoundException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::AccountNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::ConcurrentModification(String::from(
                            error_message,
                        )),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::ConstraintViolation(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RemoveAccountFromOrganizationError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "MasterCannotLeaveOrganizationException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::MasterCannotLeaveOrganization(
                            String::from(error_message),
                        ),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(RemoveAccountFromOrganizationError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::TooManyRequests(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RemoveAccountFromOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveAccountFromOrganizationError {
    fn description(&self) -> &str {
        match *self {
            RemoveAccountFromOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            RemoveAccountFromOrganizationError::AccessDenied(ref cause) => cause,
            RemoveAccountFromOrganizationError::AccountNotFound(ref cause) => cause,
            RemoveAccountFromOrganizationError::ConcurrentModification(ref cause) => cause,
            RemoveAccountFromOrganizationError::ConstraintViolation(ref cause) => cause,
            RemoveAccountFromOrganizationError::InvalidInput(ref cause) => cause,
            RemoveAccountFromOrganizationError::MasterCannotLeaveOrganization(ref cause) => cause,
            RemoveAccountFromOrganizationError::Service(ref cause) => cause,
            RemoveAccountFromOrganizationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum UpdateOrganizationalUnitError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>An OU with the same name already exists.</p>
    DuplicateOrganizationalUnit(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find an OU with the <code>OrganizationalUnitId</code> that you specified.</p>
    OrganizationalUnitNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl UpdateOrganizationalUnitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateOrganizationalUnitError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        UpdateOrganizationalUnitError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        )),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateOrganizationalUnitError::AccessDenied(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        UpdateOrganizationalUnitError::ConcurrentModification(String::from(
                            error_message,
                        )),
                    )
                }
                "DuplicateOrganizationalUnitException" => {
                    return RusotoError::Service(
                        UpdateOrganizationalUnitError::DuplicateOrganizationalUnit(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateOrganizationalUnitError::InvalidInput(
                        String::from(error_message),
                    ))
                }
                "OrganizationalUnitNotFoundException" => {
                    return RusotoError::Service(
                        UpdateOrganizationalUnitError::OrganizationalUnitNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateOrganizationalUnitError::Service(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateOrganizationalUnitError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateOrganizationalUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateOrganizationalUnitError {
    fn description(&self) -> &str {
        match *self {
            UpdateOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => cause,
            UpdateOrganizationalUnitError::AccessDenied(ref cause) => cause,
            UpdateOrganizationalUnitError::ConcurrentModification(ref cause) => cause,
            UpdateOrganizationalUnitError::DuplicateOrganizationalUnit(ref cause) => cause,
            UpdateOrganizationalUnitError::InvalidInput(ref cause) => cause,
            UpdateOrganizationalUnitError::OrganizationalUnitNotFound(ref cause) => cause,
            UpdateOrganizationalUnitError::Service(ref cause) => cause,
            UpdateOrganizationalUnitError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePolicy
#[derive(Debug, PartialEq)]
pub enum UpdatePolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit.</p> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact<a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get receive this exception when running a command immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>ORGANIZATION<em>NOT</em>IN<em>ALL</em>FEATURES<em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this master account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>A policy with the same name already exists.</p>
    DuplicatePolicy(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that isn&#39;t valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The provided policy document doesn't meet the requirements of the specified policy type. For example, the syntax might be incorrect. For details about service control policy syntax, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>AWS Organizations User Guide</i>.</p>
    MalformedPolicyDocument(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p> <p>For information on limits that affect Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Limits of AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    TooManyRequests(String),
}

impl UpdatePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePolicyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(UpdatePolicyError::AWSOrganizationsNotInUse(
                        String::from(error_message),
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdatePolicyError::AccessDenied(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdatePolicyError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(UpdatePolicyError::ConstraintViolation(
                        String::from(error_message),
                    ))
                }
                "DuplicatePolicyException" => {
                    return RusotoError::Service(UpdatePolicyError::DuplicatePolicy(String::from(
                        error_message,
                    )))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdatePolicyError::InvalidInput(String::from(
                        error_message,
                    )))
                }
                "MalformedPolicyDocumentException" => {
                    return RusotoError::Service(UpdatePolicyError::MalformedPolicyDocument(
                        String::from(error_message),
                    ))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(UpdatePolicyError::PolicyNotFound(String::from(
                        error_message,
                    )))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdatePolicyError::Service(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdatePolicyError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdatePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePolicyError {
    fn description(&self) -> &str {
        match *self {
            UpdatePolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            UpdatePolicyError::AccessDenied(ref cause) => cause,
            UpdatePolicyError::ConcurrentModification(ref cause) => cause,
            UpdatePolicyError::ConstraintViolation(ref cause) => cause,
            UpdatePolicyError::DuplicatePolicy(ref cause) => cause,
            UpdatePolicyError::InvalidInput(ref cause) => cause,
            UpdatePolicyError::MalformedPolicyDocument(ref cause) => cause,
            UpdatePolicyError::PolicyNotFound(ref cause) => cause,
            UpdatePolicyError::Service(ref cause) => cause,
            UpdatePolicyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Organizations API. Organizations clients implement this trait.
pub trait Organizations {
    /// <p>Sends a response to the originator of a handshake agreeing to the action proposed by the handshake request. </p> <p>This operation can be called only by the following principals when they also have the relevant IAM permissions:</p> <ul> <li> <p> <b>Invitation to join</b> or <b>Approve all features request</b> handshakes: only a principal from the member account. </p> <p>The user who calls the API for an invitation to join must have the <code>organizations:AcceptHandshake</code> permission. If you enabled all features in the organization, then the user must also have the <code>iam:CreateServiceLinkedRole</code> permission so that Organizations can create the required service-linked role named <i>AWSServiceRoleForOrganizations</i>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integration_services.html#orgs_integration_service-linked-roles">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p> <b>Enable all features final confirmation</b> handshake: only a principal from the master account.</p> <p>For more information about invitations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_invites.html">Inviting an AWS Account to Join Your Organization</a> in the <i>AWS Organizations User Guide</i>. For more information about requests to enable all features in the organization, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> <p>After you accept a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn accept_handshake(
        &self,
        input: AcceptHandshakeRequest,
    ) -> RusotoFuture<AcceptHandshakeResponse, AcceptHandshakeError>;

    /// <p>Attaches a policy to a root, an organizational unit (OU), or an individual account. How the policy affects accounts depends on the type of policy:</p> <ul> <li> <p> <b>Service control policy (SCP)</b> - An SCP specifies what permissions can be delegated to users in affected member accounts. The scope of influence for a policy depends on what you attach the policy to:</p> <ul> <li> <p>If you attach an SCP to a root, it affects all accounts in the organization.</p> </li> <li> <p>If you attach an SCP to an OU, it affects all accounts in that OU and in any child OUs.</p> </li> <li> <p>If you attach the policy directly to an account, then it affects only that account.</p> </li> </ul> <p>SCPs are JSON policies that specify the maximum permissions for an organization or organizational unit (OU). When you attach one SCP to a higher level root or OU, and you also attach a different SCP to a child OU or to an account, the child policy can further restrict only the permissions that pass through the parent filter and are available to the child. An SCP that is attached to a child cannot grant a permission that is not already granted by the parent. For example, imagine that the parent SCP allows permissions A, B, C, D, and E. The child SCP allows C, D, E, F, and G. The result is that the accounts affected by the child SCP are allowed to use only C, D, and E. They cannot use A or B because they were filtered out by the child OU. They also cannot use F and G because they were filtered out by the parent OU. They cannot be granted back by the child SCP; child SCPs can only filter the permissions they receive from the parent SCP.</p> <p>AWS Organizations attaches a default SCP named <code>"FullAWSAccess</code> to every root, OU, and account. This default SCP allows all services and actions, enabling any new child OU or account to inherit the permissions of the parent root or OU. If you detach the default policy, you must replace it with a policy that specifies the permissions that you want to allow in that OU or account.</p> <p>For more information about how Organizations policies permissions work, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">Using Service Control Policies</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> <p>This operation can be called only from the organization's master account.</p>
    fn attach_policy(&self, input: AttachPolicyRequest) -> RusotoFuture<(), AttachPolicyError>;

    /// <p>Cancels a handshake. Canceling a handshake sets the handshake state to <code>CANCELED</code>. </p> <p>This operation can be called only from the account that originated the handshake. The recipient of the handshake can't cancel it, but can use <a>DeclineHandshake</a> instead. After a handshake is canceled, the recipient can no longer respond to that handshake.</p> <p>After you cancel a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn cancel_handshake(
        &self,
        input: CancelHandshakeRequest,
    ) -> RusotoFuture<CancelHandshakeResponse, CancelHandshakeError>;

    /// <p><p>Creates an AWS account that is automatically a member of the organization whose credentials made the request. This is an asynchronous request that AWS performs in the background. Because <code>CreateAccount</code> operates asynchronously, it can return a successful completion message even though account initialization might still be in progress. You might need to wait a few minutes before you can successfully access the account. To check the status of the request, do one of the following:</p> <ul> <li> <p>Use the <code>OperationId</code> response element from this operation to provide as a parameter to the <a>DescribeCreateAccountStatus</a> operation.</p> </li> <li> <p>Check the AWS CloudTrail log for the <code>CreateAccountResult</code> event. For information on using AWS CloudTrail with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_monitoring.html">Monitoring the Activity in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <p/> <p>The user who calls the API to create an account must have the <code>organizations:CreateAccount</code> permission. If you enabled all features in the organization, AWS Organizations will create the required service-linked role named <code>AWSServiceRoleForOrganizations</code>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html#orgs_integrate_services-using_slrs">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> <p>AWS Organizations preconfigures the new member account with a role (named <code>OrganizationAccountAccessRole</code> by default) that grants users in the master account administrator permissions in the new member account. Principals in the master account can assume the role. AWS Organizations clones the company name and address information for the new account from the organization&#39;s master account.</p> <p>This operation can be called only from the organization&#39;s master account.</p> <p>For more information about creating accounts, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_create.html">Creating an AWS Account in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> <important> <ul> <li> <p>When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required for the account to operate as a standalone account, such as a payment method and signing the end user license agreement (EULA) is <i>not</i> automatically collected. If you must remove an account from your organization later, you can do so only after you provide the missing information. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization as a member account</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>If you get an exception that indicates that you exceeded your account limits for the organization, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>If you get an exception that indicates that the operation failed because your organization is still initializing, wait one hour and then try again. If the error persists, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>Using CreateAccount to create multiple temporary accounts isn&#39;t recommended. You can only close an account from the Billing and Cost Management Console, and you must be signed in as the root user. For information on the requirements and process for closing an account, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_close.html">Closing an AWS Account</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> </important> <note> <p>When you create a member account with this operation, you can choose whether to create the account with the <b>IAM User and Role Access to Billing Information</b> switch enabled. If you enable it, IAM users and roles that have appropriate permissions can view billing information for the account. If you disable it, only the account root user can access billing information. For information about how to disable this switch for an account, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html">Granting Access to Your Billing Information and Tools</a>.</p> </note></p>
    fn create_account(
        &self,
        input: CreateAccountRequest,
    ) -> RusotoFuture<CreateAccountResponse, CreateAccountError>;

    /// <p>Creates an AWS organization. The account whose user is calling the CreateOrganization operation automatically becomes the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/orgs_getting-started_concepts.html#account">master account</a> of the new organization.</p> <p>This operation must be called using credentials from the account that is to become the new organization's master account. The principal must also have the relevant IAM permissions.</p> <p>By default (or if you set the <code>FeatureSet</code> parameter to <code>ALL</code>), the new organization is created with all features enabled and service control policies automatically enabled in the root. If you instead choose to create the organization supporting only the consolidated billing features by setting the <code>FeatureSet</code> parameter to <code>CONSOLIDATED_BILLING"</code>, then no policy types are enabled by default and you cannot use organization policies.</p>
    fn create_organization(
        &self,
        input: CreateOrganizationRequest,
    ) -> RusotoFuture<CreateOrganizationResponse, CreateOrganizationError>;

    /// <p>Creates an organizational unit (OU) within a root or parent OU. An OU is a container for accounts that enables you to organize your accounts to apply policies according to your business requirements. The number of levels deep that you can nest OUs is dependent upon the policy types enabled for that root. For service control policies, the limit is five. </p> <p>For more information about OUs, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_ous.html">Managing Organizational Units</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn create_organizational_unit(
        &self,
        input: CreateOrganizationalUnitRequest,
    ) -> RusotoFuture<CreateOrganizationalUnitResponse, CreateOrganizationalUnitError>;

    /// <p>Creates a policy of a specified type that you can attach to a root, an organizational unit (OU), or an individual AWS account.</p> <p>For more information about policies and their use, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html">Managing Organization Policies</a>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn create_policy(
        &self,
        input: CreatePolicyRequest,
    ) -> RusotoFuture<CreatePolicyResponse, CreatePolicyError>;

    /// <p>Declines a handshake request. This sets the handshake state to <code>DECLINED</code> and effectively deactivates the request.</p> <p>This operation can be called only from the account that received the handshake. The originator of the handshake can use <a>CancelHandshake</a> instead. The originator can't reactivate a declined request, but can re-initiate the process with a new handshake request.</p> <p>After you decline a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn decline_handshake(
        &self,
        input: DeclineHandshakeRequest,
    ) -> RusotoFuture<DeclineHandshakeResponse, DeclineHandshakeError>;

    /// <p>Deletes the organization. You can delete an organization only by using credentials from the master account. The organization must be empty of member accounts.</p>
    fn delete_organization(&self) -> RusotoFuture<(), DeleteOrganizationError>;

    /// <p>Deletes an organizational unit (OU) from a root or another OU. You must first remove all accounts and child OUs from the OU that you want to delete.</p> <p>This operation can be called only from the organization's master account.</p>
    fn delete_organizational_unit(
        &self,
        input: DeleteOrganizationalUnitRequest,
    ) -> RusotoFuture<(), DeleteOrganizationalUnitError>;

    /// <p>Deletes the specified policy from your organization. Before you perform this operation, you must first detach the policy from all organizational units (OUs), roots, and accounts.</p> <p>This operation can be called only from the organization's master account.</p>
    fn delete_policy(&self, input: DeletePolicyRequest) -> RusotoFuture<(), DeletePolicyError>;

    /// <p>Retrieves Organizations-related information about the specified account.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_account(
        &self,
        input: DescribeAccountRequest,
    ) -> RusotoFuture<DescribeAccountResponse, DescribeAccountError>;

    /// <p>Retrieves the current status of an asynchronous request to create an account.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_create_account_status(
        &self,
        input: DescribeCreateAccountStatusRequest,
    ) -> RusotoFuture<DescribeCreateAccountStatusResponse, DescribeCreateAccountStatusError>;

    /// <p>Retrieves information about a previously requested handshake. The handshake ID comes from the response to the original <a>InviteAccountToOrganization</a> operation that generated the handshake.</p> <p>You can access handshakes that are ACCEPTED, DECLINED, or CANCELED for only 30 days after they change to that state. They are then deleted and no longer accessible.</p> <p>This operation can be called from any account in the organization.</p>
    fn describe_handshake(
        &self,
        input: DescribeHandshakeRequest,
    ) -> RusotoFuture<DescribeHandshakeResponse, DescribeHandshakeError>;

    /// <p><p>Retrieves information about the organization that the user&#39;s account belongs to.</p> <p>This operation can be called from any account in the organization.</p> <note> <p>Even if a policy type is shown as available in the organization, it can be disabled separately at the root level with <a>DisablePolicyType</a>. Use <a>ListRoots</a> to see the status of policy types for a specified root.</p> </note></p>
    fn describe_organization(
        &self,
    ) -> RusotoFuture<DescribeOrganizationResponse, DescribeOrganizationError>;

    /// <p>Retrieves information about an organizational unit (OU).</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_organizational_unit(
        &self,
        input: DescribeOrganizationalUnitRequest,
    ) -> RusotoFuture<DescribeOrganizationalUnitResponse, DescribeOrganizationalUnitError>;

    /// <p>Retrieves information about a policy.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_policy(
        &self,
        input: DescribePolicyRequest,
    ) -> RusotoFuture<DescribePolicyResponse, DescribePolicyError>;

    /// <p>Detaches a policy from a target root, organizational unit (OU), or account. If the policy being detached is a service control policy (SCP), the changes to permissions for IAM users and roles in affected accounts are immediate.</p> <p> <b>Note:</b> Every root, OU, and account must have at least one SCP attached. If you want to replace the default <code>FullAWSAccess</code> policy with one that limits the permissions that can be delegated, then you must attach the replacement policy before you can remove the default one. This is the authorization strategy of <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_about-scps.html#orgs_policies_whitelist">whitelisting</a>. If you instead attach a second SCP and leave the <code>FullAWSAccess</code> SCP still attached, and specify <code>"Effect": "Deny"</code> in the second SCP to override the <code>"Effect": "Allow"</code> in the <code>FullAWSAccess</code> policy (or any other attached SCP), then you are using the authorization strategy of <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_about-scps.html#orgs_policies_blacklist">blacklisting</a>. </p> <p>This operation can be called only from the organization's master account.</p>
    fn detach_policy(&self, input: DetachPolicyRequest) -> RusotoFuture<(), DetachPolicyError>;

    /// <p>Disables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you disable integration, the specified service no longer can create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in <i>new</i> accounts in your organization. This means the service can't perform operations on your behalf on any new accounts in your organization. The service can still perform operations in older accounts until the service completes its clean-up from AWS Organizations.</p> <p/> <important> <p>We recommend that you disable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the other service is aware that it can clean up any resources that are required only for the integration. How the service cleans up its resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>After you perform the <code>DisableAWSServiceAccess</code> operation, the specified service can no longer perform operations in your organization's accounts unless the operations are explicitly permitted by the IAM policies that are attached to your roles. </p> <p>For more information about integrating other services with AWS Organizations, including the list of services that work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn disable_aws_service_access(
        &self,
        input: DisableAWSServiceAccessRequest,
    ) -> RusotoFuture<(), DisableAWSServiceAccessError>;

    /// <p><p>Disables an organizational control policy type in a root. A policy of a certain type can be attached to entities in a root only if that type is enabled in the root. After you perform this operation, you no longer can attach policies of the specified type to that root or to any organizational unit (OU) or account in that root. You can undo this by using the <a>EnablePolicyType</a> operation.</p> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>If you disable a policy type for a root, it still shows as enabled for the organization if all features are enabled in that organization. Use <a>ListRoots</a> to see the status of policy types for a specified root. Use <a>DescribeOrganization</a> to see the status of policy types in the organization.</p> </note></p>
    fn disable_policy_type(
        &self,
        input: DisablePolicyTypeRequest,
    ) -> RusotoFuture<DisablePolicyTypeResponse, DisablePolicyTypeError>;

    /// <p>Enables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you enable integration, you allow the specified service to create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in all the accounts in your organization. This allows the service to perform operations on your behalf in your organization and its accounts.</p> <important> <p>We recommend that you enable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the service is aware that it can create the resources that are required for the integration. How the service creates those resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>For more information about enabling services to integrate with AWS Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account and only if the organization has <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">enabled all features</a>.</p>
    fn enable_aws_service_access(
        &self,
        input: EnableAWSServiceAccessRequest,
    ) -> RusotoFuture<(), EnableAWSServiceAccessError>;

    /// <p>Enables all features in an organization. This enables the use of organization policies that can restrict the services and actions that can be called in each account. Until you enable all features, you have access only to consolidated billing, and you can't use any of the advanced account administration features that AWS Organizations supports. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p> <important> <p>This operation is required only for organizations that were created explicitly with only the consolidated billing features enabled. Calling this operation sends a handshake to every invited account in the organization. The feature set change can be finalized and the additional features enabled only after all administrators in the invited accounts approve the change by accepting the handshake.</p> </important> <p>After you enable all features, you can separately enable or disable individual policy types in a root using <a>EnablePolicyType</a> and <a>DisablePolicyType</a>. To see the status of policy types in a root, use <a>ListRoots</a>.</p> <p>After all invited member accounts accept the handshake, you finalize the feature set change by accepting the handshake that contains <code>"Action": "ENABLE_ALL_FEATURES"</code>. This completes the change.</p> <p>After you enable all features in your organization, the master account in the organization can apply policies on all member accounts. These policies can restrict what users and even administrators in those accounts can do. The master account can apply policies that prevent accounts from leaving the organization. Ensure that your account administrators are aware of this.</p> <p>This operation can be called only from the organization's master account. </p>
    fn enable_all_features(
        &self,
    ) -> RusotoFuture<EnableAllFeaturesResponse, EnableAllFeaturesError>;

    /// <p>Enables a policy type in a root. After you enable a policy type in a root, you can attach policies of that type to the root, any organizational unit (OU), or account in that root. You can undo this by using the <a>DisablePolicyType</a> operation.</p> <p>This operation can be called only from the organization's master account.</p> <p>You can enable a policy type in a root only if that policy type is available in the organization. Use <a>DescribeOrganization</a> to view the status of available policy types in the organization.</p> <p>To view the status of policy type in a root, use <a>ListRoots</a>.</p>
    fn enable_policy_type(
        &self,
        input: EnablePolicyTypeRequest,
    ) -> RusotoFuture<EnablePolicyTypeResponse, EnablePolicyTypeError>;

    /// <p>Sends an invitation to another account to join your organization as a member account. Organizations sends email on your behalf to the email address that is associated with the other account's owner. The invitation is implemented as a <a>Handshake</a> whose details are in the response.</p> <important> <ul> <li> <p>You can invite AWS accounts only from the same seller as the master account. For example, if your organization's master account was created by Amazon Internet Services Pvt. Ltd (AISPL), an AWS seller in India, then you can only invite other AISPL accounts to your organization. You can't combine accounts from AISPL and AWS, or any other AWS seller. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/useconsolidatedbilliing-India.html">Consolidated Billing in India</a>.</p> </li> <li> <p>If you receive an exception that indicates that you exceeded your account limits for the organization or that the operation failed because your organization is still initializing, wait one hour and then try again. If the error persists after an hour, then contact <a href="https://console.aws.amazon.com/support/home#/">AWS Customer Support</a>.</p> </li> </ul> </important> <p>This operation can be called only from the organization's master account.</p>
    fn invite_account_to_organization(
        &self,
        input: InviteAccountToOrganizationRequest,
    ) -> RusotoFuture<InviteAccountToOrganizationResponse, InviteAccountToOrganizationError>;

    /// <p><p>Removes a member account from its parent organization. This version of the operation is performed by the account that wants to leave. To remove a member account as a user in the master account, use <a>RemoveAccountFromOrganization</a> instead.</p> <p>This operation can be called only from a member account in the organization.</p> <important> <ul> <li> <p>The master account in an organization with all features enabled can set service control policies (SCPs) that can restrict what administrators of member accounts can do, including preventing them from successfully calling <code>LeaveOrganization</code> and leaving the organization. </p> </li> <li> <p>You can leave an organization as a member account only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For each account that you want to make standalone, you must accept the End User License Agreement (EULA), choose a support plan, provide and verify the required contact information, and provide a current payment method. AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account is not attached to an organization. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>You can leave an organization only after you enable IAM user access to billing in your account. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> </li> </ul> </important></p>
    fn leave_organization(&self) -> RusotoFuture<(), LeaveOrganizationError>;

    /// <p>Returns a list of the AWS services that you enabled to integrate with your organization. After a service on this list creates the resources that it requires for the integration, it can perform operations on your organization and its accounts.</p> <p>For more information about integrating other services with AWS Organizations, including the list of services that currently work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn list_aws_service_access_for_organization(
        &self,
        input: ListAWSServiceAccessForOrganizationRequest,
    ) -> RusotoFuture<
        ListAWSServiceAccessForOrganizationResponse,
        ListAWSServiceAccessForOrganizationError,
    >;

    /// <p>Lists all the accounts in the organization. To request only the accounts in a specified root or organizational unit (OU), use the <a>ListAccountsForParent</a> operation instead.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> RusotoFuture<ListAccountsResponse, ListAccountsError>;

    /// <p>Lists the accounts in an organization that are contained by the specified target root or organizational unit (OU). If you specify the root, you get a list of all the accounts that are not in any OU. If you specify an OU, you get a list of all the accounts in only that OU, and not in any child OUs. To get a list of all accounts in the organization, use the <a>ListAccounts</a> operation.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_accounts_for_parent(
        &self,
        input: ListAccountsForParentRequest,
    ) -> RusotoFuture<ListAccountsForParentResponse, ListAccountsForParentError>;

    /// <p>Lists all of the organizational units (OUs) or accounts that are contained in the specified parent OU or root. This operation, along with <a>ListParents</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_children(
        &self,
        input: ListChildrenRequest,
    ) -> RusotoFuture<ListChildrenResponse, ListChildrenError>;

    /// <p>Lists the account creation requests that match the specified status that is currently being tracked for the organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_create_account_status(
        &self,
        input: ListCreateAccountStatusRequest,
    ) -> RusotoFuture<ListCreateAccountStatusResponse, ListCreateAccountStatusError>;

    /// <p>Lists the current handshakes that are associated with the account of the requesting user.</p> <p>Handshakes that are ACCEPTED, DECLINED, or CANCELED appear in the results of this API for only 30 days after changing to that state. After that they are deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called from any account in the organization.</p>
    fn list_handshakes_for_account(
        &self,
        input: ListHandshakesForAccountRequest,
    ) -> RusotoFuture<ListHandshakesForAccountResponse, ListHandshakesForAccountError>;

    /// <p>Lists the handshakes that are associated with the organization that the requesting user is part of. The <code>ListHandshakesForOrganization</code> operation returns a list of handshake structures. Each structure contains details and status about a handshake.</p> <p>Handshakes that are ACCEPTED, DECLINED, or CANCELED appear in the results of this API for only 30 days after changing to that state. After that they are deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_handshakes_for_organization(
        &self,
        input: ListHandshakesForOrganizationRequest,
    ) -> RusotoFuture<ListHandshakesForOrganizationResponse, ListHandshakesForOrganizationError>;

    /// <p>Lists the organizational units (OUs) in a parent organizational unit or root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_organizational_units_for_parent(
        &self,
        input: ListOrganizationalUnitsForParentRequest,
    ) -> RusotoFuture<ListOrganizationalUnitsForParentResponse, ListOrganizationalUnitsForParentError>;

    /// <p><p>Lists the root or organizational units (OUs) that serve as the immediate parent of the specified child OU or account. This operation, along with <a>ListChildren</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>In the current release, a child can have only a single parent. </p> </note></p>
    fn list_parents(
        &self,
        input: ListParentsRequest,
    ) -> RusotoFuture<ListParentsResponse, ListParentsError>;

    /// <p>Retrieves the list of all policies in an organization of a specified type.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_policies(
        &self,
        input: ListPoliciesRequest,
    ) -> RusotoFuture<ListPoliciesResponse, ListPoliciesError>;

    /// <p>Lists the policies that are directly attached to the specified target root, organizational unit (OU), or account. You must specify the policy type that you want included in the returned list.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_policies_for_target(
        &self,
        input: ListPoliciesForTargetRequest,
    ) -> RusotoFuture<ListPoliciesForTargetResponse, ListPoliciesForTargetError>;

    /// <p><p>Lists the roots that are defined in the current organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>Policy types can be enabled and disabled in roots. This is distinct from whether they are available in the organization. When you enable all features, you make policy types available for use in that organization. Individual policy types can then be enabled and disabled in a root. To see the availability of a policy type in an organization, use <a>DescribeOrganization</a>.</p> </note></p>
    fn list_roots(
        &self,
        input: ListRootsRequest,
    ) -> RusotoFuture<ListRootsResponse, ListRootsError>;

    /// <p>Lists all the roots, organizaitonal units (OUs), and accounts to which the specified policy is attached.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_targets_for_policy(
        &self,
        input: ListTargetsForPolicyRequest,
    ) -> RusotoFuture<ListTargetsForPolicyResponse, ListTargetsForPolicyError>;

    /// <p>Moves an account from its current source parent root or organizational unit (OU) to the specified destination parent root or OU.</p> <p>This operation can be called only from the organization's master account.</p>
    fn move_account(&self, input: MoveAccountRequest) -> RusotoFuture<(), MoveAccountError>;

    /// <p><p>Removes the specified account from the organization.</p> <p>The removed account becomes a stand-alone account that is not a member of any organization. It is no longer subject to any policies and is responsible for its own bill payments. The organization&#39;s master account is no longer charged for any expenses accrued by the member account after it is removed from the organization.</p> <p>This operation can be called only from the organization&#39;s master account. Member accounts can remove themselves with <a>LeaveOrganization</a> instead.</p> <important> <p>You can remove an account from your organization only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For an account that you want to make standalone, you must accept the End User License Agreement (EULA), choose a support plan, provide and verify the required contact information, and provide a current payment method. AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account is not attached to an organization. To remove an account that does not yet have this information, you must sign in as the member account and follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </important></p>
    fn remove_account_from_organization(
        &self,
        input: RemoveAccountFromOrganizationRequest,
    ) -> RusotoFuture<(), RemoveAccountFromOrganizationError>;

    /// <p>Renames the specified organizational unit (OU). The ID and ARN do not change. The child OUs and accounts remain in place, and any attached policies of the OU remain attached. </p> <p>This operation can be called only from the organization's master account.</p>
    fn update_organizational_unit(
        &self,
        input: UpdateOrganizationalUnitRequest,
    ) -> RusotoFuture<UpdateOrganizationalUnitResponse, UpdateOrganizationalUnitError>;

    /// <p>Updates an existing policy with a new name, description, or content. If any parameter is not supplied, that value remains unchanged. Note that you cannot change a policy's type.</p> <p>This operation can be called only from the organization's master account.</p>
    fn update_policy(
        &self,
        input: UpdatePolicyRequest,
    ) -> RusotoFuture<UpdatePolicyResponse, UpdatePolicyError>;
}
/// A client for the Organizations API.
#[derive(Clone)]
pub struct OrganizationsClient {
    client: Client,
    region: region::Region,
}

impl OrganizationsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> OrganizationsClient {
        OrganizationsClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> OrganizationsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        OrganizationsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Organizations for OrganizationsClient {
    /// <p>Sends a response to the originator of a handshake agreeing to the action proposed by the handshake request. </p> <p>This operation can be called only by the following principals when they also have the relevant IAM permissions:</p> <ul> <li> <p> <b>Invitation to join</b> or <b>Approve all features request</b> handshakes: only a principal from the member account. </p> <p>The user who calls the API for an invitation to join must have the <code>organizations:AcceptHandshake</code> permission. If you enabled all features in the organization, then the user must also have the <code>iam:CreateServiceLinkedRole</code> permission so that Organizations can create the required service-linked role named <i>AWSServiceRoleForOrganizations</i>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integration_services.html#orgs_integration_service-linked-roles">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p> <b>Enable all features final confirmation</b> handshake: only a principal from the master account.</p> <p>For more information about invitations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_invites.html">Inviting an AWS Account to Join Your Organization</a> in the <i>AWS Organizations User Guide</i>. For more information about requests to enable all features in the organization, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> <p>After you accept a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn accept_handshake(
        &self,
        input: AcceptHandshakeRequest,
    ) -> RusotoFuture<AcceptHandshakeResponse, AcceptHandshakeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.AcceptHandshake");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AcceptHandshakeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AcceptHandshakeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Attaches a policy to a root, an organizational unit (OU), or an individual account. How the policy affects accounts depends on the type of policy:</p> <ul> <li> <p> <b>Service control policy (SCP)</b> - An SCP specifies what permissions can be delegated to users in affected member accounts. The scope of influence for a policy depends on what you attach the policy to:</p> <ul> <li> <p>If you attach an SCP to a root, it affects all accounts in the organization.</p> </li> <li> <p>If you attach an SCP to an OU, it affects all accounts in that OU and in any child OUs.</p> </li> <li> <p>If you attach the policy directly to an account, then it affects only that account.</p> </li> </ul> <p>SCPs are JSON policies that specify the maximum permissions for an organization or organizational unit (OU). When you attach one SCP to a higher level root or OU, and you also attach a different SCP to a child OU or to an account, the child policy can further restrict only the permissions that pass through the parent filter and are available to the child. An SCP that is attached to a child cannot grant a permission that is not already granted by the parent. For example, imagine that the parent SCP allows permissions A, B, C, D, and E. The child SCP allows C, D, E, F, and G. The result is that the accounts affected by the child SCP are allowed to use only C, D, and E. They cannot use A or B because they were filtered out by the child OU. They also cannot use F and G because they were filtered out by the parent OU. They cannot be granted back by the child SCP; child SCPs can only filter the permissions they receive from the parent SCP.</p> <p>AWS Organizations attaches a default SCP named <code>"FullAWSAccess</code> to every root, OU, and account. This default SCP allows all services and actions, enabling any new child OU or account to inherit the permissions of the parent root or OU. If you detach the default policy, you must replace it with a policy that specifies the permissions that you want to allow in that OU or account.</p> <p>For more information about how Organizations policies permissions work, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">Using Service Control Policies</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> <p>This operation can be called only from the organization's master account.</p>
    fn attach_policy(&self, input: AttachPolicyRequest) -> RusotoFuture<(), AttachPolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.AttachPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AttachPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Cancels a handshake. Canceling a handshake sets the handshake state to <code>CANCELED</code>. </p> <p>This operation can be called only from the account that originated the handshake. The recipient of the handshake can't cancel it, but can use <a>DeclineHandshake</a> instead. After a handshake is canceled, the recipient can no longer respond to that handshake.</p> <p>After you cancel a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn cancel_handshake(
        &self,
        input: CancelHandshakeRequest,
    ) -> RusotoFuture<CancelHandshakeResponse, CancelHandshakeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.CancelHandshake");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CancelHandshakeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CancelHandshakeError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Creates an AWS account that is automatically a member of the organization whose credentials made the request. This is an asynchronous request that AWS performs in the background. Because <code>CreateAccount</code> operates asynchronously, it can return a successful completion message even though account initialization might still be in progress. You might need to wait a few minutes before you can successfully access the account. To check the status of the request, do one of the following:</p> <ul> <li> <p>Use the <code>OperationId</code> response element from this operation to provide as a parameter to the <a>DescribeCreateAccountStatus</a> operation.</p> </li> <li> <p>Check the AWS CloudTrail log for the <code>CreateAccountResult</code> event. For information on using AWS CloudTrail with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_monitoring.html">Monitoring the Activity in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <p/> <p>The user who calls the API to create an account must have the <code>organizations:CreateAccount</code> permission. If you enabled all features in the organization, AWS Organizations will create the required service-linked role named <code>AWSServiceRoleForOrganizations</code>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html#orgs_integrate_services-using_slrs">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> <p>AWS Organizations preconfigures the new member account with a role (named <code>OrganizationAccountAccessRole</code> by default) that grants users in the master account administrator permissions in the new member account. Principals in the master account can assume the role. AWS Organizations clones the company name and address information for the new account from the organization&#39;s master account.</p> <p>This operation can be called only from the organization&#39;s master account.</p> <p>For more information about creating accounts, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_create.html">Creating an AWS Account in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> <important> <ul> <li> <p>When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required for the account to operate as a standalone account, such as a payment method and signing the end user license agreement (EULA) is <i>not</i> automatically collected. If you must remove an account from your organization later, you can do so only after you provide the missing information. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization as a member account</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>If you get an exception that indicates that you exceeded your account limits for the organization, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>If you get an exception that indicates that the operation failed because your organization is still initializing, wait one hour and then try again. If the error persists, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>Using CreateAccount to create multiple temporary accounts isn&#39;t recommended. You can only close an account from the Billing and Cost Management Console, and you must be signed in as the root user. For information on the requirements and process for closing an account, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_close.html">Closing an AWS Account</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> </important> <note> <p>When you create a member account with this operation, you can choose whether to create the account with the <b>IAM User and Role Access to Billing Information</b> switch enabled. If you enable it, IAM users and roles that have appropriate permissions can view billing information for the account. If you disable it, only the account root user can access billing information. For information about how to disable this switch for an account, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html">Granting Access to Your Billing Information and Tools</a>.</p> </note></p>
    fn create_account(
        &self,
        input: CreateAccountRequest,
    ) -> RusotoFuture<CreateAccountResponse, CreateAccountError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.CreateAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateAccountResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an AWS organization. The account whose user is calling the CreateOrganization operation automatically becomes the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/orgs_getting-started_concepts.html#account">master account</a> of the new organization.</p> <p>This operation must be called using credentials from the account that is to become the new organization's master account. The principal must also have the relevant IAM permissions.</p> <p>By default (or if you set the <code>FeatureSet</code> parameter to <code>ALL</code>), the new organization is created with all features enabled and service control policies automatically enabled in the root. If you instead choose to create the organization supporting only the consolidated billing features by setting the <code>FeatureSet</code> parameter to <code>CONSOLIDATED_BILLING"</code>, then no policy types are enabled by default and you cannot use organization policies.</p>
    fn create_organization(
        &self,
        input: CreateOrganizationRequest,
    ) -> RusotoFuture<CreateOrganizationResponse, CreateOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.CreateOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateOrganizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateOrganizationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an organizational unit (OU) within a root or parent OU. An OU is a container for accounts that enables you to organize your accounts to apply policies according to your business requirements. The number of levels deep that you can nest OUs is dependent upon the policy types enabled for that root. For service control policies, the limit is five. </p> <p>For more information about OUs, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_ous.html">Managing Organizational Units</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn create_organizational_unit(
        &self,
        input: CreateOrganizationalUnitRequest,
    ) -> RusotoFuture<CreateOrganizationalUnitResponse, CreateOrganizationalUnitError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.CreateOrganizationalUnit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateOrganizationalUnitResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateOrganizationalUnitError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a policy of a specified type that you can attach to a root, an organizational unit (OU), or an individual AWS account.</p> <p>For more information about policies and their use, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html">Managing Organization Policies</a>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn create_policy(
        &self,
        input: CreatePolicyRequest,
    ) -> RusotoFuture<CreatePolicyResponse, CreatePolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.CreatePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreatePolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Declines a handshake request. This sets the handshake state to <code>DECLINED</code> and effectively deactivates the request.</p> <p>This operation can be called only from the account that received the handshake. The originator of the handshake can use <a>CancelHandshake</a> instead. The originator can't reactivate a declined request, but can re-initiate the process with a new handshake request.</p> <p>After you decline a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn decline_handshake(
        &self,
        input: DeclineHandshakeRequest,
    ) -> RusotoFuture<DeclineHandshakeResponse, DeclineHandshakeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DeclineHandshake");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DeclineHandshakeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeclineHandshakeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the organization. You can delete an organization only by using credentials from the master account. The organization must be empty of member accounts.</p>
    fn delete_organization(&self) -> RusotoFuture<(), DeleteOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DeleteOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteOrganizationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an organizational unit (OU) from a root or another OU. You must first remove all accounts and child OUs from the OU that you want to delete.</p> <p>This operation can be called only from the organization's master account.</p>
    fn delete_organizational_unit(
        &self,
        input: DeleteOrganizationalUnitRequest,
    ) -> RusotoFuture<(), DeleteOrganizationalUnitError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DeleteOrganizationalUnit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteOrganizationalUnitError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the specified policy from your organization. Before you perform this operation, you must first detach the policy from all organizational units (OUs), roots, and accounts.</p> <p>This operation can be called only from the organization's master account.</p>
    fn delete_policy(&self, input: DeletePolicyRequest) -> RusotoFuture<(), DeletePolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DeletePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves Organizations-related information about the specified account.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_account(
        &self,
        input: DescribeAccountRequest,
    ) -> RusotoFuture<DescribeAccountResponse, DescribeAccountError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DescribeAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeAccountResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the current status of an asynchronous request to create an account.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_create_account_status(
        &self,
        input: DescribeCreateAccountStatusRequest,
    ) -> RusotoFuture<DescribeCreateAccountStatusResponse, DescribeCreateAccountStatusError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeCreateAccountStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeCreateAccountStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCreateAccountStatusError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves information about a previously requested handshake. The handshake ID comes from the response to the original <a>InviteAccountToOrganization</a> operation that generated the handshake.</p> <p>You can access handshakes that are ACCEPTED, DECLINED, or CANCELED for only 30 days after they change to that state. They are then deleted and no longer accessible.</p> <p>This operation can be called from any account in the organization.</p>
    fn describe_handshake(
        &self,
        input: DescribeHandshakeRequest,
    ) -> RusotoFuture<DescribeHandshakeResponse, DescribeHandshakeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeHandshake",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeHandshakeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeHandshakeError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Retrieves information about the organization that the user&#39;s account belongs to.</p> <p>This operation can be called from any account in the organization.</p> <note> <p>Even if a policy type is shown as available in the organization, it can be disabled separately at the root level with <a>DisablePolicyType</a>. Use <a>ListRoots</a> to see the status of policy types for a specified root.</p> </note></p>
    fn describe_organization(
        &self,
    ) -> RusotoFuture<DescribeOrganizationResponse, DescribeOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeOrganizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeOrganizationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves information about an organizational unit (OU).</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_organizational_unit(
        &self,
        input: DescribeOrganizationalUnitRequest,
    ) -> RusotoFuture<DescribeOrganizationalUnitResponse, DescribeOrganizationalUnitError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeOrganizationalUnit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeOrganizationalUnitResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeOrganizationalUnitError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves information about a policy.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_policy(
        &self,
        input: DescribePolicyRequest,
    ) -> RusotoFuture<DescribePolicyResponse, DescribePolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DescribePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribePolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribePolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detaches a policy from a target root, organizational unit (OU), or account. If the policy being detached is a service control policy (SCP), the changes to permissions for IAM users and roles in affected accounts are immediate.</p> <p> <b>Note:</b> Every root, OU, and account must have at least one SCP attached. If you want to replace the default <code>FullAWSAccess</code> policy with one that limits the permissions that can be delegated, then you must attach the replacement policy before you can remove the default one. This is the authorization strategy of <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_about-scps.html#orgs_policies_whitelist">whitelisting</a>. If you instead attach a second SCP and leave the <code>FullAWSAccess</code> SCP still attached, and specify <code>"Effect": "Deny"</code> in the second SCP to override the <code>"Effect": "Allow"</code> in the <code>FullAWSAccess</code> policy (or any other attached SCP), then you are using the authorization strategy of <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_about-scps.html#orgs_policies_blacklist">blacklisting</a>. </p> <p>This operation can be called only from the organization's master account.</p>
    fn detach_policy(&self, input: DetachPolicyRequest) -> RusotoFuture<(), DetachPolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DetachPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetachPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you disable integration, the specified service no longer can create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in <i>new</i> accounts in your organization. This means the service can't perform operations on your behalf on any new accounts in your organization. The service can still perform operations in older accounts until the service completes its clean-up from AWS Organizations.</p> <p/> <important> <p>We recommend that you disable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the other service is aware that it can clean up any resources that are required only for the integration. How the service cleans up its resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>After you perform the <code>DisableAWSServiceAccess</code> operation, the specified service can no longer perform operations in your organization's accounts unless the operations are explicitly permitted by the IAM policies that are attached to your roles. </p> <p>For more information about integrating other services with AWS Organizations, including the list of services that work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn disable_aws_service_access(
        &self,
        input: DisableAWSServiceAccessRequest,
    ) -> RusotoFuture<(), DisableAWSServiceAccessError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DisableAWSServiceAccess",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisableAWSServiceAccessError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Disables an organizational control policy type in a root. A policy of a certain type can be attached to entities in a root only if that type is enabled in the root. After you perform this operation, you no longer can attach policies of the specified type to that root or to any organizational unit (OU) or account in that root. You can undo this by using the <a>EnablePolicyType</a> operation.</p> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>If you disable a policy type for a root, it still shows as enabled for the organization if all features are enabled in that organization. Use <a>ListRoots</a> to see the status of policy types for a specified root. Use <a>DescribeOrganization</a> to see the status of policy types in the organization.</p> </note></p>
    fn disable_policy_type(
        &self,
        input: DisablePolicyTypeRequest,
    ) -> RusotoFuture<DisablePolicyTypeResponse, DisablePolicyTypeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DisablePolicyType",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DisablePolicyTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DisablePolicyTypeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Enables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you enable integration, you allow the specified service to create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in all the accounts in your organization. This allows the service to perform operations on your behalf in your organization and its accounts.</p> <important> <p>We recommend that you enable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the service is aware that it can create the resources that are required for the integration. How the service creates those resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>For more information about enabling services to integrate with AWS Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account and only if the organization has <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">enabled all features</a>.</p>
    fn enable_aws_service_access(
        &self,
        input: EnableAWSServiceAccessRequest,
    ) -> RusotoFuture<(), EnableAWSServiceAccessError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.EnableAWSServiceAccess",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(EnableAWSServiceAccessError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Enables all features in an organization. This enables the use of organization policies that can restrict the services and actions that can be called in each account. Until you enable all features, you have access only to consolidated billing, and you can't use any of the advanced account administration features that AWS Organizations supports. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p> <important> <p>This operation is required only for organizations that were created explicitly with only the consolidated billing features enabled. Calling this operation sends a handshake to every invited account in the organization. The feature set change can be finalized and the additional features enabled only after all administrators in the invited accounts approve the change by accepting the handshake.</p> </important> <p>After you enable all features, you can separately enable or disable individual policy types in a root using <a>EnablePolicyType</a> and <a>DisablePolicyType</a>. To see the status of policy types in a root, use <a>ListRoots</a>.</p> <p>After all invited member accounts accept the handshake, you finalize the feature set change by accepting the handshake that contains <code>"Action": "ENABLE_ALL_FEATURES"</code>. This completes the change.</p> <p>After you enable all features in your organization, the master account in the organization can apply policies on all member accounts. These policies can restrict what users and even administrators in those accounts can do. The master account can apply policies that prevent accounts from leaving the organization. Ensure that your account administrators are aware of this.</p> <p>This operation can be called only from the organization's master account. </p>
    fn enable_all_features(
        &self,
    ) -> RusotoFuture<EnableAllFeaturesResponse, EnableAllFeaturesError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.EnableAllFeatures",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<EnableAllFeaturesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(EnableAllFeaturesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Enables a policy type in a root. After you enable a policy type in a root, you can attach policies of that type to the root, any organizational unit (OU), or account in that root. You can undo this by using the <a>DisablePolicyType</a> operation.</p> <p>This operation can be called only from the organization's master account.</p> <p>You can enable a policy type in a root only if that policy type is available in the organization. Use <a>DescribeOrganization</a> to view the status of available policy types in the organization.</p> <p>To view the status of policy type in a root, use <a>ListRoots</a>.</p>
    fn enable_policy_type(
        &self,
        input: EnablePolicyTypeRequest,
    ) -> RusotoFuture<EnablePolicyTypeResponse, EnablePolicyTypeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.EnablePolicyType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<EnablePolicyTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(EnablePolicyTypeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sends an invitation to another account to join your organization as a member account. Organizations sends email on your behalf to the email address that is associated with the other account's owner. The invitation is implemented as a <a>Handshake</a> whose details are in the response.</p> <important> <ul> <li> <p>You can invite AWS accounts only from the same seller as the master account. For example, if your organization's master account was created by Amazon Internet Services Pvt. Ltd (AISPL), an AWS seller in India, then you can only invite other AISPL accounts to your organization. You can't combine accounts from AISPL and AWS, or any other AWS seller. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/useconsolidatedbilliing-India.html">Consolidated Billing in India</a>.</p> </li> <li> <p>If you receive an exception that indicates that you exceeded your account limits for the organization or that the operation failed because your organization is still initializing, wait one hour and then try again. If the error persists after an hour, then contact <a href="https://console.aws.amazon.com/support/home#/">AWS Customer Support</a>.</p> </li> </ul> </important> <p>This operation can be called only from the organization's master account.</p>
    fn invite_account_to_organization(
        &self,
        input: InviteAccountToOrganizationRequest,
    ) -> RusotoFuture<InviteAccountToOrganizationResponse, InviteAccountToOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.InviteAccountToOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<InviteAccountToOrganizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(InviteAccountToOrganizationError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Removes a member account from its parent organization. This version of the operation is performed by the account that wants to leave. To remove a member account as a user in the master account, use <a>RemoveAccountFromOrganization</a> instead.</p> <p>This operation can be called only from a member account in the organization.</p> <important> <ul> <li> <p>The master account in an organization with all features enabled can set service control policies (SCPs) that can restrict what administrators of member accounts can do, including preventing them from successfully calling <code>LeaveOrganization</code> and leaving the organization. </p> </li> <li> <p>You can leave an organization as a member account only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For each account that you want to make standalone, you must accept the End User License Agreement (EULA), choose a support plan, provide and verify the required contact information, and provide a current payment method. AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account is not attached to an organization. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>You can leave an organization only after you enable IAM user access to billing in your account. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> </li> </ul> </important></p>
    fn leave_organization(&self) -> RusotoFuture<(), LeaveOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.LeaveOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(LeaveOrganizationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of the AWS services that you enabled to integrate with your organization. After a service on this list creates the resources that it requires for the integration, it can perform operations on your organization and its accounts.</p> <p>For more information about integrating other services with AWS Organizations, including the list of services that currently work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn list_aws_service_access_for_organization(
        &self,
        input: ListAWSServiceAccessForOrganizationRequest,
    ) -> RusotoFuture<
        ListAWSServiceAccessForOrganizationResponse,
        ListAWSServiceAccessForOrganizationError,
    > {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListAWSServiceAccessForOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListAWSServiceAccessForOrganizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListAWSServiceAccessForOrganizationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists all the accounts in the organization. To request only the accounts in a specified root or organizational unit (OU), use the <a>ListAccountsForParent</a> operation instead.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> RusotoFuture<ListAccountsResponse, ListAccountsError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListAccounts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListAccountsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAccountsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the accounts in an organization that are contained by the specified target root or organizational unit (OU). If you specify the root, you get a list of all the accounts that are not in any OU. If you specify an OU, you get a list of all the accounts in only that OU, and not in any child OUs. To get a list of all accounts in the organization, use the <a>ListAccounts</a> operation.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_accounts_for_parent(
        &self,
        input: ListAccountsForParentRequest,
    ) -> RusotoFuture<ListAccountsForParentResponse, ListAccountsForParentError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListAccountsForParent",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListAccountsForParentResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListAccountsForParentError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists all of the organizational units (OUs) or accounts that are contained in the specified parent OU or root. This operation, along with <a>ListParents</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_children(
        &self,
        input: ListChildrenRequest,
    ) -> RusotoFuture<ListChildrenResponse, ListChildrenError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListChildren");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListChildrenResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListChildrenError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the account creation requests that match the specified status that is currently being tracked for the organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_create_account_status(
        &self,
        input: ListCreateAccountStatusRequest,
    ) -> RusotoFuture<ListCreateAccountStatusResponse, ListCreateAccountStatusError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListCreateAccountStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListCreateAccountStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListCreateAccountStatusError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the current handshakes that are associated with the account of the requesting user.</p> <p>Handshakes that are ACCEPTED, DECLINED, or CANCELED appear in the results of this API for only 30 days after changing to that state. After that they are deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called from any account in the organization.</p>
    fn list_handshakes_for_account(
        &self,
        input: ListHandshakesForAccountRequest,
    ) -> RusotoFuture<ListHandshakesForAccountResponse, ListHandshakesForAccountError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListHandshakesForAccount",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListHandshakesForAccountResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListHandshakesForAccountError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the handshakes that are associated with the organization that the requesting user is part of. The <code>ListHandshakesForOrganization</code> operation returns a list of handshake structures. Each structure contains details and status about a handshake.</p> <p>Handshakes that are ACCEPTED, DECLINED, or CANCELED appear in the results of this API for only 30 days after changing to that state. After that they are deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_handshakes_for_organization(
        &self,
        input: ListHandshakesForOrganizationRequest,
    ) -> RusotoFuture<ListHandshakesForOrganizationResponse, ListHandshakesForOrganizationError>
    {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListHandshakesForOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListHandshakesForOrganizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListHandshakesForOrganizationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the organizational units (OUs) in a parent organizational unit or root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_organizational_units_for_parent(
        &self,
        input: ListOrganizationalUnitsForParentRequest,
    ) -> RusotoFuture<ListOrganizationalUnitsForParentResponse, ListOrganizationalUnitsForParentError>
    {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListOrganizationalUnitsForParent",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListOrganizationalUnitsForParentResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListOrganizationalUnitsForParentError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Lists the root or organizational units (OUs) that serve as the immediate parent of the specified child OU or account. This operation, along with <a>ListChildren</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>In the current release, a child can have only a single parent. </p> </note></p>
    fn list_parents(
        &self,
        input: ListParentsRequest,
    ) -> RusotoFuture<ListParentsResponse, ListParentsError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListParents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListParentsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListParentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the list of all policies in an organization of a specified type.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_policies(
        &self,
        input: ListPoliciesRequest,
    ) -> RusotoFuture<ListPoliciesResponse, ListPoliciesError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListPolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListPoliciesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPoliciesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the policies that are directly attached to the specified target root, organizational unit (OU), or account. You must specify the policy type that you want included in the returned list.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_policies_for_target(
        &self,
        input: ListPoliciesForTargetRequest,
    ) -> RusotoFuture<ListPoliciesForTargetResponse, ListPoliciesForTargetError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListPoliciesForTarget",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListPoliciesForTargetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListPoliciesForTargetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Lists the roots that are defined in the current organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>Policy types can be enabled and disabled in roots. This is distinct from whether they are available in the organization. When you enable all features, you make policy types available for use in that organization. Individual policy types can then be enabled and disabled in a root. To see the availability of a policy type in an organization, use <a>DescribeOrganization</a>.</p> </note></p>
    fn list_roots(
        &self,
        input: ListRootsRequest,
    ) -> RusotoFuture<ListRootsResponse, ListRootsError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListRoots");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListRootsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListRootsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all the roots, organizaitonal units (OUs), and accounts to which the specified policy is attached.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_targets_for_policy(
        &self,
        input: ListTargetsForPolicyRequest,
    ) -> RusotoFuture<ListTargetsForPolicyResponse, ListTargetsForPolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListTargetsForPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListTargetsForPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTargetsForPolicyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Moves an account from its current source parent root or organizational unit (OU) to the specified destination parent root or OU.</p> <p>This operation can be called only from the organization's master account.</p>
    fn move_account(&self, input: MoveAccountRequest) -> RusotoFuture<(), MoveAccountError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.MoveAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(MoveAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Removes the specified account from the organization.</p> <p>The removed account becomes a stand-alone account that is not a member of any organization. It is no longer subject to any policies and is responsible for its own bill payments. The organization&#39;s master account is no longer charged for any expenses accrued by the member account after it is removed from the organization.</p> <p>This operation can be called only from the organization&#39;s master account. Member accounts can remove themselves with <a>LeaveOrganization</a> instead.</p> <important> <p>You can remove an account from your organization only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For an account that you want to make standalone, you must accept the End User License Agreement (EULA), choose a support plan, provide and verify the required contact information, and provide a current payment method. AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account is not attached to an organization. To remove an account that does not yet have this information, you must sign in as the member account and follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </important></p>
    fn remove_account_from_organization(
        &self,
        input: RemoveAccountFromOrganizationRequest,
    ) -> RusotoFuture<(), RemoveAccountFromOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.RemoveAccountFromOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RemoveAccountFromOrganizationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Renames the specified organizational unit (OU). The ID and ARN do not change. The child OUs and accounts remain in place, and any attached policies of the OU remain attached. </p> <p>This operation can be called only from the organization's master account.</p>
    fn update_organizational_unit(
        &self,
        input: UpdateOrganizationalUnitRequest,
    ) -> RusotoFuture<UpdateOrganizationalUnitResponse, UpdateOrganizationalUnitError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.UpdateOrganizationalUnit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<UpdateOrganizationalUnitResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateOrganizationalUnitError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates an existing policy with a new name, description, or content. If any parameter is not supplied, that value remains unchanged. Note that you cannot change a policy's type.</p> <p>This operation can be called only from the organization's master account.</p>
    fn update_policy(
        &self,
        input: UpdatePolicyRequest,
    ) -> RusotoFuture<UpdatePolicyResponse, UpdatePolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.UpdatePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<UpdatePolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdatePolicyError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
