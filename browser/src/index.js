import { createClient } from "@connectrpc/connect";
import { createConnectTransport } from "@connectrpc/connect-web";
import { SimService } from "./gen/sim/v1/sim_pb.js";
import { createViewer } from "./viewer.js";

export { createViewer };

export async function view(canvas, baseUrl) {
  const transport = createConnectTransport({ baseUrl, useBinaryFormat: true });
  const sim = createClient(SimService, transport);
  const viewer = await createViewer(canvas, await sim.getModel({}));
  for await (const state of sim.streamState({})) viewer.setQpos(state.qpos);
}
