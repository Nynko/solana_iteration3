import * as anchor from "@coral-xyz/anchor";
import { AssetBased } from "../target/types/asset_based";


export async function add_pseudo(
    idendity: anchor.web3.PublicKey,
    owner: anchor.web3.Signer,
    pseudo: string,
    program: anchor.Program<AssetBased>,
) {

    const [pseudo_account, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("pseudo"), Buffer.from(pseudo)],
        program.programId
      );

      
    const tx = await program.methods
        .addPseudo(pseudo)
        .accountsPartial({
            owner: owner.publicKey,
            idendity,
            pseudoAccount:pseudo_account,
            payer: owner.publicKey
        })
        .signers([owner])
        .rpc()

    console.log(`Create a pseudo, tx: ${tx}`);
    

    return pseudo_account
}