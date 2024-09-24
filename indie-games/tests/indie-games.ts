import * as anchor from "@coral-xyz/anchor";
import { Program, } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js"
import { IndieGames } from "../target/types/indie_games";
import { BN } from "bn.js";
import { expect } from "chai";

describe("initialize_asset_handler", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.IndieGames as Program<IndieGames>;

  it("Initializes asset data correctly", async () => {
    const gameId = anchor.web3.Keypair.generate().publicKey;
    const name = "TestAsset";
    const symbol = "TEST";
    const uri = "https://test.com";
    const price = new BN(1000);
    const score = 5;
    const tradeOption = true;
    const collateralOption = false;
    const collateral = new BN(0);

    const [assetPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(name), gameId.toBuffer()],
      program.programId
    );

    const tx = await program.methods.initializeAssets({
      gameId,
      name,
      symbol,
      uri,
      price,
      score,
      tradeOption,
      collateralOption,
      collateral,
    })
      .accounts({
        creator: provider.wallet.publicKey,
      })
      .rpc();

    // Fetch the created account
    const assetAccount = await program.account.assetData.fetch(assetPda);

    // Assert the account data
    expect(assetAccount.game.toString()).to.equal(gameId.toString());
    expect(assetAccount.name).to.equal(name);
    expect(assetAccount.symbol).to.equal(symbol);
    expect(assetAccount.uri).to.equal(uri);
    expect(assetAccount.price.toNumber()).to.equal(price.toNumber());
    expect(assetAccount.score).to.equal(score);
    expect(assetAccount.trade).to.equal(tradeOption);
    expect(assetAccount.collateralOption).to.equal(collateralOption);
    expect(assetAccount.collateral.toNumber()).to.equal(collateral.toNumber());
  });

  it("Fails with invalid arguments", async () => {
    const gameId = anchor.web3.Keypair.generate().publicKey;
    const invalidName = "A".repeat(21); // Name longer than 20 characters
    const symbol = "TEST";
    const uri = "https://test.com";
    const price = new anchor.BN(1000);
    const score = 5;
    const tradeOption = true;
    const collateralOption = false;
    const collateral = new anchor.BN(0);

    const [assetPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(invalidName), gameId.toBuffer()],
      program.programId
    );

    try {
      await program.methods.initializeAssets({
        gameId,
        name: invalidName,
        symbol,
        uri,
        price,
        score,
        tradeOption,
        collateralOption,
        collateral,
      })
        .accounts({
          creator: provider.wallet.publicKey,
        })
        .rpc();

      // If we reach here, the test should fail
      expect.fail("Expected an error to be thrown");
    } catch (error) {
      expect(error.error.errorMessage).to.equal("Arguments dodn't matched");
    }
  });
});