<script setup lang="ts">
import { TodoItem } from "@/todoItem";
import RTATimer from "@base/RTATimer.vue";
import TodoList from "@layout/TodoList.vue";
import { nextTick, ref } from "vue";

const title = ref("タイトル");
const checkedTodoList = ref<TodoItem[]>([]);
const uncheckedTodoList = ref<TodoItem[]>([]);

const rtaTimer = ref<InstanceType<typeof RTATimer> | null>();
const todoListArea = ref<HTMLElement>();

const addTodo = async () => {
  uncheckedTodoList.value?.push({
    title: "タスク名",
    lapTime: undefined,
    elapsedTime: undefined,
    checked: false,
    checkable:
      uncheckedTodoList.value.length === 0 ||
      uncheckedTodoList.value?.every((todo) => !todo.checkable),
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
  if (uncheckedTodoList.value.every((todo) => !todo.checkable)) return;
  uncheckedTodoList.value.pop();
};
const goToNextTask = (index: number, checked: boolean) => {
  uncheckedTodoList.value[index].lapTime = rtaTimer?.value?.getElapsedTime();

  uncheckedTodoList.value[index].elapsedTime = Math.round(
    ((uncheckedTodoList.value[index]?.lapTime?.getTime() || 0) -
      (uncheckedTodoList.value[index - 1]?.lapTime?.getTime() || 0)) /
      (1000 * 60),
  );

  uncheckedTodoList.value[index].checked = checked;
  uncheckedTodoList.value[index].checkable = false;
  uncheckedTodoList.value[index + 1] &&
    (uncheckedTodoList.value[index + 1].checkable = true);

  checkedTodoList.value.push(uncheckedTodoList.value[index]);
  uncheckedTodoList.value.splice(index, 1);
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
        @checked-todo="(index, checked) => goToNextTask(index, checked)"
        v-model:checked-todo-list="checkedTodoList"
        v-model:unchecked-todo-list="uncheckedTodoList"
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
