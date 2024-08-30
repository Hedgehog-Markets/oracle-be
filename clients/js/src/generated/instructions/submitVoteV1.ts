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

import { findVoteV1Pda, findVotingV1Pda } from "../accounts";
import { expectPublicKey, getAccountMetasAndSigners } from "../shared";

// Accounts.
export type SubmitVoteV1InstructionAccounts = {
  /** Config */
  config: PublicKey | Pda;
  /** Request */
  request: PublicKey | Pda;
  /** Voting */
  voting?: PublicKey | Pda;
  /** Vote */
  vote?: PublicKey | Pda;
  /** Stake */
  stake: PublicKey | Pda;
  /** Voter */
  voter?: Signer;
  /** Payer */
  payer?: Signer;
  /** System program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type SubmitVoteV1InstructionData = {
  discriminator: number;
  value: bigint;
};

export type SubmitVoteV1InstructionDataArgs = { value: number | bigint };

export function getSubmitVoteV1InstructionDataSerializer(): Serializer<
  SubmitVoteV1InstructionDataArgs,
  SubmitVoteV1InstructionData
> {
  return mapSerializer<SubmitVoteV1InstructionDataArgs, any, SubmitVoteV1InstructionData>(
    struct<SubmitVoteV1InstructionData>(
      [
        ["discriminator", u8()],
        ["value", u64()],
      ],
      { description: "SubmitVoteV1InstructionData" },
    ),
    (value) => ({ ...value, discriminator: 10 }),
  );
}

// Args.
export type SubmitVoteV1InstructionArgs = SubmitVoteV1InstructionDataArgs;

// Instruction.
export function submitVoteV1(
  context: Pick<Context, "eddsa" | "identity" | "payer" | "programs">,
  input: SubmitVoteV1InstructionAccounts & SubmitVoteV1InstructionArgs,
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
      isWritable: false as boolean,
      value: input.config ?? null,
    },
    request: {
      index: 1,
      isWritable: false as boolean,
      value: input.request ?? null,
    },
    voting: {
      index: 2,
      isWritable: true as boolean,
      value: input.voting ?? null,
    },
    vote: { index: 3, isWritable: true as boolean, value: input.vote ?? null },
    stake: {
      index: 4,
      isWritable: false as boolean,
      value: input.stake ?? null,
    },
    voter: {
      index: 5,
      isWritable: false as boolean,
      value: input.voter ?? null,
    },
    payer: {
      index: 6,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    systemProgram: {
      index: 7,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: SubmitVoteV1InstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.voting.value) {
    resolvedAccounts.voting.value = findVotingV1Pda(context, {
      request: expectPublicKey(resolvedAccounts.request.value),
    });
  }
  if (!resolvedAccounts.vote.value) {
    resolvedAccounts.vote.value = findVoteV1Pda(context, {
      voting: expectPublicKey(resolvedAccounts.voting.value),
      stake: expectPublicKey(resolvedAccounts.stake.value),
    });
  }
  if (!resolvedAccounts.voter.value) {
    resolvedAccounts.voter.value = context.identity;
  }
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
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
  const data = getSubmitVoteV1InstructionDataSerializer().serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
