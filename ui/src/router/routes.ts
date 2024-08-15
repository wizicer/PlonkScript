import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        name: 'halo2analyzer',
        path: '/analyzer/halo2',
        component: () => import('pages/AnalyzerPage.vue'),
      },
      {
        name: 'plonky2analyzer',
        path: '/analyzer/plonky2',
        component: () => import('pages/Plonky2AnalyzerPage.vue'),
      },
      {
        name: 'plonky3analyzer',
        path: '/analyzer/plonky3',
        component: () => import('pages/Plonky3AnalyzerPage.vue'),
      },
      {
        name: 'editor',
        path: '/editor',
        component: () => import('pages/EditorPage.vue'),
      },
      { path: '/', redirect: '/editor' },
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
