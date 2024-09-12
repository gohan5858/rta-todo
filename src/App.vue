<script lang="ts" setup>
import { loadData } from "@/bindings";
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();

const theme = ref<string | null>(null);

onMounted(async () => {
  listen("update-setting", async (_event) => {
    theme.value = (await loadData()).theme;
  });
});
</script>

<template>
  <div
    :data-theme="theme"
    class="h-screen select-none overflow-hidden p-5 font-mono"
  >
    <RouterView :key="route.fullPath" class="h-full w-full" />
  </div>
</template>
