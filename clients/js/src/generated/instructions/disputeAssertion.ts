/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { DisputeAssertionArgs, DisputeAssertionArgsArgs } from "../types";
import type { Context, Pda, PublicKey, Signer, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { transactionBuilder } from "@metaplex-foundation/umi";
import { mapSerializer, struct, u8 } from "@metaplex-foundation/umi/serializers";

import { findAssertionPda, findVotingPda } from "../accounts";
import { expectPublicKey, getAccountMetasAndSigners } from "../shared";
import { getDisputeAssertionArgsSerializer } from "../types";

// Accounts.
export type DisputeAssertionInstructionAccounts = {
  /** Request */
  request: PublicKey | Pda;
  /** Assertion */
  assertion?: PublicKey | Pda;
  /** Voting */
  voting?: PublicKey | Pda;
  /** Bond mint */
  bondMint: PublicKey | Pda;
  /** Bond source token account */
  bondSource: PublicKey | Pda;
  /** Bond escrow token account */
  bondEscrow: PublicKey | Pda;
  /** Disputer */
  disputer: Signer;
  /** Payer */
  payer?: Signer;
  /** SPL token program */
  tokenProgram?: PublicKey | Pda;
  /** System program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type DisputeAssertionInstructionData = {
  discriminator: number;
  disputeAssertionArgs: DisputeAssertionArgs;
};

export type DisputeAssertionInstructionDataArgs = {
  disputeAssertionArgs: DisputeAssertionArgsArgs;
};

export function getDisputeAssertionInstructionDataSerializer(): Serializer<
  DisputeAssertionInstructionDataArgs,
  DisputeAssertionInstructionData
> {
  return mapSerializer<DisputeAssertionInstructionDataArgs, any, DisputeAssertionInstructionData>(
    struct<DisputeAssertionInstructionData>(
      [
        ["discriminator", u8()],
        ["disputeAssertionArgs", getDisputeAssertionArgsSerializer()],
      ],
      { description: "DisputeAssertionInstructionData" },
    ),
    (value) => ({ ...value, discriminator: 4 }),
  );
}

// Args.
export type DisputeAssertionInstructionArgs = DisputeAssertionInstructionDataArgs;

// Instruction.
export function disputeAssertion(
  context: Pick<Context, "eddsa" | "payer" | "programs">,
  input: DisputeAssertionInstructionAccounts & DisputeAssertionInstructionArgs,
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );

  // Accounts.
  const resolvedAccounts = {
    request: {
      index: 0,
      isWritable: true as boolean,
      value: input.request ?? null,
    },
    assertion: {
      index: 1,
      isWritable: true as boolean,
      value: input.assertion ?? null,
    },
    voting: {
      index: 2,
      isWritable: true as boolean,
      value: input.voting ?? null,
    },
    bondMint: {
      index: 3,
      isWritable: false as boolean,
      value: input.bondMint ?? null,
    },
    bondSource: {
      index: 4,
      isWritable: true as boolean,
      value: input.bondSource ?? null,
    },
    bondEscrow: {
      index: 5,
      isWritable: true as boolean,
      value: input.bondEscrow ?? null,
    },
    disputer: {
      index: 6,
      isWritable: false as boolean,
      value: input.disputer ?? null,
    },
    payer: {
      index: 7,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    tokenProgram: {
      index: 8,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    systemProgram: {
      index: 9,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: DisputeAssertionInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.assertion.value) {
    resolvedAccounts.assertion.value = findAssertionPda(context, {
      request: expectPublicKey(resolvedAccounts.request.value),
    });
  }
  if (!resolvedAccounts.voting.value) {
    resolvedAccounts.voting.value = findVotingPda(context, {
      request: expectPublicKey(resolvedAccounts.request.value),
    });
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
  const data = getDisputeAssertionInstructionDataSerializer().serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
