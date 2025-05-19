<script>
  import { invoke } from '@tauri-apps/api/core'
</script>

<button onclick={() => invoke('greet', { name: 'World' }).then(r => console.log(r))}>Click me!</button>