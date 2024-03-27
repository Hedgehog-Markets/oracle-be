/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import type {
  GetDataEnumKind,
  GetDataEnumKindContent,
  Serializer,
} from "@metaplex-foundation/umi/serializers";

import { dataEnum, struct, u64 } from "@metaplex-foundation/umi/serializers";

export type CreateAssertionArgs = { __kind: "V1"; bond: bigint; value: bigint };

export type CreateAssertionArgsArgs = {
  __kind: "V1";
  bond: number | bigint;
  value: number | bigint;
};

export function getCreateAssertionArgsSerializer(): Serializer<
  CreateAssertionArgsArgs,
  CreateAssertionArgs
> {
  return dataEnum<CreateAssertionArgs>(
    [
      [
        "V1",
        struct<GetDataEnumKindContent<CreateAssertionArgs, "V1">>([
          ["bond", u64()],
          ["value", u64()],
        ]),
      ],
    ],
    { description: "CreateAssertionArgs" },
  ) as Serializer<CreateAssertionArgsArgs, CreateAssertionArgs>;
}

// Data Enum Helpers.
export function createAssertionArgs(
  kind: "V1",
  data: GetDataEnumKindContent<CreateAssertionArgsArgs, "V1">,
): GetDataEnumKind<CreateAssertionArgsArgs, "V1">;
export function createAssertionArgs<K extends CreateAssertionArgsArgs["__kind"]>(
  kind: K,
  data?: any,
): Extract<CreateAssertionArgsArgs, { __kind: K }> {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...(data ?? {}) };
}
export function isCreateAssertionArgs<K extends CreateAssertionArgs["__kind"]>(
  kind: K,
  value: CreateAssertionArgs,
): value is CreateAssertionArgs & { __kind: K } {
  return value.__kind === kind;
}