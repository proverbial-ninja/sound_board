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
        filess = await readDir("Proverbial SoundBoard/test/", {
          dir: BaseDirectory.LocalData,
          recursive: true,
        });

        // Reads the `$APPDATA/users` directory recursively
      } catch (e) {
        console.error(e);
      }
    });
  });
</script>

<div class="grid grid-cols-6 gap-2">
  {#each filess as file}
    {#if file.path.endsWith(".mp3")}
      <Play file_path={file.path} />
    {/if}
  {/each}
</div>
