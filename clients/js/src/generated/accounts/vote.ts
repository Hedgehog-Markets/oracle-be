/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import type { AccountTypeArgs } from "../types";
import type {
  Account,
  Context,
  Pda,
  PublicKey,
  RpcAccount,
  RpcGetAccountOptions,
  RpcGetAccountsOptions,
} from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import {
  assertAccountExists,
  deserializeAccount,
  gpaBuilder,
  publicKey as toPublicKey,
} from "@metaplex-foundation/umi";
import {
  mapSerializer,
  publicKey as publicKeySerializer,
  string,
  struct,
  u64,
} from "@metaplex-foundation/umi/serializers";

import { AccountType, getAccountTypeSerializer } from "../types";

export type Vote = Account<VoteAccountData>;

export type VoteAccountData = {
  accountType: AccountType;
  stake: PublicKey;
  value: bigint;
  votes: bigint;
};

export type VoteAccountDataArgs = {
  stake: PublicKey;
  value: number | bigint;
  votes: number | bigint;
};

export function getVoteAccountDataSerializer(): Serializer<VoteAccountDataArgs, VoteAccountData> {
  return mapSerializer<VoteAccountDataArgs, any, VoteAccountData>(
    struct<VoteAccountData>(
      [
        ["accountType", getAccountTypeSerializer()],
        ["stake", publicKeySerializer()],
        ["value", u64()],
        ["votes", u64()],
      ],
      { description: "VoteAccountData" },
    ),
    (value) => ({ ...value, accountType: AccountType.Vote }),
  );
}

export function deserializeVote(rawAccount: RpcAccount): Vote {
  return deserializeAccount(rawAccount, getVoteAccountDataSerializer());
}

export async function fetchVote(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<Vote> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  assertAccountExists(maybeAccount, "Vote");
  return deserializeVote(maybeAccount);
}

export async function safeFetchVote(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<Vote | null> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  return maybeAccount.exists ? deserializeVote(maybeAccount) : null;
}

export async function fetchAllVote(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<Vote>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, "Vote");
    return deserializeVote(maybeAccount);
  });
}

export async function safeFetchAllVote(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<Vote>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeVote(maybeAccount as RpcAccount));
}

export function getVoteGpaBuilder(context: Pick<Context, "rpc" | "programs">) {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      accountType: AccountTypeArgs;
      stake: PublicKey;
      value: number | bigint;
      votes: number | bigint;
    }>({
      accountType: [0, getAccountTypeSerializer()],
      stake: [1, publicKeySerializer()],
      value: [33, u64()],
      votes: [41, u64()],
    })
    .deserializeUsing<Vote>((account) => deserializeVote(account))
    .whereField("accountType", AccountType.Vote);
}

export function getVoteSize(): number {
  return 49;
}

export function findVotePda(
  context: Pick<Context, "eddsa" | "programs">,
  seeds: {
    /** The address of the voting account. */
    voting: PublicKey;
    /** The address of the stake account. */
    stake: PublicKey;
  },
): Pda {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );
  return context.eddsa.findPda(programId, [
    string({ size: "variable" }).serialize("vote"),
    publicKeySerializer().serialize(seeds.voting),
    publicKeySerializer().serialize(seeds.stake),
  ]);
}

export async function fetchVoteFromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findVotePda>[1],
  options?: RpcGetAccountOptions,
): Promise<Vote> {
  return fetchVote(context, findVotePda(context, seeds), options);
}

export async function safeFetchVoteFromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findVotePda>[1],
  options?: RpcGetAccountOptions,
): Promise<Vote | null> {
  return safeFetchVote(context, findVotePda(context, seeds), options);
}