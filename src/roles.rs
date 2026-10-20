use crate::{
    catalog::SkillCatalog,
    generated::{Effort, Permission, RoleDefinition, RoleTools, Roles, Surface},
    runtime::{Error, SkillTarget},
};

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
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Xhigh => "xhigh",
        }
    }
}

impl Roles {
    pub fn packets(&self, skills: &SkillCatalog) -> Result<Vec<RolePacket>, Error> {
        self.role_definition_vector
            .iter()
            .map(|role| packet(role, skills))
            .collect()
    }
}

fn packet(role: &RoleDefinition, skills: &SkillCatalog) -> Result<RolePacket, Error> {
    let description = module_body(skills, &role.third_string, &role.surface)?;
    let modules = role
        .string_vector
        .iter()
        .map(|name| Ok((name, module_body(skills, name, &role.surface)?)))
        .collect::<Result<Vec<_>, Error>>()?;
    let module_names = role.string_vector.join(", ");
    let mut body = format!(
        "These are your skills, given here; do not reload them.\n\nModule IDs: {module_names}\n"
    );
    for (name, text) in modules {
        body.push_str(&format!(
            "\n<!-- module: {name} -->\n{text}\n<!-- end-module: {name} -->\n"
        ));
    }

    let name = &role.first_string;
    let model = &role.second_string;
    let effort = role.effort.lower();
    let restricted = matches!(role.permission, Permission::Restricted);
    let tools = match &role.role_tools {
        RoleTools::Inherit => None,
        RoleTools::Explicit(tools) => Some(tools.join(", ")),
    };
    let (path, text) = match role.surface {
        Surface::ClaudeAgent => {
            let tools = tools
                .map(|tools| format!("tools: [{tools}]\n"))
                .unwrap_or_default();
            (
                format!(".claude/agents/{name}.md"),
                format!(
                    "---\nname: {name}\ndescription: {description:?}\nmodel: {model:?}\neffort: {effort}\n{tools}---\n\n{body}"
                ),
            )
        }
        Surface::CodexAgent => (
            format!(".codex/agents/{name}.toml"),
            format!(
                "name = {name:?}\ndescription = {description:?}\nmodel = {model:?}\nmodel_reasoning_effort = {effort:?}\ndeveloper_instructions = {:?}\n",
                body.trim_end()
            ),
        ),
        Surface::PiAgent => {
            let restriction = if restricted {
                "disallowed_tools: 'edit, write'\n"
            } else {
                ""
            };
            (
                format!(".pi/agents/{name}.md"),
                format!(
                    "---\nname: {name}\ndescription: {description:?}\nmodel: 'openai-codex/{model}'\nthinking: {effort}\nprojectRoleIdentity: {name}\nprojectRoleDispatchKind: leaf\n{restriction}---\n\n{body}"
                ),
            )
        }
    };
    Ok(RolePacket { path, text })
}

fn module_body(skills: &SkillCatalog, name: &str, surface: &Surface) -> Result<String, Error> {
    let skill = skills
        .named(name)
        .ok_or_else(|| Error::Roles(format!("missing role module {name}")))?;
    let rendered =
        SkillTarget::for_surface(surface).render_role_body(&skill.body, &skill.source)?;
    let body = rendered
        .split_once("\n---\n")
        .map(|(_, body)| body)
        .unwrap_or(&rendered)
        .trim();
    if body.is_empty() {
        return Err(Error::Roles(format!("empty role module {name}")));
    }
    Ok(body.to_owned())
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
            role_definition_vector: vec![],
        };
        let text = RolesDocument::Roles(roles)
            .datomize(vec![])
            .protosize()
            .textualize();
        let round_tripped = Potential::<RolesDocument>::from(text.clone())
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
