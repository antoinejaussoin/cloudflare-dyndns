import fetch from "node-fetch";
import debug from "./log";

export async function getPublicIp(): Promise<string> {
  const response = await fetch("http://api.ipify.org");
  const result = await response.text();
  debug("Public IP response: ", result);
  return result;
}
