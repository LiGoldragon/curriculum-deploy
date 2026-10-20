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

impl Aspect {
    fn module_kinds(self) -> &'static [&'static str] {
        match self {
            Self::Psyche => &["intent", "spirit", "vision"],
            Self::Mind => &["inquiry", "knowledge", "role"],
            Self::Field => &["compensation", "role"],
        }
    }
}

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
        for kind in self.aspect.module_kinds() {
            let directory = self.directory.join(kind);
            let entries = match fs::read_dir(&directory) {
                Ok(entries) => entries,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => return Err(Error::Read(directory, error)),
            };
            for entry in entries {
                let path = entry
                    .map_err(|error| Error::Read(directory.clone(), error))?
                    .path();
                if !path.extension().is_some_and(|value| value == "md") {
                    continue;
                }
                let name = path
                    .file_stem()
                    .expect("markdown stem")
                    .to_string_lossy()
                    .into_owned();
                let body =
                    fs::read_to_string(&path).map_err(|error| Error::Read(path.clone(), error))?;
                skills.push(Skill {
                    name: format!("{kind}-{name}"),
                    aspect: self.aspect,
                    source: path,
                    body,
                });
            }
        }
        Ok(skills)
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
