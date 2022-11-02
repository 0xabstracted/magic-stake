export type CustomError =
  | InsufficientUserFunds
  | InsufficientVaultFunds
  | InvalidCalculation

export class InsufficientUserFunds extends Error {
  static readonly code = 6000
  readonly code = 6000
  readonly name = "InsufficientUserFunds"
  readonly msg = "Insufficient user funds"

  constructor(readonly logs?: string[]) {
    super("6000: Insufficient user funds")
  }
}

export class InsufficientVaultFunds extends Error {
  static readonly code = 6001
  readonly code = 6001
  readonly name = "InsufficientVaultFunds"
  readonly msg = "Insufficient vault funds"

  constructor(readonly logs?: string[]) {
    super("6001: Insufficient vault funds")
  }
}

export class InvalidCalculation extends Error {
  static readonly code = 6002
  readonly code = 6002
  readonly name = "InvalidCalculation"
  readonly msg = "Invalid calculation"

  constructor(readonly logs?: string[]) {
    super("6002: Invalid calculation")
  }
}

export function fromCode(code: number, logs?: string[]): CustomError | null {
  switch (code) {
    case 6000:
      return new InsufficientUserFunds(logs)
    case 6001:
      return new InsufficientVaultFunds(logs)
    case 6002:
      return new InvalidCalculation(logs)
  }

  return null
}
