<template>
  <div v-if="!data"></div>
  <div v-else>
    <div class="q-pa-md">
      <q-checkbox v-model="showTooltip" label="Show Tooltip" />

      <h6>Region Table</h6>
      <table>
        <tr v-for="({ color: c, hits }, k) in regions" :key="k">
          <td :style="`border: 1px solid ${c};`">
            {{ k }}
          </td>
          <td>
            {{ hits }}
          </td>
        </tr>
      </table>

      <h6>Gate Table</h6>
      <q-table
        :rows="gatesArray"
        flat
        bordered
        dense
        :pagination="pagination"
        :hide-pagination="true"
        row-key="name"
        selection="multiple"
        v-model:selected="selectedGates"
      >
        <template v-slot:body-cell-expressions="props">
          <q-td :props="props">
            <p v-for="(g, i) in gates[props.value]" :key="i">
              <q-badge
                v-if="g.name"
                color="cyan-7"
                :label="g.name"
                class="q-mr-sm"
              ></q-badge>
              <span class="gate_hljs" v-html="g.literal"></span>
            </p>
          </q-td>
        </template>
      </q-table>

      <h6>Lookup Table</h6>
      <q-table
        :rows="lookups"
        flat
        bordered
        dense
        :pagination="pagination"
        :hide-pagination="true"
      >
        <template v-slot:body-cell="props">
          <q-td :props="props">
            <p v-for="(g, i) in props.value" :key="i">
              <span class="gate_hljs" v-html="g"></span>
            </p>
          </q-td>
        </template>
      </q-table>
      <q-checkbox v-model="showOtherColumns" label="Show Other Columns" />
    </div>
    <div class="q-pa-md row">
      <q-table
        :rows="rows"
        :columns="filteredColumns"
        row-key="name"
        flat
        bordered
        dense
        :pagination="pagination"
        :hide-pagination="rows.length <= MAXROWS"
      >
        <template v-slot:body-cell-index="props">
          <q-td>
            {{ props.value.index }}
          </q-td>
        </template>
        <template v-slot:body-cell="props">
          <q-td
            :props="props"
            :class="[
              'bg-' + getColorByColName(props.col.name),
              getBorderOfRegion(props.value, props.col),
              highlightedCellsMap[getCellId(props.col.name, props.rowIndex)]
                ? 'highlighted-cell'
                : '',
            ]"
            :style="'border-color: ' + rmapcolor[props.value.region] + ';'"
            @mouseenter="highlightRelatedCells(props.col.name, props.rowIndex)"
            @mouseleave="clearHighlightedCells()"
          >
            <template
              v-for="(v, i) in Array.isArray(props.value.value)
                ? props.value.value
                : [props.value.value]"
              :key="i"
            >
              <q-badge
                :color="getColorByType(props.value.type, v)"
                :label="props.value.type == 'Selector' ? '' : v"
                :ref="
                (el) => {
                  const elel = (el as any)?.$el;
                  if (!elel) return;
                  const col = props.col.name;
                  if (!cellBadges[col]) cellBadges[col] = {};
                  cellBadges[col][props.value.index] = elel;
                }
              "
              >
                <q-tooltip :delay="showTooltip ? 0 : 100000">
                  {{
                    `${
                      props.value.region
                        ? `Region: ${props.value.region}\n`
                        : ''
                    }Raw: ${props.value.raw}`
                  }}
                </q-tooltip>
              </q-badge>
            </template>
          </q-td>
        </template>
        <template v-slot:header-cell="props">
          <q-th
            :props="props"
            :class="'bg-' + getColorByColName(props.col.name)"
          >
            <span v-html="props.col.label"></span>
          </q-th>
        </template>
      </q-table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Ref, computed, ref, watch } from 'vue';
import { QTableColumn } from 'quasar';
import {
  RowFieldType,
  getColumnDefinition,
  getColumns,
  MockProverData,
  getRowsAndRegions,
  RowFieldWithPosition,
  GateLiteralExpression,
  RegionInfoEntity,
  LookupLiteralExpression,
  RowsAndRegionsResponse,
  ColumnType,
  formularize,
} from 'src/services/ConstraintSystem';
import { registerGateLanguage } from 'src/services/GateLanguage';
import hljs from 'highlight.js';
registerGateLanguage();

export interface ConstraintsVisualizationProps {
  data?: MockProverData;
}
const props = withDefaults(defineProps<ConstraintsVisualizationProps>(), {
  data: undefined,
});

function getColorByColName(col: string): string {
  col = col.slice(0, col.indexOf('-'));
  return col == 'instance'
    ? 'grey'
    : col == 'advice'
    ? 'deep-orange-3'
    : col == 'fixed'
    ? 'light-blue-2'
    : col == 'selector'
    ? 'indigo-2'
    : col == 'gate'
    ? 'cyan-2'
    : col == 'inde'
    ? 'bluegrey-2'
    : 'negative';
}

function getColorByType(type: RowFieldType, value = ''): string {
  return type == 'Unassigned'
    ? 'grey'
    : type == 'Assigned'
    ? 'teal'
    : type == 'Instance'
    ? 'darkslategray'
    : type == 'Poison'
    ? 'purple'
    : type == 'Selector'
    ? value == 'true'
      ? 'green'
      : 'indigo-3'
    : type == 'Gates'
    ? 'cyan-8'
    : 'negative';
}

const MAXROWS = ref(1024);

const pagination = ref({
  page: 1,
  rowsPerPage: MAXROWS.value,
});
const columns: Ref<QTableColumn[]> = ref([]);

const showTooltip = ref(false);

const rrr: Ref<RowsAndRegionsResponse | undefined> = ref(undefined);
const rows: Ref<Record<string, RowFieldWithPosition>[]> = ref([]);
const rmap: Ref<Record<number, Record<string, string>>> = ref({});
const rmapcolor: Ref<Record<string, string>> = ref({});
const regions: Ref<RegionInfoEntity> = ref({});

const gates: Ref<Record<string, GateLiteralExpression[]>> = ref({});
// QTable value don't accept array of field value
const gatesArray: Ref<Array<{ name: string; expressions: string }>> = ref([]);
const selectedGates: Ref<Array<{ name: string; expressions: string }>> = ref(
  []
);
const selectedColumns = computed(() => {
  const v = rrr.value;
  if (!v) return [];

  return Object.keys(v.gateColumns)
    .map((_) => ({ key: _, list: v.gateColumns[_] }))
    .filter((_) => selectedGates.value.some((g) => g.name == _.key))
    .map((_) => _.list)
    .flat()
    .filter((value, index, self) => self.indexOf(value) === index);
});
const filteredColumns = computed(() => {
  const rv = rrr.value;
  if (!columns.value || !rv) return [];
  return columns.value.filter(
    (_) =>
      _.name == 'index' ||
      _.name == 'gates' ||
      selectedColumns.value.includes(_.name) ||
      (showOtherColumns.value && otherColumns.value.includes(_.name)) ||
      selectedColumns.value.map((_) => rv.selectorMaps[_]).includes(_.name)
  );
});
const lookups: Ref<Array<LookupLiteralExpression>> = ref([]);
const otherColumns: Ref<Array<string>> = ref([]);
const showOtherColumns = ref(false);

function getBorderOfRegion(
  row: RowFieldWithPosition,
  col: QTableColumn
): string {
  let border = '';
  const y = row.index;
  const x = col.name;
  const name = row.region;
  if (!name) return '';
  if (rmap.value[y - 1] && rmap.value[y - 1][x] == name)
    border += ' no_border_top';
  else border += ' cell_border_top';
  if (rmap.value[y + 1] && rmap.value[y + 1][x] == name)
    border += ' no_border_bottom';
  else border += ' cell_border_bottom';
  const colidx = columns.value.findIndex((_) => _.name == x);
  const left = columns.value[colidx - 1]?.name;
  if (left && rmap.value[y][left] == name) border += ' no_border_left';
  else border += ' cell_border_left';
  const right = columns.value[colidx + 1]?.name;
  if (right && rmap.value[y][right] == name) border += ' no_border_right';
  else border += ' cell_border_right';

  // console.log(border, columns.value, rmap.value, row, col);
  return border;
}

const cellBadges = ref<Record<string, Record<string, Element>>>({});
const highlightedCellsMap = ref<Record<string, boolean>>({});
const permutationMap = ref<Record<string, string[]>>({});

function getCellId(colName: string, rowIndex: number): string {
  return `${colName}-${rowIndex}`;
}

function highlightRelatedCells(colName: string, rowIndex: number) {
  const cellId = getCellId(colName, rowIndex);
  const relatedCells = permutationMap.value[cellId] || [];

  if (relatedCells.length > 0) {
    const newMap: Record<string, boolean> = {};
    newMap[cellId] = true;
    relatedCells.forEach((cell) => (newMap[cell] = true));
    highlightedCellsMap.value = newMap;
  }
}

function clearHighlightedCells() {
  // if empty object, do nothing
  if (Object.keys(highlightedCellsMap.value).length == 0) return;
  highlightedCellsMap.value = {};
}

function buildPermutationMap(data: MockProverData) {
  if (rows.value.length > 1024) {
    console.warn(
      `rows is too many [${rows.value.length}], skip building permutation map`
    );
    return;
  }

  const mapping = data.permutation.mapping;
  // ignore mapping when it's large
  if (mapping.reduce((pv, cv) => pv + cv.length, 0) > 500) return;
  const cols = data.permutation.columns;
  permutationMap.value = {};

  for (let c = 0; c < mapping.length; c++) {
    const mcol = mapping[c];
    for (let r = 0; r < mcol.length; r++) {
      const mrow = mcol[r];
      const col = Number(mrow[0]);
      const row = Number(mrow[1]);

      // from pointed address(col, row) to current cell(c, r)
      const tocolname = getColumnName(cols[c]);
      const fromcolname = getColumnName(cols[col]);
      if (fromcolname == tocolname && row == r) continue;

      const fromCellId = getCellId(fromcolname, row);
      const toCellId = getCellId(tocolname, r);

      // Build bidirectional mapping
      if (!permutationMap.value[fromCellId]) {
        permutationMap.value[fromCellId] = [];
      }
      if (!permutationMap.value[toCellId]) {
        permutationMap.value[toCellId] = [];
      }

      permutationMap.value[fromCellId].push(toCellId);
      permutationMap.value[toCellId].push(fromCellId);
    }
  }
}

function getColumnName(col: ColumnType): string {
  return `${col.column_type.toLowerCase()}-${col.index}`;
}

function loadData(data?: MockProverData) {
  if (!data) {
    console.warn('empty data');
    return;
  }

  rows.value = [];
  columns.value = [];
  gatesArray.value = [];
  selectedGates.value = [];
  lookups.value = [];
  showOtherColumns.value = false;
  highlightedCellsMap.value = {};
  permutationMap.value = {};

  setTimeout(() => {
    const cols = getColumnDefinition(data);
    const colsdata = getColumns(cols);
    columns.value = colsdata;
    const colorList = ['red', 'blue', 'wheat', 'green'];
    const rr = getRowsAndRegions(data, cols, colorList);
    rrr.value = rr;
    rows.value = rr.rows;
    rmap.value = rr.rmap;
    rmapcolor.value = rr.rmapcolor;
    regions.value = rr.regions;
    gates.value = rr.gates;
    lookups.value = rr.lookups.map((l) => ({
      input_expressions: l.input_expressions.map((_) =>
        formularize(hljs.highlight(_, { language: 'gate' }).value)
      ),
      table_expressions: l.table_expressions.map((_) =>
        formularize(hljs.highlight(_, { language: 'gate' }).value)
      ),
    }));
    Object.keys(gates.value).forEach(function (key) {
      gates.value[key] = gates.value[key].map((g) => ({
        name: g.name,
        literal: formularize(
          hljs.highlight(g.literal, { language: 'gate' }).value
        ),
        idx: g.idx,
      }));
    });

    gatesArray.value = Object.keys(rr.gates).map((_) => ({
      name: _,
      expressions: _,
    }));
    selectedGates.value = gatesArray.value;
    showOtherColumns.value = true;
    const gateColumnNames = Object.keys(rr.gateColumns)
      .map((_) => ({ key: _, list: rr.gateColumns[_] }))
      .filter((_) => gatesArray.value.some((g) => g.name == _.key))
      .map((_) => _.list)
      .flat()
      .filter((value, index, self) => self.indexOf(value) === index);

    otherColumns.value = columns.value
      .filter(
        (_) =>
          _.name != 'index' &&
          _.name != 'gates' &&
          !_.name.startsWith('selector') &&
          !gateColumnNames.includes(_.name)
      )
      .map((_) => _.name);

    buildPermutationMap(data);
  }, 100);
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
$pos: left, right, top, bottom;

@each $p in $pos {
  .no_border_#{$p} {
    border-#{$p}-width: 0 !important;
  }

  .cell_border_#{$p} {
    border-#{$p}-width: 1px;
  }
}

.highlighted-cell {
  position: relative;
  box-shadow: inset 0 0 10px 2px rgba(255, 255, 0, 0.7) !important;
  z-index: 2;
}

.gate_hljs {
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

h6 {
  margin-block-end: 0.5rem;
  border-bottom: 1px solid darkgrey;
}
</style>
