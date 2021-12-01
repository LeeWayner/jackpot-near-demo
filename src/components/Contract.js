import React, { useEffect, useState } from 'react';
import * as nearAPI from 'near-api-js';
import { GAS, parseNearAmount } from '../state/near';
import {
    createAccessKeyAccount,
    getContract,
} from '../utils/near-utils';

const {
    KeyPair,
    utils: { format: { formatNearAmount } }
} = nearAPI;

export const Contract = ({ near, update, account }) => {

    if (!account) return null;

    const [credits, setCredits] = useState('');
    const [amount, setAmount] = useState('');
    const [flips, setFlips] = useState([]);

    useEffect(() => {
        updateBalance();
    }, []);

    const updateBalance = async () => {
        const contract = getContract(account);
        setCredits(await contract.get_balance())
    };

    const handleDeposit = async () => {
        const contract = getContract(account);
        await contract.deposit({}, GAS, parseNearAmount(amount))
        updateBalance()
    };

    const handlePlay = async () => {
        const contract = getContract(account);
        await contract.deposit({}, GAS, parseNearAmount(amount))
        const outcome = await contract.play({}, GAS)
        flips.push(outcome < 128)
        updateBalance()
    };

    return <>
        <h3>Play</h3>
        <p>Current Credits: { formatNearAmount(credits, 0) }</p>
        <input placeholder="Bet Amount (N)" value={amount} onChange={(e) => setAmount(e.target.value)} />
        {/* <br />
        <button onClick={() => handleDeposit()}>Buy Credits</button>
        <br /> */}
        <br />
        <button onClick={() => handlePlay()}>Play Jackpot</button>
        {
            flips.map((f, i) => f ? <p key={i}>Won</p> : <p key={i}>Lost</p>)
        }
    </>;
};

