use chia_wallet_sdk::prelude::*;

#[derive(Debug, Clone, ToClvm, FromClvm)]
#[clvm(curry)]
pub struct SignatureArgs {
    pub public_key: PublicKey,
}

#[derive(Debug, Clone, ToClvm, FromClvm)]
#[clvm(list)]
pub struct SignatureSolution<P, S> {
    pub delegated_puzzle: P,
    pub delegated_solution: S,
}

compile_rue!(SignatureArgs = SIGNATURE_MOD, ".");

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;

    #[test]
    fn test_signature() -> Result<()> {
        let mut ctx = SpendContext::new();
        let mut sim = Simulator::new();

        let alice = sim.bls(1);
        let alice_p2 = StandardLayer::new(alice.pk);

        let puzzle = ctx.curry(SignatureArgs {
            public_key: alice.pk,
        })?;

        let puzzle_hash = ctx.tree_hash(puzzle).into();

        alice_p2.spend(
            &mut ctx,
            alice.coin,
            Conditions::new().create_coin(puzzle_hash, 1, Memos::None),
        )?;

        let coin = Coin::new(alice.coin.coin_id(), puzzle_hash, 1);

        let delegated_spend =
            ctx.delegated_spend(Conditions::new().create_coin(alice.puzzle_hash, 1, Memos::None))?;

        let solution = ctx.alloc(&SignatureSolution {
            delegated_puzzle: delegated_spend.puzzle,
            delegated_solution: delegated_spend.solution,
        })?;

        ctx.spend(coin, Spend::new(puzzle, solution))?;

        sim.spend_coins(ctx.take(), &[alice.sk])?;

        Ok(())
    }
}
