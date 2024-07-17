<script setup lang="ts">
  import type { Period, Range } from '~/types';
  let data = [10, 40, 15, 25, 50];
  const cardRef = ref<HTMLElement | null>(null);

  const props = defineProps({
    period: {
      type: String as PropType<Period>,
      required: true,
    },
    range: {
      type: Object as PropType<Range>,
      required: true,
    },
  });

  function addData() {
    // add random value from 0 to 50 to array
    data = [...data, Math.round(Math.random() * 50)];
  }
  function filterData() {
    data = data.filter((v) => v <= 35);
  }
</script>

<template>
  <UDashboardCard ref="cardRef" id="dashboardChart" class="h-full" :ui="{ body: { padding: '!pb-3 !px-0' } as any }">
    <template #header>
      <div>
        <p class="text-sm text-gray-500 dark:text-gray-400 font-medium mb-1">Log Playback</p>
        <!-- <p class="text-3xl text-gray-900 dark:text-white font-semibold">
          {{ formatNumber(total) }}
        </p> -->
      </div>
    </template>
    <responsive-line-chart />
  </UDashboardCard>
</template>
