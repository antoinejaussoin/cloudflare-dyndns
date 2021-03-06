import fs from "fs";
import path from "path";
import dotenv from "dotenv";
import type { Config } from "./types";

function findDotEnvPath(): string | null {
  let current = path.resolve(__dirname);
  for (let i = 0; i < 5; i++) {
    const custom = path.resolve(current, ".env");
    const example = path.resolve(current, ".env.example");
    if (fs.existsSync(custom)) {
      return custom;
    }
    if (fs.existsSync(example)) {
      return example;
    }
    current = path.resolve(current, "..");
  }
  return null;
}

const dotEnvPath = findDotEnvPath();
if (dotEnvPath) {
  dotenv.config({ path: dotEnvPath });
}

function defaults(key: string, defaultValue: string): string {
  if (process.env[key] === undefined) {
    return defaultValue;
  }
  return process.env[key]!;
}

function defaultsBool(key: string, defaultValue: boolean): boolean {
  if (process.env[key] === undefined) {
    return defaultValue;
  }
  return process.env[key]! === "true";
}

function defaultsNumber(key: string, defaultValue: number): number {
  if (process.env[key] === undefined) {
    return defaultValue;
  }
  return parseInt(process.env[key]!);
}

const config: Config = {
  CLOUDFLARE_API_TOKEN: defaults(
    "CLOUDFLARE_API_TOKEN",
    "<your cloudflare token>"
  ),
  DOMAIN: defaults("DOMAIN", "acme.com"),
  RECORD: defaults("RECORD", "www"),
  DEBUG: defaultsBool("DEBUG", false),
  CHECK_INTERVAL_SEC: defaultsNumber("CHECK_INTERVAL_SEC", 900),
};

export default config;
