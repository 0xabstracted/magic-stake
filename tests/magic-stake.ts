import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MagicStake } from "../target/types/magic_stake";

describe("magic-stake", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MagicStake as Program<MagicStake>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
