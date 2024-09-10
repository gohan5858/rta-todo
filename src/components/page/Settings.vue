<script setup lang="ts">
import KeySettingButton from "@base/KeySettingButton.vue";
import SettingsNavbar from "@layout/SettingsNavbar.vue";
import { ref } from "vue";

const stopTimerKey = ref(["SPACE"]);
</script>

<template>
  <div class="grid grid-cols-1 grid-rows-[1fr_10fr] gap-2">
    <SettingsNavbar />
    <ul class="menu overflow-auto">
      <li>
        <h2 class="menu-title">一般</h2>
        <ul>
          <li>
            <div class="flex flex-row justify-between">
              言語設定
              <select class="select select-ghost select-sm w-32">
                <option value="ja">日本語</option>
                <option value="en">English</option>
              </select>
            </div>
          </li>
          <li>
            <div class="flex flex-row justify-between">
              テーマ
              <select class="select select-ghost select-sm w-32">
                <option>sunset</option>
              </select>
            </div>
          </li>
        </ul>
      </li>
      <li>
        <h2 class="menu-title">通知</h2>
        <ul>
          <li>
            <div class="flex flex-row justify-between">
              締切が近づいたら通知
              <input type="checkbox" class="toggle" checked="true" />
            </div>
          </li>
          <li>
            <div class="flex flex-row justify-between">
              目標ラップタイムを過ぎたら通知
              <input type="checkbox" class="toggle" checked="true" />
            </div>
          </li>
        </ul>
      </li>
      <li>
        <h2 class="menu-title">ショートカット</h2>
        <ul>
          <li>
            <KeySettingButton v-model="stopTimerKey" />
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>
