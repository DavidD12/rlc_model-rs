use crate::parser::error::*;
use rl_model::model::Model as RlModel;
use rl_model::model::Named as RlNamed;
use std::collections::HashSet;

use super::*;
#[derive(Debug, Clone)]
pub struct Model {
    pub rl_model: RlModel,
    includes: Vec<String>,
    robots: Vec<Robot>,
    types: Vec<RlcType>,
    functions: Vec<Function>,
    declared_methods: Vec<DeclMethod>,
    ros_calls: HashSet<RosCall>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            rl_model: RlModel::default(),
            includes: vec![],
            robots: vec![],
            types: vec![],
            functions: vec![],
            declared_methods: vec![],
            ros_calls: HashSet::new(),
        }
    }
}

impl Model {
    // ----- Include -----
    pub fn add_include<S: Into<String>>(&mut self, file: S) {
        self.includes.push(file.into());
    }

    pub fn includes(&self) -> &Vec<String> {
        &self.includes
    }

    // pub fn skillset_map(&self) -> HashMap<String, SkillsetId> {
    //     let mut map = HashMap::new();
    //     for (i, x) in self.rl_model.skillsets().iter().enumerate() {
    //         map.insert(x.to_string(), x.id());
    //     }
    //     map
    // }

    // ----- Robots -----
    pub fn add_robot(&mut self, mut robot: Robot) -> RobotId {
        let id = RobotId(self.robots.len());
        robot.set_id(id);
        self.robots.push(robot);
        id
    }

    pub fn robots(&self) -> &Vec<Robot> {
        &self.robots
    }

    // ----- Types -----
    pub fn add_type(&mut self, mut rlc_type: RlcType) {
        let id = RlcTypeId(self.types.len());
        rlc_type.set_id(id);
        self.types.push(rlc_type);
    }

    pub fn types(&self) -> &Vec<RlcType> {
        &self.types
    }

    pub fn get_types(&self) -> Vec<RlcType> {
        self.types.clone()
    }

    pub fn get_type(&self, id: RlcTypeId) -> Option<&RlcType> {
        self.types.get(id.index())
    }

    // ----- Function -----

    pub fn add_function(&mut self, mut fun: Function) -> FunctionId {
        let id = FunctionId(self.functions.len());
        fun.set_id(id);
        self.functions.push(fun);
        id
    }

    pub fn get_function(&self, id: FunctionId) -> Option<&Function> {
        self.functions.get(id.index())
    }

    pub fn functions(&self) -> &Vec<Function> {
        &self.functions
    }

    // ----- Declared Methods -----

    pub fn add_declared_method(&mut self, mut decl: DeclMethod) -> DeclMethodId {
        let id = DeclMethodId(self.declared_methods.len());
        decl.set_id(id);
        self.declared_methods.push(decl);
        id
    }

    pub fn get_declared_method(&self, id: DeclMethodId) -> Option<&DeclMethod> {
        self.declared_methods.get(id.index())
    }

    // ----- Ros Call -----

    pub fn add_ros_call(&mut self, topic: String, typ: RosCallType) {
        self.ros_calls.insert(RosCall::new(topic, typ));
    }

    pub fn ros_calls(&self) -> &HashSet<RosCall> {
        &self.ros_calls
    }

    // ---------- ----------
    pub fn duplicate(&self) -> Result<(), RlcError> {
        // Includes
        for (i, s1) in self.includes.iter().enumerate() {
            for s2 in self.includes.iter().skip(i + 1) {
                if s1 == s2 {
                    return Err(RlcError::DuplInclude { name: s1.clone() });
                }
            }
        }

        // Robots
        for (i, r1) in self.robots.iter().enumerate() {
            for r2 in self.robots.iter().skip(i + 1) {
                if r1.name() == r2.name() {
                    return Err(RlcError::Duplicate {
                        name: r1.name().to_string().clone(),
                        first: r1.position().clone(),
                        second: r2.position().clone(),
                    });
                }
            }
        }

        // Types within self
        for (i, t1) in self.types().iter().enumerate() {
            for t2 in self.types().iter().skip(i + 1) {
                if t1.name() == t2.name() {
                    return Err(RlcError::Duplicate {
                        name: t1.name().to_string().clone(),
                        first: t1.position().clone(),
                        second: t2.position().clone(),
                    });
                }
            }
        }
        // Types within included skillsets
        for t1 in self.types().iter() {
            for t2 in self.rl_model.types().iter() {
                if t1.to_string() == t2.to_string() {
                    return Err(RlcError::DuplType {
                        name: t1.name().to_string().clone(),
                    });
                }
            }
        }

        // Functions
        for (i, fn1) in self.functions.iter().enumerate() {
            for fn2 in self.functions.iter().skip(i + 1) {
                if fn1.name() == fn2.name() {
                    return Err(RlcError::Duplicate {
                        name: (fn1.name().to_string().clone()),
                        first: (fn1.position().clone()),
                        second: (fn2.position().clone()),
                    });
                }
            }
        }
        // Declared Methods
        for (i, decl1) in self.declared_methods.iter().enumerate() {
            for decl2 in self.declared_methods.iter().skip(i + 1) {
                if decl1.name() == decl2.name() {
                    return Err(RlcError::Duplicate {
                        name: (decl1.name().to_string().clone()),
                        first: (decl1.position().clone()),
                        second: (decl2.position().clone()),
                    });
                }
            }
        }

        Ok(())
    }

    pub fn resolve_parameters(&mut self) -> Result<(), RlcError> {
        // resolve function content
        let rlc_types = self.get_types();
        for func in self.functions.iter_mut() {
            // parameters
            for param in func.parameters_mut().iter_mut() {
                match param.typ().clone() {
                    Type::Unresolved(name, _pos) => {
                        for s in self.rl_model.skillsets().iter() {
                            if s.to_string() == name.clone() {
                                param.set_type(Type::Skillset(RlNamed::id(s)));
                            }
                        }
                        for rl_t in self.rl_model.types().iter() {
                            if rl_t.name() == name.clone() {
                                param.set_type(Type::Type(rl_t.id()));
                            }
                        }
                        for rlc_t in rlc_types.iter() {
                            if rlc_t.name() == name.clone() {
                                param.set_type(Type::RlcType(rlc_t.id()));
                            }
                        }
                    }
                    _ => (),
                }
                match param.typ() {
                    Type::Unresolved(name, pos) => {
                        return Err(RlcError::Resolve {
                            element: format!("type '{}'", name),
                            position: pos.clone(),
                        })
                    }
                    _ => (),
                };
            }
        }
        Ok(())
    }

    pub fn resolve_expr(&mut self) -> Result<(), RlcError> {
        let model = self.clone();
        for func in self.functions.iter_mut() {
            print!("Func:{}\n", func.name());
            func.resolve_expr(&model)?;
        }
        Ok(())
    }

    pub fn resolve(&mut self) -> Result<(), RlcError> {
        print!("Params\n");
        self.resolve_parameters()?;
        print!("Functions\n");
        self.resolve_expr()?;
        Ok(())
    }

    // fn resolve_robots(&mut self) -> Result<(), RlcError> {
    // for r in self.robots.iter_mut() {
    //     for s in self.rl_model.skillsets().iter() {
    //         match r.skillset() {
    //             (name, _pos) => {
    //                 if *name == s.to_string() {
    //                     r.set_skillset_type(RlNamed::id(s));
    //                 }
    //             }
    //             _ => (),
    //         }
    //     }
    //     match r.skillset_type() {
    //         Type::Unresolved(name, pos) => {
    //             return Err(RlcError::Resolve {
    //                 element: format!("type '{}'", name),
    //                 position: pos.clone(),
    //             })
    //         }
    //         _ => (),
    //     };
    // }
    // Ok(())
    // }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.includes.iter() {
            writeln!(f, "include \"{}\"", x)?;
        }
        for x in self.robots.iter() {
            writeln!(f, "{}", x.to_lang(self))?;
        }
        for x in self.types.iter() {
            writeln!(f, "type {}", x)?;
        }
        write!(f, "\n")?;
        for x in self.functions.iter() {
            writeln!(f, "{}", x.to_lang(self))?;
        }
        for x in self.declared_methods.iter() {
            writeln!(f, "{}", x.to_lang(self))?;
        }
        Ok(())
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<FunctionId, Function> for Model {
    fn get(&self, id: FunctionId) -> Option<&Function> {
        self.get_function(id)
    }
}
