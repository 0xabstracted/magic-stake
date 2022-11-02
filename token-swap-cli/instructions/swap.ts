import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface SwapArgs {
  amountRequested: BN
}

export interface SwapAccounts {
  registry: PublicKey
  swapper: PublicKey
  vaultTokenIn: PublicKey
  vaultTokenOut: PublicKey
  buyerTokenInAccount: PublicKey
  buyerTokenOutAccount: PublicKey
  tokenProgram: PublicKey
}

export const layout = borsh.struct([borsh.u64("amountRequested")])

export function swap(args: SwapArgs, accounts: SwapAccounts) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registry, isSigner: false, isWritable: false },
    { pubkey: accounts.swapper, isSigner: true, isWritable: false },
    { pubkey: accounts.vaultTokenIn, isSigner: false, isWritable: true },
    { pubkey: accounts.vaultTokenOut, isSigner: false, isWritable: true },
    { pubkey: accounts.buyerTokenInAccount, isSigner: false, isWritable: true },
    {
      pubkey: accounts.buyerTokenOutAccount,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([248, 198, 158, 145, 225, 117, 135, 200])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      amountRequested: args.amountRequested,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data })
  return ix
}
