import { createSharedComposable } from '@vueuse/core';

const _useDashboard = () => {
  const route = useRoute();
  const router = useRouter();
  const isHelpSlideoverOpen = ref(false);

  defineShortcuts({
    'g-h': () => router.push('/'),
    '?': () => (isHelpSlideoverOpen.value = true),
  });

  watch(
    () => route.fullPath,
    () => {
      isHelpSlideoverOpen.value = false;
    }
  );

  return {
    isHelpSlideoverOpen,
  };
};

export const useDashboard = createSharedComposable(_useDashboard);
