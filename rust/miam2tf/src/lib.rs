pub mod loader;
pub mod mruby;
mod mruby_c;
pub mod printer;

#[derive(Debug, serde::Serialize)]
pub struct Miam {
    pub users: Vec<User>,
    pub groups: Vec<Group>,
    pub roles: Vec<Role>,
    pub managed_policies: Vec<ManagedPolicy>,
    pub instance_profiles: Vec<InstanceProfile>,
}

#[derive(Debug, serde::Serialize)]
pub struct User {
    pub user_name: String,
    pub path: Option<String>,
    pub policies: Vec<PolicyDocument>,
    pub groups: Vec<String>,
    pub attached_managed_policies: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct PolicyDocument {
    pub name: String,
    pub version: Option<String>,
    pub statements: Vec<PolicyStatement>,
}

#[derive(Debug, serde::Serialize)]
pub struct PolicyStatement {
    pub effect: String,
    pub actions: Vec<String>,
    pub resources: Vec<String>,
    pub conditions: Vec<PolicyCondition>,
}

#[derive(Debug, serde::Serialize)]
pub struct PolicyCondition {
    pub test: String,
    pub variable: String,
    pub values: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct Group {
    pub name: String,
    pub path: Option<String>,
    pub policies: Vec<PolicyDocument>,
    pub attached_managed_policies: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct Role {
    pub name: String,
    pub path: Option<String>,
    pub assume_role_policy_document: Option<PolicyDocument>,
    pub policies: Vec<PolicyDocument>,
    pub attached_managed_policies: Vec<String>,
    pub instance_profiles: Vec<String>,
    pub max_session_duration: Option<i64>,
}

#[derive(Debug, serde::Serialize)]
pub struct ManagedPolicy {
    pub name: String,
    pub path: Option<String>,
    pub policy_document: PolicyDocument,
}

#[derive(Debug, serde::Serialize)]
pub struct InstanceProfile {
    pub name: String,
    pub path: Option<String>,
}
