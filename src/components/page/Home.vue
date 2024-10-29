<script setup lang="ts">
import { commands, SaveData } from "@/bindings";
import HomeNavbar from "@layout/HomeNavbar.vue";
import { computed, ref } from "vue";

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

const title = "";
const now = new Date();
const deadline_date = ref(now.toISOString().split("T")[0]);
const deadline_time = ref(now.toTimeString().split(":").slice(0, 2).join(":"));
</script>

<template>
  <HomeNavbar v-model="displayCompleted">
    <div class="flex flex-grow flex-col gap-5">
      <!-- TODO list -->
      <div class="relative flex-grow">
        <div class="absolute flex h-full w-full flex-col overflow-auto">
          <div class="flex flex-col gap-2">
            <RouterLink
              class="flex"
              v-for="project in projects"
              :key="project.id"
              :to="{
                name: 'todo',
                params: { projectId: project.id },
              }"
            >
              <div class="btn flex-grow text-xl">
                {{ project.title }}
              </div>
            </RouterLink>
          </div>
        </div>
      </div>
      <!-- Add new TODO button -->
      <div class="flex flex-row items-center justify-center">
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
          class="btn bg-orange-400 text-xl text-black hover:bg-orange-500"
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
        <button>close</button>
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
  </HomeNavbar>
</template>
