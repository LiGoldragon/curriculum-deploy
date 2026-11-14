use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};

use datom_codec::{Actualizing, Budget, Composing, Datomizable, Potential};
use protos::{Protosizable, ReaderBudget, Textualizable};
use thiserror::Error as ThisError;

use curriculum::generated::{Roles, RolesDocument};

use crate::catalog::{Skill, SkillCatalog};
use crate::generated::{
    Checked_Data, Generated_Data, GeneratedRoleOutputDocument, GeneratedRoleOutputs, Output,
    Request, Visualized_Data,
};
use crate::role_projection;
use crate::roles::ComposesRolePackets;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("curriculum-deploy accepts exactly one inline Datom request")]
    Argument,
    #[error("{0}")]
    Datom(String),
    #[error("read {0}: {1}")]
    Read(PathBuf, io::Error),
    #[error("write {0}: {1}")]
    Write(PathBuf, io::Error),
    #[error("generated output differs: {0}")]
    Different(PathBuf),
    #[error("roles: {0}")]
    Roles(String),
    #[error("role projection source {0}: {1}")]
    RoleSource(PathBuf, String),
    #[error("role projection source rejected request: {0}")]
    RoleRejected(String),
    #[error("role projection source returned a mismatched role")]
    RoleMismatch,
    #[error("skill {name} is defined by two sources: {first} and {second}")]
    DuplicateSkill {
        name: String,
        first: String,
        second: String,
    },
    #[error("skill template {source_path}: {message}")]
    Template {
        source_path: PathBuf,
        message: String,
    },
}

trait DatomFaulting {
    fn datom_fault(self) -> Error;
}

impl DatomFaulting for datom_codec::Error {
    fn datom_fault(self) -> Error {
        Error::Datom(self.datomize(vec![]).protosize().textualize())
    }
}

trait BudgetedActualizing<T> {
    fn actualize_with_limit(&mut self, limit: i64) -> Result<T, datom_codec::Error>;
}

impl<T: Composing> BudgetedActualizing<T> for Potential<T> {
    fn actualize_with_limit(&mut self, limit: i64) -> Result<T, datom_codec::Error> {
        self.actualize(&mut Budget {
            remaining: limit,
            reader: ReaderBudget::try_from(usize::try_from(limit).expect("positive fixed budget"))
                .expect("a constant reader budget"),
            depth: 0,
            maximum_depth: limit,
        })
    }
}

trait DatomTextualizing {
    fn datom_text(&self) -> String;
}

impl<T: Datomizable> DatomTextualizing for T {
    fn datom_text(&self) -> String {
        self.datomize(vec![]).protosize().textualize()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandLine {
    arguments: Vec<String>,
}

impl CommandLine {
    pub fn from_arguments(arguments: impl IntoIterator<Item = String>) -> Self {
        Self {
            arguments: arguments.into_iter().collect(),
        }
    }

    pub fn run(&self) -> Result<String, Error> {
        let [argument] = self.arguments.as_slice() else {
            return Err(Error::Argument);
        };
        if argument.starts_with('-') {
            return Err(Error::Argument);
        }
        let request = actualize_request(argument)?;
        request.execute()
    }
}

fn actualize_request(text: &str) -> Result<Request, Error> {
    Potential::<Request>::from(text)
        .actualize_with_limit(1_024)
        .map_err(DatomFaulting::datom_fault)
}

impl Request {
    fn execute(self) -> Result<String, Error> {
        let (mode, configuration) = match self {
            Self::Generate(configuration) => (Mode::Generate, configuration),
            Self::Check(configuration) => (Mode::Check, configuration),
            Self::Visualize(configuration) => (Mode::Visualize, configuration),
            Self::ProjectRoles(configuration) => {
                return role_projection::project(configuration).map(|count| {
                    Output::RolesProjected(crate::generated::RolesProjected_Data {
                        integer: count as i64,
                    })
                    .datom_text()
                });
            }
        };
        let deployment = Deployment::read(
            PathBuf::from(configuration.first_string),
            SkillCatalog::read(&configuration.skill_source_vector)?,
            PathBuf::from(configuration.second_string),
            PathBuf::from(configuration.third_string),
        )?;
        let skills = deployment.skills.count() as i64;
        let roles = deployment.roles.role_definition_vector.len() as i64;
        let output = match mode {
            Mode::Generate => {
                deployment.write()?;
                Output::Generated(Generated_Data {
                    first_integer: skills,
                    second_integer: roles,
                })
            }
            Mode::Check => {
                deployment.check()?;
                Output::Checked(Checked_Data {
                    first_integer: skills,
                    second_integer: roles,
                })
            }
            Mode::Visualize => Output::Visualized(Visualized_Data {
                first_integer: skills,
                second_integer: roles,
            }),
        };
        Ok(output.datom_text())
    }
}

enum Mode {
    Generate,
    Check,
    Visualize,
}

/// One deployment: the skills its sources hold, and the roles Curriculum's
/// roles file declares, whose packets the Curriculum Nexus at
/// `curriculum_socket` composes.
struct Deployment {
    workspace: PathBuf,
    skills: SkillCatalog,
    roles: Roles,
    curriculum_socket: PathBuf,
}

/// A harness a skill is rendered for, each with a skill surface. A
/// `{% if pi %}` block is read and never selected: Pi has no skill surface.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SkillTarget {
    Claude,
    Codex,
    OpenCode,
}

impl SkillTarget {
    /// Every target, in output order.
    const PROJECTED: [Self; 3] = [Self::Codex, Self::Claude, Self::OpenCode];

    /// The workspace directory the harness reads its skills from.
    fn surface(self) -> &'static str {
        match self {
            Self::Codex => ".agents/skills",
            Self::Claude => ".claude/skills",
            Self::OpenCode => ".opencode/skills",
        }
    }
}

struct ConditionalBlock {
    selected: bool,
    otherwise_seen: bool,
}

trait SkillBodyRendering {
    fn rendered_skill(self, skill: &Skill) -> Result<String, Error>;
    fn rendered(self, body: &str, source: &Path) -> Result<String, Error>;
}

impl SkillBodyRendering for SkillTarget {
    /// OpenCode loads a skill only when its frontmatter names it; the other
    /// harnesses name a skill by its directory.
    fn rendered_skill(self, skill: &Skill) -> Result<String, Error> {
        let rendered = self.rendered(&skill.body, &skill.source)?;
        if self != Self::OpenCode {
            return Ok(rendered);
        }
        let name = format!("name: {}\n", skill.name);
        Ok(match rendered.strip_prefix("---\n") {
            Some(rest) => format!("---\n{name}{rest}"),
            None => format!("---\n{name}---\n{rendered}"),
        })
    }

    fn rendered(self, body: &str, source: &Path) -> Result<String, Error> {
        let mut rendered = String::new();
        let mut blocks = Vec::<ConditionalBlock>::new();
        let mut raw = false;

        for (index, line) in body.split_inclusive('\n').enumerate() {
            let line_number = index + 1;
            let directive = line.trim();
            if raw {
                if directive == "{% endraw %}" {
                    raw = false;
                } else if blocks.iter().all(|block| block.selected) {
                    rendered.push_str(line);
                }
                continue;
            }
            let selected = match directive {
                "{% if claude %}" => Some(self == Self::Claude),
                "{% if codex %}" => Some(self == Self::Codex),
                "{% if opencode %}" => Some(self == Self::OpenCode),
                "{% if pi %}" => Some(false),
                "{% raw %}" => {
                    raw = true;
                    continue;
                }
                "{% endraw %}" => {
                    return Err(Error::Template {
                        source_path: source.to_path_buf(),
                        message: format!("line {line_number}: endraw without raw"),
                    });
                }
                "{% else %}" => {
                    let block = blocks.last_mut().ok_or_else(|| Error::Template {
                        source_path: source.to_path_buf(),
                        message: format!("line {line_number}: else without if"),
                    })?;
                    if block.otherwise_seen {
                        return Err(Error::Template {
                            source_path: source.to_path_buf(),
                            message: format!("line {line_number}: repeated else"),
                        });
                    }
                    block.selected = !block.selected;
                    block.otherwise_seen = true;
                    continue;
                }
                "{% endif %}" => {
                    if blocks.pop().is_none() {
                        return Err(Error::Template {
                            source_path: source.to_path_buf(),
                            message: format!("line {line_number}: endif without if"),
                        });
                    }
                    continue;
                }
                _ => None,
            };
            if let Some(selected) = selected {
                blocks.push(ConditionalBlock {
                    selected,
                    otherwise_seen: false,
                });
                continue;
            }
            if blocks.iter().all(|block| block.selected) {
                if self == Self::Claude && directive == "user-only: true" {
                    rendered.push_str("disable-model-invocation: true\n");
                } else {
                    rendered.push_str(line);
                }
            }
        }

        if !blocks.is_empty() {
            return Err(Error::Template {
                source_path: source.to_path_buf(),
                message: "unclosed if".into(),
            });
        }
        if raw {
            return Err(Error::Template {
                source_path: source.to_path_buf(),
                message: "unclosed raw".into(),
            });
        }
        Ok(rendered)
    }
}

impl Deployment {
    fn read(
        curriculum_root: PathBuf,
        skills: SkillCatalog,
        workspace_root: PathBuf,
        curriculum_socket: PathBuf,
    ) -> Result<Self, Error> {
        let role_path = curriculum_root.join("roles.datom");
        let source =
            fs::read_to_string(&role_path).map_err(|error| Error::Read(role_path, error))?;
        let RolesDocument::Roles(roles) = Potential::<RolesDocument>::from(source)
            .actualize_with_limit(16_384)
            .map_err(DatomFaulting::datom_fault)?;
        Ok(Self {
            workspace: workspace_root,
            skills,
            roles,
            curriculum_socket,
        })
    }

    fn outputs(&self) -> Result<Vec<(PathBuf, String)>, Error> {
        let mut outputs = Vec::new();
        for skill in self.skills.skills() {
            for target in SkillTarget::PROJECTED {
                outputs.push((
                    PathBuf::from(target.surface())
                        .join(&skill.name)
                        .join("SKILL.md"),
                    target.rendered_skill(skill)?,
                ));
            }
            if user_only(&skill.body) {
                outputs.push((
                    PathBuf::from(".agents/skills")
                        .join(&skill.name)
                        .join("agents/openai.yaml"),
                    "policy:\n  allow_implicit_invocation: false\n".into(),
                ));
            }
        }
        let packets = self.roles.packets(&self.curriculum_socket)?;
        let inventory_paths = packets.iter().map(|packet| packet.path.clone()).collect();
        let inventory = GeneratedRoleOutputDocument::GeneratedRoleOutputs(GeneratedRoleOutputs {
            string_vector: inventory_paths,
        });
        let inventory_text = inventory.datom_text();
        for packet in packets {
            outputs.push((PathBuf::from(packet.path), packet.text));
        }
        outputs.push((
            PathBuf::from("skills/generated-role-outputs.datom"),
            inventory_text,
        ));
        Ok(outputs)
    }

    fn write(&self) -> Result<(), Error> {
        self.clean_previous_skills()?;
        self.clean_previous_roles()?;
        for (relative, body) in self.outputs()? {
            let path = self.safe(&relative)?;
            let parent = path.parent().expect("output parent");
            fs::create_dir_all(parent)
                .map_err(|error| Error::Write(parent.to_path_buf(), error))?;
            fs::write(&path, body).map_err(|error| Error::Write(path, error))?;
        }
        Ok(())
    }

    fn check(&self) -> Result<(), Error> {
        for (relative, expected) in self.outputs()? {
            let path = self.safe(&relative)?;
            let actual =
                fs::read_to_string(&path).map_err(|error| Error::Read(path.clone(), error))?;
            if actual != expected {
                return Err(Error::Different(path));
            }
        }
        Ok(())
    }

    fn clean_previous_skills(&self) -> Result<(), Error> {
        for target in SkillTarget::PROJECTED {
            let relative = Path::new(target.surface());
            let path = self.safe(relative)?;
            if !path.exists() {
                continue;
            }
            for entry in fs::read_dir(&path).map_err(|error| Error::Read(path.clone(), error))? {
                let entry = entry.map_err(|error| Error::Read(path.clone(), error))?;
                let skill_path = entry.path().join("SKILL.md");
                let name = entry.file_name().to_string_lossy().into_owned();
                if skill_path.is_file() && !self.skills.defines(&name) {
                    let retired = entry.path();
                    fs::remove_dir_all(&retired).map_err(|error| Error::Write(retired, error))?;
                }
            }
        }
        Ok(())
    }

    fn clean_previous_roles(&self) -> Result<(), Error> {
        let inventory = self.workspace.join("skills/generated-role-outputs.datom");
        if !inventory.exists() {
            return Ok(());
        }
        let source = fs::read_to_string(&inventory)
            .map_err(|error| Error::Read(inventory.clone(), error))?;
        let Ok(GeneratedRoleOutputDocument::GeneratedRoleOutputs(old)) =
            Potential::<GeneratedRoleOutputDocument>::from(source).actualize_with_limit(1_024)
        else {
            return Ok(());
        };
        for relative in &old.string_vector {
            let path = self.safe(&PathBuf::from(relative))?;
            if path.exists() {
                fs::remove_file(&path).map_err(|error| Error::Write(path, error))?;
            }
        }
        Ok(())
    }

    fn safe(&self, relative: &Path) -> Result<PathBuf, Error> {
        if relative.is_absolute()
            || relative
                .components()
                .any(|component| matches!(component, Component::ParentDir))
        {
            return Err(Error::Argument);
        }
        Ok(self.workspace.join(relative))
    }
}

fn user_only(body: &str) -> bool {
    body.strip_prefix("---\n")
        .and_then(|body| body.split_once("\n---\n"))
        .is_some_and(|(frontmatter, _)| frontmatter.lines().any(|line| line == "user-only: true"))
}
