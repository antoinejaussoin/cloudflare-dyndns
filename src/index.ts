import getIds from "./get-ids";
import { getPublicIp } from "./get-ip";

console.log("Welcome");

async function main() {
  try {
    const ids = await getIds();
    console.log("Ids: ", ids);
  } catch (err) {
    console.log("An error occured: ", err);
  }

  console.log("ip: ", ip);

  setInterval(async () => {
    const ip = await getPublicIp();
  }, 5000);
}

main();
