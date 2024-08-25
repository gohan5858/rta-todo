<script setup lang="ts">
import RTATimer from "@base/RTATimer.vue";
import TodoList from "@layout/TodoList.vue";
import { nextTick, ref } from "vue";

const title = ref("タイトル");
const todoList = ref<
  {
    title: string;
    lapTime: string;
    checkable: boolean;
    branchName?: string;
  }[]
>([]);
const rtaTimer = ref<InstanceType<typeof RTATimer> | null>();
const todoListArea = ref<HTMLElement>();

const addTodo = async () => {
  todoList.value?.push({
    title: "タスク名",
    lapTime: "--:--:--",
    checkable:
      todoList.value.length === 0 ||
      todoList.value?.every((todo) => !todo.checkable),
  });

  // DOMの更新を待ってからスクロールする
  await nextTick();
  todoListArea.value?.scrollTo({
    top: todoListArea.value.scrollHeight,
    behavior: "smooth",
  });
};
const removeTodo = () => {
  // NOTE: 全てがチェック可能なtodoがない = 全てのtodoが完了済みなので削除しない
  if (todoList.value.every((todo) => !todo.checkable)) return;

  todoList.value.pop();
};
const goToNextTask = (index: number) => {
  todoList.value[index].lapTime = rtaTimer?.value?.formattedTime || "--:--:--";
  todoList.value[index].checkable = false;
  todoList.value[index + 1] && (todoList.value[index + 1].checkable = true);
};
</script>

<template>
  <div class="grid grid-cols-1 grid-rows-[1fr_7fr_2fr] gap-2">
    <input
      type="text"
      placeholder="Type here"
      class="input input-bordered input-ghost w-full bg-base-300 text-center"
      :value="title"
    />
    <div
      ref="todoListArea"
      class="flex flex-col gap-5 overflow-auto bg-base-300 p-2"
    >
      <TodoList
        @checked-todo="(index) => goToNextTask(index)"
        :todo-list="todoList"
      />
      <div class="flex flex-row">
        <div
          class="btn flex-grow bg-orange-400 text-xl text-black hover:bg-orange-500"
          @click="addTodo"
        >
          +
        </div>
        <div
          class="btn flex-grow bg-cyan-400 text-xl text-black hover:bg-slate-500"
          @click="removeTodo"
        >
          -
        </div>
      </div>
    </div>
    <RTATimer ref="rtaTimer" class="bg-base-300 p-2" />
  </div>
</template>
