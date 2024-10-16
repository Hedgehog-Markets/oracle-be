//@ts-check

const path = require("path");

const rootDir = path.dirname(__dirname);
const binDir = path.join(rootDir, ".bin");

/**
 * @param {string} binary
 * @returns {string}
 */
function getProgram(binary) {
  return path.join(binDir, binary);
}

/** @type {import("@metaplex-foundation/amman").AmmanConfig} */
module.exports = {
  validator: {
    matchFeatures: "mainnet-beta",
    commitment: "finalized",
    accountsCluster: "https://api.mainnet-beta.solana.com",
    programs: [
      {
        label: "Optimistic Oracle",
        programId: "DVM3hK9SDgXLmVoLng1KrTJCzTnhw31hAnqTYP7uGCot",
        deployPath: getProgram("oracle_program.so"),
      },
    ],
  },
};
