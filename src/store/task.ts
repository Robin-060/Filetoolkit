import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { TaskStatus } from "../types/task";

export interface TaskRecord {
  id: string;
  name: string;
  status: TaskStatus;
  progress: number;
  message: string;
  startedAt: number;
  completedAt?: number;
}

export const useTaskStore = defineStore("task", () => {
  const currentTask = ref<TaskRecord | null>(null);
  const taskHistory = ref<TaskRecord[]>([]);

  const progressPercent = computed(() => (currentTask.value ? currentTask.value.progress : 0));

  function startTask(id: string, name: string) {
    const task: TaskRecord = {
      id,
      name,
      status: "running",
      progress: 0,
      message: "",
      startedAt: Date.now(),
    };
    currentTask.value = task;
  }

  function updateProgress(current: number, total: number, message?: string) {
    if (!currentTask.value) return;
    if (total > 0) {
      currentTask.value.progress = Math.round((current / total) * 100);
    }
    if (message !== undefined) {
      currentTask.value.message = message;
    }
  }

  function completeTask(id: string) {
    if (currentTask.value && currentTask.value.id === id) {
      currentTask.value.status = "done";
      currentTask.value.progress = 100;
      currentTask.value.completedAt = Date.now();
      taskHistory.value.unshift({ ...currentTask.value });
      currentTask.value = null;
    }
  }

  function failTask(id: string, message: string) {
    if (currentTask.value && currentTask.value.id === id) {
      currentTask.value.status = "error";
      currentTask.value.message = message;
      currentTask.value.completedAt = Date.now();
      taskHistory.value.unshift({ ...currentTask.value });
      currentTask.value = null;
    }
  }

  function cancelTask() {
    if (currentTask.value) {
      currentTask.value.status = "cancelled";
      currentTask.value.completedAt = Date.now();
      taskHistory.value.unshift({ ...currentTask.value });
      currentTask.value = null;
    }
  }

  return {
    currentTask,
    taskHistory,
    progressPercent,
    startTask,
    updateProgress,
    completeTask,
    failTask,
    cancelTask,
  };
});
