import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  Keypair,
} from "@solana/web3.js";
import { IndieGames } from "../target/types/indie_games";
import { BN } from "bn.js";
import { expect } from "chai";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

import { assert } from "chai";
import { StableCoin } from "../target/types/stable_coin";

//key-pair : music unfair salute relief valve tent captain reveal knock snack hip shrimp

describe("Asset Minting Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const indie_games_program = anchor.workspace
    .IndieGames as Program<IndieGames>;
  const stable_coin_program = anchor.workspace
    .StableCoin as Program<StableCoin>;
  let assetAccount: PublicKey;
  let asset_mint: PublicKey;
  let game_acc: PublicKey;
  let dsc_token_vault: PublicKey;
  let dsc_vault_authority;
  let dsc_mint: PublicKey;
  let dsc_token_ata: PublicKey;
  let signer = provider.wallet.publicKey;

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
        assetAccount: assetAccount,
        mint: asset_mint,
        gameAccount: game_acc,
        creator: signer,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
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
    assetAccount = assetAccountPda;
    const [assetMintPda, mintBump] = PublicKey.findProgramAddressSync(
      [game_acc.toBuffer(), assetAccountPda.toBuffer()],
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
    console.log(assetAccount);
    await init_assets();
    let assetAcc = await indie_games_program.account.assetData.fetch(assetAccount);
    expect(assetAcc.name).to.equal(asset.name);
  });
});
