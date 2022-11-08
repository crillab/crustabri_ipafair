use crustabri::{AAFramework, Argument};
use ipafair_sys::semantics;

pub enum IpafairSolverSemantics {
    CO,
    PR,
    ST,
}

impl From<semantics> for IpafairSolverSemantics {
    fn from(uint_sem: semantics) -> Self {
        match uint_sem {
            1 => IpafairSolverSemantics::CO,
            2 => IpafairSolverSemantics::PR,
            3 => IpafairSolverSemantics::ST,
            _ => panic!("unexpected semantics index"),
        }
    }
}

#[derive(Default)]
pub struct IpafairSolver {
    semantics: Option<IpafairSolverSemantics>,
    assumptions: Vec<usize>,
    last_extension: Option<Vec<usize>>,
}

impl IpafairSolver {
    pub fn set_semantics(&mut self, sem: IpafairSolverSemantics) {
        if self.semantics.is_some() {
            panic!("the semantics is already defined")
        }
        self.semantics = Some(sem)
    }

    pub fn add_argument(&mut self, arg: usize) {
        todo!()
    }

    pub fn remove_argument(&mut self, arg: usize) {
        todo!()
    }

    pub fn add_attack(&mut self, attacker: usize, attacked: usize) {
        todo!()
    }

    pub fn remove_attack(&mut self, attacker: usize, attacked: usize) {
        todo!()
    }

    pub fn add_assumption(&mut self, arg: usize) {
        self.assumptions.push(arg);
    }

    pub fn check_credulous_acceptance_of_assumptions(&mut self) -> bool {
        self.last_extension = None;
        todo!();
        self.assumptions.clear();
    }

    pub fn check_skeptical_acceptance_of_assumptions(&mut self) -> bool {
        self.last_extension = None;
        todo!();
        self.assumptions.clear();
    }

    pub fn in_last_extension(&self, arg: usize) -> bool {
        self.last_extension.as_ref().unwrap().contains(&arg)
    }
}
