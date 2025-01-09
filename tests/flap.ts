import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Flap } from "../target/types/flap";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { AuthorityType, createAccount, createMint, createUpdateAuthorityInstruction, getOrCreateAssociatedTokenAccount, mintTo, setAuthority, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BN } from "bn.js";

describe("flap", () => {
  
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const connection = provider.connection;
  const wallet = provider.wallet as NodeWallet;

  const admin = wallet.payer;
  const userA = anchor.web3.Keypair.generate();
  const userB = anchor.web3.Keypair.generate();

  const program = anchor.workspace.Flap as Program<Flap>;

  let flapMint: PublicKey;

  const pda = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("authority")
    ],
    program.programId
  )[0];
  
  let aFlapAccount: PublicKey;
  let bFlapAccount: PublicKey;

  it ("Setup", async () => {
    await connection.requestAirdrop(userA.publicKey, LAMPORTS_PER_SOL * 10);
    await connection.requestAirdrop(userB.publicKey, LAMPORTS_PER_SOL * 10);
    flapMint = await createMint(connection, admin, admin.publicKey, null, 2, undefined, undefined, TOKEN_PROGRAM_ID);
    aFlapAccount = await createAccount(
      connection,
      userA,
      flapMint,
      userA.publicKey,
    );
    await mintTo(
      connection,
      admin,
      flapMint,
      aFlapAccount,
      admin,
      10000 * (10 ** 6)
    )
    bFlapAccount = await createAccount(
      connection,
      userB,
      flapMint,
      userB.publicKey
    );
    await mintTo(
      connection,
      admin,
      flapMint,
      bFlapAccount,
      admin,
      10000 * (10 ** 6)
    )
    await setAuthority(
      connection,
      admin,
      flapMint,
      admin,
      AuthorityType.MintTokens,
      pda
    );

  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      flapMint,
      admin: admin.publicKey
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
