use crustabri::{
    aa::AAFramework,
    solvers::{
        CompleteSemanticsSolver, CredulousAcceptanceComputer, GroundedSemanticsSolver,
        PreferredSemanticsSolver, SkepticalAcceptanceComputer, StableSemanticsSolver,
    },
};
use ipafair_sys::semantics;

#[derive(Copy, Clone)]
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

impl From<IpafairSolverSemantics> for semantics {
    fn from(sem: IpafairSolverSemantics) -> Self {
        match sem {
            IpafairSolverSemantics::CO => 1,
            IpafairSolverSemantics::PR => 2,
            IpafairSolverSemantics::ST => 3,
        }
    }
}

impl IpafairSolverSemantics {
    fn credulous_acceptance_solver<'a>(
        &self,
        af: &'a AAFramework<usize>,
    ) -> Box<dyn CredulousAcceptanceComputer<usize> + 'a> {
        match self {
            IpafairSolverSemantics::CO | IpafairSolverSemantics::PR => {
                Box::new(CompleteSemanticsSolver::new(af))
            }
            IpafairSolverSemantics::ST => Box::new(StableSemanticsSolver::new(af)),
        }
    }

    fn skeptical_acceptance_solver<'a>(
        &self,
        af: &'a AAFramework<usize>,
    ) -> Box<dyn SkepticalAcceptanceComputer<usize> + 'a> {
        match self {
            IpafairSolverSemantics::CO => Box::new(GroundedSemanticsSolver::new(af)),
            IpafairSolverSemantics::PR => Box::new(PreferredSemanticsSolver::new(af)),
            IpafairSolverSemantics::ST => Box::new(StableSemanticsSolver::new(af)),
        }
    }
}

#[derive(Default)]
pub struct IpafairSolver {
    af: AAFramework<usize>,
    semantics: Option<IpafairSolverSemantics>,
    assumption: Option<usize>,
}

impl IpafairSolver {
    pub fn set_semantics(&mut self, sem: IpafairSolverSemantics) {
        if self.semantics.is_some() {
            panic!("the semantics is already defined")
        }
        self.semantics = Some(sem)
    }

    pub fn add_argument(&mut self, arg: usize) {
        self.af.new_argument(arg);
    }

    pub fn remove_argument(&mut self, arg: usize) {
        self.af.remove_argument(&arg).expect("no such argument");
    }

    pub fn add_attack(&mut self, attacker: usize, attacked: usize) {
        self.af
            .new_attack(&attacker, &attacked)
            .expect("no such arguments");
    }

    pub fn remove_attack(&mut self, attacker: usize, attacked: usize) {
        self.af
            .remove_attack(&attacker, &attacked)
            .expect("no such arguments");
    }

    pub fn add_assumption(&mut self, arg: usize) {
        if self.assumption.replace(arg).is_some() {
            panic!("an assumption is already present")
        }
    }

    pub fn check_credulous_acceptance_of_assumptions(&mut self) -> bool {
        let arg = self
            .af
            .argument_set()
            .get_argument(&self.assumption.take().expect("missing assumption"))
            .expect("no such argument");
        let mut solver = self
            .semantics
            .expect("the semantics is not defined")
            .credulous_acceptance_solver(&self.af);
        solver.is_credulously_accepted(arg)
    }

    pub fn check_skeptical_acceptance_of_assumptions(&mut self) -> bool {
        let arg = self
            .af
            .argument_set()
            .get_argument(&self.assumption.take().expect("missing assumption"))
            .expect("no such argument");
        let mut solver = self
            .semantics
            .expect("the semantics is not defined")
            .skeptical_acceptance_solver(&self.af);
        solver.is_skeptically_accepted(arg)
    }
}
