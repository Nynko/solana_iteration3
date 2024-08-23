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

    await mint_tokens(
      amount,
      anchor.Wallet.local().payer,
      mint,
      token_address,
      mintAuthority,
      TOKEN_PROGRAM_ID,
      program.provider.connection
    )

    const [wrapped_token_address, bump] =  await anchor.web3.PublicKey.findProgramAddressSync(
            [
              Buffer.from("wrapped_token"),
              wrapper.toBuffer(),
              mint.toBuffer(),
              user_to.toBuffer(),
            ],
            program.programId
          );

    await wrap_tokens(
      amount,
      2,
      wrapper,
      approver.publicKey,
      user_to,
      anchor.Wallet.local().payer,
      wrapped_token_address,
      mint,
      token_address,
      TOKEN_PROGRAM_ID,
      program
    )

}

// Get arguments from the command line
const args = process.argv.slice(2);
if (args.length !== 4) {
  console.error("Usage: ts-node <script> <mint> <wrapper> <amount> <user_to>");
  process.exit(1);
}

const mint = new anchor.web3.PublicKey(args[0]);
const wrapper = new anchor.web3.PublicKey(args[1]);
const amount = parseFloat(args[2]);
const user_to = new anchor.web3.PublicKey(args[3]);




main(mint, wrapper, amount,user_to).catch(e => console.log(e)).then(() => console.log("finished"));

