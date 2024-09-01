<script setup lang="ts">
import { loadData, SaveData } from "@/bindings";
import HomeNavbar from "@layout/HomeNavbar.vue";
import { computed, onMounted, ref } from "vue";

const saveData = ref<SaveData | null>(null);
const displayCompleted = ref(false);

const todoLists = computed(() =>
  saveData.value?.todoLists
    .reverse()
    .filter((todoList) =>
      displayCompleted.value ? todoList.completed : !todoList.completed,
    ),
);

onMounted(async () => {
  saveData.value = await loadData();
});
</script>

<template>
  <div class="grid grid-cols-1 grid-rows-[1fr_10fr_1fr] gap-2">
    <HomeNavbar v-model="displayCompleted" />
    <div class="flex flex-col gap-5 overflow-auto">
      <div class="flex flex-col gap-2">
        <RouterLink
          class="flex"
          v-for="todoList in todoLists"
          :key="todoList.id"
          :to="{
            name: 'todo',
            params: { id: todoList.id },
          }"
        >
          <div class="btn flex-grow text-xl">
            {{ todoList.title }}
          </div>
        </RouterLink>
      </div>
    </div>
    <div class="flex flex-row items-center justify-center">
      <button
        v-if="!displayCompleted"
        class="btn bg-orange-400 text-xl text-black hover:bg-orange-500"
      >
        +
      </button>
    </div>
  </div>
</template>
