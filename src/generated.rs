#![allow(dead_code)]
pub struct Configuration(pub protos::Text, pub protos::Text);
impl datomic::Corporal<datomic::Datom> for Configuration {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 2usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(2i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for Configuration {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
        ])
    }
}
pub enum Request {
    Generate(Configuration),
    Check(Configuration),
    Visualize(Configuration),
}
impl datomic::Corporal<datomic::Datom> for Request {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Variant(head, protos::Separator::Period, Some(body))
                if head == stringify!(Generate) =>
            {
                Ok(Self::Generate(<Configuration as datomic::Corporal<
                    datomic::Datom,
                >>::incorporate(*body)?))
            }
            datomic::Datom::Variant(head, protos::Separator::Period, Some(body))
                if head == stringify!(Check) =>
            {
                Ok(Self::Check(<Configuration as datomic::Corporal<
                    datomic::Datom,
                >>::incorporate(*body)?))
            }
            datomic::Datom::Variant(head, protos::Separator::Period, Some(body))
                if head == stringify!(Visualize) =>
            {
                Ok(Self::Visualize(<Configuration as datomic::Corporal<
                    datomic::Datom,
                >>::incorporate(*body)?))
            }
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}
impl datomic::Datomic for Request {
    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::Generate(value) => datomic::Datom::Variant(
                stringify!(Generate).to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(value))),
            ),
            Self::Check(value) => datomic::Datom::Variant(
                stringify!(Check).to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(value))),
            ),
            Self::Visualize(value) => datomic::Datom::Variant(
                stringify!(Visualize).to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(value))),
            ),
        }
    }
}
pub struct OutputGenerated(pub protos::Integer, pub protos::Integer);
pub struct OutputChecked(pub protos::Integer, pub protos::Integer);
pub struct OutputVisualized(pub protos::Integer, pub protos::Integer);
pub enum Output {
    Generated(OutputGenerated),
    Checked(OutputChecked),
    Visualized(OutputVisualized),
}
impl datomic::Corporal<datomic::Datom> for Output {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Variant(head, protos::Separator::Period, Some(body))
                if head == stringify!(Generated) =>
            {
                Ok(Self::Generated(<OutputGenerated as datomic::Corporal<
                    datomic::Datom,
                >>::incorporate(*body)?))
            }
            datomic::Datom::Variant(head, protos::Separator::Period, Some(body))
                if head == stringify!(Checked) =>
            {
                Ok(Self::Checked(<OutputChecked as datomic::Corporal<
                    datomic::Datom,
                >>::incorporate(*body)?))
            }
            datomic::Datom::Variant(head, protos::Separator::Period, Some(body))
                if head == stringify!(Visualized) =>
            {
                Ok(Self::Visualized(<OutputVisualized as datomic::Corporal<
                    datomic::Datom,
                >>::incorporate(*body)?))
            }
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}
impl datomic::Datomic for Output {
    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::Generated(value) => datomic::Datom::Variant(
                stringify!(Generated).to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(value))),
            ),
            Self::Checked(value) => datomic::Datom::Variant(
                stringify!(Checked).to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(value))),
            ),
            Self::Visualized(value) => datomic::Datom::Variant(
                stringify!(Visualized).to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(value))),
            ),
        }
    }
}
impl datomic::Corporal<datomic::Datom> for OutputGenerated {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 2usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Integer as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <protos::Integer as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(2i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for OutputGenerated {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
        ])
    }
}
impl datomic::Corporal<datomic::Datom> for OutputChecked {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 2usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Integer as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <protos::Integer as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(2i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for OutputChecked {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
        ])
    }
}
impl datomic::Corporal<datomic::Datom> for OutputVisualized {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 2usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Integer as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <protos::Integer as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(2i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for OutputVisualized {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
        ])
    }
}
pub struct GeneratedRoleOutputs(pub Vec<protos::Text>);
impl datomic::Corporal<datomic::Datom> for GeneratedRoleOutputs {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 1usize => {
                let mut iter = fields.into_iter();
                Ok(Self(<Vec<protos::Text> as datomic::Corporal<
                    datomic::Datom,
                >>::incorporate(iter.next().unwrap())?))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(1i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for GeneratedRoleOutputs {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![datomic::Datomic::datomize(&self.0)])
    }
}
pub enum Provider {
    Claude,
    ChatGpt,
}
impl datomic::Corporal<datomic::Datom> for Provider {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Bare(s) if s == stringify!(Claude) => Ok(Self::Claude),
            datomic::Datom::Bare(s) if s == stringify!(ChatGpt) => Ok(Self::ChatGpt),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}
impl datomic::Datomic for Provider {
    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::Claude => datomic::Datom::Bare(stringify!(Claude).to_owned()),
            Self::ChatGpt => datomic::Datom::Bare(stringify!(ChatGpt).to_owned()),
        }
    }
}
pub enum Permission {
    Restricted,
    Unrestricted,
}
impl datomic::Corporal<datomic::Datom> for Permission {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Bare(s) if s == stringify!(Restricted) => Ok(Self::Restricted),
            datomic::Datom::Bare(s) if s == stringify!(Unrestricted) => Ok(Self::Unrestricted),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}
impl datomic::Datomic for Permission {
    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::Restricted => datomic::Datom::Bare(stringify!(Restricted).to_owned()),
            Self::Unrestricted => datomic::Datom::Bare(stringify!(Unrestricted).to_owned()),
        }
    }
}
pub enum Effort {
    Low,
    Medium,
    High,
    Xhigh,
}
impl datomic::Corporal<datomic::Datom> for Effort {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Bare(s) if s == stringify!(Low) => Ok(Self::Low),
            datomic::Datom::Bare(s) if s == stringify!(Medium) => Ok(Self::Medium),
            datomic::Datom::Bare(s) if s == stringify!(High) => Ok(Self::High),
            datomic::Datom::Bare(s) if s == stringify!(Xhigh) => Ok(Self::Xhigh),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}
impl datomic::Datomic for Effort {
    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::Low => datomic::Datom::Bare(stringify!(Low).to_owned()),
            Self::Medium => datomic::Datom::Bare(stringify!(Medium).to_owned()),
            Self::High => datomic::Datom::Bare(stringify!(High).to_owned()),
            Self::Xhigh => datomic::Datom::Bare(stringify!(Xhigh).to_owned()),
        }
    }
}
pub enum Surface {
    ClaudeAgent,
    CodexAgent,
    PiAgent,
}
impl datomic::Corporal<datomic::Datom> for Surface {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Bare(s) if s == stringify!(ClaudeAgent) => Ok(Self::ClaudeAgent),
            datomic::Datom::Bare(s) if s == stringify!(CodexAgent) => Ok(Self::CodexAgent),
            datomic::Datom::Bare(s) if s == stringify!(PiAgent) => Ok(Self::PiAgent),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}
impl datomic::Datomic for Surface {
    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::ClaudeAgent => datomic::Datom::Bare(stringify!(ClaudeAgent).to_owned()),
            Self::CodexAgent => datomic::Datom::Bare(stringify!(CodexAgent).to_owned()),
            Self::PiAgent => datomic::Datom::Bare(stringify!(PiAgent).to_owned()),
        }
    }
}
pub struct ModelChoice(pub protos::Text, pub Option<Effort>);
impl datomic::Corporal<datomic::Datom> for ModelChoice {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 2usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Option<Effort> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(2i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for ModelChoice {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
        ])
    }
}
pub struct RoleModule(pub protos::Text, pub protos::Text);
impl datomic::Corporal<datomic::Datom> for RoleModule {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 2usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(2i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for RoleModule {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
        ])
    }
}
pub struct Model(pub protos::Text, pub Provider, pub Vec<Effort>);
impl datomic::Corporal<datomic::Datom> for Model {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 3usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Provider as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Vec<Effort> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(3i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for Model {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
            datomic::Datomic::datomize(&self.2),
        ])
    }
}
pub struct RolePermission(pub protos::Text, pub protos::Text, pub Permission);
impl datomic::Corporal<datomic::Datom> for RolePermission {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 3usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Permission as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(3i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for RolePermission {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
            datomic::Datomic::datomize(&self.2),
        ])
    }
}
pub struct RoleDepth(pub protos::Text, pub ModelChoice, pub ModelChoice);
impl datomic::Corporal<datomic::Datom> for RoleDepth {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 3usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <ModelChoice as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <ModelChoice as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(3i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for RoleDepth {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
            datomic::Datomic::datomize(&self.2),
        ])
    }
}
pub struct RoleDescription(pub protos::Text, pub protos::Text, pub protos::Text);
impl datomic::Corporal<datomic::Datom> for RoleDescription {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 3usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(3i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for RoleDescription {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
            datomic::Datomic::datomize(&self.2),
        ])
    }
}
pub struct RoleAlias(
    pub protos::Text,
    pub protos::Text,
    pub protos::Text,
    pub protos::Text,
    pub Vec<Surface>,
);
impl datomic::Corporal<datomic::Datom> for RoleAlias {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 5usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Vec<Surface> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(5i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for RoleAlias {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
            datomic::Datomic::datomize(&self.2),
            datomic::Datomic::datomize(&self.3),
            datomic::Datomic::datomize(&self.4),
        ])
    }
}
pub struct TargetInsertion(pub protos::Text, pub Surface, pub Vec<protos::Text>);
impl datomic::Corporal<datomic::Datom> for TargetInsertion {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 3usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <protos::Text as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Surface as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Vec<protos::Text> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(3i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for TargetInsertion {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
            datomic::Datomic::datomize(&self.2),
        ])
    }
}
pub struct Roles(
    pub Vec<RoleModule>,
    pub Vec<Model>,
    pub Vec<RolePermission>,
    pub Vec<RoleDepth>,
    pub Vec<RoleDescription>,
    pub Vec<RoleAlias>,
    pub Vec<protos::Text>,
    pub Vec<TargetInsertion>,
);
impl datomic::Corporal<datomic::Datom> for Roles {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 8usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <Vec<RoleModule> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Vec<Model> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Vec<RolePermission> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Vec<RoleDepth> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Vec<RoleDescription> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Vec<RoleAlias> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Vec<protos::Text> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <Vec<TargetInsertion> as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(8i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for Roles {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
            datomic::Datomic::datomize(&self.2),
            datomic::Datomic::datomize(&self.3),
            datomic::Datomic::datomize(&self.4),
            datomic::Datomic::datomize(&self.5),
            datomic::Datomic::datomize(&self.6),
            datomic::Datomic::datomize(&self.7),
        ])
    }
}
