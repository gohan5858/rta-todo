<script setup lang="ts">
import { commands, Todo } from "@/bindings";
import RTATimer from "@base/RTATimer.vue";
import TodoList from "@layout/TodoList.vue";
import TodoNavbar from "@layout/TodoNavbar.vue";
import { nextTick, Ref, ref, watch } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const projectId = route.params.projectId as string;

const project = await commands.fetchProject(projectId);

const title = ref(project.title);

watch(title, async (newTitle) => {
  console.log("title changed", newTitle);
  await commands.setTitle(projectId, newTitle);
});

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

watch(
  checkedTodoList,
  async (newCheckedTodoList) => {
    await commands.updateTodoItemTitle(
      projectId,
      newCheckedTodoList,
      uncheckedTodoList.value,
    );
  },
  { deep: true },
);
watch(
  uncheckedTodoList,
  async (newUncheckedTodoList) => {
    await commands.updateTodoItemTitle(
      projectId,
      checkedTodoList.value,
      newUncheckedTodoList,
    );
  },
  { deep: true },
);

const rtaTimer = ref<InstanceType<typeof RTATimer> | null>();
const todoListArea = ref<HTMLElement>();

const addTodoItem = async () => {
  [uncheckedTodoList.value, checkedTodoList.value] = await commands.addTodo(
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
const removeTodoItem = async () => {
  uncheckedTodoList.value = await commands.removeTodo(projectId);
};
const goToNextTask = async (_index: number, _checked: boolean) => {
  [uncheckedTodoList.value, checkedTodoList.value] =
    await commands.goToNextTodo(
      projectId,
      rtaTimer?.value?.getElapsedTime() ?? 0,
    );
};
</script>

<template>
  <div class="grid grid-cols-1 grid-rows-[1fr_7fr_2fr] gap-2">
    <TodoNavbar v-model="title" />
    <div ref="todoListArea" class="flex flex-col gap-5 overflow-auto p-2">
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
    <RTATimer :projectId="projectId" ref="rtaTimer" class="p-2" />
  </div>
</template>
