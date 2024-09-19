import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { create_spl_mint, initialize_wrapper } from "../tests/Initialize_tests";
import * as dotenv from "dotenv";
import { AssetBased } from "../target/types/asset_based";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
dotenv.config();

async function main(amount: number, user_to : anchor.web3.PublicKey){

  anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.AssetBased as Program<AssetBased>;


    const transferInstruction = anchor.web3.SystemProgram.transfer({
      fromPubkey: anchor.Wallet.local().payer.publicKey,
      toPubkey: user_to,
      lamports: amount * anchor.web3.LAMPORTS_PER_SOL, // Convert transferAmount to lamports
    });


    // Add the transfer instruction to a new transaction
    let transaction = new anchor.web3.Transaction();
    transaction.add(transferInstruction);

    const txSig = await anchor.web3.sendAndConfirmTransaction(
      program.provider.connection,
      transaction,
      [anchor.Wallet.local().payer]
    );

    console.log(`Transfer of ${amount} SOL to user1 tx : ${txSig}`);

}

// Get arguments from the command line
const args = process.argv.slice(2);
if (args.length !== 2) {
  console.error("Usage: ts-node <script> <amount> <user_to>");
  process.exit(1);
}

const amount = parseFloat(args[0]);
const user_to = new anchor.web3.PublicKey(args[1]);


main(amount,user_to).catch(e => console.log(e)).then(() => console.log("finished"));

