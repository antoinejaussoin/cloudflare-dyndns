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

async function main() {
  log("Initialization");
  let lastRecord: DnsRecord;
  try {
    lastRecord = await getDnsRecord();
    log(
      "Current DNS IP: " +
        chalk.green(lastRecord.name) +
        ": " +
        chalk.red(lastRecord.content)
    );
    if (config.DEBUG) {
      console.log("Record: ", lastRecord);
    }
  } catch (err) {
    console.log("An error occured: ", err);
  }

  setInterval(async () => {
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
  }, 5000);
}

main();
