<template>
  <div v-if="!data"></div>
  <div v-else>
    <div class="q-pa-md row" style="max-width: 800px">
      <q-list padding bordered class="rounded-borders">
        <q-expansion-item
          dense
          dense-toggle
          expand-separator
          icon="assessment"
          label="Common Circuit Data"
        >
          <q-card>
            <q-card-section>
              <h6 class="props">Statistics:</h6>
              <property-chips :data="circuit?.common" />
              <h6 class="props">Config:</h6>
              <property-chips :data="circuit?.common.config" />
              <h6 class="props">Fri Config:</h6>
              <property-chips :data="circuit?.common.config.fri_config" />
              <property-chips :data="circuit?.common.fri_params" />
              <h6 class="props">k:</h6>
              <array-chips :data="circuit?.common.k_is" />
              <h6 class="props">gates:</h6>
              <array-chips
                :data="circuit?.common.gates.map((_) => JSON.stringify(_))"
              />
              <div class="q-pa-md row">
                <q-list class="rounded-borders">
                  <q-expansion-item
                    v-for="(gate, i) in circuit?.common.gates"
                    dense
                    popup
                    expand-separator
                    :label="typeof gate == 'string' ? gate : gate.type"
                    :key="i"
                  >
                    <q-card>
                      <q-card-section v-if="gate == 'PublicInputGate'">
                        A gate whose first four wires will be equal to a hash of
                        public inputs.
                        <br />
                        <property-chips
                          :data="{
                            wires: 4,
                            constants: 0,
                            degree: 1,
                            constraints: 4,
                            wires_public_inputs_hash: '0..4',
                          }"
                        />
                      </q-card-section>
                      <q-card-section
                        v-else-if="
                          typeof gate !== 'string' &&
                          gate.type == 'ConstantGate'
                        "
                      >
                        A gate which takes a single constant parameter and
                        outputs that value.
                        <br />
                        <property-chips :data="gate" />
                        <property-chips
                          :data="{
                            wires: gate.num_consts,
                            constants: gate.num_consts,
                            degree: 1,
                            constraints: gate.num_consts,
                          }"
                        />
                      </q-card-section>
                      <q-card-section v-else>
                        {{ gate }}
                      </q-card-section>
                    </q-card>
                  </q-expansion-item>
                </q-list>
              </div>
              <h6 class="props">selectors:</h6>
              <property-chips :data="circuit?.common.selectors_info.groups" />
              <array-chips
                :data="circuit?.common.selectors_info.selector_indices"
              />
            </q-card-section>
          </q-card>
        </q-expansion-item>

        <q-expansion-item
          dense
          dense-toggle
          expand-separator
          icon="fact_check"
          label="Prover Only Circuit Data"
        >
          <q-card>
            <q-card-section>
              <digest-chip
                name="Circuit Digest"
                :data="circuit?.prover_only.circuit_digest"
              ></digest-chip>
              <!-- <property-chips
                :data="{
                  generators:
                  circuit?.prover_only.generators.length
                }"
              /> -->
              <array-property-chips :data="circuit?.prover_only" />

              other fields like constants_sigma_commitments, fft_root_table,
              generator_indices_by_watches, sigmas ignored for now.
            </q-card-section>
          </q-card>
        </q-expansion-item>

        <q-expansion-item
          dense
          dense-toggle
          expand-separator
          icon="query_stats"
          label="Verifier Only Circuit Data"
        >
          <q-card>
            <q-card-section>
              <digest-chip
                name="Circuit Digest"
                :data="circuit?.verifier_only.circuit_digest"
              ></digest-chip>
              <property-chips
                :data="{
                  constants_sigmas_cap:
                    circuit?.verifier_only.constants_sigmas_cap[1].length,
                }"
              />
            </q-card-section>
          </q-card>
        </q-expansion-item>

        <q-expansion-item
          dense
          dense-toggle
          expand-separator
          icon="fingerprint"
          label="Proof"
          v-if="proof?.proof"
        >
          <!-- header-class="text-green" -->
          <q-card>
            <q-card-section>
              <h6 class="props">Public Inputs:</h6>
              <array-chips :data="proof?.public_inputs" />

              <h6 class="props">Proof:</h6>
              <property-chips
                :data="{
                  plonk_zs_partial_products_cap:
                    proof?.proof.plonk_zs_partial_products_cap[1].length,
                  quotient_polys_cap: proof?.proof.quotient_polys_cap[1].length,
                  wires_cap: proof?.proof.wires_cap[1].length,
                }"
              />

              <h6 class="props">Opening Proof:</h6>
              <property-chips :data="proof?.proof.opening_proof" />
              Final Poly:
              <array-chips
                :data="proof?.proof.opening_proof.final_poly.coeffs"
              />

              <h6 class="props">Query round proofs</h6>
              <!-- <array-chips :data="proof?.proof.opening_proof.query_round_proofs" /> -->
              <div class="q-pa-md row">
                <q-list class="rounded-borders">
                  <q-expansion-item
                    v-for="(rproof, i) in proof?.proof.opening_proof
                      .query_round_proofs"
                    dense
                    popup
                    expand-separator
                    :label="`${i}: Proof`"
                    :key="i"
                  >
                    <q-card>
                      <q-card-section>
                        <property-chips
                          :data="{
                            steps: rproof.steps.length,
                          }"
                        />
                        <h6 class="props">
                          Initial Tree Proof
                          <span class="comment">
                            Evaluations and Merkle proofs of the original set of
                            polynomials, before they are combined into a
                            composition polynomial.</span
                          >
                        </h6>
                        <template
                          v-for="(ep, j) in rproof.initial_trees_proof
                            .evals_proofs"
                          :key="j"
                        >
                          {{
                            initial_trees_proof_evals_proofs_names[j]
                          }}
                          leafs: [{{ ep[0].length }}]
                          <array-chips :data="ep[0]" />
                          <!-- <h6 class="props">Siblings</h6> -->
                          Siblings:<br />
                          <digest-chip
                            v-for="(sib, k) in ep[1].siblings"
                            :name="k.toString()"
                            :data="sib"
                            :key="k"
                          ></digest-chip>
                          <hr />
                        </template>
                      </q-card-section>
                    </q-card>
                  </q-expansion-item>
                </q-list>
              </div>

              <h6 class="props">Opening:</h6>

              <array-property-chips :data="proof?.proof.openings" />
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
      >
        <template v-slot:body-cell-index="props">
          <q-td>
            {{ props.value }}
          </q-td>
        </template>
        <template v-slot:body-cell="props">
          <q-td :props="props">
            <!-- {{ props.value }} -->
            <q-badge
              v-if="props.value?.value"
              :label="
                props.value.value == 'None'
                  ? props.value.representative_value != 'None'
                    ? props.value.representative_value[1]
                    : '-'
                  : props.value.value[1]
              "
              :outline="
                props.value.raw_index == props.value.representative_map
                  ? false
                  : true
              "
              :color="props.value.value == 'None' ? 'grey' : 'primary'"
              class="ellipsis"
            >
              <q-tooltip :delay="showTooltip ? 0 : 100000">
                value: {{ props.value.value }}
                <br />
                index: {{ props.value.raw_index }}
                <br />
                representative map: {{ props.value.representative_map }}
                <br />
                rmap row: {{ props.value.row }}
                <br />
                rmap col: {{ props.value.col }}
              </q-tooltip>
            </q-badge>
          </q-td>
        </template>
        <template v-slot:header-cell="props">
          <q-th :props="props">
            {{ props.col.label }}
          </q-th>
        </template>
      </q-table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Ref, ref, watch } from 'vue';
import { QTableColumn } from 'quasar';
import {
  RowFieldWithPosition,
  CircuitData,
  Plonky2Data,
  Plonky2Proof,
  Witness,
} from 'src/services/plonky2/DefaultModels';
import PropertyChips from './PropertyChips.vue';
import ArrayChips from './ArrayChips.vue';
import DigestChip from './DigestChip.vue';
import ArrayPropertyChips from './ArrayPropertyChips.vue';
import { getColumns, getRows } from 'src/services/plonky2/WitnessVisualization';

export interface Plonky2VisualizationProps {
  data?: Plonky2Data;
}
const props = withDefaults(defineProps<Plonky2VisualizationProps>(), {
  data: undefined,
});

const MAXROWS = ref(1024);

const pagination = ref({
  page: 1,
  rowsPerPage: MAXROWS.value,
});
const columns: Ref<QTableColumn[]> = ref([]);

const showTooltip = ref(true);

const rows: Ref<Record<string, RowFieldWithPosition>[]> = ref([]);

const witness: Ref<Witness | undefined> = ref(undefined);
const circuit: Ref<CircuitData | undefined> = ref(undefined);
const proof: Ref<Plonky2Proof | undefined> = ref(undefined);

const initial_trees_proof_evals_proofs_names = [
  'constants_sigmas',
  'wires',
  'zs_partial',
  'quotient',
];

function loadData(data?: Plonky2Data) {
  if (!data) {
    console.warn('empty data');
    return;
  }
  console.log(data);
  witness.value = data.witness;
  circuit.value = data.data;
  proof.value = data.proof;
  columns.value = getColumns(parseInt(data.witness.num_wires));
  rows.value = getRows(data.witness);
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
h6.props {
  font-size: 1em;
  line-height: 1.25em;
  margin-block-start: 1.25em;
  margin-block-end: 0.5rem;
  border-bottom: 1px solid darkgrey;

  span.comment {
    margin-left: 1em;
    font-size: 0.75rem;
    font-weight: 100;
  }
}

.ellipsis {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 40px;
}
</style>
