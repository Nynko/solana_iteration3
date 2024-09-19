import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { AssetBased } from "../target/types/asset_based";
import { createAssociatedTokenAccountInstruction, createSyncNativeInstruction, getAccount, getAssociatedTokenAddress, NATIVE_MINT, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { wrap_tokens } from "../tests/wrapped_tokens_tests";
import * as dotenv from "dotenv";
dotenv.config();
async function main(wrapper: anchor.web3.PublicKey, amount: number, user_to : anchor.web3.PublicKey){

  anchor.setProvider(anchor.AnchorProvider.env());
  const wallet = anchor.Wallet.local().payer;
  
  const program = anchor.workspace.AssetBased as Program<AssetBased>;
  const connection = program.provider.connection;
  const approver = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(process.env.APPROVER))
  )


  const associatedTokenAccount = await getAssociatedTokenAddress(
      NATIVE_MINT,
      wallet.publicKey
    )
    
  const fetchedAccount = await getAccount(connection, associatedTokenAccount).catch(e => null);


  if(!fetchedAccount){
    // Create token account to hold your wrapped SOL
    const ataTransaction = new anchor.web3.Transaction()
    .add(
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        associatedTokenAccount,
        wallet.publicKey,
        NATIVE_MINT
      )
    );

    await anchor.web3.sendAndConfirmTransaction(connection, ataTransaction, [wallet]);
  }
    
  // Transfer SOL to associated token account and use SyncNative to update wrapped SOL balance
  const solTransferTransaction = new anchor.web3.Transaction()
    .add(
      anchor.web3.SystemProgram.transfer({
          fromPubkey: wallet.publicKey,
          toPubkey: associatedTokenAccount,
          lamports: amount * anchor.web3.LAMPORTS_PER_SOL
        }),
        createSyncNativeInstruction(
          associatedTokenAccount
      )
    )
  
  await anchor.web3.sendAndConfirmTransaction(connection, solTransferTransaction, [wallet]);
  
  const accountInfo = await getAccount(connection, associatedTokenAccount);
  
  console.log(`Native: ${accountInfo.isNative}, Lamports: ${accountInfo.amount}`);
  
  

  const [wrapped_token_address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("wrapped_token"),
      wrapper.toBuffer(),
      NATIVE_MINT.toBuffer(),
      user_to.toBuffer(),
    ],
    program.programId
  );

  await wrap_tokens(
  amount * anchor.web3.LAMPORTS_PER_SOL,
  9,
  wrapper,
  approver.publicKey,
  user_to,
  anchor.Wallet.local().payer,
  wrapped_token_address,
  NATIVE_MINT,
  accountInfo.address,
  TOKEN_PROGRAM_ID,
  program
  );

        
    

}


// Get arguments from the command line
const args = process.argv.slice(2);
if (args.length !== 3) {
  console.error("Usage: ts-node <script> <wrapper> <amount> <user_to>");
  process.exit(1);
}

const wrapper = new anchor.web3.PublicKey(args[0]);
const amount = parseFloat(args[1]);
const user_to = new anchor.web3.PublicKey(args[2]);

main(wrapper,amount,user_to).catch(e => console.log(e)).then(() => console.log("finished"));

