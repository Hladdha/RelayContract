const { CosmWasmClient , Secp256k1Wallet} = require("cosmwasm");
const { SigningCosmWasmClient} = require("@cosmjs/cosmwasm");
const { Keyring } = require("@polkadot/api");

require("dotenv").config();
// This is rpc endpoint
const rpcEndpoint = process.env.RPC_URL;
/// test cotnract address
const contractAddr = process.env.COSMOSWASM_CONTRACT_ADDRESS;

async function exexute_request_claim() {
  const client = await CosmWasmClient.connect(rpcEndpoint);
  const config = await client.queryContractSmart(contractAddr, { get_launchpad_info: {} });
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

  let transactionData = {
    block: data[0].log.blockNumber,
    transaction: data[0].log.transactionHash,
    topic0: data[0].log.topics[0],
    contract_address: data[0].token,
    sender: data[0].user,
    recipient: string,
    value: data[0].amount,
  }
  const msg = {
    request_claim: {
      transactionData
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
exexute_request_claim()
module.exports = {exexute_request_claim};
