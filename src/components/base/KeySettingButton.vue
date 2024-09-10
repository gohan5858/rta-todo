<script setup lang="ts">
import { useMagicKeys } from "@vueuse/core";
import { ref } from "vue";

const assignedKeys = defineModel<string[]>({ required: true });
const currentKeys = ref<string[]>([]);
const keyPopup = ref<HTMLDialogElement | null>(null);

const { current } = useMagicKeys({
  onEventFired(e: KeyboardEvent) {
    // keyPopupが表示されていない場合はキー入力を受け付けない
    if (!keyPopup.value?.open) return;

    if (e.type === "keydown") {
      currentKeys.value = Array.from(current).map((key) => {
        if (key === "meta") {
          return "⌘";
        } else {
          return key.toUpperCase();
        }
      });
    } else {
      assignedKeys.value = currentKeys.value;
      keyPopup.value?.close();
    }
  },
});
</script>

<template>
  <div class="flex flex-row justify-between">
    タイマー停止
    <button
      class="btn btn-neutral"
      @click="
        () => {
          keyPopup?.showModal();
        }
      "
    >
      <kbd class="kbd" v-for="key in assignedKeys" :key="key">{{ key }}</kbd>
    </button>
    <dialog class="modal" ref="keyPopup">
      <div class="modal-box">
        <h3 class="text-center text-lg font-bold">任意の組み合わせを入力</h3>
        <p class="flex flex-row justify-center gap-3 py-4">
          <kbd class="kbd" v-for="key in currentKeys" :key="key">{{ key }}</kbd>
        </p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>
  </div>
</template>
