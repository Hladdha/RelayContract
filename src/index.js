const { CosmWasmClient } = require("cosmwasm");
const { SigningCosmWasmClient } = require("@cosmjs/cosmwasm");
require("dotenv").config();
// This is rpc endpoint
const rpcEndpoint = process.env.RPC_URL;
/// test cotnract address
const contractAddr ="aura1ra5a7x995e3v8ywzsz3ammtehw8p42e7r40fdwycey3zq9lct7eszycj43";

async function exexute_request_claim() {
  const client = await CosmWasmClient.connect(rpcEndpoint);
  const config = await client.queryContractSmart(contractAddr, { get_launchpad_info: {} });
  const c = client.getContract(contractAddr);
  console.log(config);
  const client1 = new SigningCosmWasmClient(
    rpcEndpoint,
    "wasm1h59uk4gktyusfwpdl8wfy358n4qgyy674we20m",
  );
  const msg = {
    after_phase_id: {},
  };

  try {
    // Execute the CosmWasm function
    const result = await client1.execute(contractAddr, msg);
    console.log("Execution result:", result);
  } catch (error) {
    console.error("Error executing function:", error);
  }
}

module.exports = {exexute_request_claim};
