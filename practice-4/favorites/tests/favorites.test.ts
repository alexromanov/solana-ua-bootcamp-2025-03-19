import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import { airdropIfRequired, getCustomErrorMessage} from "@solana-developers/helpers";
import { expect, describe, test} from '@jest/globals';
import { systemProgramErrors } from "./system-program-errors";

describe("favorites", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Writes our favorites to the blockchain", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
      anchor.getProvider().connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    const favoriteNumber = new anchor.BN(23);
    const favoriteColor = "red";

    let tx: string | null = null;
    try {
      tx = await program.methods
        .setFavorites(favoriteNumber, favoriteColor)
        .accounts({
          user: user.publicKey,
        })
        .signers([user])
        .rpc();
    } catch (thrownObject) {
      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }
    console.log(`Tx signature: ${tx}`);

    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );

    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(favoriteColor);
    expect(dataFromPda.number.toNumber()).toEqual(favoriteNumber.toNumber());
  });

  it("Updates existing favorites on the blockchain", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
      anchor.getProvider().connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Initial values
    const initialNumber = new anchor.BN(23);
    const initialColor = "red";

    // Set initial favorites
    let tx = await program.methods
      .setFavorites(initialNumber, initialColor)
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    
    console.log(`Set favorites tx signature: ${tx}`);

    // Find PDA for favorites account
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );

    // Verify initial values
    let dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(initialColor);
    expect(dataFromPda.number.toNumber()).toEqual(initialNumber.toNumber());

    // Update values
    const newNumber = new anchor.BN(42);
    const newColor = "blue";

    // Update favorites with both values
    tx = await program.methods
      .updateFavorites(
        newNumber,  // Correct type for BN
        newColor    // Correct type for string
      )
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    
    console.log(`Update favorites tx signature: ${tx}`);

    // Verify updated values
    dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(newColor);
    expect(dataFromPda.number.toNumber()).toEqual(newNumber.toNumber());
  });
});
