<script setup lang="ts">
import ComingSoonBoard from "@base/ComingSoonBoard.vue";
import { PhCaretDoubleLeft, PhList } from "@phosphor-icons/vue";
const title = defineModel<string>({ required: true });
</script>

<template>
  <div class="navbar">
    <div class="navbar-start">
      <button class="btn btn-ghost" @click="$router.back()">
        <PhCaretDoubleLeft :size="20" />
      </button>
    </div>
    <input
      type="text"
      placeholder="プロジェクト名を入力"
      class="input input-ghost w-full text-center"
      v-model="title"
    />
    <div class="navbar-end">
      <div class="dropdown dropdown-end">
        <div tabindex="0" role="button" class="btn btn-ghost m-1">
          <PhList :size="24" />
        </div>
        <ul
          tabindex="0"
          class="menu dropdown-content z-[1] w-52 rounded-box bg-base-200 p-2 shadow"
        >
          <ComingSoonBoard />
        </ul>
      </div>
    </div>
  </div>
</template>
