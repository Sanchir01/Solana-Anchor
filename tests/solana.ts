import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solana } from "../target/types/solana";
import { PublicKey } from "@solana/web3.js";
describe("solana", () => {
  const provilder = anchor.AnchorProvider.env();
  anchor.setProvider(provilder);

  const program = anchor.workspace.Solana as Program<Solana>;
  const [solanaPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("solana")],
    program.programId
  );

  it("Is initialized!", async () => {
    try {
      const txSign = await program.methods.initialize().rpc();
      const accountData = await program.account.counter.fetch(solanaPDA);

      console.log(`trancsactionSignature: ${txSign}`);
      console.log(`Count : ${accountData.count}`);
    } catch (e) {
      console.log(`initialize error: ${e}`);
    }
  });
  it("Increment", async () => {
    const trancsactionSignature = await program.methods.increment().rpc();

    const accountData = await program.account.counter.fetch(solanaPDA);
    console.log(`trancsactionSignature: ${trancsactionSignature}`);
    console.log(`Count :${accountData.count}`);
  });
});
