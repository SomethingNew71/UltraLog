<script setup lang="ts">
  import type { LogChannel } from './types';
  import NavBar from './components/NavBar.vue';
  import ChannelCard from './components/ChannelCard.vue';
  import PlaybackChart from './components/PlaybackChart.vue';
  import FileUploadList from './components/FileUploadList.vue';
  import { PlusIcon } from '@heroicons/vue/24/solid';

  import { Event, listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api';
  import { ref } from 'vue';
  import { debounce } from 'lodash';

  const channels = ref<LogChannel[]>([]);
  const channelToAdd = ref<string>('');
  const selectedChannels = ref<LogChannel[]>([]);

  const loading = ref(false);
  let errors = ref({
    notFound: false,
    maxReached: false,
    alreadyAdded: false,
    noChannel: false,
  });

  const dropHandler = async (event: Event<string[]>) => {
    loading.value = true;
    const [filePath] = event.payload;
    channels.value = await invoke('add_file', { filePath });
    console.log(channels.value);
    loading.value = false;
  };
  listen(
    'tauri://file-drop',
    debounce(dropHandler, 5_000, {
      leading: true,
      trailing: false,
    })
  );

  async function addChannel() {
    if (channelToAdd.value) {
      if (selectedChannels.value.length > 10) {
        triggerToast('maxReached');
        console.error('Max channels reached');
        return;
      }
      if (selectedChannels.value.find((channel) => channel.name === channelToAdd.value)) {
        triggerToast('alreadyAdded');
        console.error('Channel already added');
        return;
      }

      const channel = channels.value.find((channel) => channel.name === channelToAdd.value);
      if (channel) {
        selectedChannels.value.push(channel);
        const channelData = await invoke('get_channel_data', { channelName: channel.name });
        console.log(channelData);
      } else {
        triggerToast('notFound');
        console.error('Channel not found');
      }
    } else {
      triggerToast('noChannel');
      console.error('No channel selected');
    }
  }

  function triggerToast(type: string) {
    // @ts-ignore
    errors.value[type] = true;
    setTimeout(() => {
      // @ts-ignore
      errors.value[type] = false;
    }, 3000);
  }
</script>

<template>
  <NavBar></NavBar>
  <div class="p-5">
    <div class="grid grid-cols-5 gap-4">
      <div class="h-full col-span-4">
        <PlaybackChart></PlaybackChart>
      </div>
      <div class="grid grid-cols-subgrid gap-4 content-baseline">
        <div class="card bg-base-300 shadow-xl pt-2 pb-3">
          <div class="card-body p-3">
            <div class="card-title justify-between">
              <h2 class="text-primary text-2xl font-logo">Log Channels</h2>
            </div>
            <progress v-if="loading" class="progress progress-accent w-full"></progress>
            <!-- <label class="form-control w-full">
              <select class="select select-bordered" v-model="channelToAdd" :disabled="loading">
                <option v-for="channel in channels" :value="channel.name">{{ channel.name }}</option>
              </select>
              <div class="label">
                <span class="label-text-alt">Max: 10 Channels</span>
              </div>
            </label> -->
            <v-autocomplete
              label="Channel"
              :disabled="loading || !channels.length"
              :items="channels"
              v-model="channelToAdd"
              variant="outlined"
              itemValue="name"
              itemTitle="name"
            ></v-autocomplete>
            <button @click="addChannel()" class="btn btn-success" :disabled="selectedChannels.length > 10">
              <PlusIcon class="size-6" />
              Add
            </button>
          </div>
        </div>
        <div class="card bg-base-300 shadow-xl pt-2 pb-3">
          <div class="card-body p-3">
            <div class="card-title justify-between">
              <h2 class="text-primary text-2xl font-logo pb-3">Log Files</h2>
            </div>
            <FileUploadList></FileUploadList>
          </div>
        </div>
      </div>
    </div>
    <div class="grid grid-cols-5 gap-4 pt-3">
      <ChannelCard v-for="channel in selectedChannels" :channel="channel"></ChannelCard>
    </div>
  </div>
  <div class="toast toast-end">
    <div v-if="errors.notFound" class="alert alert-warning">
      <span>Channel not found</span>
    </div>
    <div v-if="errors.maxReached" class="alert alert-warning">
      <span>Max channels reached</span>
    </div>
    <div v-if="errors.alreadyAdded" class="alert alert-warning">
      <span>Channel already added</span>
    </div>
    <div v-if="errors.noChannel" class="alert alert-warning">
      <span>No channel selected</span>
    </div>
  </div>
</template>
