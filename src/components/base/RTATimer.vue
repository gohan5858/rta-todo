<script setup lang="ts">
import { useIntervalFn } from "@vueuse/core";
import { computed, onMounted, ref } from "vue";

// ストップウォッチの経過時間（ミリ秒）を保持する変数
const elapsedTime = ref(0);
// ストップウォッチが動作中かどうかを管理する変数
const isRunning = ref(false);

// ストップウォッチを動かす関数
const { pause, resume } = useIntervalFn(
  () => {
    elapsedTime.value += 10; // 10ミリ秒ごとに更新
  },
  10,
  { immediate: false },
);

// ストップウォッチを開始する関数
const start = () => {
  if (!isRunning.value) {
    resume();
    isRunning.value = true;
  }
};
// ストップウォッチを停止する関数
const stop = () => {
  if (isRunning.value) {
    pause();
    isRunning.value = false;
  }
};
// ストップウォッチをリセットする関数
const reset = () => {
  pause();
  elapsedTime.value = 0;
  isRunning.value = false;
};

// 経過時間をフォーマットする関数
const formattedTime = computed((): string => {
  return new Date(elapsedTime.value).toISOString().substring(11, 22);
});

onMounted(() => {
  start();
});
</script>

<template>
  <div class="flex flex-col items-center justify-between">
    <div class="text-4xl">{{ formattedTime }}</div>
    <div class="flex flex-row items-center justify-center gap-3">
      <button class="btn btn-primary btn-md" @click="start">Start</button>
      <button class="btn btn-secondary btn-md" @click="stop">Pause</button>
      <button class="btn btn-warning btn-md" @click="reset">Reset</button>
    </div>
  </div>
</template>
