import fetch from "node-fetch";

export async function getPublicIp(): Promise<string> {
  const response = await fetch("http://api.ipify.org");
  const result = await response.text();
  return result;
}
