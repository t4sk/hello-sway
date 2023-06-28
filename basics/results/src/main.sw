contract;

// Result<T, E> = Ok(T) | Err(E)

enum MathError {
    DivByZero: (),
}

abi MyContract {
    fn div(x: u64, y: u64) -> Result<u64, MathError>;
}

impl MyContract for Contract {
    fn div(x: u64, y: u64) -> Result<u64, MathError> {
        if y == 0 {
            return Result::Err(MathError::DivByZero);
        }

        Result::Ok(x / y)
    }
}
