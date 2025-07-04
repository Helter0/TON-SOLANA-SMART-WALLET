const anchor = require('@coral-xyz/anchor');
const { Connection, PublicKey, Keypair } = require('@solana/web3.js');
const fs = require('fs');

async function deployContract() {
    // Connect to devnet
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    
    // Load wallet
    const keypairPath = process.env.HOME + '/.config/solana/id.json';
    const secretKey = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));
    const wallet = Keypair.fromSecretKey(new Uint8Array(secretKey));
    
    console.log('Wallet address:', wallet.publicKey.toString());
    
    // Check balance
    const balance = await connection.getBalance(wallet.publicKey);
    console.log('Balance:', balance / 1e9, 'SOL');
    
    if (balance < 1e9) {
        console.log('Requesting airdrop...');
        const airdropSignature = await connection.requestAirdrop(wallet.publicKey, 2e9);
        await connection.confirmTransaction(airdropSignature);
        console.log('Airdrop completed');
    }
    
    // For now, just show deployment info
    console.log('Contract ready for deployment');
    console.log('Program ID: CEzs3bEbqXFZ528gTj3RXDctityB7PZj5VmsEesz5oQ7');
    console.log('Next: Use anchor build && anchor deploy');
}

deployContract().catch(console.error);