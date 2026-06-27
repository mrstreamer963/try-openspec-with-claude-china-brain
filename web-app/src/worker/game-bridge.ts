/// <reference lib="webworker" />

let wasm: any = null;

async function loadWasm() {
  const wasmModule = await import('../wasm/wasm_core.js');
  await wasmModule.default();
  wasm = wasmModule;
  wasm.init();
}

self.onmessage = async (e: MessageEvent) => {
  const { type, payload } = e.data;
  if (type === 'init') {
    await loadWasm();
    self.postMessage({ type: 'initialized' });
    return;
  }
  if (!wasm) return;
  switch (type) {
    case 'update': {
      const result = wasm.update(payload.dt);
      self.postMessage({ type: 'state_update', payload: JSON.parse(result) });
      break;
    }
    case 'command': {
      wasm.send_command(JSON.stringify(payload.command));
      break;
    }
  }
};