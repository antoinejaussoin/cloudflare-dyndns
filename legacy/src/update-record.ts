import { CloudflareApiBase } from "./constants";
import {
  DnsRecord,
  Type,
  UpdateRecordPayload,
  UpdateRecordResponsePayload,
} from "./types";
import fetch from "node-fetch";
import { cloudflareHeaders } from "./cloudflare";
import config from "./config";

export default async function updateRecords(
  zoneId: string,
  recordId: string,
  ip: string
): Promise<DnsRecord> {
  const payload: Partial<UpdateRecordPayload> = {
    type: Type.A,
    name: config.RECORD,
    content: ip,
  };
  const zoneResponse = await fetch(
    `${CloudflareApiBase}/zones/${zoneId}/dns_records/${recordId}`,
    { ...cloudflareHeaders, method: "PUT", body: JSON.stringify(payload) }
  );
  const result = (await zoneResponse.json()) as UpdateRecordResponsePayload;
  return result.result;
}
