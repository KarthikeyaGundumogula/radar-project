import * as anchor from "@coral-xyz/anchor";
import { Program, } from "@coral-xyz/anchor";
import { PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Keypair } from "@solana/web3.js"
import { IndieGames } from "../target/types/indie_games";
import { BN } from "bn.js";
import { expect } from "chai";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAccount, getAssociatedTokenAddress, getMint, TOKEN_PROGRAM_ID, } from "@solana/spl-token"

import { assert } from "chai";
import { StableCoin } from "../target/types/stable_coin";

//key-pair : music unfair salute relief valve tent captain reveal knock snack hip shrimp

describe('Asset Minting Tests', () => {

    const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: PublicKey = new PublicKey(
        'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
    );
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const indie_games_program = anchor.workspace.IndieGames as Program<IndieGames>;
    const stable_coin_program = anchor.workspace.StableCoin as Program<StableCoin>;
    let assetAccount: PublicKey;
    let mint: PublicKey;
    let gameId = new Keypair();
    let dsc_token_vault: PublicKey;
    let dsc_vault_authority;
    let dsc_mint: PublicKey;

    function findAssociatedTokenAddress(
        walletAddress: PublicKey,
        tokenMintAddress: PublicKey
    ): PublicKey {
        return PublicKey.findProgramAddressSync(
            [
                walletAddress.toBuffer(),
                TOKEN_PROGRAM_ID.toBuffer(),
                tokenMintAddress.toBuffer(),
            ],
            SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID
        )[0];
    }

    before(async () => {

        const [assetAccountPda, assetBump] = PublicKey.findProgramAddressSync(
            [Buffer.from('MyAssetName'), gameId.publicKey.toBuffer()],
            indie_games_program.programId
        );
        assetAccount = assetAccountPda
        const [assetMintPda, mintBump] = PublicKey.findProgramAddressSync(
            [gameId.publicKey.toBuffer(), assetAccountPda.toBuffer()],
            indie_games_program.programId
        );
        mint = assetMintPda

        const [dscMintPda,] = PublicKey.findProgramAddressSync(
            [Buffer.from("mint")],
            stable_coin_program.programId
        )
        dsc_mint = dscMintPda;
        const [vault_pda, vault_bump] = PublicKey.findProgramAddressSync(
            [Buffer.from("token_vault")],
            indie_games_program.programId
        )
        dsc_token_vault = vault_pda;
        const [vault_authority, vault_authority_bump] = PublicKey.findProgramAddressSync(
            [Buffer.from("vault_authority")],
            indie_games_program.programId
        )
        dsc_vault_authority = vault_authority
    })

    it('intializes dsc_token_ata', async () => {

        let tx = await stable_coin_program.methods
            .initToken()
            .accountsStrict({
                mint: dsc_mint,
                payer: provider.wallet.publicKey,
                rent: SYSVAR_RENT_PUBKEY,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID
            }).rpc();

        let tx1 = await indie_games_program.methods.initializeDscVault().accountsStrict({
            dscTokenAtaAuthority: dsc_vault_authority,
            dscTokenVault: dsc_token_vault,
            tokenMint: dsc_mint,
            initializer: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID
        }).rpc();
        const vaultAccountInfo = await getAccount(provider.connection, dsc_token_vault);

        expect(vaultAccountInfo.mint.toString()).to.equal(dsc_mint.toString());
        expect(vaultAccountInfo.owner.toString()).to.equal(dsc_vault_authority.toString());
        expect(vaultAccountInfo.amount.toString()).to.equal("0"); // Initially, the balance should be 0

    })
});
