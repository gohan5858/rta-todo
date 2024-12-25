import About from "@page/About.vue";
import ComingSoon from "@page/ComingSoon.vue";
import Home from "@page/Home.vue";
import Settings from "@page/Settings.vue";
import Todo from "@page/Todo.vue";
import { createApp } from "vue";
import { createRouter, createWebHashHistory } from "vue-router";
import App from "./App.vue";
import "./assets/global.css";

const routes = [
  { path: "/", name: "home", component: Home },
  { path: "/settings", name: "settings", component: Settings },
  { path: "/about", name: "about", component: About },
  { path: "/todo/:projectId", name: "todo", component: Todo },
  {
    path: "/coming_soon",
    name: "coming_soon",
    component: ComingSoon,
  },
];
const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

createApp(App).use(router).mount("#app");
