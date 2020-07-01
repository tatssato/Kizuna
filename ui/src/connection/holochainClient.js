import { connect as hcWebClientConnect } from "@holochain/hc-web-client";
import { get } from "lodash/fp";
import { MicroOrchestrator } from "@uprtcl/micro-orchestrator";
import {
  HolochainConnectionModule,
  HolochainConnection,
} from "@uprtcl/holochain-provider";

let holochainClient;
let holochainUprtclClient;

// NB: This should be set to false when you want to run against a Holochain Conductor
// with a websocket interface running on REACT_APP_DNA_INTERFACE_URL.
// export const MOCK_DNA_CONNECTION =
//   process.env.NODE_ENV === "test" ||
//   process.env.REACT_APP_MOCK_DNA_CONNECTION === "true" ||
//   false;

// Do we need to close ws connection at some point?

export const HOLOCHAIN_LOGGING = process.env.NODE_ENV === "development";

export async function initAndGetHolochainClient() {
  if (holochainClient) return holochainClient;

  try {
    holochainClient = await hcWebClientConnect({
      url: process.env.REACT_APP_DNA_INTERFACE_URL,
      wsClient: { max_reconnects: 0 },
    });

    if (HOLOCHAIN_LOGGING) {
      console.log("🎉 Successfully connected to Holochain!");
    }
    return holochainClient;
  } catch (error) {
    if (HOLOCHAIN_LOGGING) {
      console.log(
        "😞 Holochain client connection failed -- ",
        error.toString()
      );
    }
    throw error;
  }
}

export function parseZomeCallPath(zomeCallPath) {
  const [zomeFunc, zome, instanceId] = zomeCallPath.split("/").reverse();

  return { instanceId, zome, zomeFunc };
}

export function callZome({ id, zome, func }) {
  return async function (args = {}) {
    try {
      let zomeCall;

      await initAndGetHolochainClient();
      zomeCall = holochainClient.callZome(id, zome, func);

      const rawResult = await zomeCall(args);
      const jsonResult = JSON.parse(rawResult);
      const error =
        get("Err", jsonResult) || get("SerializationError", jsonResult);
      const rawOk = get("Ok", jsonResult);
      const result = rawOk;

      if (error) throw error;

      return result;
    } catch (e) {
      const { Internal, Timeout } = { ...e };
      if (Internal) {
        const err = JSON.parse(Internal);
        if (err.constructor.name === "Object" && "code" in err) {
          throw new Error(JSON.stringify(err));
        }
      } else if (Timeout) {
        throw new Error(
          JSON.stringify({
            code: 502,
            message: "Timeout",
          })
        );
      }
      throw new Error(
        JSON.stringify({
          code: 1000,
          message: "Filler",
        })
      );
    }
  };
}

// see https://github.com/uprtcl/js-uprtcl/tree/master/providers/holochain
export async function hcUprtcl() {
  await initAndGetHolochainClient();
  console.log("xd");
  if (holochainUprtclClient) return holochainUprtclClient;
  console.log("xd1");

  holochainUprtclClient = new HolochainConnection({
    host: process.env.REACT_APP_DNA_INTERFACE_URL,
    devEnv: {
      // this property should be changed to your local paths and dna hash
      templateDnasPaths: {
        HcScjN8wBwrn3tuyg89aab3a69xsIgdzmX5P9537BqQZ5A7TEZu7qCY4Xzzjhma:
          "/Users/neilsongardose/projects/Kizuna/dnas/p2pcomm/dist/p2pcomm.dna.json",
      },
    },
  });
  console.log("xd2");

  const hcModule = new HolochainConnectionModule(holochainUprtclClient);
  console.log("xd3");

  const orchestrator = new MicroOrchestrator();
  console.log(orchestrator);

  await orchestrator.loadModule(hcModule);
  console.log(holochainUprtclClient);

  return holochainUprtclClient;
}
