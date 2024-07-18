import { createApp } from 'vue';
import PrimeVue from 'primevue/config';
import aura from '@primevue/themes/aura';
import App from './App.vue';

const app = createApp(App);

app.use(PrimeVue, {
  theme: {
    preset: aura,
  },
});

app.mount('#app');
