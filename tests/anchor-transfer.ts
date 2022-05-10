import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorTransfer } from "../target/types/anchor_transfer";

describe("anchor-transfer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorTransfer as Program<AnchorTransfer>;
  
  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);
    const from = program.provider.wallet.payer;
    console.log("from",from.publicKey.toString());
    const to = anchor.web3.Keypair.generate();
    console.log("To Address",to.publicKey.toString());
   let tx=  await program.rpc.transferNativeSol({
      accounts: {
        from: from.publicKey,
        to: to.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [from]
    
    });
  console.log("Transaction Hash",tx);
  

  });
});
