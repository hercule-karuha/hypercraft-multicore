use crate::HyperResult;

#[derive(Clone, Copy, Debug)]

pub enum HsmFunction {
    HartStart {
        hartid: u64,
        start_addr: u64,
        opaque: u64,
    },
    HartStop,
}

impl HsmFunction {
    pub(crate) fn from_regs(args: &[usize]) -> HyperResult<Self> {
        match args[6] {
            0 => Ok(Self::HartStart {
                hartid: args[0] as u64,
                start_addr: args[1] as u64,
                opaque: args[2] as u64,
            }),
            _ => panic!("Unsupported yet!"),
        }
    }
}
