<script setup lang="ts">
import { commands, SaveData } from "@/bindings";
import KeySettingButton from "@base/KeySettingButton.vue";
import SettingsNavbar from "@layout/SettingsNavbar.vue";
import { ref } from "vue";

const data = ref<SaveData>(await commands.loadData());
const isAutoStart = ref(data.value.isAutoStart);
const isNotificationOfDeadline = ref(data.value.isNotificationOfDeadline);
const isNotificationExceededGoalLapTime = ref(
  data.value.isNotificationExceededGoalLapTime,
);
const darkMode = ref(data.value.theme === "sunset");
const playPauseKey = ref(["SPACE"]);
const resetKey = ref(["R"]);
const nextTaskKey = ref(["N"]);
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
              自動起動
              <input
                type="checkbox"
                v-model="isAutoStart"
                class="toggle"
                @change="
                  async () => {
                    await commands
                      .setIsAutoStart(isAutoStart)
                      .catch(console.error);
                    data = await commands.loadData();
                  }
                "
              />
            </div>
          </li>
          <li>
            <div class="flex flex-row justify-between">
              ダークモード
              <input
                type="checkbox"
                v-model="darkMode"
                class="toggle"
                @change="
                  async () => {
                    const theme = darkMode ? 'sunset' : 'nord';
                    await commands.setTheme(theme).catch(console.error);
                    data = await commands.loadData();
                  }
                "
              />
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
              <input
                v-model="isNotificationOfDeadline"
                type="checkbox"
                class="toggle"
                checked="true"
                @change="
                  async () => {
                    data.isNotificationOfDeadline = isNotificationOfDeadline;
                    await commands
                      .setIsNotificationOfDeadline(isNotificationOfDeadline)
                      .catch(console.error);
                    data = await commands.loadData();
                  }
                "
              />
            </div>
          </li>
          <li>
            <div class="flex flex-row justify-between">
              目標ラップタイムを過ぎたら通知
              <input
                v-model="isNotificationExceededGoalLapTime"
                type="checkbox"
                class="toggle"
                checked="true"
                @change="
                  async () => {
                    data.isNotificationExceededGoalLapTime =
                      isNotificationExceededGoalLapTime;
                    await commands
                      .setIsNotificationExceededGoalLapTime(
                        isNotificationExceededGoalLapTime,
                      )
                      .catch(console.error);
                    data = await commands.loadData();
                  }
                "
              />
            </div>
          </li>
        </ul>
      </li>
      <li>
        <h2 class="menu-title">ショートカット</h2>
        <ul>
          <li>
            <KeySettingButton title="再生停止" v-model="playPauseKey" />
            <KeySettingButton title="リセット" v-model="resetKey" />
            <KeySettingButton title="次のタスクへ進む" v-model="nextTaskKey" />
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>
