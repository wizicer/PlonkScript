<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <!-- <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleLeftDrawer"
        /> -->

        <q-toolbar-title> Plonk Script </q-toolbar-title>

        <q-btn
          stretch
          :flat="$route.name != 'editor'"
          label="Editor"
          :to="{ name: 'editor' }"
          class="menu-button"
          :class="{ active: $route.name === 'editor' }"
        />
        <!-- <q-separator dark vertical inset /> -->

        <q-btn-dropdown
          label="Analyzer"
          stretch
          :flat="
            $route.name != 'plonky3analyzer' && $route.name != 'plonky2analyzer' && $route.name != 'halo2analyzer'
          "
          class="menu-button"
          :class="{
            active:
              $route.name === 'plonky3analyzer' ||
              $route.name === 'plonky2analyzer' ||
              $route.name === 'halo2analyzer',
          }"
        >
          <q-list>
            <q-item clickable v-close-popup :to="{ name: 'halo2analyzer' }">
              <q-item-section>
                <q-item-label>Halo2 Analyzer</q-item-label>
              </q-item-section>
            </q-item>

            <q-item clickable v-close-popup :to="{ name: 'plonky2analyzer' }">
              <q-item-section>
                <q-item-label>Plonky2 Analyzer</q-item-label>
              </q-item-section>
            </q-item>

            <q-item clickable v-close-popup :to="{ name: 'plonky3analyzer' }">
              <q-item-section>
                <q-item-label>Plonky3 Analyzer</q-item-label>
              </q-item-section>
            </q-item>
          </q-list>
        </q-btn-dropdown>

        <!-- <div>Demo Version</div> -->
      </q-toolbar>
    </q-header>

    <!-- <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
    >
      <q-list>
        <q-item-label
          header
        >
          Essential Links
        </q-item-label>

        <EssentialLink
          v-for="link in essentialLinks"
          :key="link.title"
          v-bind="link"
        />
      </q-list>
    </q-drawer> -->

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
//
</script>

<style lang="scss" scoped>
.menu-button.q-btn {
  background-color: $primary;

  &.active {
    background-color: darken($primary, 10);
  }
}
</style>
