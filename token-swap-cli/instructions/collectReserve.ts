import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CollectReserveAccounts {
  registry: PublicKey
  vaultTokenOut: PublicKey
  admin: PublicKey
  adminReserveAccount: PublicKey
  tokenProgram: PublicKey
}

export function collectReserve(accounts: CollectReserveAccounts) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registry, isSigner: false, isWritable: false },
    { pubkey: accounts.vaultTokenOut, isSigner: false, isWritable: true },
    { pubkey: accounts.admin, isSigner: true, isWritable: false },
    { pubkey: accounts.adminReserveAccount, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([155, 126, 75, 121, 41, 43, 43, 252])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data })
  return ix
}
