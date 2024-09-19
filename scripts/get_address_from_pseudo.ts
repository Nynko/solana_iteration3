import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { create_spl_mint, initialize_wrapper, mint_tokens } from "../tests/Initialize_tests";
import * as dotenv from "dotenv";
import { AssetBased } from "../target/types/asset_based";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { wrap_tokens } from "../tests/wrapped_tokens_tests";
dotenv.config();

async function main(psuedo: string){

  anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.AssetBased as Program<AssetBased>;
   await getAddressFromPseudo(psuedo, program).then((address) => {
    console.log("Address: ", address?.toBase58());
   });
}

// Get arguments from the command line
const args = process.argv.slice(2);
if (args.length !== 1) {
  console.error("Usage: ts-node <pseudo>");
  process.exit(1);
}

const pseudo = args[0]

main(pseudo).catch(e => console.log(e)).then(() => console.log("finished"));

export async function getAddressFromPseudo(
    pseudo: string,
    program: anchor.Program<AssetBased>,
  ): Promise<anchor.web3.PublicKey | null> {
    const [pseudo_account, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('pseudo'), Buffer.from(pseudo)],
      program.programId,
    );
  
    const account = await program.account.pseudoAccount.fetchNullable(
      pseudo_account,
    );
  
    return account ? account.owner : null;
  }