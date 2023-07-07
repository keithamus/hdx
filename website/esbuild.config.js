import esbuild from 'esbuild'
import { wasmLoader } from 'esbuild-plugin-wasm'

esbuild.build({
  entryPoints: ['src/index.js'],
  bundle: true,
  format: 'esm',
  minify: process.env.NODE_ENV === 'production',
  outfile: 'playground/index.js',
  plugins: [
      wasmLoader()
  ]
});
