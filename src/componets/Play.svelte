<script>
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import WaveSurfer from "wavesurfer.js";

  export let file_path;

  const randomNumber = Math.floor(Math.random() * 100000000);
  const filename = file_path.substring(file_path.lastIndexOf("/") + 1);
  const musicUrl = convertFileSrc(file_path);
  let sound = new Audio(musicUrl);

  const togglePlay = () => {
    sound.playPause();
  };
  onMount(() => {
    window.addEventListener("keydown", (event) => {
      if (event.key === "Escape") {
        sound.pause();
        sound.currentTime = 0;
      }
    });
  });
  onMount(() => {
    sound = WaveSurfer.create({
      container: "#waveform" + randomNumber,
      waveColor: "#4F4A85",
      cursorWidth: 0,
      progressColor: "white",
      barWidth: 4,
      interact: false,
      barRadius: 4,
      url: musicUrl,
    });
  });
</script>

<div>
  <div
    role="button"
    on:click={togglePlay}
    class="relative rounded shadow-md hover:shadow-2xl bg-slate-800"
    id="waveform{randomNumber}"
  >
    <!-- <div class="absolute top-0 ml-2 left-0 text-lg text-white z-10">Q</div> -->
  </div>
</div>
