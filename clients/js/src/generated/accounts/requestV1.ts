/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import type {
  AccountTypeArgs,
  RequestKind,
  RequestKindArgs,
  RequestState,
  RequestStateArgs,
} from "../types";
import type {
  Account,
  Context,
  DateTime,
  DateTimeInput,
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
  mapDateTimeSerializer,
  publicKey as toPublicKey,
} from "@metaplex-foundation/umi";
import {
  i64,
  mapSerializer,
  publicKey as publicKeySerializer,
  string,
  struct,
  u64,
  u8,
} from "@metaplex-foundation/umi/serializers";

import {
  AccountType,
  getAccountTypeSerializer,
  getRequestKindSerializer,
  getRequestStateSerializer,
} from "../types";

export type RequestV1 = Account<RequestV1AccountData>;

export type RequestV1AccountData = {
  accountType: AccountType;
  index: bigint;
  config: PublicKey;
  creator: PublicKey;
  reward: bigint;
  rewardMint: PublicKey;
  bond: bigint;
  bondMint: PublicKey;
  assertionTimestamp: DateTime;
  resolveTimestamp: DateTime;
  state: RequestState;
  round: number;
  value: bigint;
  arbitrator: PublicKey;
  kind: RequestKind;
  uri: string;
};

export type RequestV1AccountDataArgs = {
  index: number | bigint;
  config: PublicKey;
  creator: PublicKey;
  reward: number | bigint;
  rewardMint: PublicKey;
  bond: number | bigint;
  bondMint: PublicKey;
  assertionTimestamp: DateTimeInput;
  resolveTimestamp: DateTimeInput;
  state: RequestStateArgs;
  round: number;
  value: number | bigint;
  arbitrator: PublicKey;
  kind: RequestKindArgs;
  uri: string;
};

export function getRequestV1AccountDataSerializer(): Serializer<
  RequestV1AccountDataArgs,
  RequestV1AccountData
> {
  return mapSerializer<RequestV1AccountDataArgs, any, RequestV1AccountData>(
    struct<RequestV1AccountData>(
      [
        ["accountType", getAccountTypeSerializer()],
        ["index", u64()],
        ["config", publicKeySerializer()],
        ["creator", publicKeySerializer()],
        ["reward", u64()],
        ["rewardMint", publicKeySerializer()],
        ["bond", u64()],
        ["bondMint", publicKeySerializer()],
        ["assertionTimestamp", mapDateTimeSerializer(i64())],
        ["resolveTimestamp", mapDateTimeSerializer(i64())],
        ["state", getRequestStateSerializer()],
        ["round", u8()],
        ["value", u64()],
        ["arbitrator", publicKeySerializer()],
        ["kind", getRequestKindSerializer()],
        ["uri", string()],
      ],
      { description: "RequestV1AccountData" },
    ),
    (value) => ({ ...value, accountType: AccountType.RequestV1 }),
  );
}

export function deserializeRequestV1(rawAccount: RpcAccount): RequestV1 {
  return deserializeAccount(rawAccount, getRequestV1AccountDataSerializer());
}

export async function fetchRequestV1(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<RequestV1> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  assertAccountExists(maybeAccount, "RequestV1");
  return deserializeRequestV1(maybeAccount);
}

export async function safeFetchRequestV1(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<RequestV1 | null> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  return maybeAccount.exists ? deserializeRequestV1(maybeAccount) : null;
}

export async function fetchAllRequestV1(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<RequestV1>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, "RequestV1");
    return deserializeRequestV1(maybeAccount);
  });
}

export async function safeFetchAllRequestV1(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<RequestV1>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeRequestV1(maybeAccount as RpcAccount));
}

export function getRequestV1GpaBuilder(context: Pick<Context, "rpc" | "programs">) {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVM2j1a1AJ9hZuEXyMxA5vusKgMR2FcKJyCf3QE5R2ge",
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      accountType: AccountTypeArgs;
      index: number | bigint;
      config: PublicKey;
      creator: PublicKey;
      reward: number | bigint;
      rewardMint: PublicKey;
      bond: number | bigint;
      bondMint: PublicKey;
      assertionTimestamp: DateTimeInput;
      resolveTimestamp: DateTimeInput;
      state: RequestStateArgs;
      round: number;
      value: number | bigint;
      arbitrator: PublicKey;
      kind: RequestKindArgs;
      uri: string;
    }>({
      accountType: [0, getAccountTypeSerializer()],
      index: [1, u64()],
      config: [9, publicKeySerializer()],
      creator: [41, publicKeySerializer()],
      reward: [73, u64()],
      rewardMint: [81, publicKeySerializer()],
      bond: [113, u64()],
      bondMint: [121, publicKeySerializer()],
      assertionTimestamp: [153, mapDateTimeSerializer(i64())],
      resolveTimestamp: [161, mapDateTimeSerializer(i64())],
      state: [169, getRequestStateSerializer()],
      round: [170, u8()],
      value: [171, u64()],
      arbitrator: [179, publicKeySerializer()],
      kind: [211, getRequestKindSerializer()],
      uri: [212, string()],
    })
    .deserializeUsing<RequestV1>((account) => deserializeRequestV1(account))
    .whereField("accountType", AccountType.RequestV1);
}

export function findRequestV1Pda(
  context: Pick<Context, "eddsa" | "programs">,
  seeds: {
    /** The next request index in the oracle. */
    index: number | bigint;
  },
): Pda {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVM2j1a1AJ9hZuEXyMxA5vusKgMR2FcKJyCf3QE5R2ge",
  );
  return context.eddsa.findPda(programId, [
    string({ size: "variable" }).serialize("request"),
    u64().serialize(seeds.index),
  ]);
}

export async function fetchRequestV1FromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findRequestV1Pda>[1],
  options?: RpcGetAccountOptions,
): Promise<RequestV1> {
  return fetchRequestV1(context, findRequestV1Pda(context, seeds), options);
}

export async function safeFetchRequestV1FromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findRequestV1Pda>[1],
  options?: RpcGetAccountOptions,
): Promise<RequestV1 | null> {
  return safeFetchRequestV1(context, findRequestV1Pda(context, seeds), options);
}
