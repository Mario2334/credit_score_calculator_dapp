import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CreditScoreCalculator } from "../target/types/credit_score_calculator";
import * as assert from "node:assert";
const { SystemProgram } = anchor.web3;

describe("credit-score-calculator", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);


  const program = anchor.workspace.CreditScoreCalculator as Program<CreditScoreCalculator>;

  let _myAccount;

  it("Is initialized!", async () => {
    const myAccount = anchor.web3.Keypair.generate();

    // Add your test here.
    const tx = await program.methods.initialize(
        new anchor.BN(0),
        new anchor.BN(0),
        new anchor.BN(0),
        new anchor.BN(0),
        new anchor.BN(0)
    ).accounts({
      myAccount: myAccount.publicKey,
      user: provider.wallet.publicKey,
    })
        .signers([myAccount]).rpc();
    const account = await program.account.myAccount.fetch(
        myAccount.publicKey
    );
    console.log('Initial count: ', account.creditScore.toString());
    assert.ok(account.creditScore.toNumber() == 300);
    _myAccount = myAccount;
  });

  it('should Updates Volume History', async () => {
    const myAccount = _myAccount;

    const volumeHistory = new anchor.BN(20);
    const tx = await program.methods
        .updateVolumeHistory(volumeHistory)
        .accounts({
          myAccount: myAccount.publicKey,
        })
        .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.myAccount.fetch(myAccount.publicKey);
    console.log("Your Volume History", account.volumeHistory.toString());
    console.log("Credit Score: ", account.creditScore.toString());
    assert.ok(account.volumeHistory.eq(volumeHistory))
    assert.ok(account.creditScore.eq(new anchor.BN(300)))
  });
});
