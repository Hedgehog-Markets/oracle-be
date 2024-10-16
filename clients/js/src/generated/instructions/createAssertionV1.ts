/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { Context, Pda, PublicKey, Signer, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { findAssociatedTokenPda } from "@metaplex-foundation/mpl-toolbox";
import { transactionBuilder } from "@metaplex-foundation/umi";
import { mapSerializer, struct, u64, u8 } from "@metaplex-foundation/umi/serializers";

import { findAssertBondPda } from "../../hooked";
import { findAssertionV1Pda } from "../accounts";
import { expectPublicKey, getAccountMetasAndSigners } from "../shared";

// Accounts.
export type CreateAssertionV1InstructionAccounts = {
  /** Config */
  config: PublicKey | Pda;
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
export type CreateAssertionV1InstructionData = {
  discriminator: number;
  value: bigint;
};

export type CreateAssertionV1InstructionDataArgs = { value: number | bigint };

export function getCreateAssertionV1InstructionDataSerializer(): Serializer<
  CreateAssertionV1InstructionDataArgs,
  CreateAssertionV1InstructionData
> {
  return mapSerializer<CreateAssertionV1InstructionDataArgs, any, CreateAssertionV1InstructionData>(
    struct<CreateAssertionV1InstructionData>(
      [
        ["discriminator", u8()],
        ["value", u64()],
      ],
      { description: "CreateAssertionV1InstructionData" },
    ),
    (value) => ({ ...value, discriminator: 7 }),
  );
}

// Args.
export type CreateAssertionV1InstructionArgs = CreateAssertionV1InstructionDataArgs;

// Instruction.
export function createAssertionV1(
  context: Pick<Context, "eddsa" | "identity" | "payer" | "programs">,
  input: CreateAssertionV1InstructionAccounts & CreateAssertionV1InstructionArgs,
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVM3hK9SDgXLmVoLng1KrTJCzTnhw31hAnqTYP7uGCot",
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
    asserter: {
      index: 6,
      isWritable: false as boolean,
      value: input.asserter ?? null,
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
  const resolvedArgs: CreateAssertionV1InstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.assertion.value) {
    resolvedAccounts.assertion.value = findAssertionV1Pda(context, {
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
  const data = getCreateAssertionV1InstructionDataSerializer().serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
