import config from "./config";
import { RequestInit } from "node-fetch";

export const cloudflareHeaders: Partial<RequestInit> = {
  headers: {
    Authorization: `Bearer ${config.CLOUDFLARE_API_TOKEN}`,
    "Content-Type": "application/json",
  },
};
