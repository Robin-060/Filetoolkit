import { createRouter, createWebHashHistory } from "vue-router";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("../pages/HomePage.vue"),
    },
    {
      path: "/image",
      name: "image",
      component: () => import("../pages/ImagePage.vue"),
    },
    {
      path: "/pdf",
      name: "pdf",
      component: () => import("../pages/PdfPage.vue"),
    },
    {
      path: "/rename",
      name: "rename",
      component: () => import("../pages/RenamePage.vue"),
    },
    {
      path: "/dedup",
      name: "dedup",
      component: () => import("../pages/DedupPage.vue"),
    },
  ],
});

export default router;
