use crustabri::{
    dynamics::{
        assumptions_on_attacks::{
            DynamicCompleteSemanticsSolverAttacks, DynamicStableSemanticsSolverAttacks,
        },
        DummyDynamicConstraintsEncoder, DynamicCompleteSemanticsSolver,
        DynamicPreferredSemanticsSolver, DynamicSolver, DynamicStableSemanticsSolver,
    },
    sat::{ExternalSatSolver, SatSolver, SatSolverFactory},
    solvers::{CredulousAcceptanceComputer, SkepticalAcceptanceComputer},
};
use ipafair_sys::semantics;

#[derive(Copy, Clone)]
pub enum IpafairSolverSemantics {
    CO,
    // PR,
    ST,
}

impl From<semantics> for IpafairSolverSemantics {
    fn from(uint_sem: semantics) -> Self {
        match uint_sem {
            1 => IpafairSolverSemantics::CO,
            // 2 => IpafairSolverSemantics::PR,
            3 => IpafairSolverSemantics::ST,
            _ => panic!("unexpected semantics index"),
        }
    }
}

impl From<IpafairSolverSemantics> for semantics {
    fn from(sem: IpafairSolverSemantics) -> Self {
        match sem {
            IpafairSolverSemantics::CO => 1,
            // IpafairSolverSemantics::PR => 2,
            IpafairSolverSemantics::ST => 3,
        }
    }
}

pub trait IpafairAcceptanceSolver:
    CredulousAcceptanceComputer<usize> + SkepticalAcceptanceComputer<usize> + DynamicSolver<usize>
{
}
impl IpafairAcceptanceSolver for DynamicCompleteSemanticsSolver<usize> {}
impl IpafairAcceptanceSolver for DynamicStableSemanticsSolver<usize> {}
impl IpafairAcceptanceSolver for DynamicPreferredSemanticsSolver<usize> {}
impl IpafairAcceptanceSolver for DummyDynamicConstraintsEncoder<usize> {}
impl IpafairAcceptanceSolver for DynamicCompleteSemanticsSolverAttacks<usize> {}
impl IpafairAcceptanceSolver for DynamicStableSemanticsSolverAttacks<usize> {}

impl IpafairSolverSemantics {
    pub fn new_acceptance_solver<'a>(&self) -> Box<dyn IpafairAcceptanceSolver + 'a> {
        match self {
            IpafairSolverSemantics::CO => Box::new(DynamicCompleteSemanticsSolver::new()),
            // IpafairSolverSemantics::PR => Box::new(DynamicPreferredSemanticsSolver::new()),
            IpafairSolverSemantics::ST => Box::new(DynamicStableSemanticsSolver::new()),
        }
    }

    pub fn new_acceptance_solver_with_external_solver<'a>(
        &self,
        program: &'static str,
    ) -> Box<dyn IpafairAcceptanceSolver + 'a> {
        let solver_factory = Box::new(ExternalSatSolverFactory::new(program.to_owned()));
        match self {
            IpafairSolverSemantics::CO => Box::new(
                DynamicCompleteSemanticsSolver::new_with_sat_solver_factory(solver_factory),
            ),
            // IpafairSolverSemantics::PR => Box::new(DynamicPreferredSemanticsSolver::new()),
            IpafairSolverSemantics::ST => Box::new(
                DynamicStableSemanticsSolver::new_with_sat_solver_factory(solver_factory),
            ),
        }
    }
}

pub struct ExternalSatSolverFactory {
    program: String,
}

impl ExternalSatSolverFactory {
    pub fn new(program: String) -> Self {
        Self { program }
    }
}

impl SatSolverFactory for ExternalSatSolverFactory {
    fn new_solver(&self) -> Box<dyn SatSolver> {
        Box::new(ExternalSatSolver::new(self.program.clone(), vec![]))
    }
}

pub struct IpafairSolver {
    solver: Option<Box<dyn IpafairAcceptanceSolver>>,
    semantics: Option<IpafairSolverSemantics>,
    assumption: Option<usize>,
    certificate: Option<Vec<usize>>,
    factory: Box<FactoryType>,
}

impl Default for IpafairSolver {
    fn default() -> Self {
        Self::new_with_factory(Box::new(|s| s.new_acceptance_solver()))
    }
}

pub type FactoryType = dyn Fn(IpafairSolverSemantics) -> Box<dyn IpafairAcceptanceSolver>;

impl IpafairSolver {
    pub fn new_with_factory(factory: Box<FactoryType>) -> Self {
        Self {
            solver: None,
            semantics: None,
            assumption: None,
            certificate: None,
            factory,
        }
    }

    pub fn set_semantics(&mut self, sem: IpafairSolverSemantics) {
        if self.semantics.is_some() {
            panic!("the semantics is already defined")
        }
        self.semantics = Some(sem);
        self.solver = Some((self.factory)(sem));
    }

    pub fn add_argument(&mut self, arg: usize) {
        self.solver.as_mut().unwrap().new_argument(arg);
    }

    pub fn remove_argument(&mut self, arg: usize) {
        self.solver
            .as_mut()
            .unwrap()
            .remove_argument(&arg)
            .expect("no such argument");
    }

    pub fn add_attack(&mut self, attacker: usize, attacked: usize) {
        self.solver
            .as_mut()
            .unwrap()
            .new_attack(&attacker, &attacked)
            .expect("no such arguments");
    }

    pub fn remove_attack(&mut self, attacker: usize, attacked: usize) {
        self.solver
            .as_mut()
            .unwrap()
            .remove_attack(&attacker, &attacked)
            .expect("no such arguments");
    }

    pub fn add_assumption(&mut self, arg: usize) {
        if self.assumption.replace(arg).is_some() {
            panic!("an assumption is already present")
        }
    }

    pub fn check_credulous_acceptance_of_assumptions(&mut self) -> bool {
        let (status, certificate) = self
            .solver
            .as_mut()
            .unwrap()
            .is_credulously_accepted_with_certificate(
                &self.assumption.take().expect("missing assumption"),
            );
        self.certificate = certificate.map(|v| v.iter().map(|l| *l.label()).collect());
        status
    }

    pub fn check_skeptical_acceptance_of_assumptions(&mut self) -> bool {
        let (status, certificate) = self
            .solver
            .as_mut()
            .unwrap()
            .is_skeptically_accepted_with_certificate(
                &self.assumption.take().expect("missing assumption"),
            );
        self.certificate = certificate.map(|v| v.iter().map(|l| *l.label()).collect());
        status
    }

    pub fn check_presence_in_last_certificate(&self, label: usize) -> bool {
        self.certificate
            .as_ref()
            .unwrap()
            .iter()
            .any(|l| *l == label)
    }
}
