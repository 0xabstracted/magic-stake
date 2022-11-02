import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CreateRegistryArgs {
  rateTokenIn: BN
  rateTokenOut: BN
}

export interface CreateRegistryAccounts {
  registry: PublicKey
  vaultTokenIn: PublicKey
  vaultTokenOut: PublicKey
  admin: PublicKey
  mintTokenIn: PublicKey
  mintTokenOut: PublicKey
  tokenProgram: PublicKey
  systemProgram: PublicKey
  rent: PublicKey
}

export const layout = borsh.struct([
  borsh.u64("rateTokenIn"),
  borsh.u64("rateTokenOut"),
])

export function createRegistry(
  args: CreateRegistryArgs,
  accounts: CreateRegistryAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registry, isSigner: false, isWritable: true },
    { pubkey: accounts.vaultTokenIn, isSigner: false, isWritable: true },
    { pubkey: accounts.vaultTokenOut, isSigner: false, isWritable: true },
    { pubkey: accounts.admin, isSigner: true, isWritable: true },
    { pubkey: accounts.mintTokenIn, isSigner: false, isWritable: false },
    { pubkey: accounts.mintTokenOut, isSigner: false, isWritable: false },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.rent, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([210, 219, 233, 49, 251, 19, 135, 13])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      rateTokenIn: args.rateTokenIn,
      rateTokenOut: args.rateTokenOut,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data })
  return ix
}
