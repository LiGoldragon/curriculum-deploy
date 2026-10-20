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

impl fmt::Display for Aspect {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Psyche => "Psyche",
            Self::Mind => "Mind",
            Self::Field => "Field",
        })
    }
}

/// A declared skill source: a directory whose `*.md` files are skill sources,
/// read under one aspect.
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
        fs::read_dir(&self.directory)
            .map_err(|error| Error::Read(self.directory.clone(), error))?
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
                    aspect: self.aspect,
                    source: path,
                    body,
                })
            })
            .collect()
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

    pub fn named(&self, name: &str) -> Option<&Skill> {
        self.skills.iter().find(|skill| skill.name == name)
    }

    pub fn count(&self) -> usize {
        self.skills.len()
    }
}
