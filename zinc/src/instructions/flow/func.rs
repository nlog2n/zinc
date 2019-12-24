extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::{Call, Return};

impl<E, O> VMInstruction<E, O> for Call
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.call(self.address, self.inputs_count)
    }
}

impl<E, O> VMInstruction<E, O> for Return
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.ret(self.outputs_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_func() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            // call main
            .add(Call::new(9, 0))
            // func min(field, field) -> field
            .add(Load::new(1))
            .add(Load::new(0))
            .add(Load::new(1))
            .add(Load::new(0))
            .add(Lt)
            .add(ConditionalSelect)
            .add(Return::new(1))
            // func main
            .add(PushConst { value: 42.into() })
            .add(PushConst { value: 5.into() })
            .add(PushConst { value: 3.into() })
            .add(Call::new(2, 2))
            .test(&[3, 42])
    }
}