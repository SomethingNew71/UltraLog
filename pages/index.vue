<script setup lang="ts">
  import { sub } from 'date-fns';
  import type { Period, Range } from '~/types';

  const { isNotificationsSlideoverOpen } = useDashboard();

  const items = [
    [
      {
        label: 'New mail',
        icon: 'i-heroicons-paper-airplane',
        to: '/inbox',
      },
      {
        label: 'New user',
        icon: 'i-heroicons-user-plus',
        to: '/users',
      },
    ],
  ];

  const range = ref<Range>({ start: sub(new Date(), { days: 14 }), end: new Date() });
  const period = ref<Period>('daily');
</script>

<template>
  <UDashboardPage>
    <UDashboardPanel grow>
      <UDashboardNavbar title="Log Playback Viewer">
        <template #right>
          <p class="mt-2">Version: 1.0.0</p>

          <UDropdown :items="items">
            <UButton icon="i-heroicons-plus" size="md" class="ml-1.5 rounded-full" />
          </UDropdown>
        </template>
      </UDashboardNavbar>

      <UDashboardToolbar>
        <template #left>
          <PlaybackDateRangePicker v-model="range" class="-ml-2.5" />
          <PlaybackPeriodSelect v-model="period" :range="range" />
        </template>
      </UDashboardToolbar>

      <UDashboardPanelContent>
        <div class="grid lg:grid-cols-4 lg:items-start gap-8 mt-8 mb-3">
          <CoreDataCard v-for="i in 8" />
        </div>
        <PlaybackChart :period="period" :range="range" />
      </UDashboardPanelContent>
    </UDashboardPanel>
  </UDashboardPage>
</template>
