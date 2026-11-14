use std::{collections::BTreeMap, fmt, fs, path::PathBuf};

use crate::generated::SkillSource;
use crate::runtime::Error;

/// The aspect a skill source is declared under: the variant of its
/// `SkillSource` declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Aspect {
    Psyche,
    Mind,
    Field,
}

/// The module kinds read from every aspect repository. A module's kind does
/// not decide which repository holds it. A module's text is
/// `<kind>/<subject>/<version>.md` and its ID `<kind>:<subject>:<version>`,
/// the form role data names it by.
const MODULE_KINDS: &[&str] = &[
    "compensation",
    "intent",
    "knowledge",
    "role",
    "spirit",
    "vision",
];

impl fmt::Display for Aspect {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Psyche => "Psyche",
            Self::Mind => "Mind",
            Self::Field => "Field",
        })
    }
}

/// A declared skill source: an aspect repository whose module-kind
/// directories contain the Markdown skill sources.
pub struct DeclaredSource {
    aspect: Aspect,
    directory: PathBuf,
}

impl From<&SkillSource> for DeclaredSource {
    fn from(source: &SkillSource) -> Self {
        let (aspect, directory) = match source {
            SkillSource::Psyche(directory) => (Aspect::Psyche, directory),
            SkillSource::Mind(directory) => (Aspect::Mind, directory),
            SkillSource::Field(directory) => (Aspect::Field, directory),
        };
        Self {
            aspect,
            directory: PathBuf::from(directory),
        }
    }
}

impl DeclaredSource {
    fn skills(&self) -> Result<Vec<Skill>, Error> {
        // Read the root even when it contains no module-kind directory, so a
        // missing declared source is still a typed read error. Markdown at
        // the repository root is documentation, not a context module.
        fs::read_dir(&self.directory)
            .map_err(|error| Error::Read(self.directory.clone(), error))?;

        let mut skills = Vec::new();
        for kind in MODULE_KINDS {
            let directory = self.directory.join(kind);
            let subjects = match fs::read_dir(&directory) {
                Ok(entries) => entries,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => return Err(Error::Read(directory, error)),
            };
            for subject in subjects {
                let subject = subject
                    .map_err(|error| Error::Read(directory.clone(), error))?
                    .path();
                // A module is a version file in its subject's directory; a
                // file beside the subjects is not a module.
                if !subject.is_dir() {
                    continue;
                }
                let subject_name = subject
                    .file_name()
                    .expect("subject directory name")
                    .to_string_lossy()
                    .into_owned();
                let versions =
                    fs::read_dir(&subject).map_err(|error| Error::Read(subject.clone(), error))?;
                for version in versions {
                    let path = version
                        .map_err(|error| Error::Read(subject.clone(), error))?
                        .path();
                    if !path.extension().is_some_and(|value| value == "md") {
                        continue;
                    }
                    let version = path
                        .file_stem()
                        .expect("markdown stem")
                        .to_string_lossy()
                        .into_owned();
                    if !version.is_canonical_version() {
                        continue;
                    }
                    let body = fs::read_to_string(&path)
                        .map_err(|error| Error::Read(path.clone(), error))?;
                    skills.push(Skill {
                        name: format!("{kind}:{subject_name}:{version}"),
                        aspect: self.aspect,
                        source: path,
                        body,
                    });
                }
            }
        }
        Ok(skills)
    }
}

/// A module version is a canonical nonnegative decimal number: zero is `0`,
/// and no other version opens with `0`.
trait NamesVersion {
    fn is_canonical_version(&self) -> bool;
}

impl NamesVersion for str {
    fn is_canonical_version(&self) -> bool {
        !self.is_empty()
            && (self == "0" || !self.starts_with('0'))
            && self.bytes().all(|byte| byte.is_ascii_digit())
    }
}

pub struct Skill {
    pub name: String,
    pub aspect: Aspect,
    pub source: PathBuf,
    pub body: String,
}

/// The union of every declared source's skills, ordered by name. A skill
/// name defined by two sources is refused.
pub struct SkillCatalog {
    skills: Vec<Skill>,
}

impl SkillCatalog {
    pub fn read(sources: &[SkillSource]) -> Result<Self, Error> {
        let mut by_name = BTreeMap::<String, Skill>::new();
        for declared in sources.iter().map(DeclaredSource::from) {
            for skill in declared.skills()? {
                if let Some(first) = by_name.get(&skill.name) {
                    return Err(Error::DuplicateSkill {
                        name: skill.name,
                        first: format!("{} {}", first.aspect, first.source.display()),
                        second: format!("{} {}", skill.aspect, skill.source.display()),
                    });
                }
                by_name.insert(skill.name.clone(), skill);
            }
        }
        Ok(Self {
            skills: by_name.into_values().collect(),
        })
    }

    pub fn skills(&self) -> &[Skill] {
        &self.skills
    }

    pub fn defines(&self, name: &str) -> bool {
        self.skills.iter().any(|skill| skill.name == name)
    }

    pub fn count(&self) -> usize {
        self.skills.len()
    }
}
