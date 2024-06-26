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
  i64,
  map,
  mapSerializer,
  publicKey as publicKeySerializer,
  string,
  struct,
  u64,
} from "@metaplex-foundation/umi/serializers";

import { AccountType, getAccountTypeSerializer } from "../types";

export type Voting = Account<VotingAccountData>;

export type VotingAccountData = {
  accountType: AccountType;
  request: PublicKey;
  startTimestamp: bigint;
  endTimestamp: bigint;
  voteCount: bigint;
  modeValue: bigint;
  votes: Map<bigint, bigint>;
};

export type VotingAccountDataArgs = {
  request: PublicKey;
  startTimestamp: number | bigint;
  endTimestamp: number | bigint;
  voteCount: number | bigint;
  modeValue: number | bigint;
  votes: Map<number | bigint, number | bigint>;
};

export function getVotingAccountDataSerializer(): Serializer<
  VotingAccountDataArgs,
  VotingAccountData
> {
  return mapSerializer<VotingAccountDataArgs, any, VotingAccountData>(
    struct<VotingAccountData>(
      [
        ["accountType", getAccountTypeSerializer()],
        ["request", publicKeySerializer()],
        ["startTimestamp", i64()],
        ["endTimestamp", i64()],
        ["voteCount", u64()],
        ["modeValue", u64()],
        ["votes", map(u64(), u64())],
      ],
      { description: "VotingAccountData" },
    ),
    (value) => ({ ...value, accountType: AccountType.Voting }),
  );
}

export function deserializeVoting(rawAccount: RpcAccount): Voting {
  return deserializeAccount(rawAccount, getVotingAccountDataSerializer());
}

export async function fetchVoting(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<Voting> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  assertAccountExists(maybeAccount, "Voting");
  return deserializeVoting(maybeAccount);
}

export async function safeFetchVoting(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<Voting | null> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  return maybeAccount.exists ? deserializeVoting(maybeAccount) : null;
}

export async function fetchAllVoting(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<Voting>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, "Voting");
    return deserializeVoting(maybeAccount);
  });
}

export async function safeFetchAllVoting(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<Voting>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeVoting(maybeAccount as RpcAccount));
}

export function getVotingGpaBuilder(context: Pick<Context, "rpc" | "programs">) {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      accountType: AccountTypeArgs;
      request: PublicKey;
      startTimestamp: number | bigint;
      endTimestamp: number | bigint;
      voteCount: number | bigint;
      modeValue: number | bigint;
      votes: Map<number | bigint, number | bigint>;
    }>({
      accountType: [0, getAccountTypeSerializer()],
      request: [1, publicKeySerializer()],
      startTimestamp: [33, i64()],
      endTimestamp: [41, i64()],
      voteCount: [49, u64()],
      modeValue: [57, u64()],
      votes: [65, map(u64(), u64())],
    })
    .deserializeUsing<Voting>((account) => deserializeVoting(account))
    .whereField("accountType", AccountType.Voting);
}

export function findVotingPda(
  context: Pick<Context, "eddsa" | "programs">,
  seeds: {
    /** The address of the request. */
    request: PublicKey;
  },
): Pda {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );
  return context.eddsa.findPda(programId, [
    string({ size: "variable" }).serialize("voting"),
    publicKeySerializer().serialize(seeds.request),
  ]);
}

export async function fetchVotingFromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findVotingPda>[1],
  options?: RpcGetAccountOptions,
): Promise<Voting> {
  return fetchVoting(context, findVotingPda(context, seeds), options);
}

export async function safeFetchVotingFromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findVotingPda>[1],
  options?: RpcGetAccountOptions,
): Promise<Voting | null> {
  return safeFetchVoting(context, findVotingPda(context, seeds), options);
}
