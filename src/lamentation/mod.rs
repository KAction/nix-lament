use crate::types;
#[allow(non_snake_case)]
mod D001;
#[allow(non_snake_case)]
mod PY001;


pub static MODULES: &[types::Module] = &[
    D001::MODULE,
    PY001::MODULE,
];
