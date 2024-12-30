<script setup lang="ts">
import { commands } from "@/bindings";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, Update } from "@tauri-apps/plugin-updater";
import { onMounted, ref } from "vue";

const updateNotificationDialog = ref<HTMLDialogElement | null>(null);

const isCheckedUpdate = await commands.getIsCheckedUpdate();
let update: Update | null = null;

const currentVersion = ref("?");
const newVersion = ref("?");
const isUpdating = ref(false);

const checkUpdate = async () => {
  update = await check();
  if (update?.available) {
    currentVersion.value = update.currentVersion;
    newVersion.value = update.version;
    updateNotificationDialog.value?.showModal();
    return true;
  }
  return false;
};
defineExpose({ checkUpdate });

onMounted(async () => {
  // 再起動するまでアップデートチェックは行わない
  if (isCheckedUpdate) {
    return;
  }
  await checkUpdate();
});
</script>

<template>
  <dialog class="modal" ref="updateNotificationDialog">
    <div class="modal-box">
      <h3 class="text-lg font-bold">Update Available</h3>
      <h4 class="text-md font-bold">
        v{{ currentVersion }} → v{{ newVersion }}
      </h4>
      <p class="py-4">
        新しいバージョンが公開されています。<br />
        アップデートを行いますか？
      </p>
      <div class="flex flex-row justify-between">
        <button
          :disabled="isUpdating"
          class="btn btn-info"
          @click="
            async () => {
              updateNotificationDialog?.close();
              await commands.setIsCheckedUpdate(true);
            }
          "
        >
          後で
        </button>
        <button
          :disabled="isUpdating"
          class="btn btn-warning"
          @click="
            async () => {
              isUpdating = true;
              await update?.downloadAndInstall();
              await commands.setIsCheckedUpdate(true);
              await relaunch();
              isUpdating = false;
            }
          "
        >
          更新して再起動
        </button>
      </div>
    </div>
    <div
      v-if="isUpdating"
      class="absolute left-0 top-0 flex h-full w-full items-center justify-center backdrop-blur-sm"
    >
      <span class="loading loading-dots loading-md" />
    </div>
  </dialog>
</template>
