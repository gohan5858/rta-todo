<script setup lang="ts">
import {
  getCurrentTime,
  initiateTimer,
  pauseTimer,
  resetCurrentElapsedTime,
  resumeTimer,
  updateCurrentElapsedTime,
} from "@/bindings";
import { useIntervalFn } from "@vueuse/core";
import { computed, onMounted, onUnmounted, ref } from "vue";

onMounted(async () => {
  elapsedTime.value = await initiateTimer(props.projectId);
  previousTime.value = elapsedTime.value;
});

onUnmounted(async () => {
  await updateCurrentElapsedTime(props.projectId, elapsedTime.value);
  await resetCurrentElapsedTime();
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
    const currentTime = await getCurrentTime();
    elapsedTime.value += currentTime - previousTime.value;
    previousTime.value = currentTime;
  },
  1,
  { immediate: false },
);

const currentElapsedTimeUpdater = useIntervalFn(
  async () => {
    await updateCurrentElapsedTime(props.projectId, elapsedTime.value);
  },
  1000,
  { immediate: false },
);

// ストップウォッチを開始する関数
const start = async () => {
  if (!isRunning.value) {
    await resumeTimer();
    resume();
    currentElapsedTimeUpdater.resume();
    isRunning.value = true;
  }
};
// ストップウォッチを停止する関数
const stop = async () => {
  if (isRunning.value) {
    await pauseTimer();
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
    <div class="flex flex-row items-center justify-center gap-3">
      <button class="btn btn-primary btn-md w-full" @click="start">
        Start
      </button>
      <button class="btn btn-secondary btn-md w-full" @click="stop">
        Pause
      </button>
    </div>
  </div>
</template>
