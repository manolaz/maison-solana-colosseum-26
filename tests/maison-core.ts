import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MaisonCore } from "../target/types/maison_core";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import { expect } from "chai";
import * as crypto from "crypto";

describe("maison-core", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MaisonCore as Program<MaisonCore>;
  const admin = (provider.wallet as any).payer as Keypair;
  const sender = Keypair.generate();
  const feeTreasury = Keypair.generate();
  const beneficiaryA = Keypair.generate();
  const beneficiaryB = Keypair.generate();
  const relayer = Keypair.generate();

  let mint: PublicKey;
  let senderTokenAccount: PublicKey;
  let treasuryTokenAccount: PublicKey;
  let beneficiaryATokenAccount: PublicKey;
  let beneficiaryBTokenAccount: PublicKey;
  let protocolConfigPda: PublicKey;

  before(async () => {
    // Airdrop SOL to test accounts
    const airdropSigners = [sender, feeTreasury, beneficiaryA, beneficiaryB, relayer];
    for (const signer of airdropSigners) {
      const sig = await provider.connection.requestAirdrop(signer.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
      await provider.connection.confirmTransaction(sig);
    }

    // Derive protocol config PDA
    [protocolConfigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("protocol_config")],
      program.programId
    );

    // Create standard SPL / Token-2022 test mint
    mint = await createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      6,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    // Create associated token accounts
    senderTokenAccount = await createAccount(
      provider.connection,
      sender,
      mint,
      sender.publicKey,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    treasuryTokenAccount = await createAccount(
      provider.connection,
      feeTreasury,
      mint,
      feeTreasury.publicKey,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    beneficiaryATokenAccount = await createAccount(
      provider.connection,
      beneficiaryA,
      mint,
      beneficiaryA.publicKey,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    beneficiaryBTokenAccount = await createAccount(
      provider.connection,
      beneficiaryB,
      mint,
      beneficiaryB.publicKey,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    // Mint test liquidity to sender: 10,000 tokens (e.g. 10,000 USDC)
    await mintTo(
      provider.connection,
      admin,
      mint,
      senderTokenAccount,
      admin.publicKey,
      10_000 * 10 ** 6
    );
  });

  it("1. Initializes global protocol configuration with fee treasury and 25 bps fee", async () => {
    const feeBps = 25; // 0.25% protocol fee
    await program.methods
      .initializeProtocol(feeBps)
      .accounts({
        admin: admin.publicKey,
        protocolConfig: protocolConfigPda,
        feeTreasury: feeTreasury.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const config = await program.account.protocolConfig.fetch(protocolConfigPda);
    expect(config.admin.toBase58()).to.equal(admin.publicKey.toBase58());
    expect(config.feeTreasury.toBase58()).to.equal(feeTreasury.publicKey.toBase58());
    expect(config.protocolFeeBps).to.equal(feeBps);
    expect(config.isPaused).to.be.false;
  });

  it("2. Creates a remittance RouteProfile with multi-destination split rules", async () => {
    const profileId = Array.from(crypto.randomBytes(32));
    const [routeProfilePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("route_profile"), sender.publicKey.toBuffer(), Buffer.from(profileId)],
      program.programId
    );

    // Split: 70% (7,000 bps) to Beneficiary A, 30% (3,000 bps) to Beneficiary B
    const labelA = Array.from(Buffer.alloc(32));
    Buffer.from("Living Expenses").copy(Buffer.from(labelA));
    const labelB = Array.from(Buffer.alloc(32));
    Buffer.from("Savings Vault").copy(Buffer.from(labelB));

    const splits = [
      {
        recipient: beneficiaryA.publicKey,
        basisPoints: 7000,
        label: labelA,
        vaultType: { immediate: {} },
      },
      {
        recipient: beneficiaryB.publicKey,
        basisPoints: 3000,
        label: labelB,
        vaultType: { savingsEscrow: {} },
      },
    ];

    const dailyLimit = new anchor.BN(5_000 * 10 ** 6);
    const complianceTier = 2; // Verified Tier

    await program.methods
      .createRouteProfile(profileId, splits, dailyLimit, complianceTier)
      .accounts({
        owner: sender.publicKey,
        routeProfile: routeProfilePda,
        systemProgram: SystemProgram.programId,
      })
      .signers([sender])
      .rpc();

    const profile = await program.account.routeProfile.fetch(routeProfilePda);
    expect(profile.owner.toBase58()).to.equal(sender.publicKey.toBase58());
    expect(profile.splits.length).to.equal(2);
    expect(profile.splits[0].basisPoints).to.equal(7000);
    expect(profile.splits[1].basisPoints).to.equal(3000);
    expect(profile.isActive).to.be.true;
  });

  it("3. Executes atomic multi-destination split routing", async () => {
    const profileId = Array.from(crypto.randomBytes(32));
    const [routeProfilePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("route_profile"), sender.publicKey.toBuffer(), Buffer.from(profileId)],
      program.programId
    );

    const labelA = Array.from(Buffer.alloc(32));
    const labelB = Array.from(Buffer.alloc(32));
    const splits = [
      {
        recipient: beneficiaryA.publicKey,
        basisPoints: 8000,
        label: labelA,
        vaultType: { immediate: {} },
      },
      {
        recipient: beneficiaryB.publicKey,
        basisPoints: 2000,
        label: labelB,
        vaultType: { immediate: {} },
      },
    ];

    await program.methods
      .createRouteProfile(profileId, splits, new anchor.BN(0), 1)
      .accounts({
        owner: sender.publicKey,
        routeProfile: routeProfilePda,
        systemProgram: SystemProgram.programId,
      })
      .signers([sender])
      .rpc();

    const grossAmount = new anchor.BN(1_000 * 10 ** 6); // 1,000 USDC
    const externalRef = Array.from(crypto.randomBytes(32));

    await program.methods
      .executeSplitRouting(grossAmount, externalRef)
      .accounts({
        sender: sender.publicKey,
        protocolConfig: protocolConfigPda,
        routeProfile: routeProfilePda,
        tokenMint: mint,
        senderTokenAccount,
        feeTreasuryTokenAccount: treasuryTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .remainingAccounts([
        { pubkey: beneficiaryATokenAccount, isWritable: true, isSigner: false },
        { pubkey: beneficiaryBTokenAccount, isWritable: true, isSigner: false },
      ])
      .signers([sender])
      .rpc();

    const accA = await getAccount(provider.connection, beneficiaryATokenAccount);
    const accB = await getAccount(provider.connection, beneficiaryBTokenAccount);
    expect(Number(accA.amount)).to.be.greaterThan(0);
    expect(Number(accB.amount)).to.be.greaterThan(0);
  });

  it("4. Initiates a remittance into escrow with secret preimage hash lock", async () => {
    const externalRef = Array.from(crypto.randomBytes(32));
    const secretVoucher = crypto.randomBytes(32);
    const claimCommitment = Array.from(crypto.createHash("sha256").update(secretVoucher).digest());

    const [escrowPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), sender.publicKey.toBuffer(), Buffer.from(externalRef)],
      program.programId
    );

    const [escrowVaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), escrowPda.toBuffer(), Buffer.from("vault")],
      program.programId
    );

    const grossAmount = new anchor.BN(500 * 10 ** 6); // 500 USDC
    const timelockSeconds = new anchor.BN(3600); // 1 hour

    await program.methods
      .initiateRemittance(
        externalRef,
        grossAmount,
        { preimageHash: {} },
        claimCommitment,
        timelockSeconds
      )
      .accounts({
        sender: sender.publicKey,
        protocolConfig: protocolConfigPda,
        escrow: escrowPda,
        tokenMint: mint,
        senderTokenAccount,
        feeTreasuryTokenAccount: treasuryTokenAccount,
        escrowVault: escrowVaultPda,
        beneficiary: beneficiaryA.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([sender])
      .rpc();

    const escrow = await program.account.remittanceEscrow.fetch(escrowPda);
    expect(escrow.sender.toBase58()).to.equal(sender.publicKey.toBase58());
    expect(escrow.beneficiary.toBase58()).to.equal(beneficiaryA.publicKey.toBase58());
    expect(escrow.status.active).to.be.ok;

    // Beneficiary claims escrow using valid secret preimage voucher
    const claimPayload = Array.from(secretVoucher);
    await program.methods
      .claimEscrow(claimPayload, null, null)
      .accounts({
        claimant: beneficiaryA.publicKey,
        escrow: escrowPda,
        tokenMint: mint,
        escrowVault: escrowVaultPda,
        claimantTokenAccount: beneficiaryATokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([beneficiaryA])
      .rpc();

    const claimedEscrow = await program.account.remittanceEscrow.fetch(escrowPda);
    expect(claimedEscrow.status.claimed).to.be.ok;
  });

  it("5. Settles cross-chain remittance message (Base CCTP / Tempo TIP-403)", async () => {
    const sourceDomain = 6; // Base CCTP domain
    const remoteNonce = new anchor.BN(10482);
    const remoteSender = Array.from(crypto.randomBytes(32));
    const externalRef = Array.from(crypto.randomBytes(32));
    const amount = new anchor.BN(250 * 10 ** 6);

    const [settlementPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("settlement"),
        new anchor.BN(sourceDomain).toArrayLike(Buffer, "le", 4),
        remoteNonce.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // Relayer funds relayer token account for inbound settlement
    const relayerTokenAccount = await createAccount(
      provider.connection,
      relayer,
      mint,
      relayer.publicKey,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    await mintTo(
      provider.connection,
      admin,
      mint,
      relayerTokenAccount,
      admin.publicKey,
      500 * 10 ** 6
    );

    await program.methods
      .settleCrossChain(
        sourceDomain,
        remoteNonce,
        remoteSender,
        amount,
        externalRef,
        true // compliance verified
      )
      .accounts({
        relayer: relayer.publicKey,
        protocolConfig: protocolConfigPda,
        settlementRecord: settlementPda,
        tokenMint: mint,
        relayerTokenAccount,
        recipientTokenAccount: beneficiaryBTokenAccount,
        recipient: beneficiaryB.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([relayer])
      .rpc();

    const settlement = await program.account.settlementRecord.fetch(settlementPda);
    expect(settlement.sourceDomain).to.equal(sourceDomain);
    expect(settlement.remoteNonce.toNumber()).to.equal(10482);
    expect(settlement.recipient.toBase58()).to.equal(beneficiaryB.publicKey.toBase58());
    expect(settlement.complianceVerified).to.be.true;
  });
});
