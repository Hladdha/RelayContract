const ethers = require("ethers");
require("dotenv").config();
const {exexute_request_claim} = require("./src/index");

async function main(){
    const contractAddress = process.env.ETHEREUM_BURN_CONTRACT; ///USDC Contract
    const provider = new ethers.JsonRpcProvider(process.env.SEPOLIA_API_KEY)
    const ABI = [{"inputs":[{"internalType":"address","name":"_efiToken","type":"address"},{"internalType":"address","name":"_enjToken","type":"address"}],"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"user","type":"address"},{"indexed":true,"internalType":"address","name":"token","type":"address"},{"indexed":false,"internalType":"uint256","name":"amount","type":"uint256"}],"name":"TokenBurned","type":"event"},{"inputs":[],"name":"MAX_TOKENS_TO_BURN","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"_token","type":"address"},{"internalType":"uint256","name":"_amount","type":"uint256"}],"name":"burn","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"efiToken","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"enjToken","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"renounceOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"","type":"address"}],"name":"totalBurnedAmount","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"}];
    const contract = new ethers.Contract( contractAddress,ABI, provider);
    contract.on("TokenBurned", (user, token, amount, event)=>{
        let transferEvent ={
            user: user,
            token: token,
            amount: amount,
            eventData: event,
        }
        console.log(transferEvent)
        exexute_request_claim();
    })
}

main();