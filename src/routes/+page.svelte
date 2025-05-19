<script>
  import { invoke } from '@tauri-apps/api/core'
</script>

<button onclick={() => invoke('get_appdata').then(r => console.log(r))}>Get data</button>
<button onclick={() => invoke('set_appdata', { new: {
  api_url: 'aboba',
  access_token: 'aaa',
  refresh_token: 'bbb'
} }).then(r => console.log(r))}>Set data and get it</button>