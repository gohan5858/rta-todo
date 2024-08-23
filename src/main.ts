import Todo from "@page/Todo.vue";
import { createApp } from "vue";
import { createRouter, createWebHashHistory } from 'vue-router';
import App from "./App.vue";
import "./assets/global.css";

const routes = [
  { path: '/', redirect: '/todo' },
  { path: '/todo', name: 'todo', component: Todo },
];
const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

createApp(App).use(router).mount("#app");
