use crate::generated::{Effort, Permission, Provider, Roles, Surface};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RolePacket {
    pub path: String,
    pub text: String,
}

trait EffortRendering {
    fn lower(&self) -> &'static str;
    fn same(&self, other: &Self) -> bool;
}

impl EffortRendering for Effort {
    fn lower(&self) -> &'static str {
        match self {
            Effort::Low => "low",
            Effort::Medium => "medium",
            Effort::High => "high",
            Effort::Xhigh => "xhigh",
        }
    }

    fn same(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Low, Self::Low)
                | (Self::Medium, Self::Medium)
                | (Self::High, Self::High)
                | (Self::Xhigh, Self::Xhigh)
        )
    }
}

trait ProviderMatching {
    fn matches_surface(&self, surface: &Surface) -> bool;
}

impl ProviderMatching for Provider {
    fn matches_surface(&self, surface: &Surface) -> bool {
        matches!(
            (self, surface),
            (Self::Claude, Surface::ClaudeAgent)
                | (Self::ChatGpt, Surface::CodexAgent | Surface::PiAgent)
        )
    }
}

trait PermissionViewing {
    fn restricted(&self) -> bool;
}

impl PermissionViewing for Permission {
    fn restricted(&self) -> bool {
        matches!(self, Self::Restricted)
    }
}

trait SurfaceMatching {
    fn same(&self, other: &Self) -> bool;
}

impl SurfaceMatching for Surface {
    fn same(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::ClaudeAgent, Self::ClaudeAgent)
                | (Self::CodexAgent, Self::CodexAgent)
                | (Self::PiAgent, Self::PiAgent)
        )
    }
}

impl Roles {
    pub fn packets(&self) -> Result<Vec<RolePacket>, String> {
        let mut packets = Vec::new();
        for description in &self.role_description_vector {
            for surface in [Surface::ClaudeAgent, Surface::CodexAgent, Surface::PiAgent] {
                packets.push(self.packet(
                    &format!("{}-{}", description.first_string, description.second_string),
                    &description.first_string,
                    &description.second_string,
                    &description.third_string,
                    &surface,
                )?);
            }
        }
        for alias in &self.role_alias_vector {
            for surface in &alias.surface_vector {
                packets.push(self.packet(
                    &alias.first_string,
                    &alias.second_string,
                    &alias.third_string,
                    &alias.fourth_string,
                    surface,
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
        surface: &Surface,
    ) -> Result<RolePacket, String> {
        let permission = self
            .role_permission_vector
            .iter()
            .find(|entry| entry.first_string == discipline)
            .ok_or_else(|| format!("missing permission {discipline}"))?;
        let depth_entry = self
            .role_depth_vector
            .iter()
            .find(|entry| entry.string == depth)
            .ok_or_else(|| format!("missing depth {depth}"))?;
        let choice = match surface {
            Surface::ClaudeAgent => &depth_entry.first_model_choice,
            Surface::CodexAgent | Surface::PiAgent => &depth_entry.second_model_choice,
        };
        let model = self
            .model_vector
            .iter()
            .find(|entry| entry.string == choice.string)
            .ok_or_else(|| format!("missing model {}", choice.string))?;
        if !model.provider.matches_surface(surface)
            || choice
                .effort_option
                .as_ref()
                .is_some_and(|effort| !model.effort_vector.iter().any(|item| item.same(effort)))
        {
            return Err(format!("invalid model choice {}", choice.string));
        }
        let mut modules = Vec::<String>::new();
        if permission.permission.restricted() {
            modules.push(permission.second_string.clone());
        }
        for module_id in &self.string_vector {
            modules.push(self.module(module_id)?);
        }
        for insertion in self.target_insertion_vector.iter().filter(|entry| {
            entry.surface.same(surface) && self.string_vector.contains(&entry.string)
        }) {
            for module_id in &insertion.string_vector {
                modules.push(self.module(module_id)?);
            }
        }
        let body = modules.join("\n\n");
        let effort = choice
            .effort_option
            .as_ref()
            .map(EffortRendering::lower)
            .unwrap_or("low");
        let (path, text) = match surface {
            Surface::ClaudeAgent => (
                format!(".claude/agents/{identifier}.md"),
                format!(
                    "---\nname: {identifier}\ndescription: '{}'\nmodel: '{}'\neffort: {effort}\n---\n\n{body}\n",
                    description.replace('\'', "''"),
                    choice.string
                ),
            ),
            Surface::CodexAgent => (
                format!(".codex/agents/{identifier}.toml"),
                format!(
                    "name = \"{identifier}\"\ndescription = \"{}\"\nmodel = \"{}\"\nmodel_reasoning_effort = \"{effort}\"\ndeveloper_instructions = \"{}\"\n",
                    description.replace('"', "\\\"").replace('\n', "\\n"),
                    choice.string,
                    body.replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('\n', "\\n")
                ),
            ),
            Surface::PiAgent => {
                let restriction = if permission.permission.restricted() {
                    "disallowed_tools: 'edit, write'\n"
                } else {
                    ""
                };
                (
                    format!(".pi/agents/{identifier}.md"),
                    format!(
                        "---\nname: {identifier}\ndescription: '{}'\nmodel: 'openai-codex/{}'\nthinking: {effort}\nprojectRoleIdentity: {identifier}\nprojectRoleDispatchKind: leaf\n{restriction}---\n\n{body}\n",
                        description.replace('\'', "''"),
                        choice.string
                    ),
                )
            }
        };
        Ok(RolePacket { path, text })
    }

    fn module(&self, identifier: &str) -> Result<String, String> {
        self.role_module_vector
            .iter()
            .find(|module| module.first_string == identifier)
            .map(|module| module.second_string.clone())
            .ok_or_else(|| format!("missing role module {identifier}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::RolesDocument;
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    #[test]
    fn empty_roles_round_trip_through_the_typed_schema() {
        let roles = Roles {
            role_module_vector: vec![],
            model_vector: vec![],
            role_permission_vector: vec![],
            role_depth_vector: vec![],
            role_description_vector: vec![],
            role_alias_vector: vec![],
            string_vector: vec![],
            target_insertion_vector: vec![],
        };
        assert!(roles.packets().expect("packets").is_empty());
        let text = RolesDocument::Roles(roles)
            .datomize(vec![])
            .protosize()
            .textualize();
        let mut potential = Potential::<RolesDocument>::from(text.clone());
        let round_tripped = potential
            .actualize(&mut Budget {
                remaining: 1_024,
                reader: ReaderBudget { remaining: 1_024 },
                depth: 0,
                maximum_depth: 1_024,
            })
            .expect("canonical roles");
        assert_eq!(
            round_tripped.datomize(vec![]).protosize().textualize(),
            text,
            "round trip changed the roles"
        );
    }
}
