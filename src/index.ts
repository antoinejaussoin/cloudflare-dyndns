import getDnsRecord from "./get-record";
import { getPublicIp } from "./get-ip";
import { DnsRecord } from "./types";
import updateRecords from "./update-record";

console.log("Welcome");

async function main() {
  let lastRecord: DnsRecord;
  try {
    lastRecord = await getDnsRecord();
    console.log("Record: ", lastRecord);
  } catch (err) {
    console.log("An error occured: ", err);
  }

  setInterval(async () => {
    const ip = await getPublicIp();
    if (ip !== lastRecord.content) {
      const resp = await updateRecords(lastRecord.zone_id, lastRecord.id, ip);
      console.log("resp: ", resp);
      lastRecord = resp;
    }
  }, 5000);
}

main();
