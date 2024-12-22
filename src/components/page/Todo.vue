<script setup lang="ts">
import { commands } from "@/bindings";
import RTATimer from "@base/RTATimer.vue";
import TodoList from "@layout/TodoList.vue";
import TodoNavbar from "@layout/TodoNavbar.vue";
import { nextTick, ref, watch } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const projectId = route.params.projectId as string;

const project = ref(await commands.fetchProject(projectId));

const title = ref(project.value.title);

watch(title, async (newTitle) => {
  await commands.setTitle(projectId, newTitle);
});

const rtaTimer = ref<InstanceType<typeof RTATimer> | null>();
const todoListArea = ref<HTMLElement>();

const addTodoItem = async () => {
  project.value.todoList = await commands.addTodo(projectId);

  // DOMの更新を待ってからスクロールする
  await nextTick();
  todoListArea.value?.scrollTo({
    top: todoListArea.value.scrollHeight,
    behavior: "smooth",
  });
};
const removeTodoItem = async () => {
  project.value.todoList = await commands.removeTodo(projectId);
};
const goToNextTodo = async (parentId: string | null) => {
  project.value.todoList = await commands.goToNextTodo(projectId, parentId);
};
</script>

<template>
  <div class="grid grid-cols-1 grid-rows-[1fr_7fr_2fr] gap-2">
    <TodoNavbar :project-id="project.id" v-model="title" />
    <div ref="todoListArea" class="flex flex-col gap-5 overflow-auto p-2">
      <TodoList
        :todo-list="project.todoList"
        @check-todo="async (parentId) =>  await goToNextTodo(parentId)"
        @update-todo-title="
          async (todoList) => {
            await commands.updateTodoItemTitle(projectId, todoList);
          }
        "
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
