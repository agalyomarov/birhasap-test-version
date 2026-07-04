<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

interface Todo {
   id: number;
   description: string;
   status: string;
}

const desc = ref("");
const list = ref<Todo[]>([]);
async function addTodo() {
   await invoke("add_todo", {
      description: desc.value,
   });
   desc.value = "";
}

async function getTodos() {
   list.value = await invoke("get_todos");
}

async function updateTodo(todo: Todo) {
   return await invoke("update_todo", { todo });
}

async function deleteTodo(id: number) {
   return await invoke("delete_todo", { id });
}
</script>
<template>
   <div>
      <textarea
         name=""
         id=""
         v-model="desc"></textarea>
      <button @click="addTodo()">Add</button>
      <br />
      <p>{{ list }}</p>
      <button @click="getTodos()">Get</button>
   </div>
</template>

<style scoped></style>
