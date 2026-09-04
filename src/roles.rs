use crate::generated::{Effort, Permission, Provider, Roles, Surface};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RolePacket {
    pub path: String,
    pub text: String,
}

trait EffortRendering {
    fn lower(&self) -> &'static str;
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
}

impl Roles {
    pub fn packets(&self) -> Result<Vec<RolePacket>, String> {
        let mut packets = Vec::new();
        for description in &self.4 {
            for surface in [Surface::ClaudeAgent, Surface::CodexAgent, Surface::PiAgent] {
                packets.push(self.packet(
                    &format!("{}-{}", description.0, description.1),
                    &description.0,
                    &description.1,
                    &description.2,
                    surface,
                )?);
            }
        }
        for alias in &self.5 {
            for surface in &alias.4 {
                packets.push(self.packet(&alias.0, &alias.1, &alias.2, &alias.3, *surface)?);
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
            .2
            .iter()
            .find(|entry| entry.0 == discipline)
            .ok_or_else(|| format!("missing permission {discipline}"))?;
        let depth_entry = self
            .3
            .iter()
            .find(|entry| entry.0 == depth)
            .ok_or_else(|| format!("missing depth {depth}"))?;
        let choice = match surface {
            Surface::ClaudeAgent => &depth_entry.1,
            Surface::CodexAgent | Surface::PiAgent => &depth_entry.2,
        };
        let model = self
            .1
            .iter()
            .find(|entry| entry.0 == choice.0)
            .ok_or_else(|| format!("missing model {}", choice.0))?;
        let expected = match surface {
            Surface::ClaudeAgent => Provider::Claude,
            Surface::CodexAgent | Surface::PiAgent => Provider::ChatGpt,
        };
        if model.1 != expected
            || choice
                .1
                .as_ref()
                .is_some_and(|effort| !model.2.contains(effort))
        {
            return Err(format!("invalid model choice {}", choice.0));
        }
        let mut modules = Vec::new();
        if permission.2 == Permission::Restricted {
            modules.push(permission.1.clone());
        }
        for module_id in &self.6 {
            modules.push(self.module(module_id)?);
        }
        for insertion in self
            .7
            .iter()
            .filter(|entry| entry.1 == surface && self.6.contains(&entry.0))
        {
            for module_id in &insertion.2 {
                modules.push(self.module(module_id)?);
            }
        }
        let body = modules.join("\n\n");
        let effort = choice
            .1
            .as_ref()
            .map(EffortRendering::lower)
            .unwrap_or("low");
        let (path, text) = match surface {
            Surface::ClaudeAgent => (
                format!(".claude/agents/{identifier}.md"),
                format!(
                    "---\nname: {identifier}\ndescription: '{}'\nmodel: '{}'\neffort: {effort}\n---\n\n{body}\n",
                    description.replace('\'', "''"),
                    choice.0
                ),
            ),
            Surface::CodexAgent => (
                format!(".codex/agents/{identifier}.toml"),
                format!(
                    "name = \"{identifier}\"\ndescription = \"{}\"\nmodel = \"{}\"\nmodel_reasoning_effort = \"{effort}\"\ndeveloper_instructions = \"{}\"\n",
                    description.replace('"', "\\\"").replace('\n', "\\n"),
                    choice.0,
                    body.replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('\n', "\\n")
                ),
            ),
            Surface::PiAgent => {
                let restriction = if permission.2 == Permission::Restricted {
                    "disallowed_tools: 'edit, write'\n"
                } else {
                    ""
                };
                (
                    format!(".pi/agents/{identifier}.md"),
                    format!(
                        "---\nname: {identifier}\ndescription: '{}'\nmodel: 'openai-codex/{}'\nthinking: {effort}\nprojectRoleIdentity: {identifier}\nprojectRoleDispatchKind: leaf\n{restriction}---\n\n{body}\n",
                        description.replace('\'', "''"),
                        choice.0
                    ),
                )
            }
        };
        Ok(RolePacket { path, text })
    }

    fn module(&self, identifier: &str) -> Result<String, String> {
        self.0
            .iter()
            .find(|module| module.0 == identifier)
            .map(|module| module.1.clone())
            .ok_or_else(|| format!("missing role module {identifier}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RootWriting;
    use datomic::Datomic;

    #[test]
    fn empty_roles_round_trip_through_the_typed_schema() {
        use crate::runtime::RootReading;
        let roles = Roles(
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        );
        let text = roles.write_root();
        let round_tripped = Roles::read_root(&text).expect("canonical roles");
        // Compare through datom since Roles lacks PartialEq
        assert_eq!(
            round_tripped.datomize(),
            roles.datomize(),
            "round trip changed the roles"
        );
        assert!(roles.packets().expect("packets").is_empty());
    }
}
