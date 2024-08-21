import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AssetBased } from "../target/types/asset_based";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";
import { expect } from "chai";

export async function wrap_tokens(
  amount: number,
  decimals: number,
  wrapper: anchor.web3.PublicKey,
  approver: anchor.web3.PublicKey,
  owner_to_account: anchor.web3.PublicKey,
  owner_from_token_account: anchor.web3.Signer,
  to_wrapped_token_account: anchor.web3.PublicKey,
  mint: anchor.web3.PublicKey,
  from_token_account: anchor.web3.PublicKey,
  tokenProgram: anchor.web3.PublicKey,
  program: Program<AssetBased>,
) {
  const tx = await program.methods
    .wrapTokens(new anchor.BN(amount), decimals)
    .accountsPartial({
      toWrappedTokenAccount: to_wrapped_token_account,
      ownerToAccount: owner_to_account,
      fromTokenAccount: from_token_account,
      ownerFromTokenAccount: owner_from_token_account.publicKey,
      wrapperAccount: wrapper,
      approver: approver,
      mint: mint,
      tokenProgram: tokenProgram,
    })
    .signers([owner_from_token_account])
    .rpc();

  console.log(`Transfer of ${amount} tx : ${tx}`);
}



export async function unwrap_tokens(
  amount: number,
  decimals: number,
  wrapper: anchor.web3.PublicKey,
  approver: anchor.web3.PublicKey,
  owner_to: anchor.web3.PublicKey,
  owner_token_account: anchor.web3.Signer,
  to_wrapped_token_account: anchor.web3.PublicKey,
  mint: anchor.web3.PublicKey,
  to_token_account: anchor.web3.PublicKey,
  exit_regulator: anchor.web3.Signer,
  program: Program<AssetBased>,
  tokenProgram: anchor.web3.PublicKey = TOKEN_PROGRAM_ID
) {
  const tx = await program.methods
    .unwrapTokens(new anchor.BN(amount), decimals)
    .accountsPartial({
      userWrappedTokenAccount: to_wrapped_token_account,
      owner: owner_to,
      toTokenAccount: to_token_account,
      ownerTokenAccount: owner_token_account.publicKey,
      wrapperAccount: wrapper,
      approver: approver,
      exitRegulator: exit_regulator.publicKey,
      mint: mint,
      tokenProgram: tokenProgram,
    })
    .signers([owner_token_account,exit_regulator])
    .rpc();

  console.log(`Transfer of ${amount} tx : ${tx}`);
}

