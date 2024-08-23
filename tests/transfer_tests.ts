import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AssetBased } from "../target/types/asset_based";

export async function transfer_wtokens(
  amount: number,
  wrapper_account: anchor.web3.PublicKey,
  source_owner: anchor.web3.Signer,
  source_wrapped_account: anchor.web3.PublicKey,
  destination_owner: anchor.web3.PublicKey,
  destination_wrapped_account: anchor.web3.PublicKey,
  two_auth: anchor.web3.PublicKey,
  two_auth_signer: anchor.web3.Signer | null,
  mint: anchor.web3.PublicKey,
  approver: anchor.web3.PublicKey,
  token_program: anchor.web3.PublicKey,
  program: Program<AssetBased>
) {
  const instruction = await program.methods
    .transfer(new anchor.BN(amount))
    .accountsPartial({
      sourceOwner: source_owner.publicKey,
      payer: source_owner.publicKey,
      destinationOwner: destination_owner,
      sourceWrappedAccount: source_wrapped_account,
      destinationWrappedAccount: destination_wrapped_account,
      twoAuthSigner: two_auth_signer ? two_auth_signer.publicKey : null,
      twoAuth: two_auth,
      wrapperAccount: wrapper_account,
      mint:mint,
      approver:approver,
      tokenProgram:token_program
    })
    .instruction();

  // Create the priority fee instructions
  // const computePriceIx = anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
  //   microLamports: 10000,
  // });

  // const computeLimitIx = anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
  //   units: 50_000,
  // });

  // const transaction = new anchor.web3.Transaction().add(computePriceIx,computeLimitIx,instruction);
  const transaction = new anchor.web3.Transaction().add(instruction);

  const txSig = await anchor.web3.sendAndConfirmTransaction(
    anchor.getProvider().connection,
    transaction,
    two_auth_signer ? [source_owner, two_auth_signer] : [source_owner]
  );

  console.log(`Transfer (wrapped) of ${amount} tx : ${txSig}`);
}

export async function self_transfer_wtokens(
  amount: number,
  wrapper_account: anchor.web3.PublicKey,
  source_owner: anchor.web3.Signer,
  source_wrapped_account: anchor.web3.PublicKey,
  destination_owner: anchor.web3.PublicKey,
  destination_wrapped_account: anchor.web3.PublicKey,
  two_auth: anchor.web3.PublicKey,
  mint: anchor.web3.PublicKey,
  approver: anchor.web3.PublicKey,
  token_program: anchor.web3.PublicKey,
  program: Program<AssetBased>
) {
  const instruction = await program.methods
    .transfer(new anchor.BN(amount))
    .accountsPartial({
      sourceOwner: source_owner.publicKey,
      payer: source_owner.publicKey,
      destinationOwner: destination_owner,
      sourceWrappedAccount: source_wrapped_account,
      destinationWrappedAccount: destination_wrapped_account,
      twoAuthSigner: null,
      twoAuth: two_auth,
      wrapperAccount: wrapper_account,
      mint:mint,
      approver:approver,
      tokenProgram:token_program
    })
    .instruction();

  // Create the priority fee instructions
  // const computePriceIx = anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
  //   microLamports: 10000,
  // });

  // const computeLimitIx = anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
  //   units: 50_000,
  // });

  // const transaction = new anchor.web3.Transaction().add(computePriceIx,computeLimitIx,instruction);
  const transaction = new anchor.web3.Transaction().add(instruction);

  const txSig = await anchor.web3.sendAndConfirmTransaction(
    anchor.getProvider().connection,
    transaction,
    [source_owner]
  );

  console.log(`Self-Transfer (wrapped) of ${amount} tx : ${txSig}`);
}


export async function transfer_with_partial_sig(
  amount: number,
  wrapper_account: anchor.web3.PublicKey,
  source_owner: anchor.web3.Signer,
  source_wrapped_account: anchor.web3.PublicKey,
  destination_owner: anchor.web3.PublicKey,
  destination_wrapped_account: anchor.web3.PublicKey,
  two_auth: anchor.web3.PublicKey,
  two_auth_signer: anchor.web3.Signer | null,
  mint: anchor.web3.PublicKey,
  approver: anchor.web3.PublicKey,
  token_program: anchor.web3.PublicKey,
  program: Program<AssetBased>
) : Promise<[Buffer, {
  blockhash: anchor.web3.Blockhash;
  lastValidBlockHeight: number;
}]>{
  const instruction = await program.methods
    .transfer(new anchor.BN(amount))
    .accountsPartial({
      sourceOwner: source_owner.publicKey,
      payer: source_owner.publicKey,
      destinationOwner: destination_owner,
      sourceWrappedAccount: source_wrapped_account,
      destinationWrappedAccount: destination_wrapped_account,
      twoAuthSigner: two_auth_signer ? two_auth_signer.publicKey : null,
      twoAuth: two_auth,
      wrapperAccount: wrapper_account,
      mint:mint,
      approver:approver,
      tokenProgram: token_program
    })
    .instruction();

  const transaction = new anchor.web3.Transaction().add(instruction);

  const blockhash = await anchor.getProvider().connection.getLatestBlockhashAndContext();

  transaction.recentBlockhash = blockhash.value.blockhash;
  transaction.lastValidBlockHeight = blockhash.value.lastValidBlockHeight;
  transaction.feePayer = source_owner.publicKey;

  transaction.partialSign(source_owner);

  const serializedTransaction = transaction.serialize({
    requireAllSignatures: false,
  });

  return [serializedTransaction, { blockhash: blockhash.value.blockhash, lastValidBlockHeight: blockhash.value.lastValidBlockHeight }]
}


export async function transfer_sign_by_2_auth(
  transactionBuffer: Buffer,
  two_auth_signer: anchor.web3.Signer
): Promise<Buffer> {

  const transaction = anchor.web3.Transaction.from(transactionBuffer)

  transaction.partialSign(two_auth_signer);

  // const serializedTransaction = transaction.serialize({
  //   requireAllSignatures: true,
  // });
  const serializedTransaction = transaction.serialize();

  return serializedTransaction;

}


export async function send_transaction_buffer(
  transactionBuffer: Buffer,
  blockhashUsed: {
    blockhash: anchor.web3.Blockhash;
    lastValidBlockHeight: number;
  }) {

  const txSig = await anchor.getProvider().connection.sendRawTransaction(transactionBuffer);

  const confirmStrategy: anchor.web3.BlockheightBasedTransactionConfirmationStrategy = {
    blockhash: blockhashUsed.blockhash,
    lastValidBlockHeight: blockhashUsed.lastValidBlockHeight,
    signature: txSig
  }

  await anchor.getProvider().connection.confirmTransaction(confirmStrategy);

  console.log(`Transfer (wrapped) raw tx : ${txSig}`);

  return txSig;
}