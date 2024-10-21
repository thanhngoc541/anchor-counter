import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorCounter } from "../target/types/anchor_counter";
import { expect } from "chai";

describe("anchor-counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  let n = 0.0001234;
  let m = 0.0005678;
  let x = 123456.0;
  let formulation1Expected = 82636.46056518838;
  let formulation2Expected = 19016323.32380647;
  const program = anchor.workspace.AnchorCounter as Program<AnchorCounter>;

  const counter = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({ counter: counter.publicKey })
      .signers([counter])
      .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count).to.equal(0.0);
  });

  it("Formunation 1:", async () => {
    const tx = await program.methods
      .formulation1(n, m, x)
      .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
      .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    console.log(account.count)
    expect(account.count).to.equal(formulation1Expected);
  });

  it("Formunation 2:", async () => {
    const tx = await program.methods
      .formulation2(n, m, x)
      .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
      .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    console.log(account.count)
    expect(account.count).to.equal(formulation2Expected);
  });
});
