import * as anchor from "@coral-xyz/anchor";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import * as readline from 'readline';


const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });
  
function askQuestion(query: string): Promise<string> {
return new Promise(resolve => rl.question(query, resolve));
}

async function main(){
    const endpoint = "https://backends.nicolasbeaudouin.com/api/kong/quicknode-solana-devnet"
    const webSocketEndpoint = "wss://backends.nicolasbeaudouin.com/api/kong/quicknode-solana-devnet"
    const connection = new anchor.web3.Connection(endpoint, {commitment: 'confirmed', wsEndpoint: webSocketEndpoint})

    const blockhash = await connection.getLatestBlockhash();

    console.log("Blockhash: ", blockhash.blockhash);

    const subscriptionId = connection.onAccountChange(
        new anchor.web3.PublicKey("DLnnMxDv6MCkZtfn9zUdCVpVytc1EeQJvWG86k9DNcTg"),
        // callback for when the account changes
        accountInfo => {
          console.log(
            'Account changed:',
            accountInfo.lamports,
          );
        });

    // While user don't input 'stop':
    let input = '';
    while (input.toLowerCase() !== 'stop') {
      input = await askQuestion("type 'stop' to end: ");
    }
    rl.close();
    connection.removeAccountChangeListener(subscriptionId);
    

}

// To run `yarn run ts-node scripts/test.ts`
main().catch(e => console.log(e)).then(() => console.log("finished"));