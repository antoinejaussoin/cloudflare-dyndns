export interface Config {
  CLOUDFLARE_API_TOKEN: string;
  DOMAIN: string;
  RECORD: string;
  DEBUG: boolean;
}

export interface Ids {
  zoneId: string;
  recordId: string;
}

export interface UpdateRecordPayload {
  type: Type;
  name: string;
  content: string;
  proxied: boolean;
}

export interface GetRecordsPayload {
  result: DnsRecord[];
  success: boolean;
  errors: unknown[];
  messages: unknown[];
  result_info: ResultInfo;
}

export interface UpdateRecordResponsePayload {
  result: DnsRecord;
  success: boolean;
  errors: unknown[];
  messages: unknown[];
}

export interface DnsRecord {
  id: string;
  zone_id: string;
  zone_name: string;
  name: string;
  type: Type;
  content: string;
  proxiable: boolean;
  proxied: boolean;
  ttl: number;
  locked: boolean;
  meta: Meta;
  created_on: Date;
  modified_on: Date;
  priority?: number;
}

export interface Meta {
  auto_added: boolean;
  managed_by_apps: boolean;
  managed_by_argo_tunnel: boolean;
  source: string;
}

export enum Type {
  A = "A",
  Cname = "CNAME",
  MX = "MX",
  Txt = "TXT",
}

export interface GetZonesPayload {
  result: Zone[];
  result_info: ResultInfo;
  success: boolean;
  errors: unknown[];
  messages: unknown[];
}

export interface Zone {
  id: string;
  name: string;
  status: string;
  paused: boolean;
  type: string;
  development_mode: number;
  name_servers: string[];
  original_name_servers: string[];
  original_registrar: string;
  original_dnshost: null;
  modified_on: Date;
  created_on: Date;
  activated_on: Date;
  meta: Meta;
  owner: Owner;
  account: Account;
  permissions: string[];
  plan: Plan;
}

export interface Account {
  id: string;
  name: string;
}

export interface Meta {
  step: number;
  wildcard_proxiable: boolean;
  custom_certificate_quota: number;
  page_rule_quota: number;
  phishing_detected: boolean;
  multiple_railguns_allowed: boolean;
}

export interface Owner {
  id: string;
  type: string;
  email: string;
}

export interface Plan {
  id: string;
  name: string;
  price: number;
  currency: string;
  frequency: string;
  is_subscribed: boolean;
  can_subscribe: boolean;
  legacy_id: string;
  legacy_discount: boolean;
  externally_managed: boolean;
}

export interface ResultInfo {
  page: number;
  per_page: number;
  total_pages: number;
  count: number;
  total_count: number;
}
