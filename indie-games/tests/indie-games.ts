import * as anchor from "@coral-xyz/anchor";
import { Program, } from "@coral-xyz/anchor";
import { PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Keypair } from "@solana/web3.js"
import { IndieGames } from "../target/types/indie_games";
import { BN } from "bn.js";
import { expect } from "chai";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAccount, getAssociatedTokenAddress, getMint, TOKEN_PROGRAM_ID, } from "@solana/spl-token"

import { assert } from "chai";
import { StableCoin } from "../target/types/stable_coin";

describe('Asset Minting Tests', () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.IndieGames as Program<IndieGames>;
    let assetAccount: PublicKey;
    let mint: PublicKey;
    let userTokenAccount: PublicKey;
    let user = provider.wallet.publicKey;

    let gameId = new Keypair();

    // Initialize the asset before minting
    it('Initializes the asset account', async () => {
        const [assetAccountPda, assetBump] = PublicKey.findProgramAddressSync(
            [Buffer.from('MyAssetName'), gameId.publicKey.toBuffer()],
            program.programId
        );

        const [mintPda, mintBump] = PublicKey.findProgramAddressSync(
            [gameId.publicKey.toBuffer(), assetAccountPda.toBuffer()],
            program.programId
        );

        assetAccount = assetAccountPda;
        mint = mintPda;

        const tx = await program.methods
            .initializeAssets({
                gameId: gameId.publicKey, // Replace with actual Pubkey
                name: 'MyAssetName',
                symbol: 'ASN',
                uri: 'https://myasset.uri',
                price: new BN(100),
                score: 50,
                tradeOption: true,
                collateralOption: false,
                collateral: new BN(500),
            })
            .accountsStrict({
                assetAccount: assetAccountPda,
                mint: mintPda,
                creator: provider.wallet.publicKey,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .rpc();

        console.log("Transaction signature:", tx);
        const assetData = await program.account.assetData.fetch(assetAccountPda);
        assert.equal(assetData.name, 'MyAssetName');
        let my_acc = await getMint(provider.connection, mint);
        console.log(my_acc);

        let tokenAcc = await getAssociatedTokenAddress(mint, provider.wallet.publicKey)

        let txn = await program.methods.mintAsset({
            name: "MyAssetName",
            amount: new BN(10),
            gameId: gameId.publicKey
        }).accountsStrict({
            mint: mint,
            assetAccount: assetAccountPda,
            tokenAccount: tokenAcc,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY
        }).rpc();
        const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: PublicKey = new PublicKey(
            'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
        );
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
        let tokenBal = await getAccount(provider.connection, findAssociatedTokenAddress(provider.wallet.publicKey, mint));
        console.log(tokenBal);
    });
});
