const { CosmWasmClient , Secp256k1Wallet} = require("cosmwasm");
const { SigningCosmWasmClient} = require("@cosmjs/cosmwasm");
const { Keyring } = require("@polkadot/api");

require("dotenv").config();
// This is rpc endpoint
const rpcEndpoint = process.env.RPC_URL;
/// test cotnract address
const contractAddr = process.env.COSMOSWASM_CONTRACT_ADDRESS;

async function main() {
  const client = await CosmWasmClient.connect(rpcEndpoint);
  const config = await client.queryContractSmart(contractAddr, { PendingClaims: {} });
  const c = client.getContract(contractAddr);
  const keyring = new Keyring({ type: 'sr25519' });
  const mnemonic = process.env.MNEMOIC;
  if (!mnemonic) {
    throw new Error('MNEMONIC environment variable not defined');
  }
  const relayerAccount = keyring.addFromUri(mnemonic);
  console.log('Relayer Address: ' + relayerAccount.address);

  const client1 = new SigningCosmWasmClient(
    rpcEndpoint,
    relayerAccount.address,
  );

  const msg = {
    approve_claim: {
        config
    },
  };

  try {
    // Execute the CosmWasm function
    const result = await client1.execute(contractAddr, msg);
    console.log("Execution result:", result);
  } catch (error) {
    console.error("Error executing function:", error);
  }
}
main()
