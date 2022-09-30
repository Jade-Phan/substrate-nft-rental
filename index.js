import { ApiPromise, WsProvider } from '@polkadot/api';
import { stringToU8a, u8aToHex } from '@polkadot/util';
import { Keyring } from '@polkadot/keyring';
// Construct
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
const keyring = new Keyring({ type: 'sr25519' });

// create Alice based on the development seed

const api = await ApiPromise.create({
    provider: wsProvider
});

const alice = keyring.addFromUri('//Alice');
console.log("alice, ",alice.address)

const order = await ApiPromise.create({
    types: {
        Order: {
            maker: 'AccountId',
            taker: 'AccountId',
            fee: 'u64',
            token: 'Vec<u8>',
            due_date: 'u64'
        }
    }
});

// create the message, actual signature and verify
const orders = order.createType('Order',{
    maker: alice.address,
    fee: 1000000000000,
    token: '0x352440e2e9891e5c4ca349b656169a575f739b11008142f8585fd7b55e6a7fc0',
    due_date: 1667099678
})
const message= stringToU8a(orders);
console.log(`Message ${u8aToHex(message)}`);
const signature = alice.sign(message);
const isValid = alice.verify(message, signature, alice.publicKey);

// // output the result

console.log(`${u8aToHex(signature)} is ${isValid ? 'valid' : 'invalid'}`);