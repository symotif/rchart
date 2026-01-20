<script>
    import { invoke } from '@tauri-apps/api/tauri'
  
    let name = $state('')
    let greetMsg = $state('')
  
    async function greet() {
      greetMsg = await invoke('greet', { name })
    }
  </script>
  
  <div>
    <input id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
    <button onclick="{greet}">Greet</button>
    <p>{greetMsg}</p>
  </div>