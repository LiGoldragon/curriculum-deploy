#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct Configuration {
    pub first_string: String,
    pub second_string: String,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub enum Request {
    Generate(Configuration),
    Check(Configuration),
    Visualize(Configuration),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct Generated_Data {
    pub first_integer: i64,
    pub second_integer: i64,
}
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct Checked_Data {
    pub first_integer: i64,
    pub second_integer: i64,
}
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct Visualized_Data {
    pub first_integer: i64,
    pub second_integer: i64,
}
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub enum Output {
    Generated(Generated_Data),
    Checked(Checked_Data),
    Visualized(Visualized_Data),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct GeneratedRoleOutputs {
    pub string_vector: std::vec::Vec<String>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub enum GeneratedRoleOutputDocument {
    GeneratedRoleOutputs(GeneratedRoleOutputs),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub enum Provider {
    Claude,
    ChatGpt,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub enum Permission {
    Restricted,
    Unrestricted,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub enum Effort {
    Low,
    Medium,
    High,
    Xhigh,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub enum Surface {
    ClaudeAgent,
    CodexAgent,
    PiAgent,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct ModelChoice {
    pub string: String,
    pub effort_option: Option<Effort>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct RoleModule {
    pub first_string: String,
    pub second_string: String,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct Model {
    pub string: String,
    pub provider: Provider,
    pub effort_vector: std::vec::Vec<Effort>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct RolePermission {
    pub first_string: String,
    pub second_string: String,
    pub permission: Permission,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct RoleDepth {
    pub string: String,
    pub first_model_choice: ModelChoice,
    pub second_model_choice: ModelChoice,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct RoleDescription {
    pub first_string: String,
    pub second_string: String,
    pub third_string: String,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct RoleAlias {
    pub first_string: String,
    pub second_string: String,
    pub third_string: String,
    pub fourth_string: String,
    pub surface_vector: std::vec::Vec<Surface>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct TargetInsertion {
    pub string: String,
    pub surface: Surface,
    pub string_vector: std::vec::Vec<String>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct Roles {
    pub role_module_vector: std::vec::Vec<RoleModule>,
    pub model_vector: std::vec::Vec<Model>,
    pub role_permission_vector: std::vec::Vec<RolePermission>,
    pub role_depth_vector: std::vec::Vec<RoleDepth>,
    pub role_description_vector: std::vec::Vec<RoleDescription>,
    pub role_alias_vector: std::vec::Vec<RoleAlias>,
    pub string_vector: std::vec::Vec<String>,
    pub target_insertion_vector: std::vec::Vec<TargetInsertion>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub enum RolesDocument {
    Roles(Roles),
}
