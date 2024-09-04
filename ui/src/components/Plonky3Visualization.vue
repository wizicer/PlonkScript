<template>
  <div v-if="!data"></div>
  <div v-else>
    <div class="q-pa-md row">
      <q-list
        padding
        bordered
        class="rounded-borders"
        style="max-width: 1000px"
      >
        <q-expansion-item
          dense
          dense-toggle
          expand-separator
          icon="assessment"
          label="Constaints (Gates)"
          default-opened
        >
          <q-card>
            <q-card-section>
              <q-table
                :rows="exps"
                :columns="[
                  {
                    name: 'gates',
                    label: 'Gates',
                    align: 'left',
                    field: (r) => r,
                  },
                ]"
                flat
                bordered
                dense
                class="full-width"
                :pagination="gatePagination"
                :hide-pagination="(exps?.length ?? 0) <= MAXGATEROWS"
                :hide-header="true"
              >
                <template v-slot:body-cell="props">
                  <q-td :props="props">
                    <span class="gate_hljs" v-html="props.value"></span>
                  </q-td>
                </template>
              </q-table>
            </q-card-section>
            <!-- <q-card-section>
              <q-checkbox
                v-model="showOtherColumns"
                label="Show Other Columns"
              />
            </q-card-section> -->
          </q-card>
        </q-expansion-item>
        <q-expansion-item
          dense
          dense-toggle
          expand-separator
          icon="query_stats"
          label="Public inputs (Instances)"
          default-opened
        >
          <q-card>
            <q-card-section>
              <q-table
                :rows="pubs"
                :columns="[
                  {
                    name: 'name',
                    label: 'Name',
                    align: 'left',
                    field: 'name',
                  },
                  {
                    name: 'value',
                    label: 'Value',
                    align: 'left',
                    field: 'value',
                  },
                ]"
                flat
                bordered
                dense
                class="full-width"
                :pagination="instancePagination"
                :hide-pagination="(pubs?.length ?? 0) <= MAXINSTANCEROWS"
                :hide-header="true"
              >
                <template v-slot:body-cell-name="props">
                  <q-td :props="props">
                    <span v-html="makeSubscript(props.value)"></span>
                  </q-td>
                </template>
              </q-table>
            </q-card-section>
          </q-card>
        </q-expansion-item>
      </q-list>
    </div>

    <div class="q-pa-md row">
      <q-table
        :rows="rows"
        :columns="columns"
        row-key="name"
        flat
        bordered
        dense
        :pagination="pagination"
        :hide-pagination="rows.length <= MAXROWS"
        class="float-left"
        style="max-width: 1000px"
      >
        <template v-slot:body-cell-index="props">
          <q-td>
            {{ props.value }}
          </q-td>
        </template>
        <template v-slot:body-cell="props">
          <q-td :props="props">
            <q-badge
              v-if="props.value"
              :label="props.value.value"
              :color="
                noConstraint(props.value.row, props.value.col)
                  ? 'grey'
                  : 'primary'
              "
              class="ellipsis"
            >
              <q-tooltip
                class="bg-indigo-1 text-black"
                :delay="showTooltip ? 0 : 100000"
              >
                value: {{ props.value.value }}
                <br />
                index: {{ props.value.index }} ({{ props.value.row }},
                {{ props.value.col }})
                <br />
                gates:
                <ul class="tooltip_gate_list">
                  <li
                    v-for="(g, i) in getGates(
                      props.value.row,
                      props.value.col,
                      true
                    )"
                    :key="i"
                  >
                    <span class="gate_hljs" v-html="g"></span>
                  </li>
                </ul>
              </q-tooltip>
            </q-badge>
          </q-td>
        </template>
        <template v-slot:header-cell="props">
          <q-th :props="props">
            <span v-html="makeSubscript(props.col.label)"></span>
          </q-th>
        </template>
      </q-table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Ref, ref, watch } from 'vue';
import { QTableColumn } from 'quasar';
import { Plonky3Data, WholeRow } from 'src/services/plonky3/DefaultModels';
import {
  getColumns,
  getRows,
  stringifySymExp,
} from 'src/services/plonky3/WitnessVisualization';
import { registerGateLanguage } from 'src/services/GateLanguage';
import hljs from 'highlight.js';
registerGateLanguage();

export interface Plonky3VisualizationProps {
  data?: Plonky3Data;
}
const props = withDefaults(defineProps<Plonky3VisualizationProps>(), {
  data: undefined,
});

const MAXROWS = ref(1024);
const MAXGATEROWS = ref(10);
const MAXINSTANCEROWS = ref(10);

const gatePagination = ref({
  page: 1,
  rowsPerPage: MAXGATEROWS.value,
});
const instancePagination = ref({
  page: 1,
  rowsPerPage: MAXINSTANCEROWS.value,
});
const pagination = ref({
  page: 1,
  rowsPerPage: MAXROWS.value,
});
const columns: Ref<QTableColumn[]> = ref([]);

const showTooltip = ref(true);

const rows: Ref<WholeRow[]> = ref([]);

const exps: Ref<string[] | undefined> = ref(undefined);
const pubs: Ref<{ name: string; value: number }[] | undefined> = ref(undefined);
const isTooBig = ref(false);

function loadData(data?: Plonky3Data) {
  if (!data) {
    console.warn('empty data');
    return;
  }
  if (Number(data.trace.values.length / data.trace.width) > 1024) {
    isTooBig.value = true;
    return;
  }
  exps.value = data.symbols.map((s) =>
    makeSubscript(
      hljs.highlight(stringifySymExp(s), { language: 'gate' }).value
    )
  );
  columns.value = getColumns(data.trace.width);
  rows.value = getRows(data.trace.values, data.trace.width);
  pubs.value = data.public.map((_, i) => ({ name: `i_${i}`, value: _ }));
}

function makeSubscript(s: string | undefined) {
  if (!s) return '';
  return s.replace(/_([0-9]+)/g, '<sub>$1</sub>');
}

function noConstraint(row: number, col: number) {
  return getGates(row, col).length == 0;
}

function getGates(row: number, col: number, highlight = false): string[] {
  let clabel: string;
  try {
    clabel = makeSubscript(columns.value[col + 1].label);
  } catch (e) {
    console.warn(col, columns.value, e);
    return [];
  }
  if (!exps.value) return [];
  return exps.value
    .filter(
      (_) =>
        (_.includes(clabel) &&
          !_.includes('IsFirstRow') &&
          !_.includes('IsTransition') &&
          !_.includes('IsLastRow')) ||
        (row == 0 && _.includes(clabel) && _.includes('IsFirstRow')) ||
        (row != rows.value.length - 1 &&
          _.includes(clabel) &&
          _.includes('IsTransition')) ||
        (row == rows.value.length - 1 &&
          _.includes(
            clabel + '</span><span class="hljs-next-rotation">[1]</span>'
          ) &&
          _.includes('IsTransition')) ||
        (row == rows.value.length - 1 &&
          _.includes(clabel) &&
          _.includes('IsLastRow'))
    )
    .map((_) =>
      highlight
        ? _.replaceAll(clabel, `<span class="hljs-current">${clabel}</span>`)
        : _
    );
}

watch(
  () => props.data,
  (newValue, oldValue) => {
    if (newValue == oldValue) return;
    if (!newValue) {
      rows.value = [];
      columns.value = [];
      return;
    }
    loadData(newValue);
  }
);

loadData(props.data);
</script>

<style scoped lang="scss">
.ellipsis {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 40px;
}

.gate_hljs {
  :deep(.hljs-current) {
    color: $light-green-9;
    font-weight: bold;
  }
  :deep(.hljs-fixed) {
    color: $light-blue-7;
  }
  :deep(.hljs-advice) {
    color: $deep-orange-7;
  }
  :deep(.hljs-hex) {
    color: $teal-8;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 6rem;
    display: inline-block;
    vertical-align: bottom;
  }
  :deep(.hljs-prev-rotation) {
    color: $green-14;
  }
  :deep(.hljs-next-rotation) {
    color: $indigo-14;
  }
}

ul.tooltip_gate_list {
  margin: 0;
  padding-inline-start: 0;
  list-style-position: inside;
}
</style>
