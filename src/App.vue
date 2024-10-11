<script lang="ts" setup>
import { loadData } from "@/bindings";
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();

const theme = ref<string | null>(null);

onMounted(async () => {
  const update_theme = async () => {
    theme.value = (await loadData()).theme;
    document.body.setAttribute("data-theme", theme.value!);
  };
  update_theme();
  listen("update-setting", update_theme);
});
</script>

<template>
  <div
    :data-theme="theme"
    class="h-screen select-none overflow-hidden p-5 font-mono"
  >
    <RouterView
      :key="route.fullPath"
      class="h-full w-full"
      v-slot="{ Component }"
    >
      <Suspense timeout="0">
        <!-- NOTE: divで囲むことでSuspense配下に複数のルートノードが来ることを防ぐ
          (<Suspense> slots expect a single root node.対策) -->
        <div class="h-full w-full">
          <component :is="Component" />
        </div>

        <template #fallback>
          <div class="flex h-full w-full items-center justify-center">
            <span class="loading loading-dots loading-lg" />
          </div>
        </template>
      </Suspense>
    </RouterView>
  </div>
</template>
