<script>
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api";
  import {
    copyFile,
    BaseDirectory,
    createDir,
    readDir,
  } from "@tauri-apps/api/fs";
  import { localDataDir } from "@tauri-apps/api/path";
  import { onMount } from "svelte";
  import Play from "./Play.svelte";
  let filess = [];

  onMount(async () => {
    filess = await readDir("Proverbial SoundBoard/test/", {
      dir: BaseDirectory.LocalData,
      recursive: true,
    });
  });
  listen("tauri://file-drop-hover", (event) => {
    console.log("Received event:", event);
  });
  listen("tauri://file-drop", (event) => {
    console.log("Drop event:", event);

    event.payload.forEach(async (file) => {
      try {
        // Copy the `$APPCONFIG/app.conf` file to `$APPCONFIG/app.conf.bk`
        const appLocalDataDirPath = await localDataDir();
        console.log(".AppData.toStrinaag()");
        console.log(appLocalDataDirPath);

        const randomNumber = Math.floor(Math.random() * 100);
        const filename = file.substring(file.lastIndexOf("/") + 1);
        console.log("filename:", filename);
        // Create the `$APPDATA/users` directory
        const a = await createDir("Proverbial SoundBoard/test/", {
          dir: BaseDirectory.LocalData,
          recursive: true,
        });

        console.log("asasasa");
        console.log("a:", a);
        await copyFile(
          file,
          appLocalDataDirPath +
            "Proverbial SoundBoard/test/" +
            randomNumber +
            "-" +
            filename,
          { dir: BaseDirectory.localDataDir }
        );

        // Reads the `$APPDATA/users` directory recursively
      } catch (e) {
        console.error(e);
      }
    });
  });

  let inside = false;
  let files = {
    accepted: [],
    rejected: [],
  };

  const onDragEnter = (e) => {
    inside = true;
  };

  const onDragExit = (e) => {
    inside = false;
  };
</script>

{#each filess as file}
  {#if file.path.endsWith(".mp3")}
    <p><Play file_path={file.path} /></p>
  {/if}
{/each}
<div class="flex justify-center items-center min-h-screen">
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class:bg-red-600={inside}
    on:dragover={onDragEnter}
    on:dragleave={onDragExit}
    class="flex flex-col items-center justify-center w-1/2 p-4 border shadow-lg hover:bg-white hover:border-red-500 rounded"
  >
    <p class="my-2 text-center text-lg">
      Drop files here or click to select {inside}
    </p>
  </div>
</div>
