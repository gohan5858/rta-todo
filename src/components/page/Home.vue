<script setup lang="ts">
import { commands, events, SaveData } from "@/bindings";
import HomeNavbar from "@layout/HomeNavbar.vue";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, Update } from "@tauri-apps/plugin-updater";
import { computed, onMounted, ref } from "vue";

const saveData = ref<SaveData>(await commands.loadData());
const displayCompleted = ref(false);

const projects = computed(() =>
  saveData.value?.projects
    .reverse()
    .filter((project) =>
      displayCompleted.value ? project.completed : !project.completed,
    ),
);
const newTodoPopup = ref<HTMLDialogElement | null>(null);
const alertPopup = ref<HTMLDialogElement | null>(null);

const updateNotificationDialog = ref<HTMLDialogElement | null>(null);

const isCheckedUpdate = localStorage.getItem("isCheckedUpdate") ?? "false";
let update: Update | null = null;

// 再起動するまでアップデートチェックは行わない
if (isCheckedUpdate !== "true") {
  update = await check();
  localStorage.setItem("isCheckedUpdate", "true");
}

onMounted(() => {
  events.windowClose.listen(() => {
    localStorage.removeItem("isCheckedUpdate");
  });
  if (update?.available) {
    updateNotificationDialog.value?.showModal();
  }
});

const title = ref("");
const now = new Date();
const deadline_date = ref(now.toISOString().split("T")[0]);
const deadline_time = ref(now.toTimeString().split(":").slice(0, 2).join(":"));
</script>

<template>
  <HomeNavbar v-model="displayCompleted">
    <div class="flex flex-grow flex-col gap-5">
      <!-- TODO list -->
      <div class="relative flex-grow">
        <div
          class="absolute flex h-full w-full flex-col items-center overflow-auto"
        >
          <table class="table w-5/6">
            <tbody>
              <tr v-for="project in projects" class="hover w-full">
                <th>
                  <input
                    type="checkbox"
                    class="checkbox"
                    @input="
                      async (e) => {
                        await commands.setIsCompleteProject(
                          project.id,
                          !project.completed,
                        );
                        // チェックボックスの非チェック状態にする
                        if (e.target) {
                          (e.target as HTMLInputElement).checked = false;
                        }
                        saveData = await commands.loadData();
                      }
                    "
                  />
                </th>
                <td class="text-xl">
                  <RouterLink
                    class="flex"
                    :key="project.id"
                    :to="{
                      name: 'todo',
                      params: { projectId: project.id },
                    }"
                    >{{ project.title }}
                  </RouterLink>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Add new TODO button -->
      <div class="absolute bottom-5 right-10">
        <button
          @click="
            () => {
              const now = new Date();
              deadline_date = now.toISOString().split('T')[0];
              deadline_time = now
                .toTimeString()
                .split(':')
                .slice(0, 2)
                .join(':');
              newTodoPopup?.showModal();
            }
          "
          v-if="!displayCompleted"
          class="btn btn-lg bg-orange-400 text-xl text-black hover:bg-orange-500"
        >
          +
        </button>
      </div>
    </div>
    <dialog class="modal" ref="newTodoPopup">
      <div class="modal-box">
        <h3 class="text-center text-lg font-bold">プロジェクトを作成</h3>
        <div class="flex w-full flex-col items-center gap-3">
          <label class="form-control w-full">
            <div class="label">
              <span class="label-text">タイトル</span>
              <span class="label-text-alt text-red-500">必須</span>
            </div>
            <input
              v-model="title"
              required
              type="text"
              placeholder="〇〇をクリアする, 〇〇を実装する etc..."
              class="input input-bordered w-full"
            />
          </label>
          <label class="form-control w-full">
            <div class="label">
              <span class="label-text">締め切り</span>
              <span class="label-text-alt">任意</span>
            </div>
            <div class="flex flex-row gap-1">
              <input
                v-model="deadline_date"
                type="date"
                class="input input-bordered w-full"
              />
              <input
                v-model="deadline_time"
                type="time"
                class="input input-bordered w-full"
              />
            </div>
          </label>
          <div class="flex w-full flex-row justify-end">
            <button
              class="btn btn-primary"
              @click="
                async () => {
                  if (!title) {
                    alertPopup?.showModal();
                    return;
                  }

                  await commands.addProject(
                    title,
                    deadline_date + ' ' + deadline_time,
                  );
                  title = '';
                  saveData = await commands.loadData();
                  newTodoPopup?.close();
                }
              "
            >
              作成
            </button>
          </div>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button @click="() => (title = '')">close</button>
      </form>
    </dialog>
    <dialog class="modal" ref="alertPopup">
      <div class="modal-box text-center">
        <h3 class="text-center text-lg font-bold text-red-500">入力エラー</h3>
        <p>タイトルを入力してください。</p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>
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
              () => {
                updateNotificationDialog?.close();
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
                await relaunch();
              }
            "
          >
            今すぐ更新
          </button>
        </div>
      </div>
    </dialog>
  </HomeNavbar>
</template>

<style scoped>
.scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.scrollbar::-webkit-scrollbar {
  display: none;
}
</style>
