import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { create_spl_mint, create_spl_token_account, initialize_wrapper } from "../tests/Initialize_tests";
import * as dotenv from "dotenv";
import { AssetBased } from "../target/types/asset_based";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
dotenv.config();

async function main(){

    anchor.setProvider(anchor.AnchorProvider.local());

    const program = anchor.workspace.AssetBased as Program<AssetBased>;
    const mintAuthority = anchor.Wallet.local().payer;
    const mintFreezeAuthority = mintAuthority;
    const decimals = 2;
    const mint = await create_spl_mint(
        anchor.Wallet.local().payer,
        mintAuthority,
        mintFreezeAuthority,
        decimals,
        TOKEN_PROGRAM_ID,
        program.provider.connection
    );

    console.log("Mint PDA: ", mint.toBase58());

    const token_account = await create_spl_token_account(
        anchor.Wallet.local().payer,
        anchor.Wallet.local().payer.publicKey,
        mint,
        TOKEN_PROGRAM_ID
      );

    console.log("Token Account: ", token_account.toBase58());
    

}


main().catch(e => console.log(e)).then(() => console.log("finished"));

