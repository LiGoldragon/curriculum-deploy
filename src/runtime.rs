use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};

use datomic::{Corporal, Datom, Datomic, Separator, Textualizable};
use protos::{Conceptual, Structural};
use thiserror::Error as ThisError;

use crate::generated::{GeneratedRoleOutputs, Output, Request, Roles};
use crate::roles::RolePacket;

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
    #[error("skill template {source_path}: {message}")]
    Template {
        source_path: PathBuf,
        message: String,
    },
}

trait DatomFaulting {
    fn datom_fault(self) -> Error;
}

impl DatomFaulting for datomic::Fault {
    fn datom_fault(self) -> Error {
        Error::Datom(self.textualize())
    }
}

impl DatomFaulting for protos::Fault {
    fn datom_fault(self) -> Error {
        Error::Datom(datomic::Fault::from(self).textualize())
    }
}

/// Root-head convention: unwrap a named variant head when reading a datom file.
pub(crate) trait RootReading: Corporal<Datom, Fault = datomic::Fault> {
    const ROOT: &'static str;

    fn read_root(text: &str) -> Result<Self, datomic::Fault> {
        let delineation = text.to_owned().delineate().map_err(datomic::Fault::from)?;
        let datom: Datom = delineation.conceive()?;
        let datom = datom.normalize_meaning_to_text();
        match datom {
            Datom::Variant(head, Separator::Period, Some(body)) if head == Self::ROOT => {
                Self::incorporate(*body)
            }
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}

/// Root-head convention: wrap a named variant head when writing a datom file.
pub(crate) trait RootWriting: Datomic {
    const ROOT: &'static str;

    fn write_root(&self) -> String {
        let datom = Datom::Variant(
            Self::ROOT.to_owned(),
            Separator::Period,
            Some(Box::new(self.datomize())),
        );
        datom.textualize()
    }
}

impl RootReading for Roles {
    const ROOT: &'static str = "Roles";
}

impl RootWriting for Roles {
    const ROOT: &'static str = "Roles";
}

impl RootReading for GeneratedRoleOutputs {
    const ROOT: &'static str = "GeneratedRoleOutputs";
}

impl RootWriting for GeneratedRoleOutputs {
    const ROOT: &'static str = "GeneratedRoleOutputs";
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
    let delineation = text
        .to_owned()
        .delineate()
        .map_err(DatomFaulting::datom_fault)?;
    let datom: Datom = delineation.conceive().map_err(DatomFaulting::datom_fault)?;
    // Accept both bare `Generate.{ ... }` and wrapped `CurriculumRequest.{ Generate.{ ... } }`
    let request_datom = match &datom {
        Datom::Variant(head, Separator::Period, Some(body)) if head == "CurriculumRequest" => {
            match body.as_ref() {
                Datom::Struct(fields) if fields.len() == 1 => fields[0].clone(),
                _ => *body.clone(),
            }
        }
        _ => datom,
    };
    Request::incorporate(request_datom).map_err(DatomFaulting::datom_fault)
}

impl Request {
    fn execute(self) -> Result<String, Error> {
        let (mode, data_root, workspace_root) = match self {
            Self::Generate(c) => (Mode::Generate, PathBuf::from(&c.0), PathBuf::from(&c.1)),
            Self::Check(c) => (Mode::Check, PathBuf::from(&c.0), PathBuf::from(&c.1)),
            Self::Visualize(c) => (Mode::Visualize, PathBuf::from(&c.0), PathBuf::from(&c.1)),
        };
        let deployment = Deployment::read(data_root, workspace_root)?;
        let output = match mode {
            Mode::Generate => {
                deployment.write()?;
                Output::Generated(crate::generated::OutputGenerated(
                    deployment.skills.len() as i64,
                    deployment.roles.len() as i64,
                ))
            }
            Mode::Check => {
                deployment.check()?;
                Output::Checked(crate::generated::OutputChecked(
                    deployment.skills.len() as i64,
                    deployment.roles.len() as i64,
                ))
            }
            Mode::Visualize => Output::Visualized(crate::generated::OutputVisualized(
                deployment.skills.len() as i64,
                deployment.roles.len() as i64,
            )),
        };
        Ok(output.textualize())
    }
}

enum Mode {
    Generate,
    Check,
    Visualize,
}

struct Deployment {
    workspace: PathBuf,
    skills: Vec<Skill>,
    roles: Vec<RolePacket>,
}

struct Skill {
    name: String,
    source: PathBuf,
    body: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SkillTarget {
    Claude,
    Codex,
    Pi,
}

struct ConditionalBlock {
    selected: bool,
    otherwise_seen: bool,
}

trait SkillBodyRendering {
    fn rendered(self, body: &str, source: &Path) -> Result<String, Error>;
}

impl SkillBodyRendering for SkillTarget {
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
                "{% if pi %}" => Some(self == Self::Pi),
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
                rendered.push_str(line);
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
    fn read(data_root: PathBuf, workspace_root: PathBuf) -> Result<Self, Error> {
        let skills_root = data_root.join("skills");
        let mut skills = fs::read_dir(&skills_root)
            .map_err(|error| Error::Read(skills_root.clone(), error))?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|value| value == "md"))
            .map(|path| {
                let name = path
                    .file_stem()
                    .expect("markdown stem")
                    .to_string_lossy()
                    .into_owned();
                let body =
                    fs::read_to_string(&path).map_err(|error| Error::Read(path.clone(), error))?;
                Ok(Skill {
                    name,
                    source: path,
                    body,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        skills.sort_by(|left, right| left.name.cmp(&right.name));
        let role_path = data_root.join("roles.datom");
        let source =
            fs::read_to_string(&role_path).map_err(|error| Error::Read(role_path, error))?;
        let roles = Roles::read_root(&source).map_err(DatomFaulting::datom_fault)?;
        let packets = roles.packets().map_err(Error::Roles)?;
        Ok(Self {
            workspace: workspace_root,
            skills,
            roles: packets,
        })
    }

    fn outputs(&self) -> Result<Vec<(PathBuf, String)>, Error> {
        let mut outputs = Vec::new();
        for skill in &self.skills {
            for (surface, target) in [
                (".agents/skills", SkillTarget::Codex),
                (".claude/skills", SkillTarget::Claude),
            ] {
                outputs.push((
                    PathBuf::from(surface).join(&skill.name).join("SKILL.md"),
                    target.rendered(&skill.body, &skill.source)?,
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
        let inventory_paths: Vec<String> =
            self.roles.iter().map(|role| role.path.clone()).collect();
        let inventory = GeneratedRoleOutputs(inventory_paths);
        let inventory_text = inventory.write_root();
        for role in &self.roles {
            outputs.push((PathBuf::from(&role.path), role.text.clone()));
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
        for relative in [Path::new(".agents/skills"), Path::new(".claude/skills")] {
            let path = self.safe(relative)?;
            if !path.exists() {
                continue;
            }
            for entry in fs::read_dir(&path).map_err(|error| Error::Read(path.clone(), error))? {
                let entry = entry.map_err(|error| Error::Read(path.clone(), error))?;
                let skill_path = entry.path().join("SKILL.md");
                let name = entry.file_name().to_string_lossy().into_owned();
                if skill_path.is_file() && !self.skills.iter().any(|skill| skill.name == name) {
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
        let Ok(old) = GeneratedRoleOutputs::read_root(&source) else {
            return Ok(());
        };
        for relative in &old.0 {
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

/// Normalize datom values for Curriculum data compatibility.
/// The Curriculum's roles.datom uses parenthesized text `(...)` where the
/// new datom layer reads Meaning. This normalizer converts Meaning to Text
/// before incorporation, bridging the syntax change.
trait DatomNormalizing {
    fn normalize_meaning_to_text(self) -> Self;
}

impl DatomNormalizing for Datom {
    fn normalize_meaning_to_text(self) -> Self {
        match self {
            Datom::Meaning(content) => Datom::Text(content),
            Datom::Variant(head, sep, body) => Datom::Variant(
                head,
                sep,
                body.map(|b| Box::new(b.normalize_meaning_to_text())),
            ),
            Datom::Struct(fields) => Datom::Struct(
                fields
                    .into_iter()
                    .map(DatomNormalizing::normalize_meaning_to_text)
                    .collect(),
            ),
            Datom::Vector(items) => Datom::Vector(
                items
                    .into_iter()
                    .map(DatomNormalizing::normalize_meaning_to_text)
                    .collect(),
            ),
            Datom::Map(pairs) => Datom::Map(
                pairs
                    .into_iter()
                    .map(|datomic::Pair(k, v)| {
                        datomic::Pair(k.normalize_meaning_to_text(), v.normalize_meaning_to_text())
                    })
                    .collect(),
            ),
            other => other,
        }
    }
}
