import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AssetBased } from "../target/types/asset_based";
import {
  create_spl_mint,
  create_spl_token_account,
  initialize_wrapper,
  get_wrapped_account,
  mint_tokens,
  initialize_two_auth,
} from "./Initialize_tests";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID, transfer } from "@solana/spl-token";
import { unwrap_tokens, wrap_tokens } from "./wrapped_tokens_tests";
import { min } from "bn.js";
import { expect } from "chai";
import { create_approver_with_best_bump, create_user_with_best_bump, sendTransaction, sleep } from "./utils";
import { self_transfer_wtokens, send_transaction_buffer, transfer_sign_by_2_auth, transfer_with_partial_sig, transfer_wtokens } from "./transfer_tests";
import { issue_first_idendity } from "./idendity_tests";
import fs from "fs";
import { init_recovery, recover } from "./recovery_tests";
import { add_pseudo } from "./pseudo_tests";
import * as dotenv from "dotenv";
dotenv.config();


describe("asset_based", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AssetBased as Program<AssetBased>;
  let mint_info: MintInfo;
  let user1_info: User1Info, user2_info: UserInfo, user3_info: UserInfo, user4_info: UserInfo;
  let wrapper: WrapperInfo, two_auth_entity: anchor.web3.Keypair, issuer: anchor.web3.Keypair, approver: anchor.web3.Keypair;
  let payer = anchor.Wallet.local().payer;
  let idendity1: anchor.web3.PublicKey, idendity2: anchor.web3.PublicKey;
  let two_auth1: anchor.web3.PublicKey;

  const DEVNET = true;

  it("Init", async () => {

    let file_path: string;
    if (DEVNET) {
      file_path = "tests/data/asset_based_dev.json";
    } else {
      file_path = "tests/data/asset_based.json";
    }
    let file_exist = fs.existsSync(file_path);

    if (file_exist) {
      const data = fs.readFileSync(file_path, "utf-8");

      // Step 2: Parse the JSON data
      const parsedData = JSON.parse(data);

      const loadedData: InitReturn = {
        mint_info: {
          mint: new anchor.web3.PublicKey(parsedData.mint_info.mint),
          mintAuthority: anchor.web3.Keypair.fromSecretKey(
            new Uint8Array(parsedData.mint_info.mintAuthority)
          ),
          mintFreezeAuthority: anchor.web3.Keypair.fromSecretKey(
            new Uint8Array(parsedData.mint_info.mintFreezeAuthority)
          ),
          token_program: new anchor.web3.PublicKey(
            parsedData.mint_info.token_program
          ),
          decimals: parsedData.mint_info.decimals,
        },
        user1_info: {
          user1: anchor.web3.Keypair.fromSecretKey(
            new Uint8Array(parsedData.user1_info.user1)
          ),
          token_account: new anchor.web3.PublicKey(
            parsedData.user1_info.token_account
          ),
          wrapped_account: new anchor.web3.PublicKey(
            parsedData.user1_info.wrapped_account
          ),
        },
        user2_info: {
          user: anchor.web3.Keypair.fromSecretKey(
            new Uint8Array(parsedData.user2_info.user)
          ),
          wrapped_account: new anchor.web3.PublicKey(
            parsedData.user2_info.wrapped_account
          ),
        },
        user3_info: {
          user: anchor.web3.Keypair.fromSecretKey(
            new Uint8Array(parsedData.user3_info.user)
          ),
          wrapped_account: new anchor.web3.PublicKey(
            parsedData.user3_info.wrapped_account
          ),
        },
        user4_info: {
          user: anchor.web3.Keypair.fromSecretKey(
            new Uint8Array(parsedData.user4_info.user)
          ),
          wrapped_account: new anchor.web3.PublicKey(
            parsedData.user4_info.wrapped_account
          ),
        },
        wrapper: {
          wrapper_pda: new anchor.web3.PublicKey(
            parsedData.wrapper.wrapper_pda
          ),
        },
        issuer: anchor.web3.Keypair.fromSecretKey(
          new Uint8Array(parsedData.issuer)
        ),
        approver: anchor.web3.Keypair.fromSecretKey(
          new Uint8Array(parsedData.approver)
        ),
        two_auth_entity: anchor.web3.Keypair.fromSecretKey(
          new Uint8Array(parsedData.two_auth_entity)
        ),
      };

      mint_info = loadedData.mint_info;
      user1_info = loadedData.user1_info;
      user2_info = loadedData.user2_info;
      user3_info = loadedData.user3_info;
      user4_info = loadedData.user4_info;
      wrapper = loadedData.wrapper;
      issuer = loadedData.issuer;
      approver = loadedData.approver;
      two_auth_entity = loadedData.two_auth_entity;

      return;
    } else {
      const init_return = await init(program);
      mint_info = init_return.mint_info;
      user1_info = init_return.user1_info;
      user2_info = init_return.user2_info;
      user3_info = init_return.user3_info;
      user4_info = init_return.user4_info;
      wrapper = init_return.wrapper;
      issuer = init_return.issuer;
      approver = init_return.approver;
      two_auth_entity = init_return.two_auth_entity;

      fs.writeFileSync(
        file_path,
        JSON.stringify(
          {
            mint_info: {
              mint: mint_info.mint.toBase58(),
              mintAuthority: Array.from(mint_info.mintAuthority.secretKey),
              mintFreezeAuthority: Array.from(
                mint_info.mintFreezeAuthority.secretKey
              ),
              token_program: mint_info.token_program.toBase58(),
              decimals: mint_info.decimals,
            },
            user1_info: {
              user1: Array.from(user1_info.user1.secretKey),
              token_account: user1_info.token_account.toBase58(),
              wrapped_account: user1_info.wrapped_account.toBase58(),
            },
            user2_info: {
              user: Array.from(user2_info.user.secretKey),
              wrapped_account: user2_info.wrapped_account.toBase58(),
            },
            user3_info: {
              user: Array.from(user3_info.user.secretKey),
              wrapped_account: user2_info.wrapped_account.toBase58(),
            },
            user4_info: {
              user: Array.from(user4_info.user.secretKey),
              wrapped_account: user2_info.wrapped_account.toBase58(),
            },
            wrapper: {
              wrapper_pda: wrapper.wrapper_pda.toBase58(),
            },
            issuer: Array.from(issuer.secretKey),
            approver: Array.from(approver.secretKey),
            two_auth_entity: Array.from(two_auth_entity.secretKey),
          },
          null,
          2
        )
      );
    }


    expect(mint_info.mint).to.not.be.null;
    expect(mint_info.mintAuthority).to.not.be.null;
    expect(mint_info.mintFreezeAuthority).to.not.be.null;
    expect(mint_info.token_program).to.not.be.null;
    expect(wrapper).to.not.be.null;
    expect(issuer).to.not.be.null;
    expect(approver).to.not.be.null;
    expect(two_auth_entity).to.not.be.null;
  });

  it("Transfer Money", async () => {
    const transferAmount = 0.01; // 0.01 SOL

    // Create a transfer instruction for transferring SOL from wallet_1 to wallet_2
    const transferInstruction = anchor.web3.SystemProgram.transfer({
      fromPubkey: anchor.Wallet.local().payer.publicKey,
      toPubkey: user1_info.user1.publicKey,
      lamports: transferAmount * anchor.web3.LAMPORTS_PER_SOL, // Convert transferAmount to lamports
    });

    const transferInstruction2 = anchor.web3.SystemProgram.transfer({
      fromPubkey: anchor.Wallet.local().payer.publicKey,
      toPubkey: user2_info.user.publicKey,
      lamports: transferAmount * anchor.web3.LAMPORTS_PER_SOL, // Convert transferAmount to lamports
    });

    const transferInstruction3 = anchor.web3.SystemProgram.transfer({
      fromPubkey: anchor.Wallet.local().payer.publicKey,
      toPubkey: issuer.publicKey,
      lamports: transferAmount * anchor.web3.LAMPORTS_PER_SOL, // Convert transferAmount to lamports
    });

    const transferInstruction4 = anchor.web3.SystemProgram.transfer({
      fromPubkey: anchor.Wallet.local().payer.publicKey,
      toPubkey: approver.publicKey,
      lamports: transferAmount * anchor.web3.LAMPORTS_PER_SOL, // Convert transferAmount to lamports
    });


    // Add the transfer instruction to a new transaction
    let transaction = new anchor.web3.Transaction().add(transferInstruction);
    transaction.add(transferInstruction2);
    transaction.add(transferInstruction3);
    transaction.add(transferInstruction4);

    const txSig = await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      transaction,
      [anchor.Wallet.local().payer]
    );

    console.log(`Transfer of ${transferAmount} SOL to user1 tx : ${txSig}`);

  });

  it("Create IDs", async () => {
    try {
      idendity1 = await issue_first_idendity(
        10000000,
        user1_info.user1,
        issuer,
        approver.publicKey,
        wrapper.wrapper_pda,
        program
      );
      idendity2 = await issue_first_idendity(
        10000000,
        user2_info.user,
        issuer,
        approver.publicKey,
        wrapper.wrapper_pda,
        program
      );
    } catch (error) {
      console.log("Error", error);
      expect(error).to.be.null;
    }
  });

  const VALUE_TOKEN = 200;
  const VALUE_WTOKENS = 150;
  const VALUE_UNWRAP_WTOKENS = 50;

  it("Should fail: Test Wrapping 0 tokens from account with no token account", async () => {
    try {

      const user3_token_address = getAssociatedTokenAddressSync(
        mint_info.mint,
        user3_info.user.publicKey,
        false,
        mint_info.token_program,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

      await wrap_tokens(
        0,
        mint_info.decimals,
        wrapper.wrapper_pda,
        approver.publicKey,
        user3_info.user.publicKey,
        user3_info.user,
        user3_info.wrapped_account,
        mint_info.mint,
        user3_token_address,
        mint_info.token_program,
        program
      );

    } catch (error) {
      expect(error.logs).to.contain(
        "Program log: AnchorError caused by account: from_token_account. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    }

  });


  it("Wrap Tokens", async () => {
    try {
      await mint_tokens(
        VALUE_TOKEN,
        anchor.Wallet.local().payer,
        mint_info.mint,
        user1_info.token_account,
        mint_info.mintAuthority,
        mint_info.token_program
      );

      await wrap_tokens(
        VALUE_WTOKENS,
        mint_info.decimals,
        wrapper.wrapper_pda,
        approver.publicKey,
        user1_info.user1.publicKey,
        user1_info.user1,
        user1_info.wrapped_account,
        mint_info.mint,
        user1_info.token_account,
        mint_info.token_program,
        program
      );

      await wrap_tokens(
        0,
        mint_info.decimals,
        wrapper.wrapper_pda,
        approver.publicKey,
        user2_info.user.publicKey,
        user1_info.user1,
        user2_info.wrapped_account,
        mint_info.mint,
        user1_info.token_account,
        mint_info.token_program,
        program
      );


      const token_user_wrapped_account_balance =
        await program.provider.connection.getTokenAccountBalance(
          user1_info.wrapped_account
        );
      expect(Number(token_user_wrapped_account_balance.value.amount)).to.equal(VALUE_WTOKENS);


      const token_account_balance =
        await program.provider.connection.getTokenAccountBalance(
          user1_info.token_account
        );
      expect(Number(token_account_balance.value.amount)).to.equal(VALUE_TOKEN - VALUE_WTOKENS);

      const token_user2_wrapped_account_balance =
        await program.provider.connection.getTokenAccountBalance(
          user2_info.wrapped_account
        );
      expect(Number(token_user2_wrapped_account_balance.value.amount)).to.equal(0);


    } catch (error) {
      console.log("Error", error);
      expect(error).to.be.null;
    }
  });



  it("Unwrap Tokens", async () => {
    try {
      await unwrap_tokens(
        VALUE_UNWRAP_WTOKENS,
        mint_info.decimals,
        wrapper.wrapper_pda,
        approver.publicKey,
        user1_info.user1.publicKey,
        user1_info.user1,
        user1_info.wrapped_account,
        mint_info.mint,
        user1_info.token_account,
        issuer,
        program
      );

      const token_user_wrapped_account_balance =
        await program.provider.connection.getTokenAccountBalance(
          user1_info.wrapped_account
        );
      expect(Number(token_user_wrapped_account_balance.value.amount)).to.equal(VALUE_WTOKENS - VALUE_UNWRAP_WTOKENS);


      const token_account_balance =
        await program.provider.connection.getTokenAccountBalance(
          user1_info.token_account
        );
      expect(Number(token_account_balance.value.amount)).to.equal(VALUE_TOKEN - VALUE_WTOKENS + VALUE_UNWRAP_WTOKENS);
    } catch (error) {
      console.log("Error", error);
      expect(error).to.be.null;
    }
  });

  let USER1_BALANCE;
  let USER2_BALANCE;

  USER1_BALANCE = VALUE_WTOKENS - VALUE_UNWRAP_WTOKENS;

  it("Create 2Auth", async () => {
    try {
      two_auth1 = await initialize_two_auth(
        user1_info.user1,
        idendity1,
        approver.publicKey,
        wrapper.wrapper_pda,
        two_auth_entity,
        program
      );
    } catch (error) {
      console.log("Error", error);
      expect(error).to.be.null;
    }
  });


  it("Create Recovery", async () => {

    try {
      await init_recovery(
        [{ authority: issuer.publicKey, minSignatures: 1, minDuration: 1000 }],
        user1_info.user1,
        user1_info.user1,
        approver.publicKey,
        mint_info.mint,
        wrapper.wrapper_pda,
        program
      )
    } catch (error) {
      console.log("Error", error);
      expect(error).to.be.null;
    }
  });



  it("Transfer Tokens", async () => {


    try {
      await transfer_wtokens(
        2,
        mint_info.decimals,
        wrapper.wrapper_pda,
        user1_info.user1,
        user1_info.wrapped_account,
        user2_info.user.publicKey,
        user2_info.wrapped_account,
        two_auth1,
        two_auth_entity,
        mint_info.mint,
        approver.publicKey,
        mint_info.token_program,
        program
      );
    } catch (error) {
      console.log("Error", error);
      expect(error).to.be.null;
    }

    const user1_balance =
      await program.provider.connection.getTokenAccountBalance(
        user1_info.wrapped_account
      ).then((account) => Number(account.value.amount));

    const user2_balance = await program.provider.connection.getTokenAccountBalance(
      user2_info.wrapped_account
    ).then((account) => Number(account.value.amount));

    expect(user1_balance).to.equal(USER1_BALANCE - 2);
    expect(user2_balance).to.equal(2);

    USER1_BALANCE = USER1_BALANCE - 2;
    USER2_BALANCE = 2;
  });



  it("Pseudo", async () => {

    try {

      const idAccountBefore = await program.account.idAccount.fetch(
        idendity1,
      );

      expect(idAccountBefore.associatedPseudo).to.null;

      const pseudo_account = await add_pseudo(
        idendity1,
        user1_info.user1,
        "Testons Ensemble les amis !",
        program
      )

      console.log("[PK] Pseudo Account: ", pseudo_account);

      const account = await program.account.pseudoAccount.fetch(
        pseudo_account,
      );

      expect(account.owner.toBase58()).to.equal(user1_info.user1.publicKey.toBase58());
      expect(account.initialized).to.be.true;

      const idAccount = await program.account.idAccount.fetch(
        idendity1,
      );

      expect(idAccount.associatedPseudo.toBase58()).to.equal(pseudo_account.toBase58());

    } catch (error) {
      console.log(error);
      expect(error).to.be.null;
    }


  });

  it("Should fail: cannot use 2 times the same Pseudo", async () => {
    const pseudo = "Testons Ensemble les amis !";
    try {
      const pseudo_account = await add_pseudo(
        idendity2,
        user2_info.user,
        pseudo,
        program
      )

      console.log("[PK] Pseudo Account: ", pseudo_account);



    } catch (error) {
      const [pseudo_account, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("pseudo"), Buffer.from(pseudo)],
        program.programId
      );

      expect(error.logs).to.contains(`Allocate: account Address { address: ${pseudo_account.toBase58()}, base: None } already in use`)
      
    }
  });


  it("Unapproved Transfer Tokens", async () => {
    try {
      await transfer_wtokens(
        2,
        mint_info.decimals,
        wrapper.wrapper_pda,
        user1_info.user1,
        user1_info.wrapped_account,
        user2_info.user.publicKey,
        user2_info.wrapped_account,
        two_auth1,
        null,
        mint_info.mint,
        approver.publicKey,
        mint_info.token_program,
        program
      );
    } catch (error) {
      expect(error.logs).to.contain(
        "Program log: AnchorError occurred. Error Code: NeedTwoAuthApproval. Error Number: 6001. Error Message: Need the two auth entity approval."
      );
    }

    const user1_balance =
      await program.provider.connection.getTokenAccountBalance(
        user1_info.wrapped_account
      ).then((account) => Number(account.value.amount));

    const user2_balance = await program.provider.connection.getTokenAccountBalance(
      user2_info.wrapped_account
    ).then((account) => Number(account.value.amount));

    expect(user1_balance).to.equal(USER1_BALANCE);
    expect(user2_balance).to.equal(USER2_BALANCE);
  });

  it("Self Transfer Tokens", async () => {
    const user1_balance_init = await program.provider.connection.getTokenAccountBalance(
      user1_info.wrapped_account
    ).then((account) => Number(account.value.amount));

    try {
      await self_transfer_wtokens(
        2,
        mint_info.decimals,
        wrapper.wrapper_pda,
        user1_info.user1,
        user1_info.wrapped_account,
        user1_info.user1.publicKey,
        user1_info.wrapped_account,
        two_auth1,
        mint_info.mint,
        approver.publicKey,
        mint_info.token_program,
        program
      );
    } catch (error) {
      console.log("Error", error);
      expect(error).to.be.null;
    }

    const user1_balance =
      await program.provider.connection.getTokenAccountBalance(
        user1_info.wrapped_account
      ).then((account) => Number(account.value.amount));

    expect(user1_balance).to.equal(user1_balance_init);
  });

  it("Transfer with signature signed outside of main tx", async () => {

    try {
      const [rawTx, blockhash] = await transfer_with_partial_sig(
        2,
        mint_info.decimals,
        wrapper.wrapper_pda,
        user1_info.user1,
        user1_info.wrapped_account,
        user2_info.user.publicKey,
        user2_info.wrapped_account,
        two_auth1,
        two_auth_entity,
        mint_info.mint,
        approver.publicKey,
        mint_info.token_program,
        program
      );


      const rawTx2 = await transfer_sign_by_2_auth(rawTx,
        two_auth_entity
      );

      await send_transaction_buffer(rawTx2, blockhash);


    } catch (error) {
      console.log(error);
      expect(error).to.be.null;
    }

    const user1_balance =
      await program.provider.connection.getTokenAccountBalance(
        user1_info.wrapped_account
      ).then((account) => Number(account.value.amount));

    const user2_balance = await program.provider.connection.getTokenAccountBalance(
      user2_info.wrapped_account
    ).then((account) => Number(account.value.amount));


    expect(user1_balance).to.equal(USER1_BALANCE - 2);
    expect(user2_balance).to.equal(USER2_BALANCE + 2);

    USER1_BALANCE = USER1_BALANCE - 2;
    USER2_BALANCE = USER2_BALANCE + 2;


  })
  it("Transfer with signature from server", async () => {

    try {
      const [rawTx, blockhash] = await transfer_with_partial_sig(
        3,
        mint_info.decimals,
        wrapper.wrapper_pda,
        user1_info.user1,
        user1_info.wrapped_account,
        user2_info.user.publicKey,
        user2_info.wrapped_account,
        two_auth1,
        two_auth_entity,
        mint_info.mint,
        approver.publicKey,
        mint_info.token_program,
        program
      );

      interface RawTxData {
        transaction: String,
        blockhash: String,
      };

      let responseData;

      const signTwoAuth = async (rawData: RawTxData) => {
        try {
          const response = await fetch('http://localhost:3000/two-auth-sign', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify(rawData),
          });

          if (!response.ok) {
            throw new Error(`Error: ${response.statusText}`);
          }

          responseData = await response.json();
        } catch (error) {
          console.error('Error:', error);
        }
      };

      await signTwoAuth({ transaction: rawTx.toString('base64'), blockhash: blockhash.blockhash });

      const rawTx2 = Buffer.from(responseData.transaction, 'base64');

      const rawTx2Compare = await transfer_sign_by_2_auth(rawTx,
        two_auth_entity
      );

      expect(rawTx2.equals(rawTx2Compare)).to.be.true;


      await send_transaction_buffer(rawTx2, blockhash);



    } catch (error) {
      console.log(error);
      expect(error).to.be.null;
    }

    const user1_balance =
      await program.provider.connection.getTokenAccountBalance(
        user1_info.wrapped_account
      ).then((account) => Number(account.value.amount));

    const user2_balance = await program.provider.connection.getTokenAccountBalance(
      user2_info.wrapped_account
    ).then((account) => Number(account.value.amount));



    expect(user1_balance).to.equal(USER1_BALANCE - 3);
    expect(user2_balance).to.equal(USER2_BALANCE + 3);

    USER1_BALANCE = USER1_BALANCE - 3;
    USER2_BALANCE = USER2_BALANCE + 3;


  })

  it("Recover Account", async () => {

    try {

      await recover(
        issuer,
        approver.publicKey,
        user1_info.user1.publicKey,
        mint_info.mint,
        program,
      );

      const old_wrapped_account = await program.provider.connection.getAccountInfo(
        user1_info.wrapped_account
      );
      expect(old_wrapped_account).to.be.null;

    } catch (error) {
      console.log(error);
      expect(error).to.be.null;

    }

  })

});

interface MintInfo {
  mint: anchor.web3.PublicKey;
  mintAuthority: anchor.web3.Keypair;
  mintFreezeAuthority: anchor.web3.Keypair;
  token_program: anchor.web3.PublicKey;
  decimals: number;
}
interface User1Info {
  user1: anchor.web3.Keypair;
  token_account: anchor.web3.PublicKey;
  wrapped_account: anchor.web3.PublicKey;
}
interface UserInfo {
  user: anchor.web3.Keypair;
  wrapped_account: anchor.web3.PublicKey;
}
interface WrapperInfo {
  wrapper_pda: anchor.web3.PublicKey;
}

interface InitReturn {
  approver: anchor.web3.Keypair;
  mint_info: MintInfo;
  user1_info: User1Info;
  user2_info: UserInfo;
  user3_info: UserInfo;
  user4_info: UserInfo;
  wrapper: WrapperInfo;
  issuer: anchor.web3.Keypair;
  two_auth_entity: anchor.web3.Keypair;
}

async function init(
  program: Program<AssetBased>,
): Promise<InitReturn> {

  // const approver = await create_approver_with_best_bump(program);
  const approver = anchor.web3.Keypair.generate();
  console.log("[Pk] approver", approver.publicKey.toBase58());
  const token_program = TOKEN_PROGRAM_ID;
  const mintAuthority = anchor.web3.Keypair.generate();
  console.log("[Pk] mintAuthority", mintAuthority.publicKey.toBase58());
  const mintFreezeAuthority = anchor.web3.Keypair.generate();
  console.log(
    "[Pk] mintFreezeAuthority",
    mintFreezeAuthority.publicKey.toBase58()
  );

  // const issuer = anchor.web3.Keypair.generate();
  const issuer = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(process.env.ISSUER_ID))
  );
  console.log("[Pk] issuer", issuer.publicKey.toBase58());
  const two_auth_entity = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(process.env.TWO_AUTH_AUTHORITY))
  );
  console.log("[Pk] two_auth_entity", two_auth_entity.publicKey.toBase58());

  const decimals = 2;
  const mint = await create_spl_mint(
    anchor.Wallet.local().payer,
    mintAuthority,
    mintFreezeAuthority,
    decimals,
    token_program
  );

  const wrapper_pda = await initialize_wrapper(
    anchor.Wallet.local().payer,
    [issuer.publicKey],
    [issuer.publicKey],
    approver,
    program
  );

  const [user1, wrapped_account] = get_wrapped_account(wrapper_pda, mint, program);
  const [user2, wrapped_account2] = get_wrapped_account(wrapper_pda, mint, program);
  const [user3, wrapped_account3] = get_wrapped_account(wrapper_pda, mint, program);
  const [user4, wrapped_account4] = get_wrapped_account(wrapper_pda, mint, program);

  // const user2 = anchor.web3.Keypair.generate();
  console.log("[Pk] user1", user1.publicKey.toBase58());
  console.log("[Pk] wrapped_account", wrapped_account.toBase58());

  // const [userBis2, _wrapped_account2] = await create_user_with_best_bump(wrapper_pda, mint, program);
  // console.log("[Pk] user2", user2.publicKey.toBase58());
  // console.log("[Pk] wrapped_account 2", _wrapped_account2.toBase58());

  const token_account = await create_spl_token_account(
    anchor.Wallet.local().payer,
    user1.publicKey,
    mint,
    token_program
  );

  // const wrapped_account = await get_wrapped_account(
  //   user1,
  //   mint,
  //   approver.publicKey,
  //   wrapper_pda,
  //   program,
  // );

  // const [wrapped_account2, _bump2] = await get_wrapped_account(
  //   user2,
  //   mint,
  //   wrapper_pda,
  //   program,
  // );

  return {
    approver,
    mint_info: {
      mint,
      mintAuthority,
      mintFreezeAuthority,
      token_program,
      decimals,
    },
    user1_info: {
      user1,
      token_account,
      wrapped_account,
    },
    user2_info: {
      user: user2,
      wrapped_account: wrapped_account2,
    },
    user3_info: {
      user: user3,
      wrapped_account: wrapped_account3,
    },
    user4_info: {
      user: user4,
      wrapped_account: wrapped_account4,
    },
    wrapper: {
      wrapper_pda,
    },
    issuer: issuer,
    two_auth_entity: two_auth_entity,
  };
}
