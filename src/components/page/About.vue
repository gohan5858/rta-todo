<script setup lang="ts">
import AboutNavbar from "@layout/AboutNavbar.vue";
import { app } from "@tauri-apps/api";

const appInfo = {
  name: await app.getName(),
  version: await app.getVersion(),
};
</script>
<template>
  <div class="grid grid-cols-1 grid-rows-[1fr_5fr_4fr] gap-2">
    <AboutNavbar />
    <div class="overflow-auto p-5">
      <div class="grid grid-cols-1 gap-2">
        <div class="flex flex-row justify-between">
          <span>アプリケーション名</span>
          <span>{{ appInfo.name }}</span>
        </div>
        <div class="flex flex-row justify-between">
          <span>バージョン</span>
          <span>{{ appInfo.version }}</span>
        </div>
      </div>
    </div>
    <div class="flex flex-col justify-end gap-1">
      <button class="btn btn-primary">アップデートを確認</button>
    </div>
  </div>
</template>
