import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { create_spl_mint, initialize_wrapper, mint_tokens } from "../tests/Initialize_tests";
import * as dotenv from "dotenv";
import { AssetBased } from "../target/types/asset_based";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { wrap_tokens } from "../tests/wrapped_tokens_tests";
dotenv.config();

async function main(mint: anchor.web3.PublicKey,wrapper: anchor.web3.PublicKey, amount: number, user_to : anchor.web3.PublicKey){

    anchor.setProvider(anchor.AnchorProvider.local());

    const program = anchor.workspace.AssetBased as Program<AssetBased>;
    const mintAuthority = anchor.Wallet.local().payer;
    const approver = anchor.web3.Keypair.fromSecretKey(
      new Uint8Array(JSON.parse(process.env.APPROVER))
    )

    const token_address = getAssociatedTokenAddressSync(
      mint,
      mintAuthority.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    mint_tokens(
      amount,
      anchor.Wallet.local().payer,
      mint,
      token_address,
      mintAuthority,
      TOKEN_PROGRAM_ID,
      program.provider.connection
    )

    wrap_tokens(
      amount,
      2,
      wrapper,
      approver.publicKey,
      user_to,
      anchor.Wallet.local().payer,
      token_address,
      mint,
      token_address,
      TOKEN_PROGRAM_ID,
      program
    )

    transfer(
      amount,
      mintAuthority,
      user_to,
      mint,
      TOKEN_PROGRAM_ID,
      program.provider.connection
    )


    const transferInstruction = anchor.web3.SystemProgram.transfer({
      fromPubkey: anchor.Wallet.local().payer.publicKey,
      toPubkey: user_to,
      lamports: amount * anchor.web3.LAMPORTS_PER_SOL, // Convert transferAmount to lamports
    });


    // Add the transfer instruction to a new transaction
    let transaction = new anchor.web3.Transaction();
    transaction.add(transferInstruction);

    const txSig = await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      transaction,
      [anchor.Wallet.local().payer]
    );

    console.log(`Transfer of ${amount} SOL to user1 tx : ${txSig}`);

}

// Get arguments from the command line
const args = process.argv.slice(2);
if (args.length !== 4) {
  console.error("Usage: ts-node <script> <mint> <wrapper> <amount> <user_to>");
  process.exit(1);
}

const amount = parseFloat(args[0]);
const user_to = new anchor.web3.PublicKey(args[1]);
const mint = new anchor.web3.PublicKey(args[2]);
const wrapper = new anchor.web3.PublicKey(args[3]);


main(mint, wrapper, amount,user_to).catch(e => console.log(e)).then(() => console.log("finished"));

