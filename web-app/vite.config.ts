import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasmPack from 'vite-plugin-wasm-pack'

export default defineConfig({
  plugins: [vue(), wasmPack(['../wasm-core'])],
  worker: { format: 'es' },
})