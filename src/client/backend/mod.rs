use crate::client::span::SpanId;

pub mod disk;
pub mod encryption;
pub mod erasure_coding;
pub mod in_memory;
pub mod metrics;
pub mod network_node;
pub mod replication;

pub trait FarMemoryBackend: Send + Sync {
    // far memory backend performs prepend, so when swapping in we can append to existing memory, which is
    //  faster.
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool);
    fn swap_in(&self, id: &SpanId) -> Vec<u8>;

    fn batch_swap_out(&self, swap_out_operations: Vec<SwapOutOperation>) {
        self.batch(swap_out_operations, None);
    }

    fn batch(&self, swap_out_operations: Vec<SwapOutOperation>, swap_in: Option<&SpanId>) -> Option<Vec<u8>> {
        swap_out_operations.iter().for_each(|op| self.swap_out(op.id.clone(), &op.data, op.prepend));
        swap_in.map(|v| self.swap_in(&v))
    }

    fn on_stop(&self) {}
}

pub struct SwapOutOperation {
    id: SpanId,
    data: Vec<u8>,
    prepend: bool,
}

impl SwapOutOperation {
    pub fn new(id: SpanId, data: Vec<u8>, prepend: bool) -> Self {
        Self {
            id,
            data,
            prepend,
        }
    }
}

enum AirplaneState {
    Flying {
        speed: f64,
    },
    Boarding {
        passengers: u32,
    },
    Repair {
        reason: String
    },
    Sleeping,
}

fn do_something() -> Option<i32> {
    None
}

fn main() -> Result<(), String> {
    let t = match do_something() {
      Some(t) => t,
      None => panic!("!!!!"),
    };
    let x = t + 10;

    Ok(())
}
