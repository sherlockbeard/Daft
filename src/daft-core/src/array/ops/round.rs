use crate::datatypes::{Float32Array, Float64Array};

use super::RoundOps;
use common_error::DaftResult;

impl RoundOps for Float32Array {
    fn round(&self, digits: i32) -> DaftResult<Self> {
        self.apply(|v| v * digits as f32)
    }
}

impl RoundOps for Float64Array {
    fn round(&self, digits: i32) -> DaftResult<Self> {
        self.apply(|v| v * digits as f64)
    }
}
