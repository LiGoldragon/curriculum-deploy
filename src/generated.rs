#![allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Configuration(pub protos::Text, pub protos::Text);
impl datom_codec::Datomic for Configuration {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: protos::Text = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for Configuration {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Generate(Configuration),
    Check(Configuration),
    Visualize(Configuration),
}
impl datom_codec::Datomic for Request {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Generate" => std::result::Result::Ok(Self::Generate(datom_codec::Carrying::body(v)?)),
            "Check" => std::result::Result::Ok(Self::Check(datom_codec::Carrying::body(v)?)),
            "Visualize" => {
                std::result::Result::Ok(Self::Visualize(datom_codec::Carrying::body(v)?))
            }
            _ => std::result::Result::Err(datom_codec::Headed::reject(
                &v,
                datom_codec::Problem::UnknownVariant(
                    protos::Word::try_from(v.name).expect("variant name"),
                ),
            )),
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Request {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            match self {
                Self::Generate(p0) => datom_codec::Datom::Variant(
                    protos::Symbol::try_from("Generate").expect("static variant"),
                    std::boxed::Box::new(
                        protos::Conceivable::conceive(p0)
                            .expect("infallible datom ascent")
                            .1,
                    ),
                ),
                Self::Check(p0) => datom_codec::Datom::Variant(
                    protos::Symbol::try_from("Check").expect("static variant"),
                    std::boxed::Box::new(
                        protos::Conceivable::conceive(p0)
                            .expect("infallible datom ascent")
                            .1,
                    ),
                ),
                Self::Visualize(p0) => datom_codec::Datom::Variant(
                    protos::Symbol::try_from("Visualize").expect("static variant"),
                    std::boxed::Box::new(
                        protos::Conceivable::conceive(p0)
                            .expect("infallible datom ascent")
                            .1,
                    ),
                ),
            },
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Output {
    Generated(protos::Integer, protos::Integer),
    Checked(protos::Integer, protos::Integer),
    Visualized(protos::Integer, protos::Integer),
}
impl datom_codec::Datomic for Output {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Generated" => {
                let mut p = datom_codec::Headed::positions(v, 2)?;
                let p0: protos::Integer = datom_codec::Positional::position(&mut p)?;
                let p1: protos::Integer = datom_codec::Positional::position(&mut p)?;
                std::result::Result::Ok(Self::Generated(p0, p1))
            }
            "Checked" => {
                let mut p = datom_codec::Headed::positions(v, 2)?;
                let p0: protos::Integer = datom_codec::Positional::position(&mut p)?;
                let p1: protos::Integer = datom_codec::Positional::position(&mut p)?;
                std::result::Result::Ok(Self::Checked(p0, p1))
            }
            "Visualized" => {
                let mut p = datom_codec::Headed::positions(v, 2)?;
                let p0: protos::Integer = datom_codec::Positional::position(&mut p)?;
                let p1: protos::Integer = datom_codec::Positional::position(&mut p)?;
                std::result::Result::Ok(Self::Visualized(p0, p1))
            }
            _ => std::result::Result::Err(datom_codec::Headed::reject(
                &v,
                datom_codec::Problem::UnknownVariant(
                    protos::Word::try_from(v.name).expect("variant name"),
                ),
            )),
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Output {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            match self {
                Self::Generated(p0, p1) => datom_codec::Datom::Variant(
                    protos::Symbol::try_from("Generated").expect("static variant"),
                    std::boxed::Box::new(datom_codec::Datom::Struct(vec![
                        protos::Conceivable::conceive(p0)
                            .expect("infallible datom ascent")
                            .1,
                        protos::Conceivable::conceive(p1)
                            .expect("infallible datom ascent")
                            .1,
                    ])),
                ),
                Self::Checked(p0, p1) => datom_codec::Datom::Variant(
                    protos::Symbol::try_from("Checked").expect("static variant"),
                    std::boxed::Box::new(datom_codec::Datom::Struct(vec![
                        protos::Conceivable::conceive(p0)
                            .expect("infallible datom ascent")
                            .1,
                        protos::Conceivable::conceive(p1)
                            .expect("infallible datom ascent")
                            .1,
                    ])),
                ),
                Self::Visualized(p0, p1) => datom_codec::Datom::Variant(
                    protos::Symbol::try_from("Visualized").expect("static variant"),
                    std::boxed::Box::new(datom_codec::Datom::Struct(vec![
                        protos::Conceivable::conceive(p0)
                            .expect("infallible datom ascent")
                            .1,
                        protos::Conceivable::conceive(p1)
                            .expect("infallible datom ascent")
                            .1,
                    ])),
                ),
            },
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedRoleOutputs(pub std::vec::Vec<protos::Text>);
impl datom_codec::Datomic for GeneratedRoleOutputs {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: std::vec::Vec<protos::Text> = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for GeneratedRoleOutputs {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GeneratedRoleOutputDocument {
    GeneratedRoleOutputs(GeneratedRoleOutputs),
}
impl datom_codec::Datomic for GeneratedRoleOutputDocument {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "GeneratedRoleOutputs" => {
                std::result::Result::Ok(Self::GeneratedRoleOutputs(datom_codec::Carrying::body(v)?))
            }
            _ => std::result::Result::Err(datom_codec::Headed::reject(
                &v,
                datom_codec::Problem::UnknownVariant(
                    protos::Word::try_from(v.name).expect("variant name"),
                ),
            )),
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for GeneratedRoleOutputDocument {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            match self {
                Self::GeneratedRoleOutputs(p0) => datom_codec::Datom::Variant(
                    protos::Symbol::try_from("GeneratedRoleOutputs").expect("static variant"),
                    std::boxed::Box::new(
                        protos::Conceivable::conceive(p0)
                            .expect("infallible datom ascent")
                            .1,
                    ),
                ),
            },
        ))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Provider {
    Claude,
    ChatGpt,
}
impl datom_codec::Datomic for Provider {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Claude" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Claude)
            }
            "ChatGpt" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ChatGpt)
            }
            _ => std::result::Result::Err(datom_codec::Headed::reject(
                &v,
                datom_codec::Problem::UnknownVariant(
                    protos::Word::try_from(v.name).expect("variant name"),
                ),
            )),
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Provider {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            match self {
                Self::Claude => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("Claude").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
                Self::ChatGpt => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("ChatGpt").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
            },
        ))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Permission {
    Restricted,
    Unrestricted,
}
impl datom_codec::Datomic for Permission {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Restricted" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Restricted)
            }
            "Unrestricted" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Unrestricted)
            }
            _ => std::result::Result::Err(datom_codec::Headed::reject(
                &v,
                datom_codec::Problem::UnknownVariant(
                    protos::Word::try_from(v.name).expect("variant name"),
                ),
            )),
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Permission {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            match self {
                Self::Restricted => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("Restricted").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
                Self::Unrestricted => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("Unrestricted").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
            },
        ))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effort {
    Low,
    Medium,
    High,
    Xhigh,
}
impl datom_codec::Datomic for Effort {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Low" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Low)
            }
            "Medium" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Medium)
            }
            "High" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::High)
            }
            "Xhigh" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Xhigh)
            }
            _ => std::result::Result::Err(datom_codec::Headed::reject(
                &v,
                datom_codec::Problem::UnknownVariant(
                    protos::Word::try_from(v.name).expect("variant name"),
                ),
            )),
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Effort {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            match self {
                Self::Low => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("Low").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
                Self::Medium => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("Medium").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
                Self::High => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("High").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
                Self::Xhigh => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("Xhigh").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
            },
        ))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Surface {
    ClaudeAgent,
    CodexAgent,
    PiAgent,
}
impl datom_codec::Datomic for Surface {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "ClaudeAgent" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ClaudeAgent)
            }
            "CodexAgent" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::CodexAgent)
            }
            "PiAgent" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::PiAgent)
            }
            _ => std::result::Result::Err(datom_codec::Headed::reject(
                &v,
                datom_codec::Problem::UnknownVariant(
                    protos::Word::try_from(v.name).expect("variant name"),
                ),
            )),
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Surface {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            match self {
                Self::ClaudeAgent => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("ClaudeAgent").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
                Self::CodexAgent => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("CodexAgent").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
                Self::PiAgent => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("PiAgent").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
            },
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelChoice(pub protos::Text, pub std::option::Option<Effort>);
impl datom_codec::Datomic for ModelChoice {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: std::option::Option<Effort> = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for ModelChoice {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoleModule(pub protos::Text, pub protos::Text);
impl datom_codec::Datomic for RoleModule {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: protos::Text = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RoleModule {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model(pub protos::Text, pub Provider, pub std::vec::Vec<Effort>);
impl datom_codec::Datomic for Model {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: Provider = datom_codec::Positional::position(&mut p)?;
        let p2: std::vec::Vec<Effort> = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for Model {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.2)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RolePermission(pub protos::Text, pub protos::Text, pub Permission);
impl datom_codec::Datomic for RolePermission {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p2: Permission = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RolePermission {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.2)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoleDepth(pub protos::Text, pub ModelChoice, pub ModelChoice);
impl datom_codec::Datomic for RoleDepth {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: ModelChoice = datom_codec::Positional::position(&mut p)?;
        let p2: ModelChoice = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RoleDepth {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.2)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoleDescription(pub protos::Text, pub protos::Text, pub protos::Text);
impl datom_codec::Datomic for RoleDescription {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p2: protos::Text = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RoleDescription {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.2)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoleAlias(
    pub protos::Text,
    pub protos::Text,
    pub protos::Text,
    pub protos::Text,
    pub std::vec::Vec<Surface>,
);
impl datom_codec::Datomic for RoleAlias {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 5)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p2: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p3: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p4: std::vec::Vec<Surface> = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RoleAlias {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.2)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.3)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.4)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetInsertion(
    pub protos::Text,
    pub Surface,
    pub std::vec::Vec<protos::Text>,
);
impl datom_codec::Datomic for TargetInsertion {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: Surface = datom_codec::Positional::position(&mut p)?;
        let p2: std::vec::Vec<protos::Text> = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for TargetInsertion {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.2)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Roles(
    pub std::vec::Vec<RoleModule>,
    pub std::vec::Vec<Model>,
    pub std::vec::Vec<RolePermission>,
    pub std::vec::Vec<RoleDepth>,
    pub std::vec::Vec<RoleDescription>,
    pub std::vec::Vec<RoleAlias>,
    pub std::vec::Vec<protos::Text>,
    pub std::vec::Vec<TargetInsertion>,
);
impl datom_codec::Datomic for Roles {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 8)?;
        let p0: std::vec::Vec<RoleModule> = datom_codec::Positional::position(&mut p)?;
        let p1: std::vec::Vec<Model> = datom_codec::Positional::position(&mut p)?;
        let p2: std::vec::Vec<RolePermission> = datom_codec::Positional::position(&mut p)?;
        let p3: std::vec::Vec<RoleDepth> = datom_codec::Positional::position(&mut p)?;
        let p4: std::vec::Vec<RoleDescription> = datom_codec::Positional::position(&mut p)?;
        let p5: std::vec::Vec<RoleAlias> = datom_codec::Positional::position(&mut p)?;
        let p6: std::vec::Vec<protos::Text> = datom_codec::Positional::position(&mut p)?;
        let p7: std::vec::Vec<TargetInsertion> = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5, p6, p7))
    }
}
impl protos::Conceivable<datom_codec::Datom> for Roles {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.2)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.3)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.4)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.5)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.6)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.7)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RolesDocument {
    Roles(Roles),
}
impl datom_codec::Datomic for RolesDocument {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Roles" => std::result::Result::Ok(Self::Roles(datom_codec::Carrying::body(v)?)),
            _ => std::result::Result::Err(datom_codec::Headed::reject(
                &v,
                datom_codec::Problem::UnknownVariant(
                    protos::Word::try_from(v.name).expect("variant name"),
                ),
            )),
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for RolesDocument {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            match self {
                Self::Roles(p0) => datom_codec::Datom::Variant(
                    protos::Symbol::try_from("Roles").expect("static variant"),
                    std::boxed::Box::new(
                        protos::Conceivable::conceive(p0)
                            .expect("infallible datom ascent")
                            .1,
                    ),
                ),
            },
        ))
    }
}
