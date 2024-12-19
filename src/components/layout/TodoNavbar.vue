<script setup lang="ts">
import { commands } from "@/bindings";
import { PhCaretDoubleLeft, PhList, PhTrashSimple } from "@phosphor-icons/vue";
const props = defineProps<{
  projectId: string;
}>();
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
          class="menu dropdown-content z-[1] w-52 rounded-box p-2 backdrop-blur-sm"
        >
          <li>
            <button
              class="btn btn-ghost flex flex-row gap-2"
              @click="
                async () => {
                  await commands.removeProject(props.projectId);
                  $router.back();
                }
              "
            >
              <PhTrashSimple />
              <span>プロジェクトを削除</span>
            </button>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
