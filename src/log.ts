/* eslint-disable @typescript-eslint/no-explicit-any */
import config from "./config";

export default function debug(message?: any, ...params: any[]) {
  if (config.DEBUG) {
    console.log(message, ...params);
  }
}
