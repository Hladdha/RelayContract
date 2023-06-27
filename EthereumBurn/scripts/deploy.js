const hre = require("hardhat");

async function main() {
    const Enfi = await hre.ethers.getContractFactory("EfiToken");
    const enfi = await Enfi.deploy();
    await enfi.deployed();
    const Enjin = await hre.ethers.getContractFactory("EnjToken");
    const enjin = await Enjin.deploy();
    await enjin.deployed();
    console.log(enfi.address);
    console.log(enjin.address);

    const Lock = await hre.ethers.getContractFactory("EthereumBurn");
    const lock = await Lock.deploy(enfi.address, enjin.address);
    await lock.deployed();

    console.log(
        `deployed to ${lock.address}`
      );
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
