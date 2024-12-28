<script setup lang="ts">
import { commands } from "@/bindings";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, Update } from "@tauri-apps/plugin-updater";
import { onMounted, ref } from "vue";

const updateNotificationDialog = ref<HTMLDialogElement | null>(null);

const isCheckedUpdate = await commands.getIsCheckedUpdate();
let update: Update | null = null;

await new Promise((resolve) => setTimeout(resolve, 3000));

// 再起動するまでアップデートチェックは行わない
if (!isCheckedUpdate) {
  update = await check();
}

onMounted(() => {
  if (update?.available) {
    updateNotificationDialog.value?.showModal();
  }
});
</script>

<template>
  <dialog class="modal" ref="updateNotificationDialog">
    <div class="modal-box">
      <h3 class="text-lg font-bold">Update Available</h3>
      <h4 class="text-md font-bold">
        v{{ update?.currentVersion }} → v{{ update?.version }}
      </h4>
      <p class="py-4">
        新しいバージョンが公開されています。<br />
        アップデートを行いますか？
      </p>
      <div class="flex flex-row justify-between">
        <button
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
          class="btn btn-warning"
          @click="
            async () => {
              await update?.downloadAndInstall();
              await commands.setIsCheckedUpdate(true);
              await relaunch();
            }
          "
        >
          今すぐ更新
        </button>
      </div>
    </div>
  </dialog>
</template>
