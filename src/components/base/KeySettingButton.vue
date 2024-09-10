<script setup lang="ts">
import { os } from "@tauri-apps/api";
import { OsType } from "@tauri-apps/api/os";
import { useMagicKeys } from "@vueuse/core";
import { onMounted, ref } from "vue";

const props = withDefaults(
  defineProps<{
    title: string;
    /** キー組み合わせの最大数 */
    maxKeys?: number;
  }>(),
  {
    maxKeys: 3,
  },
);

const assignedKeys = defineModel<string[]>({ required: true });

const currentKeys = ref<string[]>([]);
const keyPopup = ref<HTMLDialogElement | null>(null);

const osName = ref<OsType>("Darwin");

onMounted(async () => {
  osName.value = await os.type();
});

const { current } = useMagicKeys({
  onEventFired(e: KeyboardEvent) {
    // keyPopupが表示されていない場合はキー入力を受け付けない
    if (!keyPopup.value?.open || Array.from(current).length > props.maxKeys)
      return;

    if (e.type === "keydown") {
      currentKeys.value = Array.from(current).map((key) => {
        if (key === "meta") {
          switch (osName.value) {
            case "Darwin":
              return "⌘";
            case "Windows_NT":
              return "WIN";
            case "Linux":
              return "SUPER";
          }
        } else if (key === " ") {
          return "SPACE";
        } else {
          return key.toUpperCase();
        }
      });
    } else {
      // metaキー, Ctrl, shift, alt, space 他キーの順番でソート
      assignedKeys.value = currentKeys.value.sort((a, b) => {
        if (a === "⌘") return -1;
        if (b === "⌘") return 1;
        if (a === "Win") return -1;
        if (b === "Win") return 1;
        if (a === "Super") return -1;
        if (b === "Super") return 1;
        if (a === "CTRL") return -1;
        if (b === "CTRL") return 1;
        if (a === "SHIFT") return -1;
        if (b === "SHIFT") return 1;
        if (a === "SPACE") return -1;
        if (b === "SPACE") return 1;
        if (a === "ALT") return -1;
        if (b === "ALT") return 1;
        if (a === "SPACE") return -1;
        if (b === "SPACE") return 1;
        return a.localeCompare(b);
      });
      currentKeys.value = [];
      keyPopup.value?.close();
    }
  },
});
</script>

<template>
  <div
    class="flex flex-row justify-between"
    @click="
      () => {
        keyPopup?.showModal();
      }
    "
  >
    {{ props.title }}
    <div class="flex flex-row items-center gap-2">
      <kbd class="kbd" v-for="key in assignedKeys" :key="key">{{ key }}</kbd>
    </div>

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
