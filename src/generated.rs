#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Configuration {
    pub first_string: String,
    pub second_string: String,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Request {
    Generate(Configuration),
    Check(Configuration),
    Visualize(Configuration),
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Generated_Data {
    pub first_integer: i64,
    pub second_integer: i64,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Checked_Data {
    pub first_integer: i64,
    pub second_integer: i64,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Visualized_Data {
    pub first_integer: i64,
    pub second_integer: i64,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Output {
    Generated(Generated_Data),
    Checked(Checked_Data),
    Visualized(Visualized_Data),
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct GeneratedRoleOutputs {
    pub string_vector: std::vec::Vec<String>,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum GeneratedRoleOutputDocument {
    GeneratedRoleOutputs(GeneratedRoleOutputs),
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Provider {
    Claude,
    ChatGpt,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Permission {
    Restricted,
    Unrestricted,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Effort {
    Low,
    Medium,
    High,
    Xhigh,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Surface {
    ClaudeAgent,
    CodexAgent,
    PiAgent,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct ModelChoice {
    pub string: String,
    pub effort_option: Option<Effort>,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct RoleModule {
    pub first_string: String,
    pub second_string: String,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Model {
    pub string: String,
    pub provider: Provider,
    pub effort_vector: std::vec::Vec<Effort>,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct RolePermission {
    pub first_string: String,
    pub second_string: String,
    pub permission: Permission,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct RoleDepth {
    pub string: String,
    pub first_model_choice: ModelChoice,
    pub second_model_choice: ModelChoice,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct RoleDescription {
    pub first_string: String,
    pub second_string: String,
    pub third_string: String,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct RoleAlias {
    pub first_string: String,
    pub second_string: String,
    pub third_string: String,
    pub fourth_string: String,
    pub surface_vector: std::vec::Vec<Surface>,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct TargetInsertion {
    pub string: String,
    pub surface: Surface,
    pub string_vector: std::vec::Vec<String>,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
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
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum RolesDocument {
    Roles(Roles),
}
