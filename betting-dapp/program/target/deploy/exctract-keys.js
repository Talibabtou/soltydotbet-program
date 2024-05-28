const fs = require('fs');
const path = require('path');
const { Keypair } = require('@solana/web3.js');

// Path to your key-pair JSON file
const keypairPath = path.resolve(__dirname, 'betting_contract-keypair.json');

// Read the key-pair JSON file
const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));

// Create a Keypair object
const keypair = Keypair.fromSecretKey(Uint8Array.from(keypairData));

// Extract the public key and private key
const publicKey = keypair.publicKey.toString();
const privateKey = keypair.secretKey.toString();

console.log('Public Key:', publicKey);
console.log('Private Key:', privateKey);