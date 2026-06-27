import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasmPack from 'vite-plugin-wasm-pack'
import path from 'path'
import fs from 'fs'
import { fileURLToPath } from 'url'

const __dirname = fileURLToPath(new URL('.', import.meta.url))

export default defineConfig({
  plugins: [
    vue(),
    wasmPack(['../wasm-core']),
    {
      name: 'serve-wasm',
      configureServer(server) {
        server.middlewares.use((req, res, next) => {
          if (req.url && req.url.endsWith('.wasm')) {
            const basename = path.basename(req.url)
            const wasmPath = path.resolve(__dirname, '../wasm-core/pkg', basename)
            if (fs.existsSync(wasmPath)) {
              res.setHeader('Content-Type', 'application/wasm')
              res.setHeader('Cache-Control', 'no-cache')
              fs.createReadStream(wasmPath).pipe(res)
              return
            }
          }
          next()
        })
      }
    }
  ],
  worker: { format: 'es' },
})