import { Program } from "@coral-xyz/anchor";
import { initialize_wrapper } from "../tests/Initialize_tests";
import * as dotenv from "dotenv";
import { AssetBased } from "../target/types/asset_based";
dotenv.config();

async function main(){

    anchor.setProvider(anchor.AnchorProvider.local());

    const program = anchor.workspace.AnoncredsSolana as Program<AssetBased>;

    const approver = anchor.web3.Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(process.env.APPROVER))
      )
      // After make this script connect to a wallet instead (or do a quick frontend for it and use the wallet connect functionnalities)
      // To connect with a ledger and only sign using a ledger
      // Or use a multisig wallet

    const wrapper_pda = await initialize_wrapper(
        anchor.Wallet.local().payer,
        [anchor.Wallet.local().payer.publicKey],
        [anchor.Wallet.local().payer.publicKey],
        approver,
        program
    );

}


main().catch(e => console.log(e)).then(() => console.log("finished"));

