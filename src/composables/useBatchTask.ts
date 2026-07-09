import { ref, onUnmounted, readonly } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useTaskStore } from "../store/task";
import type { TaskStatus } from "../types/task";

/**
 * 批量任务执行抽象。
 * 封装了 "invoke 后端命令 → 监听 task-progress 事件 → 更新 Pinia store" 的标准流程。
 *
 * 用法:
 *   const { run, cancel, progress, status } = useBatchTask("compress_images");
 *   const results = await run("compress_images", { files, quality: 80 });
 *
 * 返回值:
 *   run(invokeCmd, args)  — 执行任务,返回后端响应
 *   cancel()              — 取消当前任务(更新 store 状态)
 *   progress / status     — 只读响应式引用,可在模板中直接使用
 */

export interface TaskProgress {
  progress: number; // 0-100
  status: TaskStatus;
  message: string;
}

/** A-2 worker 推送的进度事件载荷 */
interface ProgressPayload {
  current: number;
  total: number;
  message?: string;
}

export function useBatchTask(taskName: string) {
  const store = useTaskStore();

  const progress = ref(0);
  const status = ref<TaskStatus>("idle");
  const message = ref("");

  let unlisten: UnlistenFn | null = null;

  /** 生成简短的唯一任务 ID */
  function generateTaskId(): string {
    return `${taskName}-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
  }

  /** 执行后端命令,自动管理进度和状态 */
  async function run<T>(invokeCmd: string, args: Record<string, unknown> = {}): Promise<T> {
    const taskId = generateTaskId();

    // 1. 启动任务(store + 本地状态)
    progress.value = 0;
    status.value = "running";
    message.value = "";
    store.startTask(taskId, taskName);

    // 2. 监听后端进度事件
    try {
      unlisten = await listen<ProgressPayload>("task-progress", (event) => {
        const { current, total } = event.payload;
        const msg = event.payload.message ?? "";
        progress.value = total > 0 ? Math.round((current / total) * 100) : 0;
        message.value = msg;
        store.updateProgress(current, total, msg);
      });
    } catch {
      // 非 Tauri 环境(浏览器 dev 模式)时 listen 不可用,忽略
    }

    // 3. 调用后端命令
    try {
      const result = await invoke<T>(invokeCmd, args);

      // 4. 成功
      progress.value = 100;
      status.value = "done";
      store.completeTask(taskId);

      return result;
    } catch (err) {
      // 5. 失败
      const errMsg = err instanceof Error ? err.message : String(err);

      // 区分用户取消 vs 真实错误
      if (errMsg.includes("cancelled") || errMsg.includes("取消")) {
        status.value = "cancelled";
        message.value = "任务已取消";
        store.cancelTask();
      } else {
        status.value = "error";
        message.value = errMsg;
        store.failTask(taskId, errMsg);
      }

      throw err;
    } finally {
      // 6. 解绑事件监听(无论成功/失败/取消)
      cleanup();
    }
  }

  /** 取消当前任务 */
  function cancel() {
    if (status.value === "running") {
      store.cancelTask();
      status.value = "cancelled";
      message.value = "正在取消...";
    }
  }

  /** 清理事件监听器 */
  function cleanup() {
    if (unlisten) {
      unlisten();
      unlisten = null;
    }
  }

  // 组件卸载时自动清理,防止内存泄漏
  onUnmounted(cleanup);

  return {
    run,
    cancel,
    progress: readonly(progress),
    status: readonly(status),
    message: readonly(message),
  };
}
