<script>
  import { onMount } from "svelte";
  import { WebMidi } from "webmidi";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api";
  import File from "../componets/File.svelte";

  onMount(() => {
    window.addEventListener("keydown", (event) => {
      console.log("Spacebar clicked");
    });
  });
  onMount(() => {
    invoke("open_midi_connection", { inputIdx: 1 });

    listen("midi_message", (event) => {
      console.log(event);
    });
  });
</script>

<h1>Welcome to SvelteKit</h1>
<p>
  Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the
  documentation
  <File />
</p>
