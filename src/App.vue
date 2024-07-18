<script setup lang="ts">
  import ChannelCard from './components/ChannelCard.vue';
  import type { LogChannel } from './types';

  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api';
  import { ref } from 'vue';

  const channels = ref<LogChannel[]>([]);

  listen('tauri://file-drop', (event) => {
    const [filePath] = event.payload as string[];
    invoke('add_file', { filePath }).then((rawChannels: any) => {
      channels.value = JSON.parse(rawChannels);
      console.log(channels.value);
    });
  });
</script>

<template>
  <div class="container">
    <ChannelCard></ChannelCard>
  </div>
</template>
