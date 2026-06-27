/// <reference lib="webworker" />

import init, { init as wasmInit, send_command, update } from 'wasm-core';

let wasmReady = false;

async function loadWasm() {
  await init();
  wasmInit();
  wasmReady = true;
}

self.onmessage = async (e: MessageEvent) => {
  const { type, payload } = e.data;
  if (type === 'init') {
    await loadWasm();
    self.postMessage({ type: 'initialized' });
    return;
  }
  if (!wasmReady) return;
  switch (type) {
    case 'update': {
      const result = update(payload.dt);
      self.postMessage({ type: 'state_update', payload: JSON.parse(result) });
      break;
    }
    case 'command': {
      send_command(JSON.stringify(payload.command));
      break;
    }
  }
};