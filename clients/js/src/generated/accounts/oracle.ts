/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import type { AccountTypeArgs, Config, ConfigArgs } from "../types";
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

import { AccountType, getAccountTypeSerializer, getConfigSerializer } from "../types";

export type Oracle = Account<OracleAccountData>;

export type OracleAccountData = {
  accountType: AccountType;
  nextIndex: bigint;
  authority: PublicKey;
  config: Config;
};

export type OracleAccountDataArgs = {
  nextIndex: number | bigint;
  authority: PublicKey;
  config: ConfigArgs;
};

export function getOracleAccountDataSerializer(): Serializer<
  OracleAccountDataArgs,
  OracleAccountData
> {
  return mapSerializer<OracleAccountDataArgs, any, OracleAccountData>(
    struct<OracleAccountData>(
      [
        ["accountType", getAccountTypeSerializer()],
        ["nextIndex", u64()],
        ["authority", publicKeySerializer()],
        ["config", getConfigSerializer()],
      ],
      { description: "OracleAccountData" },
    ),
    (value) => ({ ...value, accountType: AccountType.Oracle }),
  );
}

export function deserializeOracle(rawAccount: RpcAccount): Oracle {
  return deserializeAccount(rawAccount, getOracleAccountDataSerializer());
}

export async function fetchOracle(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<Oracle> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  assertAccountExists(maybeAccount, "Oracle");
  return deserializeOracle(maybeAccount);
}

export async function safeFetchOracle(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<Oracle | null> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  return maybeAccount.exists ? deserializeOracle(maybeAccount) : null;
}

export async function fetchAllOracle(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<Oracle>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, "Oracle");
    return deserializeOracle(maybeAccount);
  });
}

export async function safeFetchAllOracle(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<Oracle>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeOracle(maybeAccount as RpcAccount));
}

export function getOracleGpaBuilder(context: Pick<Context, "rpc" | "programs">) {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      accountType: AccountTypeArgs;
      nextIndex: number | bigint;
      authority: PublicKey;
      config: ConfigArgs;
    }>({
      accountType: [0, getAccountTypeSerializer()],
      nextIndex: [1, u64()],
      authority: [9, publicKeySerializer()],
      config: [41, getConfigSerializer()],
    })
    .deserializeUsing<Oracle>((account) => deserializeOracle(account))
    .whereField("accountType", AccountType.Oracle);
}

export function getOracleSize(): number {
  return 51;
}

export function findOraclePda(context: Pick<Context, "eddsa" | "programs">): Pda {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );
  return context.eddsa.findPda(programId, [string({ size: "variable" }).serialize("oracle")]);
}

export async function fetchOracleFromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  options?: RpcGetAccountOptions,
): Promise<Oracle> {
  return fetchOracle(context, findOraclePda(context), options);
}

export async function safeFetchOracleFromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  options?: RpcGetAccountOptions,
): Promise<Oracle | null> {
  return safeFetchOracle(context, findOraclePda(context), options);
}
