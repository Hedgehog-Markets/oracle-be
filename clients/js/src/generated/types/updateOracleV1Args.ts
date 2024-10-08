/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import type { PublicKey } from "@metaplex-foundation/umi";
import type {
  GetDataEnumKind,
  GetDataEnumKindContent,
  Serializer,
} from "@metaplex-foundation/umi/serializers";

import {
  dataEnum,
  publicKey as publicKeySerializer,
  struct,
} from "@metaplex-foundation/umi/serializers";

export type UpdateOracleV1Args = {
  __kind: "Authority";
  newAuthority: PublicKey;
};

export type UpdateOracleV1ArgsArgs = UpdateOracleV1Args;

export function getUpdateOracleV1ArgsSerializer(): Serializer<
  UpdateOracleV1ArgsArgs,
  UpdateOracleV1Args
> {
  return dataEnum<UpdateOracleV1Args>(
    [
      [
        "Authority",
        struct<GetDataEnumKindContent<UpdateOracleV1Args, "Authority">>([
          ["newAuthority", publicKeySerializer()],
        ]),
      ],
    ],
    { description: "UpdateOracleV1Args" },
  );
}

// Data Enum Helpers.
export function updateOracleV1Args(
  kind: "Authority",
  data: GetDataEnumKindContent<UpdateOracleV1ArgsArgs, "Authority">,
): GetDataEnumKind<UpdateOracleV1ArgsArgs, "Authority">;
export function updateOracleV1Args<K extends UpdateOracleV1ArgsArgs["__kind"]>(
  kind: K,
  data?: any,
): Extract<UpdateOracleV1ArgsArgs, { __kind: K }> {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...(data ?? {}) };
}
export function isUpdateOracleV1Args<K extends UpdateOracleV1Args["__kind"]>(
  kind: K,
  value: UpdateOracleV1Args,
): value is UpdateOracleV1Args & { __kind: K } {
  return value.__kind === kind;
}
