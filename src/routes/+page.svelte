<script>
  import { onMount } from "svelte";
  import { WebMidi } from "webmidi";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api";
  import Octave from "../componets/Octave.svelte";

  onMount(() => {
    invoke("open_midi_connection", { inputIdx: 1 });

    listen("midi_message", (event) => {
      console.log(event);
    });
  });
</script>

<div class="p-2">
  <Octave />
</div>
