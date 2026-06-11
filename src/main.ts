import init, { set_title, sum_array } from 'rust-wasm'
import wasmUrl from 'rust-wasm/rust_wasm_bg.wasm?url'

async function initwasm() {
  await init(wasmUrl)

  console.log('求和结果为：', sum_array([10, 20, 30]))

  set_title('锈化一切！！！')
}

initwasm()
