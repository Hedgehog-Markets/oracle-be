// @ts-check

import { execFile } from "child_process";
import path from "path";
import { fileURLToPath } from "url";

import * as k from "@metaplex-foundation/kinobi";
import { bold } from "colorette";
import { ESLint } from "eslint";

const __dirname = fileURLToPath(new URL(".", import.meta.url));

const rootDir = path.dirname(__dirname);

const idlDir = path.join(rootDir, "idls");
const clientDir = path.join(rootDir, "clients");

const start = Date.now();

console.log("generating client code...");

const kinobi = k.createFromIdl(path.join(idlDir, "oracle.json"), undefined, true);

kinobi.update(
  k.updateProgramsVisitor({
    oracle: {
      name: "optimisticOracle",
    },
  }),
);

// Update accounts.
kinobi.update(
  k.updateAccountsVisitor({
    oracleV1: {
      seeds: [k.constantPdaSeedNodeFromString("utf8", "oracle")],
    },
    currencyV1: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "currency"),
        k.variablePdaSeedNode("mint", k.publicKeyTypeNode(), "The address of the currency mint."),
      ],
    },
    requestV1: {
      size: null,
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "request"),
        k.variablePdaSeedNode(
          "index",
          k.numberTypeNode("u64"),
          "The next request index in the oracle.",
        ),
      ],
    },
    assertionV1: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "assertion"),
        k.variablePdaSeedNode("request", k.publicKeyTypeNode(), "The address of the request."),
      ],
    },
    stakeV1: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "stake"),
        k.variablePdaSeedNode("wallet", k.publicKeyTypeNode(), "The address of the wallet."),
      ],
    },
    votingV1: {
      size: null,
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "voting"),
        k.variablePdaSeedNode("request", k.publicKeyTypeNode(), "The address of the request."),
      ],
    },
    voteV1: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "vote"),
        k.variablePdaSeedNode(
          "voting",
          k.publicKeyTypeNode(),
          "The address of the voting account.",
        ),
        k.variablePdaSeedNode("stake", k.publicKeyTypeNode(), "The address of the stake account."),
      ],
    },
  }),
);

const ataPdaValueNode = (mint = "mint", owner = "owner") =>
  k.pdaValueNode(k.pdaLinkNode("associatedToken", "mplToolbox"), [
    k.pdaSeedValueNode("mint", k.accountValueNode(mint)),
    k.pdaSeedValueNode("owner", k.accountValueNode(owner)),
  ]);

// Set default values for instruction accounts.
kinobi.update(
  k.setInstructionAccountDefaultValuesVisitor([
    {
      account: "oracle",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("oracleV1"),
    },
    {
      account: "assertion",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("assertionV1", [
        k.pdaSeedValueNode("request", k.accountValueNode("request")),
      ]),
    },
    {
      account: "voting",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("votingV1", [
        k.pdaSeedValueNode("request", k.accountValueNode("request")),
      ]),
    },
    {
      account: "vote",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("voteV1", [
        k.pdaSeedValueNode("voting", k.accountValueNode("voting")),
        k.pdaSeedValueNode("stake", k.accountValueNode("stake")),
      ]),
    },
  ]),
);

// Update instructions.
kinobi.update(
  k.updateInstructionsVisitor({
    createRequestV1: {
      accounts: {
        // TODO: Default rewardMint to SOL/USDC?
        rewardSource: {
          defaultValue: ataPdaValueNode("rewardMint", "creator"),
        },
        rewardEscrow: {
          defaultValue: k.pdaValueNode(k.pdaLinkNode("reward", "hooked"), [
            k.pdaSeedValueNode("request", k.accountValueNode("request")),
          ]),
        },
        creator: {
          defaultValue: k.identityValueNode(),
        },
      },
    },
    createAssertionV1: {
      accounts: {
        bondSource: {
          defaultValue: ataPdaValueNode("bondMint", "asserter"),
        },
        bondEscrow: {
          defaultValue: k.pdaValueNode(k.pdaLinkNode("assertBond", "hooked"), [
            k.pdaSeedValueNode("request", k.accountValueNode("request")),
          ]),
        },
        governanceSource: {
          defaultValue: ataPdaValueNode("governanceMint", "asserter"),
        },
        governanceEscrow: {
          defaultValue: k.pdaValueNode(k.pdaLinkNode("assertGovernanceBond", "hooked"), [
            k.pdaSeedValueNode("request", k.accountValueNode("request")),
          ]),
        },
        asserter: {
          defaultValue: k.identityValueNode(),
        },
      },
    },
  }),
);

/** @param {string} name */
const accountType = (name) => ({
  field: "accountType",
  value: k.enumValueNode("AccountType", name),
});

// Set account discriminators.
kinobi.update(
  k.setAccountDiscriminatorFromFieldVisitor({
    oracleV1: accountType("OracleV1"),
    configV1: accountType("ConfigV1"),
    stakeV1: accountType("StakeV1"),
    requestV1: accountType("RequestV1"),
    assertionV1: accountType("AssertionV1"),
    currencyV1: accountType("CurrencyV1"),
    votingV1: accountType("VotingV1"),
    voteV1: accountType("VoteV1"),
  }),
);

// Fix UnixTimestamp type.
kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: (node) => node.kind === "definedTypeLinkNode" && node.name === "unixTimestamp",
      transform: (node) => {
        k.assertIsNode(node, "definedTypeLinkNode");

        return k.dateTimeTypeNode(k.numberTypeNode("i64"));
      },
    },
  ]),
);

// Render Rust.
{
  const crateDir = path.join(clientDir, "rust");
  const rustDir = path.join(crateDir, "src", "generated");

  console.log(`writing rust client to ${bold(path.relative(rootDir, rustDir))}...`);

  kinobi.accept(
    k.renderRustVisitor(rustDir, {
      crateFolder: crateDir,
      formatCode: true,
    }),
  );

  console.log("cleaning up generated rust client code...");

  execFile("cargo", ["fmt", `--manifest-path=${path.join(crateDir, "Cargo.toml")}`]);
}

// Render JavaScript.
{
  const jsDir = path.join(clientDir, "js", "src", "generated");

  console.log(`writing js client to ${bold(path.relative(rootDir, jsDir))}...`);

  kinobi.accept(
    k.renderJavaScriptVisitor(jsDir, {
      formatCode: true,
    }),
  );

  console.log("cleaning up generated js client code...");

  const eslint = new ESLint({
    cache: true,
    cacheLocation: path.join(rootDir, "node_modules", ".cache", "eslint-kinobi"),
    cacheStrategy: "content",
    fix: true,
  });
  const lintResults = await eslint.lintFiles(jsDir);

  await ESLint.outputFixes(lintResults);

  const eslintFormatter = await eslint.loadFormatter();
  const lintOutput = await eslintFormatter.format(lintResults);

  if (lintOutput) {
    console.error(lintOutput);
  }
}

console.log(`done in ${bold(`${Date.now() - start}ms`)}`);
