/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { Context, Pda, PublicKey, Signer, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { transactionBuilder } from "@metaplex-foundation/umi";
import { mapSerializer, struct, u64, u8 } from "@metaplex-foundation/umi/serializers";

import { findOracleV1Pda } from "../accounts";
import { getAccountMetasAndSigners } from "../shared";

// Accounts.
export type CreateStakeV1InstructionAccounts = {
  /** Oracle */
  oracle?: PublicKey | Pda;
  /** Stake */
  stake: Signer;
  /** Stake */
  mint: PublicKey | Pda;
  /** Stake source token account */
  stakeSource: PublicKey | Pda;
  /** Stake pool token account */
  stakePool: PublicKey | Pda;
  /** Stake owner */
  wallet: Signer;
  /** Payer */
  payer?: Signer;
  /** SPL token program */
  tokenProgram?: PublicKey | Pda;
  /** System program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type CreateStakeV1InstructionData = {
  discriminator: number;
  amount: bigint;
};

export type CreateStakeV1InstructionDataArgs = { amount: number | bigint };

export function getCreateStakeV1InstructionDataSerializer(): Serializer<
  CreateStakeV1InstructionDataArgs,
  CreateStakeV1InstructionData
> {
  return mapSerializer<CreateStakeV1InstructionDataArgs, any, CreateStakeV1InstructionData>(
    struct<CreateStakeV1InstructionData>(
      [
        ["discriminator", u8()],
        ["amount", u64()],
      ],
      { description: "CreateStakeV1InstructionData" },
    ),
    (value) => ({ ...value, discriminator: 12 }),
  );
}

// Args.
export type CreateStakeV1InstructionArgs = CreateStakeV1InstructionDataArgs;

// Instruction.
export function createStakeV1(
  context: Pick<Context, "eddsa" | "payer" | "programs">,
  input: CreateStakeV1InstructionAccounts & CreateStakeV1InstructionArgs,
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVM2j1a1AJ9hZuEXyMxA5vusKgMR2FcKJyCf3QE5R2ge",
  );

  // Accounts.
  const resolvedAccounts = {
    oracle: {
      index: 0,
      isWritable: false as boolean,
      value: input.oracle ?? null,
    },
    stake: {
      index: 1,
      isWritable: true as boolean,
      value: input.stake ?? null,
    },
    mint: { index: 2, isWritable: true as boolean, value: input.mint ?? null },
    stakeSource: {
      index: 3,
      isWritable: true as boolean,
      value: input.stakeSource ?? null,
    },
    stakePool: {
      index: 4,
      isWritable: true as boolean,
      value: input.stakePool ?? null,
    },
    wallet: {
      index: 5,
      isWritable: true as boolean,
      value: input.wallet ?? null,
    },
    payer: {
      index: 6,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    tokenProgram: {
      index: 7,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    systemProgram: {
      index: 8,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: CreateStakeV1InstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.oracle.value) {
    resolvedAccounts.oracle.value = findOracleV1Pda(context);
  }
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.tokenProgram.value) {
    resolvedAccounts.tokenProgram.value = context.programs.getPublicKey(
      "splToken",
      "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    );
    resolvedAccounts.tokenProgram.isWritable = false;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      "splSystem",
      "11111111111111111111111111111111",
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: Array<ResolvedAccount> = Object.values(resolvedAccounts).sort(
    (a, b) => a.index - b.index,
  );

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(orderedAccounts, "programId", programId);

  // Data.
  const data = getCreateStakeV1InstructionDataSerializer().serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
