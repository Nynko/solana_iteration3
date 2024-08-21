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
  mintTo,
} from "@solana/spl-token";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";
import { expect } from "chai";

export async function create_spl_mint(
  payer: anchor.web3.Signer,
  mintAuthority: anchor.web3.Keypair,
  freezeAuthority: anchor.web3.Keypair,
  decimals: number,
  token_program: anchor.web3.PublicKey = TOKEN_PROGRAM_ID,
  connection: anchor.web3.Connection = anchor.getProvider().connection
): Promise<anchor.web3.PublicKey> {
  const mint = await createMint(
    connection,
    payer,
    mintAuthority.publicKey,
    freezeAuthority.publicKey,
    decimals,
    undefined,
    undefined,
    token_program
  );

  console.log("[Pk] Mint", mint.toBase58());

  return mint;
}

export async function create_spl_token_account(
  payer: anchor.web3.Signer,
  owner: anchor.web3.PublicKey,
  mint: anchor.web3.PublicKey,
  token_program: anchor.web3.PublicKey = TOKEN_PROGRAM_ID
): Promise<anchor.web3.PublicKey> {
  const token_address = getAssociatedTokenAddressSync(
    mint,
    owner,
    false,
    token_program,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  const instruction = createAssociatedTokenAccountInstruction(
    payer.publicKey,
    token_address,
    owner,
    mint,
    token_program,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  const tx = new anchor.web3.Transaction().add(instruction);
  const txSig = await anchor.web3.sendAndConfirmTransaction(
    anchor.getProvider().connection,
    tx,
    [payer]
  );
  console.log("Create token account tx", txSig);

  console.log("[Pk] Token account", token_address.toBase58());

  return token_address;
}

export async function initialize_wrapper(
  payer: anchor.web3.Signer,
  issuers: anchor.web3.PublicKey[],
  exit_regulators: anchor.web3.PublicKey[],
  approver: anchor.web3.Signer,
  program: Program<AssetBased>
): Promise<anchor.web3.PublicKey> {
  const [wrapper_account, bump] =
    await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("wrapper"), approver.publicKey.toBuffer()],
      program.programId
    );

  console.log("[Pk] Wrapper account", wrapper_account.toBase58());

  const tx = await program.methods
    .initializeWrapper(issuers, exit_regulators)
    .accountsPartial({
      payer: payer.publicKey,
      approver: approver.publicKey,
      wrapperAccount: wrapper_account,
    })
    .signers([payer, approver])
    .rpc();

  console.log("Init tx", tx);

  return wrapper_account;
}


// export async function get_wrapped_account(
//   owner: anchor.web3.Signer,
//   mint: anchor.web3.PublicKey,
//   wrapper_account: anchor.web3.PublicKey,
//   program: Program<AssetBased>,
// ): Promise<[anchor.web3.PublicKey, number]> {
//   const [wrapped_account, bump] =
//     await anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("wrapped_token"),
//         wrapper_account.toBuffer(),
//         mint.toBuffer(),
//         owner.publicKey.toBuffer(),
//       ],
//       program.programId
//     );

//   return [wrapped_account,bump];
// }


export function get_wrapped_account(wrapper_pda: anchor.web3.PublicKey, mint: anchor.web3.PublicKey, program: Program<AssetBased>): [anchor.web3.Keypair, anchor.web3.PublicKey] {
  let bump: number = 1;
  let wrapped_account: anchor.web3.PublicKey;
  let user1: anchor.web3.Keypair;
  while (bump != 255) {
    user1 = anchor.web3.Keypair.generate();
    [wrapped_account, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("wrapped_token"),
        wrapper_pda.toBuffer(),
        mint.toBuffer(),
        user1.publicKey.toBuffer(),
      ],
      program.programId
    );
  }

  return [user1, wrapped_account]
}

export async function mint_tokens(
  amount: number,
  payer: anchor.web3.Signer,
  mint: anchor.web3.PublicKey,
  token_account: anchor.web3.PublicKey,
  mintAuthority: anchor.web3.Keypair,
  token_program: anchor.web3.PublicKey,
  connection: anchor.web3.Connection = anchor.getProvider().connection
) {
  const tx = await mintTo(
    connection,
    payer,
    mint,
    token_account,
    mintAuthority,
    amount,
    [],
    undefined,
    token_program
  );

  console.log(
    `Minted ${amount} tokens to ${token_account.toBase58()} tx : ${tx}`
  );
}

export async function initialize_two_auth(
  owner: anchor.web3.Signer,
  idendity: anchor.web3.PublicKey,
  approver: anchor.web3.PublicKey,
  wrapper_account: anchor.web3.PublicKey,
  two_auth_entity: anchor.web3.Signer,
  program: Program<AssetBased>
): Promise<anchor.web3.PublicKey> {
  const [two_auth, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("two_auth"),
      wrapper_account.toBuffer(),
      owner.publicKey.toBuffer(),
    ],
    program.programId
  );

  console.log(wrapper_account);


  console.log("[Pk] Two Auth account", two_auth.toBase58());

  const tx = await program.methods
    .initializeTwoAuth({
      functions: [
        { onMax: { max: new anchor.BN(10) } },
        {
          counterResetOnMax: {
            max: new anchor.BN(10),
            counter: new anchor.BN(0),
          },
        },
        {
          counterResetOnTime: {
            max: new anchor.BN(10),
            duration: { seconds: [1] },
            lastResetTime: new anchor.BN(56),
            counter: new anchor.BN(0),
          },
        },
        {
          counterWithTimeWindow: {
            max: new anchor.BN(10),
            window: {
              duration: { days: [30] },
              lastValueTime: new anchor.BN(0),
              window: [],
              startIndex: 0,
            },
          },
        },
        { deactivateForUserSpecificWhiteList: { whiteList: [] } },
        { always: {} },
      ],
      allowedIssuers: [approver],
    })
    .accountsPartial({
      wrapperAccount: wrapper_account,
      approver: approver,
      owner: owner.publicKey,
      payer: anchor.Wallet.local().publicKey,
      twoAuth: two_auth,
      idendity: idendity,
      twoAuthEntity: two_auth_entity.publicKey,
    })
    .signers([owner, anchor.Wallet.local().payer, two_auth_entity])
    .rpc();

  console.log("Init two_auth tx", tx);

  return two_auth;
}
