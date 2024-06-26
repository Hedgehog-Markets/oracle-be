/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { UpdateOracleV1Args, UpdateOracleV1ArgsArgs } from "../types";
import type { Context, Pda, PublicKey, Signer, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { transactionBuilder } from "@metaplex-foundation/umi";
import { mapSerializer, struct, u8 } from "@metaplex-foundation/umi/serializers";

import { findOracleV1Pda } from "../accounts";
import { getAccountMetasAndSigners } from "../shared";
import { getUpdateOracleV1ArgsSerializer } from "../types";

// Accounts.
export interface UpdateOracleV1InstructionAccounts {
  /** Oracle account */
  oracle?: PublicKey | Pda;
  /** Oracle authority */
  authority?: Signer;
}

// Data.
export interface UpdateOracleV1InstructionData {
  discriminator: number;
  updateOracleV1Args: UpdateOracleV1Args;
}

export interface UpdateOracleV1InstructionDataArgs {
  updateOracleV1Args: UpdateOracleV1ArgsArgs;
}

export function getUpdateOracleV1InstructionDataSerializer(): Serializer<
  UpdateOracleV1InstructionDataArgs,
  UpdateOracleV1InstructionData
> {
  return mapSerializer<UpdateOracleV1InstructionDataArgs, any, UpdateOracleV1InstructionData>(
    struct<UpdateOracleV1InstructionData>(
      [
        ["discriminator", u8()],
        ["updateOracleV1Args", getUpdateOracleV1ArgsSerializer()],
      ],
      { description: "UpdateOracleV1InstructionData" },
    ),
    (value) => ({ ...value, discriminator: 1 }),
  );
}

// Args.
export type UpdateOracleV1InstructionArgs = UpdateOracleV1InstructionDataArgs;

// Instruction.
export function updateOracleV1(
  context: Pick<Context, "eddsa" | "identity" | "programs">,
  input: UpdateOracleV1InstructionAccounts & UpdateOracleV1InstructionArgs,
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );

  // Accounts.
  const resolvedAccounts = {
    oracle: {
      index: 0,
      isWritable: true as boolean,
      value: input.oracle ?? null,
    },
    authority: {
      index: 1,
      isWritable: false as boolean,
      value: input.authority ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: UpdateOracleV1InstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.oracle.value) {
    resolvedAccounts.oracle.value = findOracleV1Pda(context);
  }
  if (!resolvedAccounts.authority.value) {
    resolvedAccounts.authority.value = context.identity;
  }

  // Accounts in order.
  const orderedAccounts: Array<ResolvedAccount> = Object.values(resolvedAccounts).sort(
    (a, b) => a.index - b.index,
  );

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(orderedAccounts, "programId", programId);

  // Data.
  const data = getUpdateOracleV1InstructionDataSerializer().serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
