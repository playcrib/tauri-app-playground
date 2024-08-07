<script setup lang="ts">
import { ref } from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";
import { Command } from '@tauri-apps/plugin-shell';

const output = ref("");
const name = ref("");

type DownloadEvent =
  | {
      event: 'started';
      data: {
        url: string;
        downloadId: number;
        contentLength: number;
      };
    }
  | {
      event: 'progress';
      data: {
        downloadId: number;
        chunkLength: number;
      };
    }
  | {
      event: 'finished';
      data: {
        downloadId: number;
      };
    };

const onEvent = new Channel<DownloadEvent>();
onEvent.onmessage = (message) => {
  console.log(`got download event ${message.event}, ${message.data}`);
  output.value = message.event
};

async function download() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // const command = Command.sidecar('binaries/yt-dlp', [
  //   name.value
  // ]);
  // output.value = await command.execute();

  await invoke('download', {
    url: name.value,
    onEvent,
  });
}
</script>

<template>
  <form class="row" @submit.prevent="download">
    <input id="download-input" v-model="name" placeholder="Enter a url..." />
    <button type="submit">Download</button>
  </form>

  <p>{{ output }}</p>
</template>
