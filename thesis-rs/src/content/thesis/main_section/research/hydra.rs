use crate::engine::{Block, paragraph};

/**
 * "Hydra : Resilient and Highly Available Remote Memory"
 * see https://www.usenix.org/system/files/fast22-lee.pdf
 * (currently at page 4).
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
 * CodingSets - coding group placement algorithm for erasure-coded data.
 */
pub fn hydra() -> Block {
    Block::Multiple(vec![
        Block::SubsectionHeader("Hydra: Resilient and Highly Available Remote Memory".to_owned()),
        Block::Placeholder(Box::new(paragraph("no text here yet")), "add description of hydra".to_owned()),
    ])
}