<script setup lang="ts">
import { addTodo, fetchProject, goToNextTodo, Todo } from "@/bindings";
import RTATimer from "@base/RTATimer.vue";
import TodoList from "@layout/TodoList.vue";
import { nextTick, Ref, ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const projectId = route.params.projectId as string;

const project = await fetchProject(projectId);

const title = ref(project.title);

const [uncheckedTodoList, checkedTodoList] = project.todoList.reduce(
  ([unchecked, checked], todo) => {
    if (todo.checked) {
      checked.value.push(todo);
    } else {
      unchecked.value.push(todo);
    }
    return [unchecked, checked];
  },
  [ref([]), ref([])] as [Ref<Todo[]>, Ref<Todo[]>],
);

const rtaTimer = ref<InstanceType<typeof RTATimer> | null>();
const todoListArea = ref<HTMLElement>();

const addTodoItem = async () => {
  [uncheckedTodoList.value, checkedTodoList.value] = await addTodo(
    projectId,
    "新しいタスク",
  );

  // DOMの更新を待ってからスクロールする
  await nextTick();
  todoListArea.value?.scrollTo({
    top: todoListArea.value.scrollHeight,
    behavior: "smooth",
  });
};
const removeTodoItem = () => {
  // NOTE: 全てがチェック可能なtodoがない = 全てのtodoが完了済みなので削除しない
  if (uncheckedTodoList.value.every((todo) => !todo.checkable)) return;
  uncheckedTodoList.value.pop();
};
const goToNextTask = async (_index: number, _checked: boolean) => {
  const rtaTimer = ref<InstanceType<typeof RTATimer> | null>();
  [uncheckedTodoList.value, checkedTodoList.value] = await goToNextTodo(
    projectId,
    rtaTimer?.value?.getElapsedTime() ?? 0,
  );
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
          @click="addTodoItem"
        >
          +
        </div>
        <div
          class="btn flex-grow bg-cyan-400 text-xl text-black hover:bg-slate-500"
          @click="removeTodoItem"
        >
          -
        </div>
      </div>
    </div>
    <RTATimer ref="rtaTimer" class="bg-base-300 p-2" />
  </div>
</template>
