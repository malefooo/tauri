import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import naive from 'naive-ui'

createApp(App)
    .use(naive as any)
    .mount("#app");


