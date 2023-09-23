use crate::engine::{Block, paragraph};

/**
 * "Hydra : Resilient and Highly Available Remote Memory"
 * see https://www.usenix.org/system/files/fast22-lee.pdf
 * (currently at page 7).
 * 
 * cons:
 * - it uses RDMA (requires NIC that supports it).
 *
 * challenges:
 * - expanded failure domains.
 * - tail at scale.
 * 
 * existing solutions address by:
 * - local disk backup (access latency is too high).
 * - remote in-memory replication (cost is too high).
 * - remote in-memory erasure coding.
 * - compression.
 * 
 * Erasure coding works better for larger chunks for data.
 * CodingSets - coding group placement algorithm for erasure-coded data.
 * 
 * components:
 * - resilience manager - coordinates erasure-coded resilience operations during remote read/write.
 * - resource monitor - handles the memory management in a remote machine.
 * 
 * improving reliability:
 * - different modes of erasure coding
 * 
 * improving latency:
 * - during remote write, resilience manager applies erasure coding to splits, encodes them using Reed-Solomon codes
 * and decreases latency by avoiding the batch waiting time.
 * - resilience manager sends the data splits first, then encodes and sens the parity splits asynchronously.
 */
pub fn hydra() -> Block {
    Block::Multiple(vec![
        Block::SubsectionHeader("Hydra: Resilient and Highly Available Remote Memory".to_owned()),
        Block::Placeholder(Box::new(paragraph("no text here yet")), "add description of hydra".to_owned()),
    ])
}