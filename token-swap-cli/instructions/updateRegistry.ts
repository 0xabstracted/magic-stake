import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface UpdateRegistryArgs {
  rateTokenIn: BN
  rateTokenOut: BN
}

export interface UpdateRegistryAccounts {
  registry: PublicKey
  admin: PublicKey
}

export const layout = borsh.struct([
  borsh.u64("rateTokenIn"),
  borsh.u64("rateTokenOut"),
])

export function updateRegistry(
  args: UpdateRegistryArgs,
  accounts: UpdateRegistryAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registry, isSigner: false, isWritable: true },
    { pubkey: accounts.admin, isSigner: true, isWritable: false },
  ]
  const identifier = Buffer.from([37, 83, 110, 3, 169, 151, 161, 237])
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
