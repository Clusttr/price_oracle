import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PriceOracle } from "../target/types/price_oracle";
import {assert} from "chai";

describe("price_oracle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PriceOracle as Program<PriceOracle>;
  const wallet = anchor.AnchorProvider.env().wallet as anchor.Wallet
  const assetMint = new anchor.web3.PublicKey("Fnd3WMEGywcTjp3hdBnAepfJjcMJ2N1RwPpGqoV8Qsmp")
  const asset = anchor.web3.PublicKey.findProgramAddressSync(
      [assetMint.toBuffer()],
      program.programId
  )[0];
  const usdcDecimals = 10 ** 2

  it("should asset init!", async () => {
    let value = new anchor.BN(3000 * usdcDecimals) //$30,000
    let ar = 10 * usdcDecimals //10%
    let rent = 10 * usdcDecimals //$1000
      console.log({assetValue: value.toNumber()})
    const tx = await program.methods.initAsset(value, ar, rent)
        .accounts({
          signer: wallet.publicKey,
          asset,
          assetMint
        })
        .rpc();
    console.log("Your transaction signature", tx);
    const assetAccount = await program.account.asset.fetch(asset);
    assert(assetAccount.id.toString() === assetMint.toString(), "Asset id error")
  });

    it('should update asset', async () => {
        let value = new anchor.BN(33_200 * usdcDecimals) //$30,000
        let ar = 11.2 * usdcDecimals //10%
        let rent = 1075 * usdcDecimals //$1000
        const tx = await program.methods.updateAsset(value, ar, rent)
            .accounts({
                signer: wallet.publicKey,
                asset,
                assetMint
            })
            .rpc()
        console.log("Your transaction signature", tx);
        const assetAccount = await program.account.asset.fetch(asset);
        assert(assetAccount.value.eq(value), "Failed to update asset value error")
        assert(assetAccount.appreciationRate === ar, "Failed to update appreciation rate value error")
        assert(assetAccount.rent === rent, "Failed to update rent value error")
        console.log({assetAccount_afterUpdate: assetAccount})
    });

    it('should increase revenue', async () => {
        const initAssetAccount = await program.account.asset.fetch(asset);
        let revenue = new anchor.BN(1_600_00) //$2,000
        const tx = await program.methods.increaseRevenue(revenue)
            .accounts({
                signer: wallet.publicKey,
                asset,
                assetMint
            })
            .rpc()
        console.log("Your transaction signature", tx);
        const assetAccount = await program.account.asset.fetch(asset);
        const newRevenueValue = initAssetAccount.cumulativeRevenue.add(revenue)
        assert(assetAccount.cumulativeRevenue.eq(newRevenueValue),
            `expected revenue to be ${newRevenueValue.toNumber()} but found ${assetAccount.cumulativeRevenue.toNumber()}`)
        console.log({newRevenueValue_afterIncrease: newRevenueValue})
    });

    it('should increase cost', async () => {
        const initAssetAccount = await program.account.asset.fetch(asset);
        let cost = new anchor.BN(207_60) //$207.6
        const tx = await program.methods.increaseCost(cost)
            .accounts({
                signer: wallet.publicKey,
                asset,
                assetMint
            })
            .rpc()
        console.log("Your transaction signature", tx);
        const assetAccount = await program.account.asset.fetch(asset);
        const newCostValue = initAssetAccount.cumulativeMaintenanceCost.add(cost)
        assert(assetAccount.cumulativeMaintenanceCost.eq(newCostValue),
            `expected cost to be ${newCostValue.toNumber()}, but found  ${assetAccount.cumulativeMaintenanceCost.toNumber()} `)
        console.log({newCostValue_afterIncrease: newCostValue})
    });
});
