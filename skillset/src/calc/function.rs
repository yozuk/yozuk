use super::{CalcError, Decimal};
use bigdecimal::ToPrimitive;
use phf::phf_map;

type Func = fn(&[Decimal]) -> Result<Decimal, CalcError>;

pub static TABLE: phf::Map<&'static str, Func> = phf_map! {
    "abs" => fn_abs,
    "sqrt" => fn_sqrt,
    "cbrt" => fn_cbrt,
    "sin" => fn_sin,
    "cos" => fn_cos,
    "tan" => fn_tan,
    "asin" => fn_asin,
    "acos" => fn_acos,
    "atan" => fn_atan,
    "atan2" => fn_atan2,
    "sinh" => fn_sinh,
    "cosh" => fn_cosh,
    "tanh" => fn_tanh,
    "asinh" => fn_asinh,
    "acosh" => fn_acosh,
    "atanh" => fn_atanh,
};

fn fn_abs(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].abs())
}

fn fn_sqrt(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().sqrt().into())
}

fn fn_cbrt(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().cbrt().into())
}

fn fn_sin(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().sin().into())
}

fn fn_cos(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().cos().into())
}

fn fn_tan(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().tan().into())
}

fn fn_asin(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().asin().into())
}

fn fn_acos(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().acos().into())
}

fn fn_atan(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().atan().into())
}

fn fn_atan2(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(2, args)?;
    Ok(args[0]
        .to_f64()
        .unwrap()
        .atan2(args[1].to_f64().unwrap())
        .into())
}

fn fn_sinh(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().sinh().into())
}

fn fn_cosh(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().cosh().into())
}

fn fn_tanh(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().tanh().into())
}

fn fn_asinh(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().asinh().into())
}

fn fn_acosh(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().acosh().into())
}

fn fn_atanh(args: &[Decimal]) -> Result<Decimal, CalcError> {
    check_args(1, args)?;
    Ok(args[0].to_f64().unwrap().atanh().into())
}

fn check_args(n: usize, args: &[Decimal]) -> Result<(), CalcError> {
    if n == args.len() {
        Ok(())
    } else {
        Err(CalcError::WrongNumberOfArguments)
    }
}
