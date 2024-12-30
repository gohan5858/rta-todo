<script setup lang="ts">
import UpdateDialog from "@base/UpdateDialog.vue";
import AboutNavbar from "@layout/AboutNavbar.vue";
import { app } from "@tauri-apps/api";
import { onMounted, ref } from "vue";

const appInfo = ref({ name: "?", version: "?" });
const updateDialog = ref<InstanceType<typeof UpdateDialog> | null>(null);
const notUpdateAvailableDialog = ref<HTMLDialogElement | null>(null);
const isCheckingUpdate = ref(false);

onMounted(async () => {
  appInfo.value.name = await app.getName();
  appInfo.value.version = await app.getVersion();
});
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
      <button
        @click="
          async () => {
            isCheckingUpdate = true;
            const isUpdateAble = await updateDialog?.checkUpdate();
            isCheckingUpdate = false;
            if (!isUpdateAble) {
              notUpdateAvailableDialog?.showModal();
            }
          }
        "
        class="btn btn-primary"
      >
        アップデートを確認
      </button>
    </div>
    <UpdateDialog ref="updateDialog" />
    <dialog class="modal" ref="notUpdateAvailableDialog">
      <div class="modal-box">
        <p class="py-4">入手可能な更新はありません。</p>
        <div class="flex flex-row justify-end">
          <button
            class="btn btn-warning"
            @click="
              async () => {
                notUpdateAvailableDialog?.close();
              }
            "
          >
            閉じる
          </button>
        </div>
      </div>
    </dialog>
    <div
      v-if="isCheckingUpdate"
      class="absolute left-0 top-0 flex h-full w-full items-center justify-center backdrop-blur-sm"
    >
      <span class="loading loading-dots loading-md" />
    </div>
  </div>
</template>
