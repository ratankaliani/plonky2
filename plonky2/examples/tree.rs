#![allow(clippy::upper_case_acronyms)]

use anyhow::Result;
use plonky2::field::extension::Extendable;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::hash::hash_types::RichField;
use plonky2::hash::merkle_tree::MerkleTree;
use plonky2::hash::merkle_proofs::verify_merkle_proof_to_cap;
fn random_data<F: RichField>(n: usize, k: usize) -> Vec<Vec<F>> {
    (0..n).map(|_| F::rand_vec(k)).collect()
}

fn verify_all_leaves<
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F>,
        const D: usize,
    >(
        leaves: &Vec<Vec<F>>,
        cap_height: usize,
    ) -> Result<()> {
        let tree = MerkleTree::<F, C::Hasher>::new(leaves.clone(), cap_height);
        for (i, leaf) in leaves.into_iter().enumerate() {
            let proof = tree.prove(i);
            verify_merkle_proof_to_cap(leaf.to_vec(), i, &tree.cap, &proof)?;
        }
        Ok(())
}
/// Verify all leaves using merkle tree library
fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let log_n = 8;
    let n = 1 << log_n;
    let leaves = random_data::<F>(n, 7);

    verify_all_leaves::<F, C, D>(&leaves, 1)?;

    println!(
        "Leaves: {}",
        leaves
            .iter()
            .map(|leaf| format!("{:?}", leaf))
            .collect::<Vec<_>>()
            .join(", ")
    );

    Ok(())
}
