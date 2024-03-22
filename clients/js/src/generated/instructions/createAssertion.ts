/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { CreateAssertionArgs, CreateAssertionArgsArgs } from "../types";
import type { Context, Pda, PublicKey, Signer, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { findAssociatedTokenPda } from "@metaplex-foundation/mpl-toolbox";
import { transactionBuilder } from "@metaplex-foundation/umi";
import { mapSerializer, struct, u8 } from "@metaplex-foundation/umi/serializers";

import { findAssertBondPda } from "../../hooked";
import { findAssertionPda, findOraclePda } from "../accounts";
import { expectPublicKey, getAccountMetasAndSigners } from "../shared";
import { getCreateAssertionArgsSerializer } from "../types";

// Accounts.
export type CreateAssertionInstructionAccounts = {
  /** Program oracle account */
  oracle?: PublicKey | Pda;
  /** Request */
  request: PublicKey | Pda;
  /** Assertion */
  assertion?: PublicKey | Pda;
  /** Bond mint */
  bondMint: PublicKey | Pda;
  /** Bond source token account */
  bondSource?: PublicKey | Pda;
  /** Bond escrow token account */
  bondEscrow?: PublicKey | Pda;
  /** Governance mint */
  governanceMint: PublicKey | Pda;
  /** Governance source token account */
  governanceSource: PublicKey | Pda;
  /** Governance escrow token account */
  governanceEscrow: PublicKey | Pda;
  /** Asserter */
  asserter?: Signer;
  /** Payer */
  payer?: Signer;
  /** SPL token program */
  tokenProgram?: PublicKey | Pda;
  /** System program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type CreateAssertionInstructionData = {
  discriminator: number;
  createAssertionArgs: CreateAssertionArgs;
};

export type CreateAssertionInstructionDataArgs = {
  createAssertionArgs: CreateAssertionArgsArgs;
};

export function getCreateAssertionInstructionDataSerializer(): Serializer<
  CreateAssertionInstructionDataArgs,
  CreateAssertionInstructionData
> {
  return mapSerializer<CreateAssertionInstructionDataArgs, any, CreateAssertionInstructionData>(
    struct<CreateAssertionInstructionData>(
      [
        ["discriminator", u8()],
        ["createAssertionArgs", getCreateAssertionArgsSerializer()],
      ],
      { description: "CreateAssertionInstructionData" },
    ),
    (value) => ({ ...value, discriminator: 2 }),
  );
}

// Args.
export type CreateAssertionInstructionArgs = CreateAssertionInstructionDataArgs;

// Instruction.
export function createAssertion(
  context: Pick<Context, "eddsa" | "identity" | "payer" | "programs">,
  input: CreateAssertionInstructionAccounts & CreateAssertionInstructionArgs,
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
      isWritable: false as boolean,
      value: input.oracle ?? null,
    },
    request: {
      index: 1,
      isWritable: true as boolean,
      value: input.request ?? null,
    },
    assertion: {
      index: 2,
      isWritable: true as boolean,
      value: input.assertion ?? null,
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
    governanceMint: {
      index: 6,
      isWritable: false as boolean,
      value: input.governanceMint ?? null,
    },
    governanceSource: {
      index: 7,
      isWritable: true as boolean,
      value: input.governanceSource ?? null,
    },
    governanceEscrow: {
      index: 8,
      isWritable: true as boolean,
      value: input.governanceEscrow ?? null,
    },
    asserter: {
      index: 9,
      isWritable: false as boolean,
      value: input.asserter ?? null,
    },
    payer: {
      index: 10,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    tokenProgram: {
      index: 11,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    systemProgram: {
      index: 12,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: CreateAssertionInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.oracle.value) {
    resolvedAccounts.oracle.value = findOraclePda(context);
  }
  if (!resolvedAccounts.assertion.value) {
    resolvedAccounts.assertion.value = findAssertionPda(context, {
      request: expectPublicKey(resolvedAccounts.request.value),
    });
  }
  if (!resolvedAccounts.asserter.value) {
    resolvedAccounts.asserter.value = context.identity;
  }
  if (!resolvedAccounts.bondSource.value) {
    resolvedAccounts.bondSource.value = findAssociatedTokenPda(context, {
      mint: expectPublicKey(resolvedAccounts.bondMint.value),
      owner: expectPublicKey(resolvedAccounts.asserter.value),
    });
  }
  if (!resolvedAccounts.bondEscrow.value) {
    resolvedAccounts.bondEscrow.value = findAssertBondPda(context, {
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
  const data = getCreateAssertionInstructionDataSerializer().serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
