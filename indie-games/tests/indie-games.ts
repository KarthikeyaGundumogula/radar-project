import * as anchor from "@coral-xyz/anchor";
import { Program, } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js"
import { IndieGames } from "../target/types/indie_games";
import { BN } from "bn.js";
import { expect } from "chai";
import { getAccount, getAssociatedTokenAddress } from "@solana/spl-token"

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

    console.log("Account INitialized");
    const [mintPda] = PublicKey.findProgramAddressSync(
      [gameId.toBuffer(), assetPda.toBuffer()],
      program.programId
    );

    const associatedTokenAccount = await getAssociatedTokenAddress(
      mintPda,
      provider.wallet.publicKey
    )

    program.methods.mintAsset(
      {
        name: name,
        amount: new BN(10),
        gameId: gameId
      }).accounts({
        user: provider.wallet.publicKey
      }).rpc()

    const userAta = await getAccount(provider.connection, associatedTokenAccount);
    console.log(userAta.amount)
  });
});