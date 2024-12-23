<script setup lang="ts">
import { commands, Todo, TodoList } from "@/bindings";
import RTATimer from "@base/RTATimer.vue";
import TodoListContainer from "@layout/TodoListContainer.vue";
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

const flattenTodoList = (todoList: TodoList): TodoList => {
  const flattenTodos = (todos: Todo[]): Todo[] => {
    return todos.reduce((acc: Todo[], todo: Todo) => {
      const { subTodoList, ...rest } = todo;
      acc.push({
        ...rest,
        subTodoList: { checked_todos: [], unchecked_todos: [] },
      });
      acc.push(...flattenTodos(subTodoList.checked_todos));
      acc.push(...flattenTodos(subTodoList.unchecked_todos));
      return acc;
    }, []);
  };

  return {
    checked_todos: flattenTodos(todoList.checked_todos),
    unchecked_todos: flattenTodos(todoList.unchecked_todos),
  };
};
</script>

<template>
  <div class="grid grid-cols-1 grid-rows-[1fr_7fr_2fr] gap-2">
    <TodoNavbar :project-id="project.id" v-model="title" />
    <div ref="todoListArea" class="flex flex-col gap-5 overflow-auto p-2">
      <TodoListContainer
        :max-nest-level="1"
        :todo-list="project.todoList"
        @check-todo="async (parentId) => await goToNextTodo(parentId)"
        @update-todo-list="
          async (todoList) => {
            // 子タスクをフラット化する
            todoList.unchecked_todos = todoList.unchecked_todos.map((todo) => {
              todo.subTodoList = flattenTodoList(todo.subTodoList);
              return todo;
            });
            todoList.checked_todos = todoList.checked_todos.map((todo) => {
              todo.subTodoList = flattenTodoList(todo.subTodoList);
              return todo;
            });

            await commands.updateTodoList(projectId, todoList);
            project.todoList = todoList;
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
