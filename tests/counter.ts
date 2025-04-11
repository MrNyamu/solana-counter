import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { PublicKey } from "@solana/web3.js";

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  //Creating a counter account
  //Programmed Derived Address - (PDA)
  const [counterPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter")], // Seed
    program.programId // Program ID
  );

  it("Is initialized!", async () => {
    try {
      //Calling initialize instruction
      const txSig = await program.methods.initialize().rpc();

      //We fetch the value of the account data
      const accountData = await program.account.counter.fetch(counterPDA);//Fetching the account data using the counterPDA
      console.log(`Transaction Signature: ${txSig}`);
      console.log(`Count: ${accountData.count}`);
    } catch (error) {
      // If PDA Account already created, then we expect an error
      console.log(error);
    }
  });

  it("Increment", async () => {
    const transactionSignature = await program.methods.increment().rpc();

    const accountData = await program.account.counter.fetch(counterPDA);

    console.log(`Transaction Signature: ${transactionSignature}`);
    console.log(`Count: ${accountData.count}`);
  });
});