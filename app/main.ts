import {fetchEvents} from "./event_checker";
import {AnchorProvider, Program, Wallet, web3} from "@coral-xyz/anchor";
import idl from "../target/idl/price_oracle.json"
import {IDL, PriceOracle} from "../target/types/price_oracle"
import fs from "fs";
import * as anchor from "@coral-xyz/anchor";

const RPC = "http://localhost:8899"
const connection = new web3.Connection(RPC)
const adminKeypair = loadWalletKey()
const wallet = new Wallet(adminKeypair)
const assetMint = new web3.PublicKey("Fnd3WMEGywcTjp3hdBnAepfJjcMJ2N1RwPpGqoV8Qsmp")

const programId = new web3.PublicKey(idl.metadata.address)
const provider = new AnchorProvider(connection, wallet, {})
const program = new Program<PriceOracle>(IDL, programId, provider)

const asset = anchor.web3.PublicKey.findProgramAddressSync(
    [assetMint.toBuffer()],
    program.programId
)[0];

async function main() {
    const assets = await fetchEvents(program, connection, asset)
    console.log({assets})
}

main()
    .then(_ => {})
    .catch(err => {throw err})

export function loadWalletKey(): web3.Keypair {
    return web3.Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(fs.readFileSync("/Users/matthewchukwuemeka/.config/solana/id.json").toString()))
    )
}