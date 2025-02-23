import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Flap } from "../target/types/flap";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { AuthorityType, createAccount, createMint, getAssociatedTokenAddressSync, mintTo, setAuthority, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BN } from "bn.js";
import { ON_DEMAND_DEVNET_PID, ON_DEMAND_DEVNET_QUEUE, Randomness } from "@switchboard-xyz/on-demand";

describe("flap", async () => {
  
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
  let stableMint: PublicKey;

  const pda = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("authority")
    ],
    program.programId
  )[0];
  
  let aFlapAccount: PublicKey;
  let bFlapAccount: PublicKey;
  let aStableAccount: PublicKey;
  let bStableAccount: PublicKey;
  const gameKeypair = anchor.web3.Keypair.generate();

  const betAmount = 10000 * (10 ** 6);

  const sbQueue = ON_DEMAND_DEVNET_QUEUE;
  const sbProgramId = ON_DEMAND_DEVNET_PID;
  const sbIdl = await anchor.Program.fetchIdl(sbProgramId, provider);
  const sbProgram = new anchor.Program(sbIdl, provider);
  const rngKp = Keypair.generate();
  const raffleKeypair = Keypair.generate();

  let randomnessAccountData: PublicKey;
  it ("Setup", async () => {
    await connection.requestAirdrop(userA.publicKey, LAMPORTS_PER_SOL * 10);
    await connection.requestAirdrop(userB.publicKey, LAMPORTS_PER_SOL * 10);
    flapMint = await createMint(connection, admin, admin.publicKey, null, 6, undefined, undefined, TOKEN_PROGRAM_ID);
    stableMint = await createMint(connection, admin, admin.publicKey, null, 6, undefined, undefined, TOKEN_PROGRAM_ID);
    aFlapAccount = await createAccount(
      connection,
      userA,
      flapMint,
      userA.publicKey,
    );
    aStableAccount = await createAccount(
      connection,
      userA,
      stableMint,
      userA.publicKey,
    )
    await mintTo(
      connection,
      admin,
      flapMint,
      aFlapAccount,
      admin,
      betAmount * 2
    )
    await mintTo(
      connection,
      admin,
      stableMint,
      aStableAccount,
      admin,
      betAmount
    )
    bFlapAccount = await createAccount(
      connection,
      userB,
      flapMint,
      userB.publicKey
    );
    bStableAccount = await createAccount(
      connection,
      userB,
      stableMint,
      userB.publicKey
    );
    await mintTo(
      connection,
      admin,
      flapMint,
      bFlapAccount,
      admin,
      betAmount
    )
    await mintTo(
      connection,
      admin,
      stableMint,
      bStableAccount,
      admin,
      betAmount
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

  it ("Deposit", async () => {
    const tx1 = await program.methods.deposit(new BN(betAmount * 2)).accounts({
      owner: userA.publicKey,
      playerFlapAccount: aFlapAccount
    }).signers([userA]).rpc().catch(e => console.log(e));
    console.log("Your transaction signature", tx1);
    const tx2 = await program.methods.deposit(new BN(betAmount)).accounts({
      owner: userB.publicKey,
      playerFlapAccount: bFlapAccount
    }).signers([userB]).rpc().catch(e => console.log(e));
    console.log("Your transaction signature", tx2);
  });

  it ("Withdraw", async () => {
    const tx = await program.methods.withdraw(new BN(betAmount)).accounts({
      owner: userA.publicKey,
      playerFlapAccount: aFlapAccount
    }).signers([userA]).rpc();
    console.log("Your transaction signature", tx);
  })

  it ("Create game", async () => {
    const tx = await program.methods.createGame(
      new BN(betAmount)
    ).accounts({
      creator: userA.publicKey,
      game: gameKeypair.publicKey
    }).signers([userA, gameKeypair]).rpc();
    console.log("Your transaction signature", tx);
  });

  it ("Join game", async () => {
    const [randomness, ix] = await Randomness.create(sbProgram, rngKp, sbQueue);
    randomnessAccountData = randomness.pubkey;
    const tx = await program.methods.joinGame().accounts({
      signer: userB.publicKey,
      game: gameKeypair.publicKey,
      randomnessAccountData
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it ("Set flip", async () => {
    const tx = await program.methods.settleFlip().accounts({
      signer: userB.publicKey,
      randomnessAccountData
    }).signers([userB]).rpc();
    console.log("Your transaction signature", tx);
  });

  it ("Create raffle", async () => {
    const now = new BN(Math.floor(Date.now() / 1000));

    const tx = await program.methods.createRaffle(
      new BN(0),
      new BN(10 ** 6),
      now,
      new BN(Math.floor(Date.now() / 1000 + 5)),
      2,
      0
    ).accounts({
      stableMint,
      raffle: raffleKeypair.publicKey
    }).signers([raffleKeypair]).rpc();
    console.log("Your transaction signature", tx);
  });
  it ("Buy ticket", async () => {
    const tx = await program.methods.buyTicket(
      1
    ).accounts({
      signer: userA.publicKey,
      stableMint,
      stableAccount: aStableAccount,
      raffle: raffleKeypair.publicKey
    }).signers([userA]).rpc();
    console.log("Your transaction signature", tx);
  });

  it ("Reveal Winner", async () => {
    const stableAccount = getAssociatedTokenAddressSync(
      stableMint,
      admin.publicKey
    )
    const tx = await program.methods.revealWinner().accounts({
      raffle: raffleKeypair.publicKey,
      stableMint,
      stableAccount,
    }).rpc();
    console.log("Your transaction signature", tx);
  })

  it ("Cliam prize", async () => {
    const tx = await program.methods.claimePrize().accounts({
      winner: userA.publicKey,
      raffle: raffleKeypair.publicKey,
      flapMint,
      winnerFlapAccount: aFlapAccount
    }).signers([userA]).rpc();
  })
});
