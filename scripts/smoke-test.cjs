// End-to-end check of the web client against the deployed contract.
// Each wallet can check in only once, so every run uses a brand-new Testnet
// keypair funded by Friendbot (nothing is stored). Usage:
//   node scripts/smoke-test.cjs [ignored-alias]
const fs = require("fs");
const path = require("path");
const StellarSdk = require("@stellar/stellar-sdk");
const makeClient = require("../docs/checkin-client.js");

const cfgSrc = fs.readFileSync(path.join(__dirname, "../docs/config.js"), "utf8");
const window = {};
new Function("window", cfgSrc)(window);
const cfg = window.CONFIG;

const kp = StellarSdk.Keypair.random();
const sign = async (xdr) => {
  const tx = StellarSdk.TransactionBuilder.fromXDR(xdr, cfg.networkPassphrase);
  tx.sign(kp);
  return tx.toXDR();
};

(async () => {
  const fund = await fetch("https://friendbot.stellar.org/?addr=" + kp.publicKey());
  if (!fund.ok) throw new Error("friendbot " + fund.status);

  const client = makeClient(StellarSdk, cfg);
  const before = await client.count();
  if (await client.hasCheckedIn(kp.publicKey())) throw new Error("fresh wallet already checked in?");
  const name = "Smoke test " + new Date().toISOString().slice(0, 16);
  const { hash, id } = await client.checkIn(kp.publicKey(), name, sign);
  const after = await client.count();
  const [latest] = await client.list(1);
  if (after !== before + 1 || id !== after || latest.id !== id || latest.name !== name)
    throw new Error(`unexpected state before=${before} after=${after} id=${id}`);
  if (!(await client.hasCheckedIn(kp.publicKey()))) throw new Error("has_checked_in should be true");

  let dupRejected = false;
  try { await client.checkIn(kp.publicKey(), name, sign); } catch (e) { dupRejected = /#1\b/.test(e.message); }
  if (!dupRejected) throw new Error("second check-in was not rejected");

  console.log(`OK count ${before} -> ${after}, attendee #${id}, duplicate rejected`);
  console.log(`Proof: https://stellar.expert/explorer/testnet/tx/${hash}`);
})().catch((e) => { console.error("FAIL", e.message); process.exit(1); });
