import getDnsRecord from "./get-record";
import { getPublicIp } from "./get-ip";
import { DnsRecord } from "./types";
import updateRecords from "./update-record";
import chalk from "chalk";
import config from "./config";

console.log(chalk.yellow("----------------------------"));
console.log("🌍 Cloudflare Dynamic DNS 🔁");
console.log(chalk.yellow("----------------------------"));
console.log();

function log(text: string) {
  console.log(chalk.blue(`[${new Date().toLocaleTimeString()}] `) + text);
}

async function getDnsEntry() {
  const record = await getDnsRecord();
  log(
    "Current DNS IP: " +
      chalk.green(record.name) +
      ": " +
      chalk.red(record.content)
  );
  if (config.DEBUG) {
    console.log("Record: ", record);
  }
  return record;
}

async function main() {
  log("Initialization");

  try {
    let lastRecord: DnsRecord = await getDnsEntry();
    await update(lastRecord);

    setInterval(async () => {
      try {
        log("Refreshing DNS entry");
        lastRecord = await getDnsEntry();
        await update(lastRecord);
      } catch (ex) {
        log("❗️ " + chalk.red("An error occurred: ") + ex);
      }
    }, config.CHECK_INTERVAL_SEC * 1000);
  } catch (ex) {
    log("❗️ " + chalk.red("An error occurred when initializing: ") + ex);
    throw ex;
  }
}

async function update(lastRecord: DnsRecord) {
  const ip = await getPublicIp();
  if (ip !== lastRecord.content) {
    log(
      "🔴 The public IP has changed: " +
        chalk(ip) +
        " vs DNS " +
        chalk(lastRecord.content)
    );
    const record = await updateRecords(lastRecord.zone_id, lastRecord.id, ip);
    log("🟠 The DNS record has been updated to " + chalk.red(record.content));
    if (config.DEBUG) {
      console.log("resp: ", record);
    }

    lastRecord = record;
  } else {
    log("✅ The current public IP is the same (" + chalk.red(ip) + ")");
  }
}

main();
