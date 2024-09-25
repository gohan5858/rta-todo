<script setup lang="ts">
import { loadData, SaveData } from "@/bindings";
import HomeNavbar from "@layout/HomeNavbar.vue";
import { computed, ref } from "vue";

const saveData = ref<SaveData>(await loadData());
const displayCompleted = ref(false);

const projects = computed(() =>
  saveData.value?.projects
    .reverse()
    .filter((project) =>
      displayCompleted.value ? project.completed : !project.completed,
    ),
);
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
                params: { id: project.id },
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
          v-if="!displayCompleted"
          class="btn bg-orange-400 text-xl text-black hover:bg-orange-500"
        >
          +
        </button>
      </div>
    </div>
  </HomeNavbar>
</template>
