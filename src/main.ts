import { createApp } from "vue";

// Vuetify
import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'

import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

// Components
import App from "./App.vue";

const vuetify = createVuetify({
  components,
  directives,
  theme: {
      defaultTheme: "light",
  }
});

const app = createApp(App);
app.use(vuetify);
app.mount("#app");
