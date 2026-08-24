use datom::{
    DatomFault, DatomProblem, DatomRealizing, DatomRoot, DatomTextualizing, PositionAdvancing,
    RecordPosition,
};
use protos::{
    Block, Head, Headed, RealizeScope, RealizeScoping, Shape, ShapeDefined, TextualizeScope,
    TextualizeScoping,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Roles {
    modules: Vec<RoleModule>,
    models: Vec<Model>,
    permissions: Vec<RolePermission>,
    depths: Vec<RoleDepth>,
    descriptions: Vec<RoleDescription>,
    aliases: Vec<RoleAlias>,
    universal_modules: Vec<String>,
    target_insertions: Vec<TargetInsertion>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RoleModule {
    identifier: String,
    body: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Model {
    identifier: String,
    provider: Provider,
    efforts: Vec<Effort>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct RolePermission {
    discipline: String,
    body: String,
    permission: Permission,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct RoleDepth {
    depth: String,
    claude: ModelChoice,
    chat_gpt: ModelChoice,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct ModelChoice {
    model: String,
    effort: Option<Effort>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct RoleDescription {
    discipline: String,
    depth: String,
    body: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct RoleAlias {
    identifier: String,
    discipline: String,
    depth: String,
    body: String,
    surfaces: Vec<Surface>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct TargetInsertion {
    module: String,
    surface: Surface,
    insertions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Provider {
    Claude,
    ChatGpt,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Permission {
    Restricted,
    Unrestricted,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Effort {
    Low,
    Medium,
    High,
    Xhigh,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Surface {
    Claude,
    Codex,
    Pi,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RolePacket {
    pub path: String,
    pub text: String,
}

fn fault(problem: DatomProblem) -> DatomFault {
    DatomFault { problem }
}
fn record(block: &Block) -> Result<(), DatomFault> {
    (block.shape == Shape::Braced && block.head().is_none())
        .then_some(())
        .ok_or_else(|| fault(DatomProblem::Shape))
}
fn token(block: &Block) -> Result<String, DatomFault> {
    (block.shape == Shape::Bare && block.head().is_none())
        .then_some(block.body.0.clone())
        .ok_or_else(|| fault(DatomProblem::Shape))
}
fn parse_provider(value: String) -> Result<Provider, DatomFault> {
    match value.as_str() {
        "Claude" => Ok(Provider::Claude),
        "ChatGpt" => Ok(Provider::ChatGpt),
        _ => Err(fault(DatomProblem::Value)),
    }
}
fn parse_permission(value: String) -> Result<Permission, DatomFault> {
    match value.as_str() {
        "Restricted" => Ok(Permission::Restricted),
        "Unrestricted" => Ok(Permission::Unrestricted),
        _ => Err(fault(DatomProblem::Value)),
    }
}
fn parse_effort(value: &str) -> Result<Effort, DatomFault> {
    match value {
        "Low" => Ok(Effort::Low),
        "Medium" => Ok(Effort::Medium),
        "High" => Ok(Effort::High),
        "Xhigh" => Ok(Effort::Xhigh),
        _ => Err(fault(DatomProblem::Value)),
    }
}
fn parse_surface(value: String) -> Result<Surface, DatomFault> {
    match value.as_str() {
        "ClaudeAgent" => Ok(Surface::Claude),
        "CodexAgent" => Ok(Surface::Codex),
        "PiAgent" => Ok(Surface::Pi),
        _ => Err(fault(DatomProblem::Value)),
    }
}
fn effort_text(effort: Effort) -> &'static str {
    match effort {
        Effort::Low => "Low",
        Effort::Medium => "Medium",
        Effort::High => "High",
        Effort::Xhigh => "Xhigh",
    }
}
fn effort_lower(effort: Effort) -> &'static str {
    match effort {
        Effort::Low => "low",
        Effort::Medium => "medium",
        Effort::High => "high",
        Effort::Xhigh => "xhigh",
    }
}
fn surface_text(surface: Surface) -> &'static str {
    match surface {
        Surface::Claude => "ClaudeAgent",
        Surface::Codex => "CodexAgent",
        Surface::Pi => "PiAgent",
    }
}

impl ShapeDefined for Roles {
    type Selection = ();
    fn shapes() -> &'static [Shape] {
        &[Shape::DottedBraced]
    }
    fn select(shape: Shape, head: Option<&Head>) -> Option<Self::Selection> {
        (shape == Shape::DottedBraced && head == Some(&Head("Roles".into()))).then_some(())
    }
}
impl DatomRoot for Roles {
    fn root_head() -> Head {
        Head("Roles".into())
    }
}
impl DatomRealizing for Roles {
    fn realize_block(scope: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
        if Self::select(block.shape, block.head()).is_none() {
            return Err(fault(DatomProblem::Shape));
        }
        let mut position = RecordPosition::default();
        let (
            mut modules,
            mut models,
            mut permissions,
            mut depths,
            mut descriptions,
            mut aliases,
            mut universal_modules,
            mut target_insertions,
        ) = (None, None, None, None, None, None, None, None);
        scope.realize_body(&mut |child, value| {
            match position.next_position() {
                0 => modules = Some(Vec::<RoleModule>::realize_block(child, value)?),
                1 => models = Some(Vec::<Model>::realize_block(child, value)?),
                2 => permissions = Some(Vec::<RolePermission>::realize_block(child, value)?),
                3 => depths = Some(Vec::<RoleDepth>::realize_block(child, value)?),
                4 => descriptions = Some(Vec::<RoleDescription>::realize_block(child, value)?),
                5 => aliases = Some(Vec::<RoleAlias>::realize_block(child, value)?),
                6 => universal_modules = Some(Vec::<String>::realize_block(child, value)?),
                7 => target_insertions = Some(Vec::<TargetInsertion>::realize_block(child, value)?),
                _ => return Err(fault(DatomProblem::ExtraPosition)),
            };
            Ok(())
        })?;
        Ok(Self {
            modules: modules.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            models: models.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            permissions: permissions.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            depths: depths.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            descriptions: descriptions.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            aliases: aliases.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            universal_modules: universal_modules
                .ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            target_insertions: target_insertions
                .ok_or_else(|| fault(DatomProblem::MissingPosition))?,
        })
    }
}
impl DatomTextualizing for Roles {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        self.modules.textualize_in(scope)?;
        self.models.textualize_in(scope)?;
        self.permissions.textualize_in(scope)?;
        self.depths.textualize_in(scope)?;
        self.descriptions.textualize_in(scope)?;
        self.aliases.textualize_in(scope)?;
        self.universal_modules.textualize_in(scope)?;
        self.target_insertions.textualize_in(scope)
    }
}

macro_rules! three_record {
    ($type:ident, $a:ident:$at:ty, $b:ident:$bt:ty, $c:ident:$ct:ty) => {
        impl DatomRealizing for $type {
            fn realize_block(
                scope: &mut RealizeScope<'_>,
                block: &Block,
            ) -> Result<Self, DatomFault> {
                record(block)?;
                let mut position = RecordPosition::default();
                let (mut $a, mut $b, mut $c) = (None, None, None);
                scope.realize_body(&mut |child, value| {
                    match position.next_position() {
                        0 => $a = Some(<$at>::realize_block(child, value)?),
                        1 => $b = Some(<$bt>::realize_block(child, value)?),
                        2 => $c = Some(<$ct>::realize_block(child, value)?),
                        _ => return Err(fault(DatomProblem::ExtraPosition)),
                    };
                    Ok(())
                })?;
                Ok(Self {
                    $a: $a.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
                    $b: $b.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
                    $c: $c.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
                })
            }
        }
        impl DatomTextualizing for $type {
            fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
                scope.textualize_block(Shape::Braced, None, |body| {
                    self.$a.textualize_in(body)?;
                    self.$b.textualize_in(body)?;
                    self.$c.textualize_in(body)
                })
            }
        }
    };
}
macro_rules! two_record {
    ($type:ident, $a:ident:$at:ty, $b:ident:$bt:ty) => {
        impl DatomRealizing for $type {
            fn realize_block(
                scope: &mut RealizeScope<'_>,
                block: &Block,
            ) -> Result<Self, DatomFault> {
                record(block)?;
                let mut position = RecordPosition::default();
                let (mut $a, mut $b) = (None, None);
                scope.realize_body(&mut |child, value| {
                    match position.next_position() {
                        0 => $a = Some(<$at>::realize_block(child, value)?),
                        1 => $b = Some(<$bt>::realize_block(child, value)?),
                        _ => return Err(fault(DatomProblem::ExtraPosition)),
                    };
                    Ok(())
                })?;
                Ok(Self {
                    $a: $a.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
                    $b: $b.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
                })
            }
        }
        impl DatomTextualizing for $type {
            fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
                scope.textualize_block(Shape::Braced, None, |body| {
                    self.$a.textualize_in(body)?;
                    self.$b.textualize_in(body)
                })
            }
        }
    };
}
two_record!(RoleModule, identifier:String, body:String);

macro_rules! bare_enum {
    ($type:ident, $parse:ident, $text:ident) => {
        impl DatomRealizing for $type {
            fn realize_block(_: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
                $parse(token(block)?)
            }
        }
        impl DatomTextualizing for $type {
            fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
                let value = $text(*self);
                scope.textualize_block(Shape::Bare, None, |body| {
                    body.emit_scalar(value);
                    Ok(())
                })
            }
        }
    };
}
fn provider_text(value: Provider) -> &'static str {
    match value {
        Provider::Claude => "Claude",
        Provider::ChatGpt => "ChatGpt",
    }
}
fn permission_text(value: Permission) -> &'static str {
    match value {
        Permission::Restricted => "Restricted",
        Permission::Unrestricted => "Unrestricted",
    }
}
bare_enum!(Provider, parse_provider, provider_text);
bare_enum!(Permission, parse_permission, permission_text);
impl DatomRealizing for Effort {
    fn realize_block(_: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
        parse_effort(&token(block)?)
    }
}
impl DatomTextualizing for Effort {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        scope.textualize_block(Shape::Bare, None, |body| {
            body.emit_scalar(effort_text(*self));
            Ok(())
        })
    }
}
impl DatomRealizing for Surface {
    fn realize_block(_: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
        parse_surface(token(block)?)
    }
}
impl DatomTextualizing for Surface {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        scope.textualize_block(Shape::Bare, None, |body| {
            body.emit_scalar(surface_text(*self));
            Ok(())
        })
    }
}

impl DatomRealizing for Model {
    fn realize_block(scope: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
        record(block)?;
        let mut p = RecordPosition::default();
        let (mut identifier, mut provider, mut efforts) = (None, None, None);
        scope.realize_body(&mut |child, value| {
            match p.next_position() {
                0 => identifier = Some(String::realize_block(child, value)?),
                1 => provider = Some(Provider::realize_block(child, value)?),
                2 => efforts = Some(Vec::<Effort>::realize_block(child, value)?),
                _ => return Err(fault(DatomProblem::ExtraPosition)),
            };
            Ok(())
        })?;
        Ok(Self {
            identifier: identifier.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            provider: provider.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            efforts: efforts.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
        })
    }
}
impl DatomTextualizing for Model {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        scope.textualize_block(Shape::Braced, None, |body| {
            self.identifier.textualize_in(body)?;
            self.provider.textualize_in(body)?;
            self.efforts.textualize_in(body)
        })
    }
}
impl DatomRealizing for ModelChoice {
    fn realize_block(scope: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
        record(block)?;
        let mut p = RecordPosition::default();
        let (mut model, mut effort) = (None, None);
        scope.realize_body(&mut |child, value| {
            match p.next_position() {
                0 => model = Some(String::realize_block(child, value)?),
                1 => effort = Some(OptionEffort::realize_block(child, value)?.0),
                _ => return Err(fault(DatomProblem::ExtraPosition)),
            };
            Ok(())
        })?;
        Ok(Self {
            model: model.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            effort: effort.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
        })
    }
}
impl DatomTextualizing for ModelChoice {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        scope.textualize_block(Shape::Braced, None, |body| {
            self.model.textualize_in(body)?;
            OptionEffort(self.effort).textualize_in(body)
        })
    }
}
struct OptionEffort(Option<Effort>);
impl DatomRealizing for OptionEffort {
    fn realize_block(_: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
        let value = token(block)?;
        if value == "None" {
            return Ok(Self(None));
        }
        let Some(effort) = value.strip_prefix("Some.") else {
            return Err(fault(DatomProblem::Value));
        };
        Ok(Self(Some(parse_effort(effort)?)))
    }
}
impl DatomTextualizing for OptionEffort {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        let value = match self.0 {
            None => "None".into(),
            Some(effort) => format!("Some.{}", effort_text(effort)),
        };
        scope.textualize_block(Shape::Bare, None, |body| {
            body.emit_scalar(&value);
            Ok(())
        })
    }
}
three_record!(RolePermission, discipline:String, body:String, permission:Permission);
three_record!(RoleDescription, discipline:String, depth:String, body:String);
impl DatomRealizing for RoleDepth {
    fn realize_block(scope: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
        record(block)?;
        let mut p = RecordPosition::default();
        let (mut depth, mut claude, mut chat_gpt) = (None, None, None);
        scope.realize_body(&mut |child, value| {
            match p.next_position() {
                0 => depth = Some(String::realize_block(child, value)?),
                1 => claude = Some(ModelChoice::realize_block(child, value)?),
                2 => chat_gpt = Some(ModelChoice::realize_block(child, value)?),
                _ => return Err(fault(DatomProblem::ExtraPosition)),
            };
            Ok(())
        })?;
        Ok(Self {
            depth: depth.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            claude: claude.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            chat_gpt: chat_gpt.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
        })
    }
}
impl DatomTextualizing for RoleDepth {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        scope.textualize_block(Shape::Braced, None, |body| {
            self.depth.textualize_in(body)?;
            self.claude.textualize_in(body)?;
            self.chat_gpt.textualize_in(body)
        })
    }
}
impl DatomRealizing for RoleAlias {
    fn realize_block(scope: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
        record(block)?;
        let mut p = RecordPosition::default();
        let (mut identifier, mut discipline, mut depth, mut body, mut surfaces) =
            (None, None, None, None, None);
        scope.realize_body(&mut |child, value| {
            match p.next_position() {
                0 => identifier = Some(String::realize_block(child, value)?),
                1 => discipline = Some(String::realize_block(child, value)?),
                2 => depth = Some(String::realize_block(child, value)?),
                3 => body = Some(String::realize_block(child, value)?),
                4 => surfaces = Some(Vec::<Surface>::realize_block(child, value)?),
                _ => return Err(fault(DatomProblem::ExtraPosition)),
            };
            Ok(())
        })?;
        Ok(Self {
            identifier: identifier.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            discipline: discipline.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            depth: depth.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            body: body.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            surfaces: surfaces.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
        })
    }
}
impl DatomTextualizing for RoleAlias {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        scope.textualize_block(Shape::Braced, None, |body| {
            self.identifier.textualize_in(body)?;
            self.discipline.textualize_in(body)?;
            self.depth.textualize_in(body)?;
            self.body.textualize_in(body)?;
            self.surfaces.textualize_in(body)
        })
    }
}
impl DatomRealizing for TargetInsertion {
    fn realize_block(scope: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
        record(block)?;
        let mut p = RecordPosition::default();
        let (mut module, mut surface, mut insertions) = (None, None, None);
        scope.realize_body(&mut |child, value| {
            match p.next_position() {
                0 => module = Some(String::realize_block(child, value)?),
                1 => surface = Some(Surface::realize_block(child, value)?),
                2 => insertions = Some(Vec::<String>::realize_block(child, value)?),
                _ => return Err(fault(DatomProblem::ExtraPosition)),
            };
            Ok(())
        })?;
        Ok(Self {
            module: module.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            surface: surface.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            insertions: insertions.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
        })
    }
}
impl DatomTextualizing for TargetInsertion {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        scope.textualize_block(Shape::Braced, None, |body| {
            self.module.textualize_in(body)?;
            self.surface.textualize_in(body)?;
            self.insertions.textualize_in(body)
        })
    }
}

impl Roles {
    pub fn packets(&self) -> Result<Vec<RolePacket>, String> {
        let mut packets = Vec::new();
        for description in &self.descriptions {
            for surface in [Surface::Claude, Surface::Codex, Surface::Pi] {
                packets.push(self.packet(
                    &format!("{}-{}", description.discipline, description.depth),
                    &description.discipline,
                    &description.depth,
                    &description.body,
                    surface,
                )?);
            }
        }
        for alias in &self.aliases {
            for surface in &alias.surfaces {
                packets.push(self.packet(
                    &alias.identifier,
                    &alias.discipline,
                    &alias.depth,
                    &alias.body,
                    *surface,
                )?);
            }
        }
        Ok(packets)
    }
    fn packet(
        &self,
        identifier: &str,
        discipline: &str,
        depth: &str,
        description: &str,
        surface: Surface,
    ) -> Result<RolePacket, String> {
        let permission = self
            .permissions
            .iter()
            .find(|entry| entry.discipline == discipline)
            .ok_or_else(|| format!("missing permission {discipline}"))?;
        let depth = self
            .depths
            .iter()
            .find(|entry| entry.depth == depth)
            .ok_or_else(|| format!("missing depth {depth}"))?;
        let choice = match surface {
            Surface::Claude => &depth.claude,
            Surface::Codex | Surface::Pi => &depth.chat_gpt,
        };
        let model = self
            .models
            .iter()
            .find(|entry| entry.identifier == choice.model)
            .ok_or_else(|| format!("missing model {}", choice.model))?;
        let expected = match surface {
            Surface::Claude => Provider::Claude,
            Surface::Codex | Surface::Pi => Provider::ChatGpt,
        };
        if model.provider != expected
            || choice
                .effort
                .is_some_and(|effort| !model.efforts.contains(&effort))
        {
            return Err(format!("invalid model choice {}", choice.model));
        }
        let mut modules = Vec::new();
        if permission.permission == Permission::Restricted {
            modules.push(permission.body.clone());
        }
        for identifier in &self.universal_modules {
            modules.push(self.module(identifier)?);
        }
        for insertion in self.target_insertions.iter().filter(|entry| {
            entry.surface == surface && self.universal_modules.contains(&entry.module)
        }) {
            for identifier in &insertion.insertions {
                modules.push(self.module(identifier)?);
            }
        }
        let body = modules.join("\n\n");
        let effort = choice.effort.map(effort_lower).unwrap_or("low");
        let (path, text) = match surface {
            Surface::Claude => (
                format!(".claude/agents/{identifier}.md"),
                format!(
                    "---\nname: {identifier}\ndescription: '{}'\nmodel: '{}'\neffort: {effort}\n---\n\n{body}\n",
                    description.replace('\'', "''"),
                    choice.model
                ),
            ),
            Surface::Codex => (
                format!(".codex/agents/{identifier}.toml"),
                format!(
                    "name = \"{identifier}\"\ndescription = \"{}\"\nmodel = \"{}\"\nmodel_reasoning_effort = \"{effort}\"\ndeveloper_instructions = \"{}\"\n",
                    description.replace('"', "\\\"").replace('\n', "\\n"),
                    choice.model,
                    body.replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('\n', "\\n")
                ),
            ),
            Surface::Pi => {
                let restriction = if permission.permission == Permission::Restricted {
                    "disallowed_tools: 'edit, write'\n"
                } else {
                    ""
                };
                (
                    format!(".pi/agents/{identifier}.md"),
                    format!(
                        "---\nname: {identifier}\ndescription: '{}'\nmodel: 'openai-codex/{}'\nthinking: {effort}\nprojectRoleIdentity: {identifier}\nprojectRoleDispatchKind: leaf\n{restriction}---\n\n{body}\n",
                        description.replace('\'', "''"),
                        choice.model
                    ),
                )
            }
        };
        Ok(RolePacket { path, text })
    }
    fn module(&self, identifier: &str) -> Result<String, String> {
        self.modules
            .iter()
            .find(|module| module.identifier == identifier)
            .map(|module| module.body.clone())
            .ok_or_else(|| format!("missing role module {identifier}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datom::{DatomRoot, DatomText};
    use protos::Realize;
    #[test]
    fn empty_roles_round_trip_through_the_typed_schema() {
        let roles = Roles {
            modules: vec![],
            models: vec![],
            permissions: vec![],
            depths: vec![],
            descriptions: vec![],
            aliases: vec![],
            universal_modules: vec![],
            target_insertions: vec![],
        };
        let canonical = roles.textualize_source().expect("canonical");
        assert_eq!(
            DatomText::<Roles>::from(canonical)
                .realize()
                .expect("canonical roles"),
            roles
        );
        assert!(roles.packets().expect("packets").is_empty());
    }
}
