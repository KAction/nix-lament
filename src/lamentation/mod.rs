use crate::types;
#[allow(non_snake_case)]
mod D001;
#[allow(non_snake_case)]
mod PY001;
#[allow(non_snake_case)]
mod RFC0169;

pub static MODULES: &[types::Module] = &[D001::MODULE, PY001::MODULE, RFC0169::MODULE];
