import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Solanainit } from "../target/types/solanainit";
import {
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
  createInitializeMintInstruction,
} from "@solana/spl-token";

describe("solanainit", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)


  const mintkey = anchor.web3.Keypair.generate()

  // let assiosatedTokenAccount = undefined;

  it("Is initialized!", async () => {
      
      const program = anchor.workspace.Solanainit as Program<Solanainit>;
      const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s')
      const lamports:number = await program.provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE)

    const getMetadata = async(
      mint : anchor.web3.PublicKey,
    ) : Promise<anchor.web3.PublicKey> =>{
      return(
          await anchor.web3.PublicKey.findProgramAddress([
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
          ],
            TOKEN_METADATA_PROGRAM_ID
          )
      )[0];
    };

    const getMasterEdition = async (
      mint: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
      return (
        await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
            Buffer.from("edition"),
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();

    // creating a mint acccount using anchor
    const NftTokenAccount = await getAssociatedTokenAddress(
      mintKey.publicKey,
      program.provider.publicKey
    )

    console.log("NFT Account: ", NftTokenAccount.toBase58());
    
    const mint_tx = new anchor.web3.Transaction().add(
        anchor.web3.SystemProgram.createAccount({
          fromPubkey : program.provider.publicKey,
          newAccountPubkey : mintKey.publicKey,
          space:MINT_SIZE,
          programId : TOKEN_PROGRAM_ID,
          lamports
        }),

        createInitializeMintInstruction(
          mintKey.publicKey,
          0,
          program.provider.publicKey,
          program.provider.publicKey
        ),

        createAssociatedTokenAccountInstruction(
          program.provider.publicKey,
          NftTokenAccount,
          program.provider.publicKey,
          mintKey.publicKey
        )
    )

    const response = await program.provider.sendAndConfirm(mint_tx,[mintKey])

    console.log(
      await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
    )

    console.log("Account: ", response);
    console.log("Mint key: ", mintKey.publicKey.toString());
    console.log("User: ", program.provider.publicKey.toString());
    const metadataAddress = await getMetadata(mintKey.publicKey);
    const masterEdition = await getMasterEdition(mintKey.publicKey);
    console.log("Metadata address: ", metadataAddress.toBase58());
    console.log("MasterEdition: ", masterEdition.toBase58());

    const tx = await program.methods.mintTo(
      mintKey.publicKey,
      "https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA",
      "First NFT",
    ).accounts({
      mintAuthority : program.provider.publicKey,
      mint: mintKey.publicKey,
          tokenAccount: NftTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          metadata: metadataAddress,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          payer: program.provider.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          masterEdition: masterEdition,
    }).rpc()
})


  

})


// ATA - associative token account
// user wallet account vs ATA
// in order to mint a token there will be following things needed
// user wallet account, ATA for token which will be controlled by wallet
// if you transfer tokens then first there need to be an ATA account controlled by reciever and then tokens can be transffered to that account 