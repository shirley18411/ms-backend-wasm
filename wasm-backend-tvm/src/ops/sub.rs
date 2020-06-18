use super::types::*;
use tvm_runtime::{Module as _, SystemLibModule};

extern "C" {
    fn __wasm_call_ctors();
}

pub struct TVMSubOp {}

impl TVMSubOp {
    pub fn new() -> Self {
        Self {}
    }
}

impl Operator for TVMSubOp {
    fn init(&self, a_shape: Vec<usize>, b_shape: Vec<usize>, c_shape: Vec<usize>) -> Status {
        if !((a_shape.len() == b_shape.len()
            && a_shape
                .iter()
                .zip(&b_shape)
                .filter(|&(a, b)| a == b)
                .count()
                == a_shape.len())
            && (b_shape.len() == c_shape.len()
                && b_shape
                    .iter()
                    .zip(&c_shape)
                    .filter(|&(b, c)| b == c)
                    .count()
                    == c_shape.len()))
        {
            println!("Both dimension size and shape for Sub operator should be equal!");
            return Status::InitFailed;
        }

        println!("TVM Sub operator init success!");
        Status::Succeed
    }

    fn launch(&self, inputs: Vec<Tensor>, output: Tensor) -> (Status, Tensor) {
        if inputs.len() != 2 {
            println!("Inputs tensor length should be 2!");
            return (Status::LaunchFailed, Tensor::default());
        }
        let mut l_tensor = inputs.get(0).unwrap().as_dltensor();
        let mut r_tensor = inputs.get(1).unwrap().as_dltensor();
        let mut out_tensor = output.as_dltensor();

        unsafe {
            // This is necessary to invoke TVMBackendRegisterSystemLibSymbol
            // API calls.
            __wasm_call_ctors();
        }
        let syslib = SystemLibModule::default();
        let sub = syslib.get_function("sub").expect("sub function not found!");
        call_packed!(sub, &mut l_tensor, &mut r_tensor, &mut out_tensor).unwrap();

        let output: Tensor = out_tensor.into();
        println!("TVM Sub operator run success!");
        (Status::Succeed, output)
    }
}
