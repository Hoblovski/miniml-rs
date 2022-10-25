//! SECD language semantics definition: interpreter.
use std::collections::HashMap;

use super::langdef::{BinOp, BrOp, SECDInstr, SECDVal, UnaOp};

pub struct SECDState(pub usize, pub Vec<SECDVal>, pub Vec<SECDVal>);

#[derive(Debug)]
pub enum SECDEffect {
    Println(String),
}

pub struct SECDMachine {
    pub instrs: Vec<SECDInstr>,
    pub state: SECDState,
    pub effects: Vec<SECDEffect>,
    // TODO: make labels integer. remove this
    pc_from_label: HashMap<String, usize>,
}

pub type SECDStepResult = Result<(), String>;

impl SECDMachine {
    pub fn init(instrs: Vec<SECDInstr>) -> Self {
        let mut pc_from_label = HashMap::new();
        for (i, instr) in instrs.iter().enumerate() {
            if let SECDInstr::Label(label) = instr {
                pc_from_label.insert(label.clone(), i);
            }
        }
        Self {
            instrs,
            state: SECDState(pc_from_label["main"], Vec::new(), Vec::new()),
            pc_from_label,
            effects: Vec::new(),
        }
    }

    pub fn step(&mut self) -> SECDStepResult {
        let SECDState(pc, stk, env) = &mut self.state;
        let instr = self
            .instrs
            .get(*pc)
            .expect(format!("pc {pc} out of range").as_str());
        match instr {
            SECDInstr::Halt => Err("execution halted".to_string()),
            SECDInstr::Pop(n) => {
                *stk = stk[..stk.len() - n].to_vec();
                Ok(())
            }
            SECDInstr::Apply => {
                let arg = stk.pop().expect("cannot pop arg");
                let cl = stk.pop().expect("cannot pop closure");
                if let SECDVal::ClosureVal {
                    focused_fn: Some(focused_fn),
                    mutrec_fns,
                    env: env1,
                } = cl
                {
                    // TODO: builtin
                    stk.push(SECDVal::EnvVal(env.clone()));
                    stk.push(SECDVal::PCVal(*pc + 1));
                    *pc = focused_fn;
                    *env = env1.clone();
                    if mutrec_fns.len() > 0 {
                        env.push(SECDVal::ClosureVal {
                            focused_fn: None,
                            mutrec_fns: mutrec_fns,
                            env: env1.clone(),
                        });
                    }
                    env.push(arg);
                    Ok(())
                } else {
                    Err(format!("apply to non-ready-closure {cl:?}!"))
                }
            }
            SECDInstr::Const(val) => {
                *pc += 1;
                stk.push(val.clone());
                Ok(())
            }
            SECDInstr::Access(n) => {
                *pc += 1;
                let val = env
                    .get(env.len() - *n)
                    .expect(format!("access {n} oob").as_str());
                stk.push(val.clone());
                Ok(())
            }
            SECDInstr::Focus(n) => {
                *pc += 1;
                let cl = stk.pop().unwrap();
                if let SECDVal::ClosureVal {
                    focused_fn,
                    mutrec_fns,
                    env: env1,
                } = cl
                {
                    if let Some { .. } = focused_fn {
                        return Err("re-focusing closure".to_string());
                    }
                    if *n > mutrec_fns.len() {
                        return Err("focus oob".to_string());
                    }
                    stk.push(SECDVal::ClosureVal {
                        focused_fn: Some(mutrec_fns[*n - 1]),
                        mutrec_fns,
                        env: env1,
                    });
                    Ok(())
                } else {
                    Err("focus to non-closure".to_string())
                }
            }
            SECDInstr::Return => {
                let retval = stk.pop().unwrap();
                let retpc = if let SECDVal::PCVal(retpc) = stk.pop().unwrap() {
                    retpc
                } else {
                    return Err("return without valid ret pc".to_string());
                };
                let retenv = if let SECDVal::EnvVal(retenv) = stk.pop().unwrap() {
                    retenv
                } else {
                    return Err("return without valid ret env".to_string());
                };
                *pc = retpc;
                stk.push(retval);
                *env = retenv;
                Ok(())
            }
            SECDInstr::Closure(label) => {
                *pc += 1;
                let focused_fn = Some(self.pc_from_label[label]);
                let cl = SECDVal::ClosureVal {
                    focused_fn: focused_fn,
                    mutrec_fns: Vec::new(),
                    env: env.clone(),
                };
                stk.push(cl);
                Ok(())
            }
            SECDInstr::Closures(labels) => {
                *pc += 1;
                let mutrec_fns = labels.iter().map(|l| self.pc_from_label[l]).collect();
                let cl = SECDVal::ClosureVal {
                    focused_fn: None,
                    mutrec_fns,
                    env: env.clone(),
                };
                env.push(cl);
                Ok(())
            }
            SECDInstr::Builtin(op) => {
                *pc += 1;
                // TODO
                match op.as_str() {
                    "println" => {
                        // let v = stk.pop();
                        // self.effects.push(SECDEffect::Println(format!("{v:?}")));
                        // Ok(())
                        todo!()
                    }
                    _ => todo!(),
                }
            }
            SECDInstr::Binary(op) => {
                *pc += 1;
                let rhs = stk.pop().unwrap();
                let lhs = stk.pop().unwrap();
                let res = Self::eval_binop(*op, lhs, rhs).unwrap();
                stk.push(res);
                Ok(())
            }
            SECDInstr::Unary(op) => {
                *pc += 1;
                let arg = stk.pop().unwrap();
                let res = Self::eval_unaop(*op, arg).unwrap();
                stk.push(res);
                Ok(())
            }
            SECDInstr::Branch(op, label) => {
                let br_dst = self.pc_from_label[label];
                match op {
                    BrOp::Br => {
                        *pc = br_dst;
                        Ok(())
                    }
                    BrOp::BrFalse => {
                        let arg = stk.pop().unwrap();
                        let arg = if let SECDVal::IntVal(v) = arg {
                            v
                        } else {
                            return Err("bad arg for BrFalse".to_string());
                        };
                        if arg == 0 {
                            *pc = br_dst;
                        } else {
                            *pc += 1;
                        }
                        Ok(())
                    }
                    _ => todo!(),
                }
            }
            SECDInstr::Label(_) => {
                *pc += 1;
                Ok(())
            }
            SECDInstr::PushEnv => {
                *pc += 1;
                let v = stk.pop().unwrap();
                env.push(v);
                Ok(())
            }
        }
    }

    fn eval_binop(op: BinOp, lhs: SECDVal, rhs: SECDVal) -> Option<SECDVal> {
        match (lhs, rhs) {
            (SECDVal::IntVal(lhs), SECDVal::IntVal(rhs)) => match op {
                BinOp::Add => Some(SECDVal::IntVal(lhs + rhs)),
                BinOp::Sub => Some(SECDVal::IntVal(lhs - rhs)),
                BinOp::Mul => Some(SECDVal::IntVal(lhs * rhs)),
                BinOp::Div => Some(SECDVal::IntVal(lhs / rhs)),
                BinOp::Rem => Some(SECDVal::IntVal(lhs % rhs)),
                BinOp::Gt => Some(SECDVal::IntVal((lhs > rhs).into())),
                BinOp::Lt => Some(SECDVal::IntVal((lhs < rhs).into())),
                BinOp::Ge => Some(SECDVal::IntVal((lhs >= rhs).into())),
                BinOp::Le => Some(SECDVal::IntVal((lhs <= rhs).into())),
                BinOp::Eq => Some(SECDVal::IntVal((lhs == rhs).into())),
                BinOp::Ne => Some(SECDVal::IntVal((lhs != rhs).into())),
                BinOp::Land => todo!(),
                BinOp::Lor => todo!(),
                BinOp::Lxor => todo!(),
            },
            _ => todo!(),
        }
    }

    fn eval_unaop(_op: UnaOp, _arg: SECDVal) -> Option<SECDVal> {
        todo!()
    }
}
