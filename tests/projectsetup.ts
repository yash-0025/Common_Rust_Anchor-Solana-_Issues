import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Myanchorproject } from "../target/types/myanchorproject";

describe("myanchorproject", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Myanchorproject as Program<Myanchorproject>;
  const baseAccount = anchor.web3.Keypair.generate();

  it("Should initialize account", async () => {
    await program.methods
      .initialize(new anchor.BN(42))
      .accounts({
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([baseAccount])
      .rpc();

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log("Stored data:", account.data.toString()); // Should log: 42
  });
});
