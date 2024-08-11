<script setup lang="ts">
import { ref } from "vue";
import { listen } from '@tauri-apps/api/event'
import { invoke, Channel } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { downloadDir } from "@tauri-apps/api/path";

const output = ref("");
const name = ref("");

type DownloadValue =
  | {
      url: string;
      status: string;
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
  let item = downloads.value.find(x => 'url' in x && x.url == name.value)
  if (item) {
    item.status = event.payload as string
  } else {
    downloads.value.push({ url: name.value, status: event.payload as string })
  }
})

async function download() {
  await invoke('download', {
    url: name.value,
    onEvent,
  });
}

async function save_to() {
  const selected = await open({
    directory: true,
    defaultPath: await downloadDir(),
  });
  console.log(selected);
}
</script>

<template>
  <v-card>
    <v-toolbar color="transparent">
      <v-toolbar-title class="text-h6" text="Downloader"></v-toolbar-title>

      <template v-slot:append>
        <v-btn icon="mdi-dots-vertical" @click="save_to()"></v-btn>
      </template>
    </v-toolbar>

    <v-card-text class="d-flex align-center">

      <v-text-field
        label="Url"
        hide-details
        v-model="name"
      />

      <v-btn
        color="primary"
        class="ms-5"
        :disabled="!name || !name.startsWith('http')"
        @click="download()">
        Download
      </v-btn>

    </v-card-text>
  </v-card>

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
</template>
