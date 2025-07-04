import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TonSolanaWallet } from "../target/types/ton_solana_wallet";
import { expect } from "chai";
import * as crypto from "crypto";

describe("ton-solana-wallet", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TonSolanaWallet as Program<TonSolanaWallet>;
  
  let factory: anchor.web3.Keypair;
  let wallet: anchor.web3.Keypair;
  let authority: anchor.web3.Keypair;
  
  // Mock TON keypair for testing
  const tonKeyPair = crypto.generateKeyPairSync('ed25519');
  const tonPublicKey = Array.from(tonKeyPair.publicKey.export({ type: 'spki', format: 'der' }).slice(-32));

  before(async () => {
    factory = anchor.web3.Keypair.generate();
    authority = anchor.web3.Keypair.generate();
    
    // Airdrop SOL for testing
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(authority.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL)
    );
  });

  it("Initializes the factory", async () => {
    await program.methods
      .initializeFactory()
      .accounts({
        factory: factory.publicKey,
        authority: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([factory, authority])
      .rpc();

    const factoryAccount = await program.account.factory.fetch(factory.publicKey);
    expect(factoryAccount.authority.toString()).to.equal(authority.publicKey.toString());
    expect(factoryAccount.walletCount.toString()).to.equal("0");
  });

  it("Creates a new smart wallet", async () => {
    const nonce = new anchor.BN(1);
    
    const [walletPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("wallet"),
        Buffer.from(tonPublicKey),
        nonce.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );

    await program.methods
      .createWallet(tonPublicKey as any, nonce)
      .accounts({
        factory: factory.publicKey,
        wallet: walletPda,
        owner: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    const walletAccount = await program.account.smartWallet.fetch(walletPda);
    expect(walletAccount.factory.toString()).to.equal(factory.publicKey.toString());
    expect(walletAccount.tonPublicKey).to.deep.equal(tonPublicKey);
    expect(walletAccount.isActive).to.be.true;

    const factoryAccount = await program.account.factory.fetch(factory.publicKey);
    expect(factoryAccount.walletCount.toString()).to.equal("1");
  });

  it("Executes a transaction with valid TON signature", async () => {
    // This test would require proper TON signature generation
    // For now, it's a placeholder to show the structure
    console.log("Transaction execution test - requires TON signature implementation");
  });

  it("Fails to execute transaction with invalid signature", async () => {
    // Test invalid signature handling
    console.log("Invalid signature test - requires implementation");
  });
});