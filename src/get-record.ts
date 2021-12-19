import config from "./config";
import { CloudflareApiBase } from "./constants";
import { DnsRecord, GetRecordsPayload, GetZonesPayload } from "./types";
import fetch from "node-fetch";
import { cloudflareHeaders } from "./cloudflare";

export default async function getDnsRecord(): Promise<DnsRecord> {
  const zoneResponse = await fetch(
    `${CloudflareApiBase}/zones`,
    cloudflareHeaders
  );
  const zones = (await zoneResponse.json()) as GetZonesPayload;
  const zone = zones.result.find((z) => z.name === config.DOMAIN);
  if (!zone) {
    throw new Error("Cannot find zone (domain) " + config.DOMAIN);
  }

  const recordsResponse = await fetch(
    `${CloudflareApiBase}/zones/${zone.id}/dns_records`,
    cloudflareHeaders
  );
  const records = (await recordsResponse.json()) as GetRecordsPayload;
  const record = records.result.find(
    (r) =>
      r.name ===
      (config.RECORD ? `${config.RECORD}.${config.DOMAIN}` : config.DOMAIN)
  );
  console.log("FOund record: ", record);
  if (!record) {
    throw new Error(
      `Cannot find record ${config.RECORD} in zone (domain) ${config.DOMAIN}`
    );
  }
  return record;
}
