import * as anchor from "@coral-xyz/anchor";
import fs from "fs";
import { AssetBased } from "../target/types/asset_based";
import { Program } from "@coral-xyz/anchor";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

export function load_keypair(filename: string): anchor.web3.Keypair {
  const secret = JSON.parse(fs.readFileSync(filename).toString()) as number[];
  const secretKey = Uint8Array.from(secret);
  return anchor.web3.Keypair.fromSecretKey(secretKey);
}

export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function sendTransaction(
  from: anchor.web3.Keypair,
  to: anchor.web3.PublicKey,
  amount: number
) {
  const transaction = new anchor.web3.Transaction().add(
    anchor.web3.SystemProgram.transfer({
      fromPubkey: from.publicKey,
      toPubkey: to,
      lamports: amount,
    })
  );

  // Sign transaction, broadcast, and confirm
  const signature = await anchor.web3.sendAndConfirmTransaction(
    anchor.getProvider().connection,
    transaction,
    [from]
  );

  console.log(`Transfer to ${to.toBase58()} of ${amount} tx : ${signature}`);
}

export async function create_user_with_best_bump(
  mint: anchor.web3.PublicKey,
  wrapper_account: anchor.web3.PublicKey,
  program: Program<AssetBased>,
) : Promise<[anchor.web3.Keypair, anchor.web3.PublicKey]>{
  let owner : anchor.web3.Keypair;
  let wrapped_account : anchor.web3.PublicKey;
  let bump: number; // = 0;

  // while (bump != 255) {


      owner = anchor.web3.Keypair.generate();

      [wrapped_account, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("wrapped_token"),
          wrapper_account.toBuffer(),
          mint.toBuffer(),
          owner.publicKey.toBuffer(),
        ],
        program.programId
      );
  
      // [wrappedToken, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      //   [
      //     Buffer.from('wrapped_token'),
      //     wrapper.toBuffer(),
      //     mint.toBuffer(),
      //     user.publicKey.toBuffer(),
      //   ],
      //   program.programId,
      // );
  // }

  return [owner,wrapped_account];
}

      // wrappedToken = anchor.web3.PublicKey.createProgramAddressSync(
      //   [
      //     Buffer.from('wrapped_token'),
      //     wrapper.toBuffer(),
      //     mint.toBuffer(),
      //     user.publicKey.toBuffer(),
      //     Buffer.from([bump]),
      //   ],
      //   program.programId,
      // );

export async function create_approver_with_best_bump(
  program: Program<AssetBased>,
) {
  let approver : anchor.web3.Keypair;
  let wrapper : anchor.web3.PublicKey;
  let onCurve = true;
  const bump = 255;
  while (onCurve) {
    try {
      approver = anchor.web3.Keypair.generate();
      wrapper = anchor.web3.PublicKey.createProgramAddressSync(
        [
          Buffer.from("wrapper"),
          approver.publicKey.toBuffer(),
          Buffer.from([bump]),
        ],
        program.programId
      );
      if (!anchor.web3.PublicKey.isOnCurve(wrapper)){
        onCurve = false;
      }

    } catch (error) {
      continue;
    }
  }

  console.log("wrapper:  ", wrapper.toBase58());
  
  
  return approver;
}
