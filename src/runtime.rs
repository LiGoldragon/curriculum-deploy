use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};

use datom::{
    DatomFault, DatomProblem, DatomRealizing, DatomRoot, DatomText, DatomTextualizing,
    PositionAdvancing, RecordPosition,
};
use protos::{
    Block, Head, Headed, Realize, RealizeScope, RealizeScoping, Shape, ShapeDefined, SourceText,
    TextualizeScope, TextualizeScoping,
};
use thiserror::Error as ThisError;

use crate::roles::{RolePacket, Roles};

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("curriculum-deploy accepts exactly one inline Datom object")]
    Argument,
    #[error("Datom configuration: {0:?}")]
    Datom(DatomFault),
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
        if argument.starts_with('-') || !argument.starts_with("CurriculumRequest.{") {
            return Err(Error::Argument);
        }
        let request = DatomText::<CurriculumRequest>::from(SourceText(argument.clone()))
            .realize()
            .map_err(Error::Datom)?;
        request.execute()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CurriculumRequest {
    Generate(Configuration),
    Check(Configuration),
    Visualize(Configuration),
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Configuration {
    data_root: PathBuf,
    workspace_root: PathBuf,
}
fn fault(problem: DatomProblem) -> DatomFault {
    DatomFault { problem }
}
impl ShapeDefined for CurriculumRequest {
    type Selection = ();
    fn shapes() -> &'static [Shape] {
        &[Shape::DottedBraced]
    }
    fn select(shape: Shape, head: Option<&Head>) -> Option<Self::Selection> {
        (shape == Shape::DottedBraced && head == Some(&Head("CurriculumRequest".into())))
            .then_some(())
    }
}
impl DatomRoot for CurriculumRequest {
    fn root_head() -> Head {
        Head("CurriculumRequest".into())
    }
}
impl DatomRealizing for CurriculumRequest {
    fn realize_block(scope: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
        if Self::select(block.shape, block.head()).is_none() {
            return Err(fault(DatomProblem::Shape));
        }
        let values = scope.realize_body(&mut |child, value| match value.head() {
            Some(head) if head == &Head("Generate".into()) => {
                Ok(Self::Generate(Configuration::realize_block(child, value)?))
            }
            Some(head) if head == &Head("Check".into()) => {
                Ok(Self::Check(Configuration::realize_block(child, value)?))
            }
            Some(head) if head == &Head("Visualize".into()) => {
                Ok(Self::Visualize(Configuration::realize_block(child, value)?))
            }
            _ => Err(fault(DatomProblem::Shape)),
        })?;
        match values.len() {
            1 => Ok(values.into_iter().next().expect("one value")),
            0 => Err(fault(DatomProblem::MissingPosition)),
            _ => Err(fault(DatomProblem::ExtraPosition)),
        }
    }
}
impl DatomTextualizing for CurriculumRequest {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        let (head, configuration) = match self {
            Self::Generate(value) => ("Generate", value),
            Self::Check(value) => ("Check", value),
            Self::Visualize(value) => ("Visualize", value),
        };
        scope.textualize_block(Shape::DottedBraced, Some(&Head(head.into())), |body| {
            configuration.textualize_in(body)
        })
    }
}
impl DatomRealizing for Configuration {
    fn realize_block(scope: &mut RealizeScope<'_>, _: &Block) -> Result<Self, DatomFault> {
        let mut position = RecordPosition::default();
        let (mut data_root, mut workspace_root) = (None, None);
        scope.realize_body(&mut |child, value| {
            match position.next_position() {
                0 => data_root = Some(PathBuf::realize_block(child, value)?),
                1 => workspace_root = Some(PathBuf::realize_block(child, value)?),
                _ => return Err(fault(DatomProblem::ExtraPosition)),
            };
            Ok(())
        })?;
        Ok(Self {
            data_root: data_root.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
            workspace_root: workspace_root.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
        })
    }
}
impl DatomTextualizing for Configuration {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        self.data_root.textualize_in(scope)?;
        self.workspace_root.textualize_in(scope)
    }
}

impl CurriculumRequest {
    fn execute(self) -> Result<String, Error> {
        let (mode, configuration) = match self {
            Self::Generate(value) => (Mode::Generate, value),
            Self::Check(value) => (Mode::Check, value),
            Self::Visualize(value) => (Mode::Visualize, value),
        };
        let deployment = Deployment::read(configuration)?;
        match mode {
            Mode::Generate => {
                deployment.write()?;
                Ok(format!(
                    "Generated.{{{} {}}}",
                    deployment.skills.len(),
                    deployment.roles.len()
                ))
            }
            Mode::Check => {
                deployment.check()?;
                Ok(format!(
                    "Checked.{{{} {}}}",
                    deployment.skills.len(),
                    deployment.roles.len()
                ))
            }
            Mode::Visualize => Ok(format!(
                "Visualized.{{{} {}}}",
                deployment.skills.len(),
                deployment.roles.len()
            )),
        }
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
    fn read(configuration: Configuration) -> Result<Self, Error> {
        let skills_root = configuration.data_root.join("skills");
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
        let role_path = configuration.data_root.join("roles.datom");
        let source =
            fs::read_to_string(&role_path).map_err(|error| Error::Read(role_path, error))?;
        let roles = DatomText::<Roles>::from(SourceText(source))
            .realize()
            .map_err(Error::Datom)?
            .packets()
            .map_err(Error::Roles)?;
        Ok(Self {
            workspace: configuration.workspace_root,
            skills,
            roles,
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
        let inventory = GeneratedRoleOutputs {
            paths: self
                .roles
                .iter()
                .map(|role| PathBuf::from(&role.path))
                .collect(),
        }
        .textualize_source()
        .map_err(Error::Datom)?
        .0;
        for role in &self.roles {
            outputs.push((PathBuf::from(&role.path), role.text.clone()));
        }
        outputs.push((
            PathBuf::from("skills/generated-role-outputs.datom"),
            inventory,
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
        let old = DatomText::<GeneratedRoleOutputs>::from(SourceText(source))
            .realize()
            .map_err(Error::Datom)?;
        for relative in old.paths {
            let path = self.safe(&relative)?;
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct GeneratedRoleOutputs {
    paths: Vec<PathBuf>,
}
impl ShapeDefined for GeneratedRoleOutputs {
    type Selection = ();
    fn shapes() -> &'static [Shape] {
        &[Shape::DottedBraced]
    }
    fn select(shape: Shape, head: Option<&Head>) -> Option<Self::Selection> {
        (shape == Shape::DottedBraced && head == Some(&Head("GeneratedRoleOutputs".into())))
            .then_some(())
    }
}
impl DatomRoot for GeneratedRoleOutputs {
    fn root_head() -> Head {
        Head("GeneratedRoleOutputs".into())
    }
}
impl DatomRealizing for GeneratedRoleOutputs {
    fn realize_block(scope: &mut RealizeScope<'_>, block: &Block) -> Result<Self, DatomFault> {
        if Self::select(block.shape, block.head()).is_none() {
            return Err(fault(DatomProblem::Shape));
        }
        let mut paths = None;
        let mut position = RecordPosition::default();
        scope.realize_body(&mut |child, value| {
            match position.next_position() {
                0 => paths = Some(Vec::<PathBuf>::realize_block(child, value)?),
                _ => return Err(fault(DatomProblem::ExtraPosition)),
            };
            Ok(())
        })?;
        Ok(Self {
            paths: paths.ok_or_else(|| fault(DatomProblem::MissingPosition))?,
        })
    }
}
impl DatomTextualizing for GeneratedRoleOutputs {
    fn textualize_in(&self, scope: &mut TextualizeScope<'_>) -> Result<(), DatomFault> {
        self.paths.textualize_in(scope)
    }
}
