/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { UpdateConfigV1Args, UpdateConfigV1ArgsArgs } from "../types";
import type { Context, Pda, PublicKey, Signer, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { transactionBuilder } from "@metaplex-foundation/umi";
import { mapSerializer, struct, u8 } from "@metaplex-foundation/umi/serializers";

import { getAccountMetasAndSigners } from "../shared";
import { getUpdateConfigV1ArgsSerializer } from "../types";

// Accounts.
export type UpdateConfigV1InstructionAccounts = {
  /** Config */
  config: PublicKey | Pda;
  /** Config authority */
  authority?: Signer;
};

// Data.
export type UpdateConfigV1InstructionData = {
  discriminator: number;
  updateConfigV1Args: UpdateConfigV1Args;
};

export type UpdateConfigV1InstructionDataArgs = {
  updateConfigV1Args: UpdateConfigV1ArgsArgs;
};

export function getUpdateConfigV1InstructionDataSerializer(): Serializer<
  UpdateConfigV1InstructionDataArgs,
  UpdateConfigV1InstructionData
> {
  return mapSerializer<UpdateConfigV1InstructionDataArgs, any, UpdateConfigV1InstructionData>(
    struct<UpdateConfigV1InstructionData>(
      [
        ["discriminator", u8()],
        ["updateConfigV1Args", getUpdateConfigV1ArgsSerializer()],
      ],
      { description: "UpdateConfigV1InstructionData" },
    ),
    (value) => ({ ...value, discriminator: 3 }),
  );
}

// Args.
export type UpdateConfigV1InstructionArgs = UpdateConfigV1InstructionDataArgs;

// Instruction.
export function updateConfigV1(
  context: Pick<Context, "identity" | "programs">,
  input: UpdateConfigV1InstructionAccounts & UpdateConfigV1InstructionArgs,
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVM2j1a1AJ9hZuEXyMxA5vusKgMR2FcKJyCf3QE5R2ge",
  );

  // Accounts.
  const resolvedAccounts = {
    config: {
      index: 0,
      isWritable: true as boolean,
      value: input.config ?? null,
    },
    authority: {
      index: 1,
      isWritable: false as boolean,
      value: input.authority ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: UpdateConfigV1InstructionArgs = { ...input };

  // Default values.
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
  const data = getUpdateConfigV1InstructionDataSerializer().serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
