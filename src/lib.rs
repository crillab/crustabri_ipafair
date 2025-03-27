#![allow(clippy::not_unsafe_ptr_arg_deref)]

use const_cstr::const_cstr;
use crustabri::{
    dynamics::assumptions_on_attacks::{
        DynamicCompleteSemanticsSolverAttacks, DynamicStableSemanticsSolverAttacks,
    },
    dynamics::DummyDynamicConstraintsEncoder,
    solvers::{CompleteSemanticsSolver, StableSemanticsSolver},
};
use ipafair_solver::{FactoryType, IpafairSolver, IpafairSolverSemantics};
use ipafair_sys::semantics;

mod ipafair_solver;

const_cstr! {
    IPAFAIR_SIGNATURE = "ICCMA'23";
}

const STATUS_YES: ::std::os::raw::c_int = 10;
const STATUS_NO: ::std::os::raw::c_int = 20;

#[no_mangle]
pub extern "C" fn ipafair_signature() -> *const ::std::os::raw::c_char {
    IPAFAIR_SIGNATURE.as_ptr()
}

#[no_mangle]
pub extern "C" fn ipafair_init() -> *mut ::std::os::raw::c_void {
    Box::into_raw(Box::<IpafairSolver>::default()) as *mut _
}

#[no_mangle]
pub extern "C" fn ipafair_init_dummy() -> *mut ::std::os::raw::c_void {
    let factory: Box<FactoryType> = Box::new(move |s| match s {
        IpafairSolverSemantics::CO => Box::new(DummyDynamicConstraintsEncoder::new(
            Some(Box::new(|af| Box::new(CompleteSemanticsSolver::new(af)))),
            None,
        )),
        IpafairSolverSemantics::ST => Box::new(DummyDynamicConstraintsEncoder::new(
            Some(Box::new(|af| Box::new(StableSemanticsSolver::new(af)))),
            Some(Box::new(|af| Box::new(StableSemanticsSolver::new(af)))),
        )),
    });
    Box::into_raw(Box::new(IpafairSolver::new_with_factory(factory))) as *mut _
}

#[no_mangle]
pub extern "C" fn ipafair_init_attacks(
    factor: std::os::raw::c_double,
) -> *mut ::std::os::raw::c_void {
    if factor < 1. {
        panic!("factor must be at least 1");
    }
    let factory: Box<FactoryType> = Box::new(move |s| match s {
        IpafairSolverSemantics::CO => Box::new(
            DynamicCompleteSemanticsSolverAttacks::new_with_arg_factor(factor),
        ),
        IpafairSolverSemantics::ST => Box::new(
            DynamicStableSemanticsSolverAttacks::new_with_arg_factor(factor),
        ),
    });
    Box::into_raw(Box::new(IpafairSolver::new_with_factory(factory))) as *mut _
}

#[no_mangle]
pub extern "C" fn ipafair_init_with_external_solver(
    c_path: *const std::os::raw::c_char,
) -> *mut ::std::os::raw::c_void {
    if c_path.is_null() {
        return std::ptr::null_mut();
    }
    let cstr_path = unsafe { std::ffi::CStr::from_ptr(c_path) };
    let path = if let Ok(p) = cstr_path.to_str() {
        p
    } else {
        return std::ptr::null_mut();
    };
    Box::into_raw(Box::new(IpafairSolver::new_with_factory(Box::new(|s| {
        s.new_acceptance_solver_with_external_solver(path)
    })))) as *mut _
}

#[no_mangle]
pub extern "C" fn ipafair_release(solver_ptr: *mut ::std::os::raw::c_void) {
    debug_assert!(!solver_ptr.is_null());
    unsafe {
        drop(Box::from_raw(solver_ptr as *mut IpafairSolver));
    }
}

macro_rules! solver_from_ptr {
    ($ptr:ident) => {
        unsafe {
            debug_assert!(!$ptr.is_null());
            &mut *($ptr as *mut IpafairSolver)
        }
    };
}

fn i32_arg_to_usize(n: i32) -> usize {
    if n <= 0 {
        panic!("invalid argument: {}", n)
    }
    n as usize
}

#[no_mangle]
pub extern "C" fn ipafair_set_semantics(
    solver_ptr: *mut ::std::os::raw::c_void,
    uint_sem: semantics,
) {
    let solver = solver_from_ptr!(solver_ptr);
    let sem = IpafairSolverSemantics::from(uint_sem);
    solver.set_semantics(sem)
}

#[no_mangle]
pub extern "C" fn ipafair_add_argument(solver_ptr: *mut ::std::os::raw::c_void, arg: i32) {
    let solver = solver_from_ptr!(solver_ptr);
    solver.add_argument(i32_arg_to_usize(arg))
}

#[no_mangle]
pub extern "C" fn ipafair_del_argument(solver_ptr: *mut ::std::os::raw::c_void, arg: i32) {
    let solver = solver_from_ptr!(solver_ptr);
    solver.remove_argument(i32_arg_to_usize(arg))
}

#[no_mangle]
pub extern "C" fn ipafair_add_attack(solver_ptr: *mut ::std::os::raw::c_void, s: i32, t: i32) {
    let solver = solver_from_ptr!(solver_ptr);
    solver.add_attack(i32_arg_to_usize(s), i32_arg_to_usize(t))
}

#[no_mangle]
pub extern "C" fn ipafair_del_attack(solver_ptr: *mut ::std::os::raw::c_void, s: i32, t: i32) {
    let solver = solver_from_ptr!(solver_ptr);
    solver.remove_attack(i32_arg_to_usize(s), i32_arg_to_usize(t))
}

#[no_mangle]
pub extern "C" fn ipafair_assume(solver_ptr: *mut ::std::os::raw::c_void, arg: i32) {
    let solver = solver_from_ptr!(solver_ptr);
    solver.add_assumption(i32_arg_to_usize(arg))
}

#[no_mangle]
pub extern "C" fn ipafair_solve_cred(
    solver_ptr: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    let solver = solver_from_ptr!(solver_ptr);
    if solver.check_credulous_acceptance_of_assumptions() {
        STATUS_YES
    } else {
        STATUS_NO
    }
}

#[no_mangle]
pub extern "C" fn ipafair_solve_skept(
    solver_ptr: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    let solver = solver_from_ptr!(solver_ptr);
    if solver.check_skeptical_acceptance_of_assumptions() {
        STATUS_YES
    } else {
        STATUS_NO
    }
}

#[no_mangle]
pub extern "C" fn ipafair_val(solver_ptr: *mut ::std::os::raw::c_void, arg: i32) -> i32 {
    let solver = solver_from_ptr!(solver_ptr);
    if solver.check_presence_in_last_certificate(i32_arg_to_usize(arg)) {
        arg
    } else {
        -arg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;

    fn coreo_test_co_for_solver(solver: *mut c_void) {
        ipafair_set_semantics(solver, semantics::from(IpafairSolverSemantics::CO));
        ipafair_add_argument(solver, 1);
        ipafair_add_argument(solver, 2);
        ipafair_add_attack(solver, 1, 2);
        ipafair_assume(solver, 1);
        assert_eq!(STATUS_YES, ipafair_solve_cred(solver));
        assert_eq!(1, ipafair_val(solver, 1));
        assert_eq!(-2, ipafair_val(solver, 2));
        ipafair_assume(solver, 2);
        assert_eq!(STATUS_NO, ipafair_solve_cred(solver));

        ipafair_del_attack(solver, 1, 2);
        ipafair_assume(solver, 1);
        assert_eq!(STATUS_YES, ipafair_solve_cred(solver));
        assert_eq!(1, ipafair_val(solver, 1));
        assert_eq!(2, ipafair_val(solver, 2));
        ipafair_assume(solver, 2);
        assert_eq!(STATUS_YES, ipafair_solve_cred(solver));
        assert_eq!(1, ipafair_val(solver, 1));
        assert_eq!(2, ipafair_val(solver, 2));

        ipafair_add_argument(solver, 3);
        ipafair_add_attack(solver, 3, 2);
        ipafair_add_attack(solver, 2, 1);
        ipafair_assume(solver, 1);
        assert_eq!(STATUS_YES, ipafair_solve_cred(solver));
        assert_eq!(1, ipafair_val(solver, 1));
        assert_eq!(-2, ipafair_val(solver, 2));
        assert_eq!(3, ipafair_val(solver, 3));
        ipafair_assume(solver, 2);
        assert_eq!(STATUS_NO, ipafair_solve_cred(solver));
        ipafair_assume(solver, 3);
        assert_eq!(STATUS_YES, ipafair_solve_cred(solver));
        assert_eq!(1, ipafair_val(solver, 1));
        assert_eq!(-2, ipafair_val(solver, 2));
        assert_eq!(3, ipafair_val(solver, 3));

        ipafair_del_argument(solver, 1);
        ipafair_add_argument(solver, 4);
        ipafair_add_attack(solver, 4, 3);
        ipafair_add_attack(solver, 3, 4);
        ipafair_assume(solver, 2);
        assert_eq!(STATUS_YES, ipafair_solve_cred(solver));
        assert!(
            ipafair_val(solver, 4) == 4
                && ipafair_val(solver, 2) == 2
                && ipafair_val(solver, 3) == -3
        );
        ipafair_assume(solver, 3);
        assert_eq!(STATUS_YES, ipafair_solve_cred(solver));
        assert!(
            ipafair_val(solver, 4) == -4
                && ipafair_val(solver, 2) == -2
                && ipafair_val(solver, 3) == 3
        );
        ipafair_assume(solver, 4);
        assert_eq!(STATUS_YES, ipafair_solve_cred(solver));
        assert!(
            ipafair_val(solver, 4) == 4
                && ipafair_val(solver, 2) == 2
                && ipafair_val(solver, 3) == -3
        );
    }

    #[test]
    fn coreo_test_co() {
        coreo_test_co_for_solver(ipafair_init());
    }

    #[test]
    fn coreo_test_co_dummy() {
        coreo_test_co_for_solver(ipafair_init_dummy());
    }
}
