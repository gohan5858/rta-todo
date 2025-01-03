<script setup lang="ts">
import { commands } from "@/bindings";
import { useIntervalFn } from "@vueuse/core";
import { computed, onMounted, onUnmounted, ref } from "vue";

onMounted(async () => {
  elapsedTime.value = await commands.initiateTimer(props.projectId);
  previousTime.value = elapsedTime.value;
});

onUnmounted(async () => {
  await commands.updateCurrentElapsedTime(props.projectId, elapsedTime.value);
  await commands.resetCurrentElapsedTime();
});

const props = defineProps<{
  projectId: string;
}>();

// ストップウォッチの経過時間（ミリ秒）を保持する変数
const elapsedTime = ref(0);
const previousTime = ref(0);
// ストップウォッチが動作中かどうかを管理する変数
const isRunning = ref(false);

// ストップウォッチを動かす関数
const { pause, resume } = useIntervalFn(
  async () => {
    const currentTime = await commands.getCurrentTime();
    elapsedTime.value += currentTime - previousTime.value;
    previousTime.value = currentTime;
  },
  1,
  { immediate: false },
);

const currentElapsedTimeUpdater = useIntervalFn(
  async () => {
    await commands.updateCurrentElapsedTime(props.projectId, elapsedTime.value);
  },
  1000,
  { immediate: false },
);

// ストップウォッチを開始する関数
const start = async () => {
  if (!isRunning.value) {
    await commands.resumeTimer();
    resume();
    currentElapsedTimeUpdater.resume();
    isRunning.value = true;
  }
};
// ストップウォッチを停止する関数
const stop = async () => {
  if (isRunning.value) {
    await commands.pauseTimer();
    pause();
    currentElapsedTimeUpdater.pause();
    isRunning.value = false;
  }
};
// 経過時間をフォーマットする関数
const formattedTime = computed((): string => {
  return new Date(elapsedTime.value).toISOString().substring(11, 22);
});

const getElapsedTime = () => {
  return elapsedTime.value;
};

defineExpose({
  getElapsedTime,
});
</script>

<template>
  <div class="flex flex-col items-center justify-between">
    <div class="text-4xl">{{ formattedTime }}</div>
    <div class="flex w-80 flex-row items-center justify-center gap-3">
      <button
        class="btn btn-primary btn-md w-full"
        @click="
          if (isRunning) {
            stop();
          } else {
            start();
          }
        "
      >
        Start/Stop
      </button>
    </div>
  </div>
</template>
