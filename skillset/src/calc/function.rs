use super::{CalcError, Decimal};
use phf::phf_map;

type Func = fn(&[Decimal]) -> Result<Decimal, CalcError>;

pub static TABLE: phf::Map<&'static str, Func> = phf_map! {
    "abs" => abs,
};

fn abs(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].abs())
}

fn check_args(n: usize, args: &[Decimal]) -> Result<(), CalcError> {
    if n == args.len() {
        Ok(())
    } else {
        Err(CalcError::WrongNumberOfArguments)
    }
}
