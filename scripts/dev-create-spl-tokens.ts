import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { create_spl_mint, create_spl_token_account, initialize_wrapper } from "../tests/Initialize_tests";
import * as dotenv from "dotenv";
import { AssetBased } from "../target/types/asset_based";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
dotenv.config();

async function main(user: anchor.web3.PublicKey, mint: anchor.web3.PublicKey){

  anchor.setProvider(anchor.AnchorProvider.env());
  

    const token_account = await create_spl_token_account(
        anchor.Wallet.local().payer,
        user,
        mint,
        TOKEN_PROGRAM_ID
      );

    console.log("Token Account: ", token_account.toBase58());
    

}

// Get arguments from the command line
const args = process.argv.slice(2);
if (args.length !== 2) {
  console.error("Usage: ts-node <script> <mint> <user>");
  process.exit(1);
}
const mint = new anchor.web3.PublicKey(args[0]);
const user = new anchor.web3.PublicKey(args[1]);


main(user, mint).catch(e => console.log(e)).then(() => console.log("finished"));

