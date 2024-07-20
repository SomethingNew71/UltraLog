import { createApp } from 'vue';
import App from './App.vue';
import 'vuetify/styles';
import { createVuetify } from 'vuetify';
import { VAutocomplete } from 'vuetify/components';

const app = createApp(App);
const vuetify = createVuetify({
  components: {
    VAutocomplete,
  },
  theme: {
    themes: {
      dark: {
        colors: {
          primary: '#71784E',
          secondary: '#F6F7EB',
          accent: '#BF4E30',
          neutral: '#292524',
          'base-100': '#292524',
          info: '#476C9B',
          success: '#9FA677',
          warning: '#FDC149',
          error: '#871E1C',
        },
      },
      light: {
        colors: {
          primary: '#71784E',
          secondary: '#F6F7EB',
          accent: '#BF4E30',
          neutral: '#292524',
          'base-100': '#292524',
          info: '#476C9B',
          success: '#9FA677',
          warning: '#FDC149',
          error: '#871E1C',
        },
      },
    },
  },
});

app.use(vuetify);
app.mount('#app');
