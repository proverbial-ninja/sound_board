<script>
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import WaveSurfer from "wavesurfer.js";

  export let file_path;
  const filename = file_path.substring(file_path.lastIndexOf("/") + 1);
  const musicUrl = convertFileSrc(file_path);
  let sound = new Audio(musicUrl);

  onMount(() => {
    sound = WaveSurfer.create({
      container: "#waveform",
      waveColor: "#4F4A85",
      progressColor: "#383351",
      url: musicUrl,
    });
  });
</script>

<button
  on:click={() => sound.play()}
  class="btn btn-lg truncate btn-secondary m-2">Play {filename}</button
>
<div id="waveform"></div>
