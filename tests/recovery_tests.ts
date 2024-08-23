import * as anchor from "@coral-xyz/anchor";
import { AssetBased } from "../target/types/asset_based";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_2022_PROGRAM_ID,
    createAssociatedTokenAccountInstruction,
    getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { expect } from "chai";


export interface RecoveryAuthority {
    authority: anchor.web3.PublicKey,
    minSignatures: number,
    minDuration: number,
}

export async function init_recovery(
    authorities: RecoveryAuthority[],
    payer: anchor.web3.Signer,
    owner: anchor.web3.Signer,
    approver: anchor.web3.PublicKey,
    mint: anchor.web3.PublicKey,
    wrapper: anchor.web3.PublicKey,
    program: anchor.Program<AssetBased>,
) {
    const tx = await program.methods
        .initializeRecovery(authorities)
        .accountsPartial({
            payer: payer.publicKey,
            owner: owner.publicKey,
            approver: approver,
            mint: mint,
            wrapperAccount: wrapper,
        })
        .signers([payer, owner])
        .rpc();

    console.log("Your transaction signature for recovery", tx);
}

export async function recover(
  mainRecoverAuthority: anchor.web3.Signer,
  approver: anchor.web3.PublicKey,
  lostAccount: anchor.web3.PublicKey,
  mint: anchor.web3.PublicKey,
  program: anchor.Program<AssetBased>
) {
    const tx = await program.methods
      .recoverAccount()
      .accounts({
        mainRecoveryAuthority : mainRecoverAuthority.publicKey,
        approver,
        owner: lostAccount,
        mint,
      })
      .signers([mainRecoverAuthority])
      .rpc();

    console.log("Your transaction signature for recovery", tx);

}

// export async function test_recovery_missing_signers(
//   args: AccountArgs,
//   program: anchor.Program<AssetBased>
// ) {
//   let user1 = args.users[0];
//   let user2 = args.users[1];
//   let user3 = args.users[2];
//   let issuer = args.issuer;
//   try {
//     const tx = await program.methods
//       .recoverAccount()
//       .accounts({
//         idendity: user1.idendity,
//         owner: user1.owner.publicKey,
//         lastTx: user1.last_tx,
//         tokenAccount: user1.token_account,
//         tokenProgram: TOKEN_2022_PROGRAM_ID,
//         newTokenAccount: user2.token_account,
//         newOwner: user2.owner.publicKey,
//         mint: args.mint,
//         recoveryAuthority: user1.recovery,
//       })
//       .remainingAccounts([
//         { pubkey: user2.owner.publicKey, isSigner: true, isWritable: false },
//       ])
//       .signers([user2.owner])
//       .rpc();

//     console.log("Your transaction signature for recovery", tx);
//     expect.fail("This test should fail");
//   } catch (error) {
//     expect((error as anchor.AnchorError).logs).to.contain(
//       "Program log: AnchorError occurred. Error Code: NotEnoughSignatures. Error Number: 6001. Error Message: Not enough signatures."
//     );
//   }
// }

// export async function test_recovery_already_recovered(
//   args: AccountArgs,
//   program: anchor.Program<AssetBased>
// ) {
//   let user1 = args.users[0];
//   let user2 = args.users[1];
//   let user3 = args.users[2];
//   let issuer = args.issuer;
//   try {
//     const tx = await program.methods
//       .recoverAccount()
//       .accounts({
//         idendity: user1.idendity,
//         owner: user1.owner.publicKey,
//         lastTx: user1.last_tx,
//         tokenAccount: user1.token_account,
//         tokenProgram: TOKEN_2022_PROGRAM_ID,
//         newTokenAccount: user2.token_account,
//         newOwner: user2.owner.publicKey,
//         mint: args.mint,
//         recoveryAuthority: user1.recovery,
//       })
//       .remainingAccounts([
//         { pubkey: user3.owner.publicKey, isSigner: true, isWritable: false },
//         { pubkey: user2.owner.publicKey, isSigner: true, isWritable: false },
//       ])
//       .signers([user3.owner, user2.owner])
//       .rpc();

//     console.log("Your transaction signature for recovery", tx);
//   } catch (error) {
//     expect((error as anchor.AnchorError).logs).to.match(
//       /Program log: AnchorError occurred. Error Code: IdendityAlreadyRecovered. Error Number: 6004. Error Message: Idendity already recovered.|Program log: AnchorError caused by account: .*\. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized./
//     );
//   }
// }

// export async function test_recovery_more_signers(
//   args: AccountArgs,
//   program: anchor.Program<AssetBased>
// ) {
//   let user1 = args.users[0];
//   let user2 = args.users[1];
//   let user3 = args.users[2];
//   try {
//     const tx = await program.methods
//       .recoverAccount()
//       .accounts({
//         idendity: user2.idendity,
//         owner: user2.owner.publicKey,
//         lastTx: user2.last_tx,
//         tokenAccount: user2.token_account,
//         tokenProgram: TOKEN_2022_PROGRAM_ID,
//         newTokenAccount: user3.token_account,
//         newOwner: user3.owner.publicKey,
//         mint: args.mint,
//         recoveryAuthority: user2.recovery,
//       })
//       .remainingAccounts([
//         { pubkey: user1.owner.publicKey, isSigner: true, isWritable: false },
//         { pubkey: user3.owner.publicKey, isSigner: true, isWritable: false },
//         { pubkey: user2.owner.publicKey, isSigner: true, isWritable: false },
//       ])
//       .signers([user1.owner, user2.owner, user3.owner])
//       .rpc();

//     console.log("Your transaction signature for recovery", tx);
//   } catch (error) {
//     console.log(error);
//   }

//   const account = await program.account.idAccount.fetch(user2.idendity);
//   expect(account.recoveredTokenAddress[0].toBase58()).to.be.equal(
//     user3.token_account.toBase58()
//   );
//   const token_account = await program.provider.connection.getAccountInfo(
//     user2.token_account
//   );
//   expect(token_account).to.be.null;
// }
