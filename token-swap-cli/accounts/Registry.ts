import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface RegistryFields {
  admin: PublicKey
  vaultTokenIn: PublicKey
  vaultTokenOut: PublicKey
  rateTokenIn: BN
  rateTokenOut: BN
  mintTokenIn: PublicKey
  mintTokenOut: PublicKey
}

export interface RegistryJSON {
  admin: string
  vaultTokenIn: string
  vaultTokenOut: string
  rateTokenIn: string
  rateTokenOut: string
  mintTokenIn: string
  mintTokenOut: string
}

export class Registry {
  readonly admin: PublicKey
  readonly vaultTokenIn: PublicKey
  readonly vaultTokenOut: PublicKey
  readonly rateTokenIn: BN
  readonly rateTokenOut: BN
  readonly mintTokenIn: PublicKey
  readonly mintTokenOut: PublicKey

  static readonly discriminator = Buffer.from([
    47, 174, 110, 246, 184, 182, 252, 218,
  ])

  static readonly layout = borsh.struct([
    borsh.publicKey("admin"),
    borsh.publicKey("vaultTokenIn"),
    borsh.publicKey("vaultTokenOut"),
    borsh.u64("rateTokenIn"),
    borsh.u64("rateTokenOut"),
    borsh.publicKey("mintTokenIn"),
    borsh.publicKey("mintTokenOut"),
  ])

  constructor(fields: RegistryFields) {
    this.admin = fields.admin
    this.vaultTokenIn = fields.vaultTokenIn
    this.vaultTokenOut = fields.vaultTokenOut
    this.rateTokenIn = fields.rateTokenIn
    this.rateTokenOut = fields.rateTokenOut
    this.mintTokenIn = fields.mintTokenIn
    this.mintTokenOut = fields.mintTokenOut
  }

  static async fetch(
    c: Connection,
    address: PublicKey
  ): Promise<Registry | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(PROGRAM_ID)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[]
  ): Promise<Array<Registry | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(PROGRAM_ID)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): Registry {
    if (!data.slice(0, 8).equals(Registry.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = Registry.layout.decode(data.slice(8))

    return new Registry({
      admin: dec.admin,
      vaultTokenIn: dec.vaultTokenIn,
      vaultTokenOut: dec.vaultTokenOut,
      rateTokenIn: dec.rateTokenIn,
      rateTokenOut: dec.rateTokenOut,
      mintTokenIn: dec.mintTokenIn,
      mintTokenOut: dec.mintTokenOut,
    })
  }

  toJSON(): RegistryJSON {
    return {
      admin: this.admin.toString(),
      vaultTokenIn: this.vaultTokenIn.toString(),
      vaultTokenOut: this.vaultTokenOut.toString(),
      rateTokenIn: this.rateTokenIn.toString(),
      rateTokenOut: this.rateTokenOut.toString(),
      mintTokenIn: this.mintTokenIn.toString(),
      mintTokenOut: this.mintTokenOut.toString(),
    }
  }

  static fromJSON(obj: RegistryJSON): Registry {
    return new Registry({
      admin: new PublicKey(obj.admin),
      vaultTokenIn: new PublicKey(obj.vaultTokenIn),
      vaultTokenOut: new PublicKey(obj.vaultTokenOut),
      rateTokenIn: new BN(obj.rateTokenIn),
      rateTokenOut: new BN(obj.rateTokenOut),
      mintTokenIn: new PublicKey(obj.mintTokenIn),
      mintTokenOut: new PublicKey(obj.mintTokenOut),
    })
  }
}
