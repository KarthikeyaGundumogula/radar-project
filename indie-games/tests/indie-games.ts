import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  Keypair,
  SendTransactionError,
} from "@solana/web3.js";
import { BN } from "bn.js";
import { expect } from "chai";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { IndieGames } from "../target/types/indie_games";
import { StableCoin } from "../target/types/stable_coin";

//key-pair : music unfair salute relief valve tent captain reveal knock snack hip shrimp

describe("Asset Minting Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const indie_games_program = anchor.workspace
    .IndieGames as Program<IndieGames>;
  const stable_coin_program = anchor.workspace
    .StableCoin as Program<StableCoin>;
  let asset_data_account: PublicKey;
  let asset_mint: PublicKey;
  let game_acc: PublicKey;
  let dsc_token_vault: PublicKey;
  let dsc_vault_authority: PublicKey;
  let dsc_mint: PublicKey;
  let dsc_token_ata: PublicKey;
  let signer = provider.wallet.publicKey;
  let asset_ata_auth: PublicKey;
  let asset_ata: PublicKey;

  let game = {
    name: "Game",
    description: "Game Description",
  };

  let asset = {
    name: "asset",
    symbol: "AST",
    uri: "URI",
    price: new BN(10),
    score: 10,
  };

  const get_dsc = async () => {
    await stable_coin_program.methods
      .mintTokens(new BN(10))
      .accountsStrict({
        mint: dsc_mint,
        destination: dsc_token_ata,
        payer: provider.wallet.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();
  };

  const init_dsc_vault = async () => {
    let tx1 = await indie_games_program.methods
      .initializeDscVault()
      .accountsStrict({
        dscTokenAtaAuthority: dsc_vault_authority,
        dscTokenVault: dsc_token_vault,
        tokenMint: dsc_mint,
        initializer: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
  };

  const init_game = async () => {
    await indie_games_program.methods
      .initializeGame({
        owner: provider.wallet.publicKey,
        name: game.name,
        description: game.description,
      })
      .accountsStrict({
        gameAccount: game_acc,
        initializer: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  };

  const init_assets = async () => {
    await indie_games_program.methods
      .initializeAssets({
        gameId: game_acc,
        name: asset.name,
        symbol: asset.symbol,
        uri: asset.uri,
        price: asset.price,
        score: asset.score,
        tradeOption: true,
        collateralOption: false,
        collateralRatio: new BN(0),
      })
      .accountsStrict({
        assetAccount: asset_data_account,
        mint: asset_mint,
        gameAccount: game_acc,
        creator: signer,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  };

  const mint_assets = async () => {
    try {
      await indie_games_program.methods
        .mintAssetAsOwner({
          amount: new BN(10),
          assetName: asset.name,
          gameName: game.name,
          holder: signer,
        })
        .accountsStrict({
          mint: asset_mint,
          assetAccount: asset_data_account,
          destinationAta: asset_ata,
          collateralTokenAccount: dsc_token_vault,
          userDscTokenAta: dsc_token_ata,
          destinationAtaAuthority: asset_ata_auth,
          gameAccount: game_acc,
          user: signer,
          systemProgram: SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
    } catch (error) {
      if (error instanceof SendTransactionError) {
        console.error("Transaction failed. Logs:");
        console.error(error.logs);
        // You can also access error.message for the error message

        // Optional: Use expect to fail the test with a custom message
        expect.fail(
          `Transaction failed: ${error.message}\nLogs: ${error.logs.join("\n")}`
        );
      } else {
        // If it's not a SendTransactionError, rethrow it
        throw error;
      }
    }
  };

  before(async () => {
    const [gamepda, gameBump] = PublicKey.findProgramAddressSync(
      [signer.toBuffer(), Buffer.from(game.name)],
      indie_games_program.programId
    );
    game_acc = gamepda;
    const [assetAccountPda, assetBump] = PublicKey.findProgramAddressSync(
      [Buffer.from(asset.name), game_acc.toBuffer()],
      indie_games_program.programId
    );
    asset_data_account = assetAccountPda;
    const [assetMintPda, mintBump] = PublicKey.findProgramAddressSync(
      [game_acc.toBuffer(), asset_data_account.toBuffer()],
      indie_games_program.programId
    );
    asset_mint = assetMintPda;
    const [dscMintPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("mint")],
      stable_coin_program.programId
    );
    dsc_mint = dscMintPda;
    const [vault_pda, vault_bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("token_vault")],
      indie_games_program.programId
    );
    dsc_token_vault = vault_pda;
    const [vault_authority, vault_authority_bump] =
      PublicKey.findProgramAddressSync(
        [Buffer.from("vault_authority")],
        indie_games_program.programId
      );
    dsc_vault_authority = vault_authority;
    const [assetAuth] = PublicKey.findProgramAddressSync(
      [signer.toBuffer(), asset_mint.toBuffer()],
      indie_games_program.programId
    );
    asset_ata_auth = assetAuth;
    const tokenAccount = await getAssociatedTokenAddress(
      asset_mint,
      assetAuth,
      true // allowOwnerOffCurve set to true
    );
    asset_ata = tokenAccount;
    let dscTokenAccount = await getAssociatedTokenAddress(
      dsc_mint,
      provider.wallet.publicKey
    );
    dsc_token_ata = dscTokenAccount;

    let tx = await stable_coin_program.methods
      .initToken()
      .accountsStrict({
        mint: dsc_mint,
        payer: provider.wallet.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
  });
  it("intializes dsc_token_vault", async () => {
    await init_dsc_vault();
    const vaultAccountInfo = await getAccount(
      provider.connection,
      dsc_token_vault
    );
    expect(vaultAccountInfo.owner.toString()).to.equal(
      dsc_vault_authority.toString()
    );
  });

  it("mint some dsc token", async () => {
    await get_dsc();
    let acc = await getAccount(provider.connection, dsc_token_ata);
    expect(acc.amount.toString()).to.equal("10");
  });

  it("initializes game", async () => {
    await init_game();
    let gameAcc = await indie_games_program.account.gameState.fetch(game_acc);
    expect(gameAcc.owner.toString()).to.equal(signer.toString());
  });

  it("initializes assets", async () => {
    await init_assets();
    let assetAcc = await indie_games_program.account.assetData.fetch(
      asset_data_account
    );
    expect(assetAcc.name).to.equal(asset.name);
  });

  it(" mint assets as a owner", async () => {
    await mint_assets();
  });
});
