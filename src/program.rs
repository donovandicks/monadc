use crate::{
    types::{Value, Vid},
    utils::UniqueIdMaker,
};

#[derive(Debug)]
pub struct Program {
    vid_maker: UniqueIdMaker<Vid>,
    initial_registers: [Value; 4],
    next_input_id: usize,
}

impl Default for Program {
    fn default() -> Self {
        let mut vid_maker = Vid::unique_id_maker();
        let initial_registers = [
            Value::Exact(vid_maker.make_new_id(), 0),
            Value::Exact(vid_maker.make_new_id(), 0),
            Value::Exact(vid_maker.make_new_id(), 0),
            Value::Exact(vid_maker.make_new_id(), 0),
        ];

        Self {
            vid_maker,
            initial_registers,
            next_input_id: 0,
        }
    }
}

impl Program {
    /// Retrieve the initial registers
    pub fn initial_registers(&self) -> [Value; 4] {
        self.initial_registers
    }

    /// Produce a new `Value::Exact`
    pub fn new_exact_value(&mut self, val: i64) -> Value {
        Value::Exact(self.vid_maker.make_new_id(), val)
    }

    /// Produce a new `Value::Unknown`
    pub fn new_unknown_value(&mut self) -> Value {
        Value::Unknown(self.vid_maker.make_new_id())
    }

    /// Produce a new `Value::Input`
    pub fn new_input_value(&mut self) -> Value {
        let next_input_id = self.next_input_id;
        self.next_input_id += 1;
        Value::Input(self.vid_maker.make_new_id(), next_input_id)
    }
}
