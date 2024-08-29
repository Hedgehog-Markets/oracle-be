/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import type { AccountTypeArgs, Bounds, BoundsArgs } from "../types";
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
} from "@metaplex-foundation/umi/serializers";

import { AccountType, getAccountTypeSerializer, getBoundsSerializer } from "../types";

export type CurrencyV1 = Account<CurrencyV1AccountData>;

export type CurrencyV1AccountData = {
  accountType: AccountType;
  config: PublicKey;
  mint: PublicKey;
  rewardRange: Bounds;
  bondRange: Bounds;
};

export type CurrencyV1AccountDataArgs = {
  config: PublicKey;
  mint: PublicKey;
  rewardRange: BoundsArgs;
  bondRange: BoundsArgs;
};

export function getCurrencyV1AccountDataSerializer(): Serializer<
  CurrencyV1AccountDataArgs,
  CurrencyV1AccountData
> {
  return mapSerializer<CurrencyV1AccountDataArgs, any, CurrencyV1AccountData>(
    struct<CurrencyV1AccountData>(
      [
        ["accountType", getAccountTypeSerializer()],
        ["config", publicKeySerializer()],
        ["mint", publicKeySerializer()],
        ["rewardRange", getBoundsSerializer()],
        ["bondRange", getBoundsSerializer()],
      ],
      { description: "CurrencyV1AccountData" },
    ),
    (value) => ({ ...value, accountType: AccountType.CurrencyV1 }),
  );
}

export function deserializeCurrencyV1(rawAccount: RpcAccount): CurrencyV1 {
  return deserializeAccount(rawAccount, getCurrencyV1AccountDataSerializer());
}

export async function fetchCurrencyV1(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<CurrencyV1> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  assertAccountExists(maybeAccount, "CurrencyV1");
  return deserializeCurrencyV1(maybeAccount);
}

export async function safeFetchCurrencyV1(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<CurrencyV1 | null> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  return maybeAccount.exists ? deserializeCurrencyV1(maybeAccount) : null;
}

export async function fetchAllCurrencyV1(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<CurrencyV1>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, "CurrencyV1");
    return deserializeCurrencyV1(maybeAccount);
  });
}

export async function safeFetchAllCurrencyV1(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<CurrencyV1>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeCurrencyV1(maybeAccount as RpcAccount));
}

export function getCurrencyV1GpaBuilder(context: Pick<Context, "rpc" | "programs">) {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVM2j1a1AJ9hZuEXyMxA5vusKgMR2FcKJyCf3QE5R2ge",
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      accountType: AccountTypeArgs;
      config: PublicKey;
      mint: PublicKey;
      rewardRange: BoundsArgs;
      bondRange: BoundsArgs;
    }>({
      accountType: [0, getAccountTypeSerializer()],
      config: [1, publicKeySerializer()],
      mint: [33, publicKeySerializer()],
      rewardRange: [65, getBoundsSerializer()],
      bondRange: [81, getBoundsSerializer()],
    })
    .deserializeUsing<CurrencyV1>((account) => deserializeCurrencyV1(account))
    .whereField("accountType", AccountType.CurrencyV1);
}

export function getCurrencyV1Size(): number {
  return 97;
}

export function findCurrencyV1Pda(
  context: Pick<Context, "eddsa" | "programs">,
  seeds: {
    /** The address of the currency mint. */
    mint: PublicKey;
  },
): Pda {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVM2j1a1AJ9hZuEXyMxA5vusKgMR2FcKJyCf3QE5R2ge",
  );
  return context.eddsa.findPda(programId, [
    string({ size: "variable" }).serialize("currency"),
    publicKeySerializer().serialize(seeds.mint),
  ]);
}

export async function fetchCurrencyV1FromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findCurrencyV1Pda>[1],
  options?: RpcGetAccountOptions,
): Promise<CurrencyV1> {
  return fetchCurrencyV1(context, findCurrencyV1Pda(context, seeds), options);
}

export async function safeFetchCurrencyV1FromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findCurrencyV1Pda>[1],
  options?: RpcGetAccountOptions,
): Promise<CurrencyV1 | null> {
  return safeFetchCurrencyV1(context, findCurrencyV1Pda(context, seeds), options);
}
