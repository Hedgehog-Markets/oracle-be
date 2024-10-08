/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import type { Program } from "@metaplex-foundation/umi";

import { ProgramError } from "@metaplex-foundation/umi";

type ProgramErrorConstructor = new (program: Program, cause?: Error) => ProgramError;
const codeToErrorMap = new Map<number, ProgramErrorConstructor>();
const nameToErrorMap = new Map<string, ProgramErrorConstructor>();

/** DeserializationError: Failed to deserialize account */
export class DeserializationErrorError extends ProgramError {
  override readonly name: string = "DeserializationError";

  readonly code: number = 0x0; // 0

  constructor(program: Program, cause?: Error) {
    super("Failed to deserialize account", program, cause);
  }
}
codeToErrorMap.set(0x0, DeserializationErrorError);
nameToErrorMap.set("DeserializationError", DeserializationErrorError);

/** SerializationError: Failed to serialize account */
export class SerializationErrorError extends ProgramError {
  override readonly name: string = "SerializationError";

  readonly code: number = 0x1; // 1

  constructor(program: Program, cause?: Error) {
    super("Failed to serialize account", program, cause);
  }
}
codeToErrorMap.set(0x1, SerializationErrorError);
nameToErrorMap.set("SerializationError", SerializationErrorError);

/** RewardBounds: Reward must be within valid bounds */
export class RewardBoundsError extends ProgramError {
  override readonly name: string = "RewardBounds";

  readonly code: number = 0x2; // 2

  constructor(program: Program, cause?: Error) {
    super("Reward must be within valid bounds", program, cause);
  }
}
codeToErrorMap.set(0x2, RewardBoundsError);
nameToErrorMap.set("RewardBounds", RewardBoundsError);

/** BondBounds: Bond must be within valid bounds */
export class BondBoundsError extends ProgramError {
  override readonly name: string = "BondBounds";

  readonly code: number = 0x3; // 3

  constructor(program: Program, cause?: Error) {
    super("Bond must be within valid bounds", program, cause);
  }
}
codeToErrorMap.set(0x3, BondBoundsError);
nameToErrorMap.set("BondBounds", BondBoundsError);

/** InvalidValue: Value is not valid for the request */
export class InvalidValueError extends ProgramError {
  override readonly name: string = "InvalidValue";

  readonly code: number = 0x4; // 4

  constructor(program: Program, cause?: Error) {
    super("Value is not valid for the request", program, cause);
  }
}
codeToErrorMap.set(0x4, InvalidValueError);
nameToErrorMap.set("InvalidValue", InvalidValueError);

/** InvalidBps: Invalid basis points value */
export class InvalidBpsError extends ProgramError {
  override readonly name: string = "InvalidBps";

  readonly code: number = 0x5; // 5

  constructor(program: Program, cause?: Error) {
    super("Invalid basis points value", program, cause);
  }
}
codeToErrorMap.set(0x5, InvalidBpsError);
nameToErrorMap.set("InvalidBps", InvalidBpsError);

/** DisputerIsAsserter: Disputer cannot be the same as the asserter */
export class DisputerIsAsserterError extends ProgramError {
  override readonly name: string = "DisputerIsAsserter";

  readonly code: number = 0x6; // 6

  constructor(program: Program, cause?: Error) {
    super("Disputer cannot be the same as the asserter", program, cause);
  }
}
codeToErrorMap.set(0x6, DisputerIsAsserterError);
nameToErrorMap.set("DisputerIsAsserter", DisputerIsAsserterError);

/** NotAsserted: Request does not have an assertion */
export class NotAssertedError extends ProgramError {
  override readonly name: string = "NotAsserted";

  readonly code: number = 0x7; // 7

  constructor(program: Program, cause?: Error) {
    super("Request does not have an assertion", program, cause);
  }
}
codeToErrorMap.set(0x7, NotAssertedError);
nameToErrorMap.set("NotAsserted", NotAssertedError);

/** NotDisputed: Request is not disputed */
export class NotDisputedError extends ProgramError {
  override readonly name: string = "NotDisputed";

  readonly code: number = 0x8; // 8

  constructor(program: Program, cause?: Error) {
    super("Request is not disputed", program, cause);
  }
}
codeToErrorMap.set(0x8, NotDisputedError);
nameToErrorMap.set("NotDisputed", NotDisputedError);

/** NotResolved: Request is not resolved */
export class NotResolvedError extends ProgramError {
  override readonly name: string = "NotResolved";

  readonly code: number = 0x9; // 9

  constructor(program: Program, cause?: Error) {
    super("Request is not resolved", program, cause);
  }
}
codeToErrorMap.set(0x9, NotResolvedError);
nameToErrorMap.set("NotResolved", NotResolvedError);

/** AlreadyAsserted: Request already has an assertion */
export class AlreadyAssertedError extends ProgramError {
  override readonly name: string = "AlreadyAsserted";

  readonly code: number = 0xa; // 10

  constructor(program: Program, cause?: Error) {
    super("Request already has an assertion", program, cause);
  }
}
codeToErrorMap.set(0xa, AlreadyAssertedError);
nameToErrorMap.set("AlreadyAsserted", AlreadyAssertedError);

/** AlreadyDisputed: Assertion has already been disputed */
export class AlreadyDisputedError extends ProgramError {
  override readonly name: string = "AlreadyDisputed";

  readonly code: number = 0xb; // 11

  constructor(program: Program, cause?: Error) {
    super("Assertion has already been disputed", program, cause);
  }
}
codeToErrorMap.set(0xb, AlreadyDisputedError);
nameToErrorMap.set("AlreadyDisputed", AlreadyDisputedError);

/** AlreadyResolved: Request has already been resolved */
export class AlreadyResolvedError extends ProgramError {
  override readonly name: string = "AlreadyResolved";

  readonly code: number = 0xc; // 12

  constructor(program: Program, cause?: Error) {
    super("Request has already been resolved", program, cause);
  }
}
codeToErrorMap.set(0xc, AlreadyResolvedError);
nameToErrorMap.set("AlreadyResolved", AlreadyResolvedError);

/** AssertionTooEarly: Request is not accepting assertion yet */
export class AssertionTooEarlyError extends ProgramError {
  override readonly name: string = "AssertionTooEarly";

  readonly code: number = 0xd; // 13

  constructor(program: Program, cause?: Error) {
    super("Request is not accepting assertion yet", program, cause);
  }
}
codeToErrorMap.set(0xd, AssertionTooEarlyError);
nameToErrorMap.set("AssertionTooEarly", AssertionTooEarlyError);

/** DisputeWindowNotExpired: Dispute window has not expired */
export class DisputeWindowNotExpiredError extends ProgramError {
  override readonly name: string = "DisputeWindowNotExpired";

  readonly code: number = 0xe; // 14

  constructor(program: Program, cause?: Error) {
    super("Dispute window has not expired", program, cause);
  }
}
codeToErrorMap.set(0xe, DisputeWindowNotExpiredError);
nameToErrorMap.set("DisputeWindowNotExpired", DisputeWindowNotExpiredError);

/** DisputeWindowExpired: Dispute window has expired */
export class DisputeWindowExpiredError extends ProgramError {
  override readonly name: string = "DisputeWindowExpired";

  readonly code: number = 0xf; // 15

  constructor(program: Program, cause?: Error) {
    super("Dispute window has expired", program, cause);
  }
}
codeToErrorMap.set(0xf, DisputeWindowExpiredError);
nameToErrorMap.set("DisputeWindowExpired", DisputeWindowExpiredError);

/** VotingWindowNotExpired: Voting window has not expired */
export class VotingWindowNotExpiredError extends ProgramError {
  override readonly name: string = "VotingWindowNotExpired";

  readonly code: number = 0x10; // 16

  constructor(program: Program, cause?: Error) {
    super("Voting window has not expired", program, cause);
  }
}
codeToErrorMap.set(0x10, VotingWindowNotExpiredError);
nameToErrorMap.set("VotingWindowNotExpired", VotingWindowNotExpiredError);

/** VotingWindowExpired: Voting window has expired */
export class VotingWindowExpiredError extends ProgramError {
  override readonly name: string = "VotingWindowExpired";

  readonly code: number = 0x11; // 17

  constructor(program: Program, cause?: Error) {
    super("Voting window has expired", program, cause);
  }
}
codeToErrorMap.set(0x11, VotingWindowExpiredError);
nameToErrorMap.set("VotingWindowExpired", VotingWindowExpiredError);

/** ArbitrationWindowNotExpired: Arbitration window has not expired */
export class ArbitrationWindowNotExpiredError extends ProgramError {
  override readonly name: string = "ArbitrationWindowNotExpired";

  readonly code: number = 0x12; // 18

  constructor(program: Program, cause?: Error) {
    super("Arbitration window has not expired", program, cause);
  }
}
codeToErrorMap.set(0x12, ArbitrationWindowNotExpiredError);
nameToErrorMap.set("ArbitrationWindowNotExpired", ArbitrationWindowNotExpiredError);

/** OracleAuthorityMismatch: Oracle authority address does not match */
export class OracleAuthorityMismatchError extends ProgramError {
  override readonly name: string = "OracleAuthorityMismatch";

  readonly code: number = 0x13; // 19

  constructor(program: Program, cause?: Error) {
    super("Oracle authority address does not match", program, cause);
  }
}
codeToErrorMap.set(0x13, OracleAuthorityMismatchError);
nameToErrorMap.set("OracleAuthorityMismatch", OracleAuthorityMismatchError);

/** ConfigAuthorityMismatch: Config authority address does not match */
export class ConfigAuthorityMismatchError extends ProgramError {
  override readonly name: string = "ConfigAuthorityMismatch";

  readonly code: number = 0x14; // 20

  constructor(program: Program, cause?: Error) {
    super("Config authority address does not match", program, cause);
  }
}
codeToErrorMap.set(0x14, ConfigAuthorityMismatchError);
nameToErrorMap.set("ConfigAuthorityMismatch", ConfigAuthorityMismatchError);

/** ConfigMismatch: Config address does not match */
export class ConfigMismatchError extends ProgramError {
  override readonly name: string = "ConfigMismatch";

  readonly code: number = 0x15; // 21

  constructor(program: Program, cause?: Error) {
    super("Config address does not match", program, cause);
  }
}
codeToErrorMap.set(0x15, ConfigMismatchError);
nameToErrorMap.set("ConfigMismatch", ConfigMismatchError);

/** CurrencyMintMismatch: Currency mint address does not match */
export class CurrencyMintMismatchError extends ProgramError {
  override readonly name: string = "CurrencyMintMismatch";

  readonly code: number = 0x16; // 22

  constructor(program: Program, cause?: Error) {
    super("Currency mint address does not match", program, cause);
  }
}
codeToErrorMap.set(0x16, CurrencyMintMismatchError);
nameToErrorMap.set("CurrencyMintMismatch", CurrencyMintMismatchError);

/** RewardMintMismatch: Bond mint address does not match */
export class RewardMintMismatchError extends ProgramError {
  override readonly name: string = "RewardMintMismatch";

  readonly code: number = 0x17; // 23

  constructor(program: Program, cause?: Error) {
    super("Bond mint address does not match", program, cause);
  }
}
codeToErrorMap.set(0x17, RewardMintMismatchError);
nameToErrorMap.set("RewardMintMismatch", RewardMintMismatchError);

/** BondMintMismatch: Bond mint address does not match */
export class BondMintMismatchError extends ProgramError {
  override readonly name: string = "BondMintMismatch";

  readonly code: number = 0x18; // 24

  constructor(program: Program, cause?: Error) {
    super("Bond mint address does not match", program, cause);
  }
}
codeToErrorMap.set(0x18, BondMintMismatchError);
nameToErrorMap.set("BondMintMismatch", BondMintMismatchError);

/** StakeMintMismatch: Stake mint address does not match */
export class StakeMintMismatchError extends ProgramError {
  override readonly name: string = "StakeMintMismatch";

  readonly code: number = 0x19; // 25

  constructor(program: Program, cause?: Error) {
    super("Stake mint address does not match", program, cause);
  }
}
codeToErrorMap.set(0x19, StakeMintMismatchError);
nameToErrorMap.set("StakeMintMismatch", StakeMintMismatchError);

/** StakeVoterMismatch: Stake delegate does not match voter */
export class StakeVoterMismatchError extends ProgramError {
  override readonly name: string = "StakeVoterMismatch";

  readonly code: number = 0x1a; // 26

  constructor(program: Program, cause?: Error) {
    super("Stake delegate does not match voter", program, cause);
  }
}
codeToErrorMap.set(0x1a, StakeVoterMismatchError);
nameToErrorMap.set("StakeVoterMismatch", StakeVoterMismatchError);

/** AsserterMismatch: Asserter address does not match */
export class AsserterMismatchError extends ProgramError {
  override readonly name: string = "AsserterMismatch";

  readonly code: number = 0x1b; // 27

  constructor(program: Program, cause?: Error) {
    super("Asserter address does not match", program, cause);
  }
}
codeToErrorMap.set(0x1b, AsserterMismatchError);
nameToErrorMap.set("AsserterMismatch", AsserterMismatchError);

/** DisputerMismatch: Disputer address does not match */
export class DisputerMismatchError extends ProgramError {
  override readonly name: string = "DisputerMismatch";

  readonly code: number = 0x1c; // 28

  constructor(program: Program, cause?: Error) {
    super("Disputer address does not match", program, cause);
  }
}
codeToErrorMap.set(0x1c, DisputerMismatchError);
nameToErrorMap.set("DisputerMismatch", DisputerMismatchError);

/** IncorrectClaimer: Incorrect claimer */
export class IncorrectClaimerError extends ProgramError {
  override readonly name: string = "IncorrectClaimer";

  readonly code: number = 0x1d; // 29

  constructor(program: Program, cause?: Error) {
    super("Incorrect claimer", program, cause);
  }
}
codeToErrorMap.set(0x1d, IncorrectClaimerError);
nameToErrorMap.set("IncorrectClaimer", IncorrectClaimerError);

/** IncorrectVote: Value voted for is not the resolved value */
export class IncorrectVoteError extends ProgramError {
  override readonly name: string = "IncorrectVote";

  readonly code: number = 0x1e; // 30

  constructor(program: Program, cause?: Error) {
    super("Value voted for is not the resolved value", program, cause);
  }
}
codeToErrorMap.set(0x1e, IncorrectVoteError);
nameToErrorMap.set("IncorrectVote", IncorrectVoteError);

/**
 * Attempts to resolve a custom program error from the provided error code.
 * @category Errors
 */
export function getOptimisticOracleErrorFromCode(
  code: number,
  program: Program,
  cause?: Error,
): ProgramError | null {
  const constructor = codeToErrorMap.get(code);
  return constructor ? new constructor(program, cause) : null;
}

/**
 * Attempts to resolve a custom program error from the provided error name, i.e. 'Unauthorized'.
 * @category Errors
 */
export function getOptimisticOracleErrorFromName(
  name: string,
  program: Program,
  cause?: Error,
): ProgramError | null {
  const constructor = nameToErrorMap.get(name);
  return constructor ? new constructor(program, cause) : null;
}
