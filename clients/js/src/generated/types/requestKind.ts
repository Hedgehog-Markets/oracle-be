/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { scalarEnum } from "@metaplex-foundation/umi/serializers";

export enum RequestKind {
  YesNo,
}

export type RequestKindArgs = RequestKind;

export function getRequestKindSerializer(): Serializer<RequestKindArgs, RequestKind> {
  return scalarEnum<RequestKind>(RequestKind, {
    description: "RequestKind",
  }) as Serializer<RequestKindArgs, RequestKind>;
}
