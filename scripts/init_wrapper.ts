import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { initialize_wrapper } from "../tests/Initialize_tests";
import * as dotenv from "dotenv";
import { AssetBased } from "../target/types/asset_based";
dotenv.config();

async function main(){

    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.AssetBased as Program<AssetBased>;

    const approver = anchor.web3.Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(process.env.APPROVER))
      )

    console.log("Approver: ", approver.publicKey.toBase58());
    
    
    const issuer = new anchor.web3.PublicKey(process.env.ISSUER_ID_PK);

      // After make this script connect to a wallet instead (or do a quick frontend for it and use the wallet connect functionnalities)
      // To connect with a ledger and only sign using a ledger
      // Or use a multisig wallet

    const wrapper_pda = await initialize_wrapper(
        anchor.Wallet.local().payer,
        [issuer],
        [issuer],
        approver,
        program
    );

    console.log("Wrapper PDA: ", wrapper_pda.toBase58());
    

}


main().catch(e => console.log(e)).then(() => console.log("finished"));

