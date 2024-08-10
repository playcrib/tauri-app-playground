<script setup lang="ts">
import { ref } from "vue";
import { listen } from '@tauri-apps/api/event'
import { invoke, Channel } from "@tauri-apps/api/core";

const output = ref("");
const outputs = ref([]);
const name = ref("");

type DownloadValue =
  | {
      url: string;
      status: number;
    };
const downloads = ref<Array<DownloadValue>>([]);

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

await listen('rs2js', (event) => {
  console.log("js: rs2js: " + event)
  // let input = event.payload
  // outputs.value.push({ timestamp: Date.now(), message: input })
  let item = downloads.value.find(x => 'url' in x && x.url == name.value)
  if (item) {
    item.status += 1
  } else {
    downloads.value.push({ url: name.value, status: 1 })
  }
})

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
    <!-- <input id="download-input" v-model="name" placeholder="Enter a url..." /> -->
    <v-text-field
      v-model="name"
      :counter="10"
      label="Download URL"
      hide-details
      required
    ></v-text-field>
    <!-- <button type="submit">Download</button> -->
    <v-btn type="submit" text="Download"></v-btn>
  </form>

  <v-table>
    <thead>
      <tr>
        <th class="text-left">
          URL
        </th>
        <th class="text-left">
          Status
        </th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="item in downloads"
        :key="item.url"
      >
        <td class="text-left">{{ item.url }}</td>
        <td class="text-left">{{ item.status }}</td>
      </tr>
    </tbody>
  </v-table>

  <p>{{ output }}</p>
  <p>{{ outputs }}</p>
</template>
