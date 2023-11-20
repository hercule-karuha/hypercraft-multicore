use crate::HyperResult;

#[derive(Clone, Copy, Debug)]

pub enum HsmFunction {
    HartStart {
        hartid: usize,
        start_addr: usize,
        opaque: usize,
    },
    HartStop,
}

impl HsmFunction {
    pub(crate) fn from_regs(args: &[usize]) -> HyperResult<Self> {
        match args[6] {
            0 => Ok(Self::HartStart {
                hartid: args[0],
                start_addr: args[1],
                opaque: args[2],
            }),
            _ => panic!("Unsupported yet!"),
        }
    }
}
