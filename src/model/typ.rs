use super::*;
use crate::parser::Position;
use rl_model::model::Named;
use rl_model::model::{SkillsetId, TypeId};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    //
    Undefined,
    Unresolved(String, Option<Position>),
    //
    Boolean,
    //
    Skillset(SkillsetId),
    Type(TypeId),
    RlcType(RlcTypeId),
}

impl ToLang for Type {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            Type::Unresolved(name, _) => format!("{}?", name),
            Type::Undefined => "undef".into(),
            //
            Type::Skillset(id) => {
                let x = model.rl_model.get_skillset(*id).unwrap();
                x.name().into()
            }
            Type::Type(id) => {
                let x = model.rl_model.get_type(*id).unwrap();
                x.name().into()
            }
            Type::RlcType(id) => {
                let x = model.get_type(*id).unwrap();
                x.name().into()
            }
            Type::Boolean => todo!(),
        }
    }
}
