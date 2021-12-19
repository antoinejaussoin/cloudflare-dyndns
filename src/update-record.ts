import { CloudflareApiBase } from "./constants";
import { Ids, UpdateRecordPayload, UpdateRecordResponsePayload } from "./types";
import fetch from "node-fetch";
import { cloudflareHeaders } from "./cloudflare";

export default async function updateRecords(
  ids: Ids,
  ip: string
): Promise<boolean> {
  const payload: Partial<UpdateRecordPayload> = {
    content: ip,
  };
  const zoneResponse = await fetch(
    `${CloudflareApiBase}/zones/${ids.zoneId}/dns_records/${ids.recordId}`,
    { ...cloudflareHeaders, method: "PUT", body: JSON.stringify(payload) }
  );
  const result = (await zoneResponse.json()) as UpdateRecordResponsePayload;
  return result.success;
}
