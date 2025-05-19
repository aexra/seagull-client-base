<script>
  import { invoke } from '@tauri-apps/api/core'
</script>

<button onclick={() => invoke('get_api_url').then(r => console.log(r))}>Click me!</button>